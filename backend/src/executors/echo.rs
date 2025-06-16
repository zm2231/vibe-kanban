use async_trait::async_trait;
use tokio::process::{Child, Command};
use uuid::Uuid;

use crate::executor::{Executor, ExecutorError};
use crate::models::task::Task;

/// A dummy executor that echoes the task title and description
pub struct EchoExecutor;

#[async_trait]
impl Executor for EchoExecutor {
    fn executor_type(&self) -> &'static str {
        "echo"
    }

    async fn spawn(
        &self,
        pool: &sqlx::PgPool,
        task_id: Uuid,
        _worktree_path: &str,
    ) -> Result<Child, ExecutorError> {
        // Get the task to fetch its description
        let task = Task::find_by_id(pool, task_id)
            .await?
            .ok_or(ExecutorError::TaskNotFound)?;

        let message = format!(
            "Executing task: {} - {}",
            task.title,
            task.description.as_deref().unwrap_or("No description")
        );

        let child = Command::new("echo")
            .kill_on_drop(true)
            .arg(&message)
            .spawn()
            .map_err(ExecutorError::SpawnFailed)?;

        Ok(child)
    }

    fn description(&self) -> &'static str {
        "Echoes the task title and description"
    }
}
