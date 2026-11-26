#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskKind {
    NamePalette,
    AssignRoles,
    LabelComponents,
    TagEffects,
    DescribeAnimation,
    ClassifyLayouts,
    NameIcons,
    WriteDesignMd,
}

impl TaskKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NamePalette => "name_palette",
            Self::AssignRoles => "assign_roles",
            Self::LabelComponents => "label_components",
            Self::TagEffects => "tag_effects",
            Self::DescribeAnimation => "describe_animation",
            Self::ClassifyLayouts => "classify_layouts",
            Self::NameIcons => "name_icons",
            Self::WriteDesignMd => "write_design_md",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTask {
    pub kind: TaskKind,
    pub input_file: PathBuf,
    pub screenshots: Vec<PathBuf>,
    pub output_schema: PathBuf,
    pub instructions: String,
}

#[derive(Debug, Clone)]
pub struct RunContext {
    pub timeout_secs: u64,
    pub keep_tasks: bool,
    pub max_items_per_call: usize,
}

impl Default for RunContext {
    fn default() -> Self {
        Self {
            timeout_secs: 300,
            keep_tasks: false,
            max_items_per_call: 20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOutput {
    pub raw_json: String,
    pub agent_id: String,
    pub agent_version: Option<String>,
}
