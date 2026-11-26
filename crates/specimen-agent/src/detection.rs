use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDetection {
    pub id: &'static str,
    pub name: &'static str,
    pub installed: bool,
    pub version: Option<String>,
}

const SUPPORTED_AGENTS: &[(&str, &str, &str)] = &[
    ("claude", "Claude Code", "claude"),
    ("codex", "Codex CLI", "codex"),
    ("antigravity", "Google Antigravity (agy)", "agy"),
    ("gemini", "Gemini CLI", "gemini"),
    ("copilot", "GitHub Copilot", "gh"),
];

/// Detects locally available AI coding agent CLIs.
pub fn detect_agents() -> Vec<AgentDetection> {
    let mut detections = Vec::new();

    for &(id, name, bin) in SUPPORTED_AGENTS {
        let (installed, version) = check_cli_version(bin);
        detections.push(AgentDetection {
            id,
            name,
            installed,
            version,
        });
    }

    // Manual mode is always available
    detections.push(AgentDetection {
        id: "manual",
        name: "Manual File Exchange",
        installed: true,
        version: Some("builtin".to_string()),
    });

    detections
}

fn check_cli_version(bin: &str) -> (bool, Option<String>) {
    let res = if cfg!(target_os = "windows") {
        Command::new("where.exe").arg(bin).output()
    } else {
        Command::new("which").arg(bin).output()
    };

    if let Ok(out) = res {
        if out.status.success() {
            // Found in PATH, now probe version
            let version = Command::new(bin)
                .arg("--version")
                .output()
                .ok()
                .filter(|v| v.status.success())
                .map(|v| String::from_utf8_lossy(&v.stdout).trim().to_string());
            return (true, version);
        }
    }

    (false, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_agents_includes_manual() {
        let agents = detect_agents();
        let manual = agents.iter().find(|a| a.id == "manual").unwrap();
        assert!(manual.installed);
    }
}
