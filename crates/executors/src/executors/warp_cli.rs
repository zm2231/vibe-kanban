use std::{path::Path, process::Stdio, sync::Arc};

use async_trait::async_trait;
use command_group::{AsyncCommandGroup, AsyncGroupChild};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use ts_rs::TS;
use utils::{msg_store::MsgStore, shell::get_shell_command};

use crate::{
    command::{apply_overrides, CmdOverrides, CommandBuilder},
    executors::{AppendPrompt, ExecutorError, StandardCodingAgentExecutor},
    logs::{
        stderr_processor::normalize_stderr_logs,
        utils::EntryIndexProvider,
        plain_text_processor::PlainTextLogProcessor,
        NormalizedEntry,
        NormalizedEntryType,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS, JsonSchema)]
pub struct WarpCli {
    #[serde(default)]
    pub append_prompt: AppendPrompt,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_servers: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub extra_flags: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(flatten)]
    pub cmd: CmdOverrides,
}

impl WarpCli {
    fn build_command_builder(&self) -> CommandBuilder {
        let mut builder = CommandBuilder::new(self.binary.clone().unwrap_or_else(|| "warp".to_string()))
            .params(["agent", "run"]);

        if let Some(profile) = &self.profile {
            builder = builder.extend_params(["--profile", profile]);
        }

        if !self.mcp_servers.is_empty() {
            for server in &self.mcp_servers {
                builder = builder.extend_params(["--mcp-server", server]);
            }
        }

        if !self.extra_flags.is_empty() {
            builder = builder.extend_params(self.extra_flags.clone());
        }

        apply_overrides(builder, &self.cmd)
    }

    fn shell_escape(s: &str) -> String {
        format!("'{}'", s.replace('\'', "'\\''"))
    }
}

#[async_trait]
impl StandardCodingAgentExecutor for WarpCli {
    async fn spawn(
        &self,
        current_dir: &Path,
        prompt: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        let (shell_cmd, shell_arg) = get_shell_command();
        let mut builder = self.build_command_builder();
        let combined_prompt = self.append_prompt.combine_prompt(prompt);
        builder = builder.extend_params([
            "--prompt".to_string(),
            Self::shell_escape(&combined_prompt),
        ]);
        let warp_command = builder.build_initial();

        let mut command = Command::new(shell_cmd);
        command
            .kill_on_drop(true)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .current_dir(current_dir)
            .arg(shell_arg)
            .arg(&warp_command);

        let child = command.group_spawn()?;
        Ok(child)
    }

    async fn spawn_follow_up(
        &self,
        _current_dir: &Path,
        _prompt: &str,
        _session_id: &str,
    ) -> Result<AsyncGroupChild, ExecutorError> {
        Err(ExecutorError::FollowUpNotSupported(
            "Warp CLI does not support follow-up sessions".to_string(),
        ))
    }

    fn normalize_logs(&self, msg_store: Arc<MsgStore>, _worktree_path: &Path) {
        let entry_index_provider = EntryIndexProvider::start_from(&msg_store);
        normalize_stderr_logs(msg_store.clone(), entry_index_provider.clone());

        tokio::spawn(async move {
            use futures::StreamExt;
            let mut stdout = msg_store.stdout_chunked_stream();
            let mut processor = PlainTextLogProcessor::builder()
                .normalized_entry_producer(Box::new(|content: String| NormalizedEntry {
                    timestamp: None,
                    entry_type: NormalizedEntryType::AssistantMessage,
                    content,
                    metadata: None,
                }))
                .index_provider(entry_index_provider)
                .build();

            while let Some(Ok(chunk)) = stdout.next().await {
                for patch in processor.process(chunk) {
                    msg_store.push_patch(patch);
                }
            }
        });
    }

    fn default_mcp_config_path(&self) -> Option<std::path::PathBuf> {
        None
    }
}

