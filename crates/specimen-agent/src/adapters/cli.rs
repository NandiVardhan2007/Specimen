#![forbid(unsafe_code)]

use async_trait::async_trait;
use std::fs;
use std::process::Stdio;
use std::time::Duration;
use tempfile::TempDir;
use tokio::process::Command;
use tokio::time::timeout;

use crate::adapters::AgentAdapter;
use crate::detection::AgentDetection;
use crate::error::{AgentError, Result};
use crate::task::{AgentOutput, AgentTask, RunContext};

/// Generic CLI Agent Adapter executing agent processes in isolated scratch directories.
pub struct CliAdapter {
    pub cli_id: String,
    pub command: String,
    pub args: Vec<String>,
}

impl CliAdapter {
    pub fn new(cli_id: &str, command: &str, args: &[&str]) -> Self {
        Self {
            cli_id: cli_id.to_string(),
            command: command.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
        }
    }
}

#[async_trait]
impl AgentAdapter for CliAdapter {
    fn id(&self) -> &'static str {
        Box::leak(self.cli_id.clone().into_boxed_str())
    }

    async fn detect(&self) -> AgentDetection {
        let which_cmd = if cfg!(windows) { "where" } else { "which" };
        let installed = std::process::Command::new(which_cmd)
            .arg(&self.command)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false);

        let version = if installed {
            std::process::Command::new(&self.command)
                .arg("--version")
                .output()
                .ok()
                .and_then(|out| {
                    if out.status.success() {
                        String::from_utf8(out.stdout)
                            .ok()
                            .map(|s| s.trim().to_string())
                    } else {
                        None
                    }
                })
        } else {
            None
        };

        AgentDetection {
            id: Box::leak(self.cli_id.clone().into_boxed_str()),
            name: Box::leak(self.command.clone().into_boxed_str()),
            installed,
            version,
        }
    }

    async fn run(&self, task: &AgentTask, cx: &RunContext) -> Result<AgentOutput> {
        let scratch_dir = TempDir::new()?;
        let scratch_path = scratch_dir.path();

        // 1. Write task files
        fs::write(scratch_path.join("INSTRUCTIONS.md"), &task.instructions)?;
        fs::copy(&task.input_file, scratch_path.join("task.json"))?;

        if task.output_schema.exists() {
            fs::copy(&task.output_schema, scratch_path.join("output.schema.json"))?;
        }

        // 2. Prepare Command
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.args);
        cmd.current_dir(scratch_path);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        // Minimal clean environment (Invariant: no secrets passed)
        cmd.env_remove("AWS_SECRET_ACCESS_KEY");
        cmd.env_remove("GITHUB_TOKEN");
        cmd.env_remove("OPENAI_API_KEY");

        // 3. Execute with timeout
        let dur = Duration::from_secs(cx.timeout_secs);
        let output_res = timeout(dur, cmd.output())
            .await
            .map_err(|_| AgentError::Timeout(format!("CLI exceeded {}s timeout", cx.timeout_secs)))?
            .map_err(|e| AgentError::SpawnFailed(e.to_string()))?;

        if !output_res.status.success() {
            let stderr = String::from_utf8_lossy(&output_res.stderr);
            return Err(AgentError::ProcessFailed(format!(
                "CLI failed with {}: {}",
                output_res.status, stderr
            )));
        }

        // Check if output.json was produced, otherwise fallback to stdout
        let output_json_path = scratch_path.join("output.json");
        let raw_json = if output_json_path.exists() {
            fs::read_to_string(output_json_path)?
        } else {
            String::from_utf8_lossy(&output_res.stdout).to_string()
        };

        if cx.keep_tasks {
            let _ = scratch_dir.keep(); // preserve tempdir for debugging
        }

        Ok(AgentOutput {
            raw_json,
            agent_id: self.cli_id.clone(),
            agent_version: None,
        })
    }
}
