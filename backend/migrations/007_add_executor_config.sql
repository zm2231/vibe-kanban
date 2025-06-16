-- Add executor_config column to task_attempts table
ALTER TABLE task_attempts ADD COLUMN executor_config JSONB;
