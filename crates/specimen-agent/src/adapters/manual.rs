#![forbid(unsafe_code)]

use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

use crate::adapters::AgentAdapter;
use crate::detection::AgentDetection;
use crate::error::{AgentError, Result};
use crate::task::{AgentOutput, AgentTask, RunContext};

pub struct ManualAdapter {
    pub tasks_root: PathBuf,
}

impl Default for ManualAdapter {
    fn default() -> Self {
        Self {
            tasks_root: PathBuf::from(".cache/tasks"),
        }
    }
}

impl ManualAdapter {
    pub fn new(tasks_root: PathBuf) -> Self {
        Self { tasks_root }
    }

    /// Prepares task directory and files for human/manual execution.
    pub fn prepare_task_bundle(&self, task_id: &str, task: &AgentTask) -> Result<PathBuf> {
        let task_dir = self.tasks_root.join(task_id);
        fs::create_dir_all(&task_dir)?;

        let instructions_path = task_dir.join("INSTRUCTIONS.md");
        fs::write(&instructions_path, &task.instructions)?;

        let input_path = task_dir.join("task.json");
        fs::copy(&task.input_file, &input_path)?;

        if task.output_schema.exists() {
            let schema_path = task_dir.join("output.schema.json");
            fs::copy(&task.output_schema, &schema_path)?;
        }

        Ok(task_dir)
    }

    /// Imports a completed manual `output.json` file.
    pub fn import_output(output_path: &Path) -> Result<AgentOutput> {
        if !output_path.exists() {
            return Err(AgentError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Output file not found at: {}", output_path.display()),
            )));
        }

        let raw_json = fs::read_to_string(output_path)?;
        Ok(AgentOutput {
            raw_json,
            agent_id: "manual".to_string(),
            agent_version: None,
        })
    }
}

#[async_trait]
impl AgentAdapter for ManualAdapter {
    fn id(&self) -> &'static str {
        "manual"
    }

    async fn detect(&self) -> AgentDetection {
        AgentDetection {
            id: "manual",
            name: "Manual Mode",
            installed: true,
            version: Some("builtin".to_string()),
        }
    }

    async fn run(&self, task: &AgentTask, _cx: &RunContext) -> Result<AgentOutput> {
        let task_id = format!("task-{}", std::process::id());
        let bundle_dir = self.prepare_task_bundle(&task_id, task)?;

        let expected_output = bundle_dir.join("output.json");
        if expected_output.exists() {
            return Self::import_output(&expected_output);
        }

        Err(AgentError::ValidationFailed(format!(
            "Manual task prepared at {}. Run your agent and save result as output.json",
            bundle_dir.display()
        )))
    }
}
