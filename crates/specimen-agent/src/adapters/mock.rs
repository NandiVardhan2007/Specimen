#![forbid(unsafe_code)]

use async_trait::async_trait;
use std::sync::{Arc, Mutex};

use crate::adapters::AgentAdapter;
use crate::detection::AgentDetection;
use crate::error::{AgentError, Result};
use crate::task::{AgentOutput, AgentTask, RunContext};

/// Mock agent adapter for deterministic contract testing and injection defense tests.
#[derive(Clone)]
pub struct MockAdapter {
    pub canned_response: Arc<Mutex<Option<String>>>,
    pub should_timeout: bool,
    pub should_fail: bool,
}

impl Default for MockAdapter {
    fn default() -> Self {
        Self {
            canned_response: Arc::new(Mutex::new(None)),
            should_timeout: false,
            should_fail: false,
        }
    }
}

impl MockAdapter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_response(response: &str) -> Self {
        Self {
            canned_response: Arc::new(Mutex::new(Some(response.to_string()))),
            should_timeout: false,
            should_fail: false,
        }
    }

    pub fn with_timeout() -> Self {
        Self {
            canned_response: Arc::new(Mutex::new(None)),
            should_timeout: true,
            should_fail: false,
        }
    }

    pub fn with_failure() -> Self {
        Self {
            canned_response: Arc::new(Mutex::new(None)),
            should_timeout: false,
            should_fail: true,
        }
    }
}

#[async_trait]
impl AgentAdapter for MockAdapter {
    fn id(&self) -> &'static str {
        "mock"
    }

    async fn detect(&self) -> AgentDetection {
        AgentDetection {
            id: "mock",
            name: "Mock Test Agent",
            installed: true,
            version: Some("0.1.0-mock".to_string()),
        }
    }

    async fn run(&self, _task: &AgentTask, _cx: &RunContext) -> Result<AgentOutput> {
        if self.should_timeout {
            return Err(AgentError::Timeout("Mock timeout triggered".to_string()));
        }
        if self.should_fail {
            return Err(AgentError::ProcessFailed("Mock exit code 1".to_string()));
        }

        let resp = self
            .canned_response
            .lock()
            .unwrap()
            .clone()
            .unwrap_or_else(|| r#"{"results": []}"#.to_string());

        Ok(AgentOutput {
            raw_json: resp,
            agent_id: "mock".to_string(),
            agent_version: Some("0.1.0-mock".to_string()),
        })
    }
}
