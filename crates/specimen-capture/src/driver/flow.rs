#![forbid(unsafe_code)]

use std::collections::BTreeMap;
use std::time::Duration;
use url::Url;

use specimen_core::{FlowAction, FlowScript, FlowStep};

use crate::policy::RateLimiter;
use crate::session::{
    CaptureError, ColorScheme, DomSnapshot, Image, NavOutcome, Result, Session, SettlePolicy,
    ShotOpts, Viewport,
};

/// A labeled capture checkpoint recorded during a flow run.
#[derive(Debug, Clone)]
pub struct LabeledCapture {
    pub label: String,
    pub snapshot: DomSnapshot,
    pub screenshot: Image,
}

/// Recorded outcome and artifacts from a flow execution.
#[derive(Debug, Clone)]
pub struct FlowEvidence {
    pub start_url: Url,
    pub nav_outcome: NavOutcome,
    pub captures: Vec<LabeledCapture>,
    pub screenshots: BTreeMap<String, Image>,
    pub steps_executed: usize,
}

#[derive(Debug, Clone)]
pub struct FlowDriverOptions {
    pub allow_eval: bool,
    pub default_timeout_ms: u64,
}

impl Default for FlowDriverOptions {
    fn default() -> Self {
        Self {
            allow_eval: false,
            default_timeout_ms: 10_000,
        }
    }
}

pub struct FlowDriver;

impl FlowDriver {
    /// Executes a scripted interaction flow sequentially against the browser session.
    pub async fn run<S: Session>(
        session: &mut S,
        script: &FlowScript,
        options: &FlowDriverOptions,
        rate_limiter: Option<&RateLimiter>,
    ) -> Result<FlowEvidence> {
        let start_url = Url::parse(&script.start)
            .map_err(|e| CaptureError::Navigation(format!("Invalid start URL: {}", e)))?;

        if let Some(limiter) = rate_limiter {
            if let Some(host) = start_url.host_str() {
                limiter.wait_turn(host).await;
            }
        }

        let nav_outcome = session.navigate(&start_url).await?;

        // Initial emulation if specified
        if let Some(vp_str) = &script.viewport {
            let vp = parse_viewport(vp_str);
            let scheme = parse_scheme(script.scheme.as_deref());
            session.emulate(&vp, scheme).await?;
        }

        let mut captures = Vec::new();
        let mut screenshots = BTreeMap::new();
        let mut steps_executed = 0;

        for (idx, step) in script.steps.iter().enumerate() {
            let result =
                Self::execute_step(session, step, options, &mut captures, &mut screenshots).await;

            if let Err(e) = result {
                if step.optional {
                    eprintln!("Warning: optional flow step {} failed: {}", idx + 1, e);
                } else {
                    return Err(e);
                }
            }
            steps_executed += 1;
        }

        Ok(FlowEvidence {
            start_url,
            nav_outcome,
            captures,
            screenshots,
            steps_executed,
        })
    }

    async fn execute_step<S: Session>(
        session: &mut S,
        step: &FlowStep,
        options: &FlowDriverOptions,
        captures: &mut Vec<LabeledCapture>,
        screenshots: &mut BTreeMap<String, Image>,
    ) -> Result<()> {
        match &step.action {
            FlowAction::Wait {
                selector,
                timeout_ms,
                ms,
            } => {
                if let Some(duration_ms) = ms {
                    tokio::time::sleep(Duration::from_millis(*duration_ms)).await;
                }
                if let Some(sel) = selector {
                    let timeout = timeout_ms.unwrap_or(options.default_timeout_ms);
                    session.wait_for_selector(sel, timeout).await?;
                }
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Screenshot { name, full_page } => {
                let img = session
                    .screenshot(&ShotOpts {
                        full_page: *full_page,
                        quality: 90,
                    })
                    .await?;
                screenshots.insert(name.clone(), img);
            }
            FlowAction::Hover { selector } => {
                session.hover(selector).await?;
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Click { selector, text } => {
                session.click(selector.as_deref(), text.as_deref()).await?;
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Type { selector, text } => {
                if selector.to_lowercase().contains("password") {
                    eprintln!(
                        "Warning: 'type' action directed at password field '{}'. Credentials must not be recorded or committed.",
                        selector
                    );
                }
                session.type_text(selector, text).await?;
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Press { key } => {
                session.press_key(key).await?;
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Scroll {
                to,
                step_px,
                pause_ms,
            } => {
                let step_amount = step_px.unwrap_or(400);
                let pause = Duration::from_millis(pause_ms.unwrap_or(150));

                if let Some(dest) = to {
                    if dest == "bottom" {
                        for _ in 0..4 {
                            session.scroll_by(step_amount).await?;
                            tokio::time::sleep(pause).await;
                        }
                    } else if dest == "top" {
                        session.scroll_by(-2000).await?;
                    }
                } else {
                    session.scroll_by(step_amount).await?;
                }
                session.settle(&SettlePolicy::default()).await?;
            }
            FlowAction::Capture { label } => {
                session.settle(&SettlePolicy::default()).await?;
                let snapshot = session.snapshot().await?;
                let screenshot = session
                    .screenshot(&ShotOpts {
                        full_page: false,
                        quality: 90,
                    })
                    .await?;
                captures.push(LabeledCapture {
                    label: label.clone(),
                    snapshot,
                    screenshot,
                });
            }
            FlowAction::SetScheme { scheme } => {
                let cs = parse_scheme(Some(scheme));
                let default_vp = Viewport {
                    name: "desktop".to_string(),
                    width: 1920,
                    height: 1080,
                    dpr: 1.0,
                };
                session.emulate(&default_vp, cs).await?;
            }
            FlowAction::Resize { width, height } => {
                let vp = Viewport {
                    name: format!("custom_{}x{}", width, height),
                    width: *width,
                    height: *height,
                    dpr: 1.0,
                };
                session.emulate(&vp, ColorScheme::Light).await?;
            }
            FlowAction::Eval { script } => {
                if !options.allow_eval {
                    return Err(CaptureError::Cdp(
                        "Flow step 'eval' is disabled unless --allow-eval is specified".to_string(),
                    ));
                }
                session.evaluate(script).await?;
            }
        }

        Ok(())
    }
}

fn parse_viewport(val: &str) -> Viewport {
    match val {
        "mobile" => Viewport {
            name: "mobile".to_string(),
            width: 390,
            height: 844,
            dpr: 2.0,
        },
        "tablet" => Viewport {
            name: "tablet".to_string(),
            width: 768,
            height: 1024,
            dpr: 2.0,
        },
        "desktop" => Viewport {
            name: "desktop".to_string(),
            width: 1920,
            height: 1080,
            dpr: 1.0,
        },
        other => {
            if let Some((w, h)) = other.split_once('x') {
                if let (Ok(width), Ok(height)) = (w.trim().parse::<u32>(), h.trim().parse::<u32>())
                {
                    return Viewport {
                        name: other.to_string(),
                        width,
                        height,
                        dpr: 1.0,
                    };
                }
            }
            Viewport {
                name: "desktop".to_string(),
                width: 1920,
                height: 1080,
                dpr: 1.0,
            }
        }
    }
}

fn parse_scheme(val: Option<&str>) -> ColorScheme {
    match val {
        Some("dark") => ColorScheme::Dark,
        _ => ColorScheme::Light,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::MockSession;

    #[tokio::test]
    async fn test_flow_driver_execution() {
        let mut session = MockSession::default();
        let script = FlowScript {
            start: "https://example.com/login".to_string(),
            viewport: Some("desktop".to_string()),
            scheme: Some("dark".to_string()),
            steps: vec![
                FlowStep {
                    action: FlowAction::Wait {
                        selector: Some("h1".to_string()),
                        timeout_ms: Some(2000),
                        ms: None,
                    },
                    optional: false,
                },
                FlowStep {
                    action: FlowAction::Type {
                        selector: "input[type=email]".to_string(),
                        text: "designer@example.com".to_string(),
                    },
                    optional: false,
                },
                FlowStep {
                    action: FlowAction::Click {
                        selector: Some("button.submit".to_string()),
                        text: None,
                    },
                    optional: false,
                },
                FlowStep {
                    action: FlowAction::Capture {
                        label: "after-login".to_string(),
                    },
                    optional: false,
                },
            ],
        };

        let options = FlowDriverOptions::default();
        let evidence = FlowDriver::run(&mut session, &script, &options, None)
            .await
            .unwrap();

        assert_eq!(evidence.steps_executed, 4);
        assert_eq!(evidence.captures.len(), 1);
        assert_eq!(evidence.captures[0].label, "after-login");
        assert!(session
            .recorded_actions
            .iter()
            .any(|a| a.starts_with("wait_selector:h1")));
        assert!(session
            .recorded_actions
            .iter()
            .any(|a| a.contains("designer@example.com")));
    }
}
