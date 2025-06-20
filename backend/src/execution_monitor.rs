use chrono::{DateTime, Utc};
use git2::Repository;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use uuid::Uuid;

use crate::models::{
    task::{Task, TaskStatus},
    task_attempt::{TaskAttempt, TaskAttemptStatus},
    task_attempt_activity::{CreateTaskAttemptActivity, TaskAttemptActivity},
};

#[derive(Debug)]
pub enum ExecutionType {
    SetupScript,
    CodingAgent,
    DevServer,
}

#[derive(Debug)]
pub struct RunningExecution {
    pub task_attempt_id: Uuid,
    pub execution_type: ExecutionType,
    pub child: tokio::process::Child,
    pub started_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    running_executions: Arc<Mutex<HashMap<Uuid, RunningExecution>>>,
    pub db_pool: sqlx::SqlitePool,
    config: Arc<tokio::sync::RwLock<crate::models::config::Config>>,
}

impl AppState {
    pub fn new(
        db_pool: sqlx::SqlitePool,
        config: Arc<tokio::sync::RwLock<crate::models::config::Config>>,
    ) -> Self {
        Self {
            running_executions: Arc::new(Mutex::new(HashMap::new())),
            db_pool,
            config,
        }
    }

    // Running executions getters
    pub async fn has_running_execution(&self, attempt_id: Uuid) -> bool {
        let executions = self.running_executions.lock().await;
        executions
            .values()
            .any(|exec| exec.task_attempt_id == attempt_id)
    }

    pub async fn get_running_execution_ids(&self) -> Vec<Uuid> {
        let executions = self.running_executions.lock().await;
        executions.keys().copied().collect()
    }

    pub async fn get_running_executions_for_monitor(&self) -> Vec<(Uuid, Uuid, bool, Option<i32>)> {
        let mut executions = self.running_executions.lock().await;
        let mut completed_executions = Vec::new();

        for (execution_id, running_exec) in executions.iter_mut() {
            match running_exec.child.try_wait() {
                Ok(Some(status)) => {
                    let success = status.success();
                    let exit_code = status.code();
                    completed_executions.push((
                        *execution_id,
                        running_exec.task_attempt_id,
                        success,
                        exit_code,
                    ));
                }
                Ok(None) => {
                    // Still running
                }
                Err(e) => {
                    tracing::error!("Error checking process status: {}", e);
                    completed_executions.push((
                        *execution_id,
                        running_exec.task_attempt_id,
                        false,
                        None,
                    ));
                }
            }
        }

        // Remove completed executions from the map
        for (execution_id, _, _, _) in &completed_executions {
            executions.remove(execution_id);
        }

        completed_executions
    }

    // Running executions setters
    pub async fn add_running_execution(&self, execution_id: Uuid, execution: RunningExecution) {
        let mut executions = self.running_executions.lock().await;
        executions.insert(execution_id, execution);
    }

    pub async fn remove_running_execution(&self, execution_id: Uuid) -> Option<RunningExecution> {
        let mut executions = self.running_executions.lock().await;
        executions.remove(&execution_id)
    }

    pub async fn stop_running_execution(
        &self,
        attempt_id: Uuid,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let mut executions = self.running_executions.lock().await;
        let mut execution_id_to_remove = None;
        let mut stopped = false;

        // Find the execution for this attempt
        for (exec_id, execution) in executions.iter_mut() {
            if execution.task_attempt_id == attempt_id {
                // Kill the process
                match execution.child.kill().await {
                    Ok(_) => {
                        stopped = true;
                        execution_id_to_remove = Some(*exec_id);
                        tracing::info!("Stopped execution for task attempt {}", attempt_id);
                        break;
                    }
                    Err(e) => {
                        tracing::error!("Failed to kill process for attempt {}: {}", attempt_id, e);
                        return Err(Box::new(e));
                    }
                }
            }
        }

        // Remove the stopped execution from the map
        if let Some(exec_id) = execution_id_to_remove {
            executions.remove(&exec_id);
        }

        Ok(stopped)
    }

    // Config getters
    pub async fn get_sound_alerts_enabled(&self) -> bool {
        let config = self.config.read().await;
        config.sound_alerts
    }

    pub async fn get_config(&self) -> crate::models::config::Config {
        let config = self.config.read().await;
        config.clone()
    }

    // Config setters
    pub async fn update_config<F>(&self, update_fn: F)
    where
        F: FnOnce(&mut crate::models::config::Config),
    {
        let mut config = self.config.write().await;
        update_fn(&mut config);
    }
}

/// Commit any unstaged changes in the worktree after execution completion
async fn commit_execution_changes(
    worktree_path: &str,
    attempt_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Run git operations in a blocking task since git2 is synchronous
    let worktree_path = worktree_path.to_string();
    tokio::task::spawn_blocking(move || {
        let worktree_repo = Repository::open(&worktree_path)?;

        // Check if there are any changes to commit
        let status = worktree_repo.statuses(None)?;
        let has_changes = status.iter().any(|entry| {
            let flags = entry.status();
            flags.contains(git2::Status::INDEX_NEW)
                || flags.contains(git2::Status::INDEX_MODIFIED)
                || flags.contains(git2::Status::INDEX_DELETED)
                || flags.contains(git2::Status::WT_NEW)
                || flags.contains(git2::Status::WT_MODIFIED)
                || flags.contains(git2::Status::WT_DELETED)
        });

        if !has_changes {
            return Ok::<(), Box<dyn std::error::Error + Send + Sync>>(());
        }

        // Get the current signature for commits
        let signature = worktree_repo.signature()?;

        // Get the current HEAD commit
        let head = worktree_repo.head()?;
        let parent_commit = head.peel_to_commit()?;

        // Stage all changes
        let mut worktree_index = worktree_repo.index()?;
        worktree_index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        worktree_index.write()?;

        let tree_id = worktree_index.write_tree()?;
        let tree = worktree_repo.find_tree(tree_id)?;

        // Create commit for the changes
        let commit_message = format!("Task attempt {} - Final changes", attempt_id);
        worktree_repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &commit_message,
            &tree,
            &[&parent_commit],
        )?;

        Ok(())
    })
    .await??;

    Ok(())
}

/// Play a system sound notification
async fn play_sound_notification() {
    // Use platform-specific sound notification
    if cfg!(target_os = "macos") {
        let _ = tokio::process::Command::new("afplay")
            .arg("/System/Library/Sounds/Glass.aiff")
            .spawn();
    } else if cfg!(target_os = "linux") {
        // Try different Linux notification sounds
        if let Ok(_) = tokio::process::Command::new("paplay")
            .arg("/usr/share/sounds/alsa/Front_Left.wav")
            .spawn()
        {
            // Success with paplay
        } else if let Ok(_) = tokio::process::Command::new("aplay")
            .arg("/usr/share/sounds/alsa/Front_Left.wav")
            .spawn()
        {
            // Success with aplay
        } else {
            // Try system bell as fallback
            let _ = tokio::process::Command::new("echo")
                .arg("-e")
                .arg("\\a")
                .spawn();
        }
    } else if cfg!(target_os = "windows") {
        let _ = tokio::process::Command::new("powershell")
            .arg("-c")
            .arg("[console]::beep(800, 300)")
            .spawn();
    }
}

pub async fn execution_monitor(app_state: AppState) {
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(5));

    loop {
        interval.tick().await;

        // Check for orphaned task attempts with latest activity status = ExecutorRunning but no running execution
        let executor_running_attempt_ids =
            match TaskAttemptActivity::find_attempts_with_latest_executor_running_status(
                &app_state.db_pool,
            )
            .await
            {
                Ok(attempts) => attempts,
                Err(e) => {
                    tracing::error!("Failed to query executor running attempts: {}", e);
                    continue;
                }
            };

        for attempt_id in executor_running_attempt_ids {
            // Check if this attempt has a running execution
            if !app_state.has_running_execution(attempt_id).await {
                // This is an orphaned task attempt - mark it as paused
                let activity_id = Uuid::new_v4();
                let create_activity = CreateTaskAttemptActivity {
                    task_attempt_id: attempt_id,
                    status: Some(TaskAttemptStatus::ExecutorFailed),
                    note: Some("Execution lost (server restart or crash)".to_string()),
                };

                if let Err(e) = TaskAttemptActivity::create(
                    &app_state.db_pool,
                    &create_activity,
                    activity_id,
                    TaskAttemptStatus::ExecutorFailed,
                )
                .await
                {
                    tracing::error!(
                        "Failed to create paused activity for orphaned attempt: {}",
                        e
                    );
                } else {
                    tracing::info!("Marked orphaned task attempt {} as paused", attempt_id);

                    // Get task attempt and task to access task_id and project_id for status update
                    if let Ok(Some(task_attempt)) =
                        TaskAttempt::find_by_id(&app_state.db_pool, attempt_id).await
                    {
                        if let Ok(Some(task)) =
                            Task::find_by_id(&app_state.db_pool, task_attempt.task_id).await
                        {
                            // Update task status to InReview
                            if let Err(e) = Task::update_status(
                                &app_state.db_pool,
                                task.id,
                                task.project_id,
                                TaskStatus::InReview,
                            )
                            .await
                            {
                                tracing::error!("Failed to update task status to InReview for orphaned attempt: {}", e);
                            }
                        }
                    }
                }
            }
        }

        // Note: Execution starting logic moved to create_task_attempt endpoint

        // Check for completed processes
        let completed_executions = app_state.get_running_executions_for_monitor().await;

        // Handle completed executions
        for (execution_id, task_attempt_id, success, exit_code) in completed_executions {
            let status_text = if success {
                "completed successfully"
            } else {
                "failed"
            };
            let exit_text = if let Some(code) = exit_code {
                format!(" with exit code {}", code)
            } else {
                String::new()
            };

            tracing::info!("Execution {} {}{}", execution_id, status_text, exit_text);

            // Play sound notification if enabled
            if app_state.get_sound_alerts_enabled().await {
                play_sound_notification().await;
            }

            // Get task attempt to access worktree path for committing changes
            if let Ok(Some(task_attempt)) =
                TaskAttempt::find_by_id(&app_state.db_pool, task_attempt_id).await
            {
                // Commit any unstaged changes after execution completion
                if let Err(e) =
                    commit_execution_changes(&task_attempt.worktree_path, task_attempt_id).await
                {
                    tracing::error!(
                        "Failed to commit execution changes for attempt {}: {}",
                        task_attempt_id,
                        e
                    );
                } else {
                    tracing::info!(
                        "Successfully committed execution changes for attempt {}",
                        task_attempt_id
                    );
                }

                // Create task attempt activity with appropriate completion status
                let activity_id = Uuid::new_v4();
                let status = if success {
                    TaskAttemptStatus::ExecutorComplete
                } else {
                    TaskAttemptStatus::ExecutorFailed
                };
                let create_activity = CreateTaskAttemptActivity {
                    task_attempt_id,
                    status: Some(status.clone()),
                    note: Some(format!("Execution completed{}", exit_text)),
                };

                if let Err(e) = TaskAttemptActivity::create(
                    &app_state.db_pool,
                    &create_activity,
                    activity_id,
                    status,
                )
                .await
                {
                    tracing::error!("Failed to create paused activity: {}", e);
                } else {
                    tracing::info!(
                        "Task attempt {} set to paused after execution completion",
                        task_attempt_id
                    );

                    // Get task to access task_id and project_id for status update
                    if let Ok(Some(task)) =
                        Task::find_by_id(&app_state.db_pool, task_attempt.task_id).await
                    {
                        // Update task status to InReview
                        if let Err(e) = Task::update_status(
                            &app_state.db_pool,
                            task.id,
                            task.project_id,
                            TaskStatus::InReview,
                        )
                        .await
                        {
                            tracing::error!("Failed to update task status to InReview for completed attempt: {}", e);
                        }
                    }
                }
            } else {
                tracing::error!(
                    "Failed to find task attempt {} for execution completion",
                    task_attempt_id
                );
            }
        }
    }
}
