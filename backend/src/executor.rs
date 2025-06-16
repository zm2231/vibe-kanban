use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use tokio::process::Child;
use ts_rs::TS;
use uuid::Uuid;

use crate::executors::EchoExecutor;

#[derive(Debug)]
pub enum ExecutorError {
    SpawnFailed(std::io::Error),
    TaskNotFound,
    DatabaseError(sqlx::Error),
}

impl std::fmt::Display for ExecutorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutorError::SpawnFailed(e) => write!(f, "Failed to spawn process: {}", e),
            ExecutorError::TaskNotFound => write!(f, "Task not found"),
            ExecutorError::DatabaseError(e) => write!(f, "Database error: {}", e),
        }
    }
}

impl std::error::Error for ExecutorError {}

impl From<sqlx::Error> for ExecutorError {
    fn from(err: sqlx::Error) -> Self {
        ExecutorError::DatabaseError(err)
    }
}

/// Trait for defining CLI commands that can be executed for task attempts
#[async_trait]
pub trait Executor: Send + Sync {
    /// Get the unique identifier for this executor type
    fn executor_type(&self) -> &'static str;
    
    /// Spawn the command for a given task attempt
    async fn spawn(&self, pool: &sqlx::PgPool, task_id: Uuid, worktree_path: &str) -> Result<Child, ExecutorError>;
    
    /// Get a human-readable description of what this executor does
    fn description(&self) -> &'static str;
}

/// Configuration for different executor types
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "lowercase")]
#[ts(export)]
pub enum ExecutorConfig {
    Echo,
    // Future executors can be added here
    // Shell { command: String },
    // Docker { image: String, command: String },
}

impl ExecutorConfig {
    pub fn create_executor(&self) -> Box<dyn Executor> {
        match self {
            ExecutorConfig::Echo => Box::new(EchoExecutor),
        }
    }
    
    pub fn executor_type(&self) -> &'static str {
        match self {
            ExecutorConfig::Echo => "echo",
        }
    }
}


