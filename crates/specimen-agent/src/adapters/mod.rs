#![forbid(unsafe_code)]

pub mod cli;
pub mod manual;
pub mod mock;

use async_trait::async_trait;

use crate::detection::AgentDetection;
use crate::error::Result;
use crate::task::{AgentOutput, AgentTask, RunContext};

#[async_trait]
pub trait AgentAdapter: Send + Sync {
    fn id(&self) -> &'static str;
    async fn detect(&self) -> AgentDetection;
    async fn run(&self, task: &AgentTask, cx: &RunContext) -> Result<AgentOutput>;
}
