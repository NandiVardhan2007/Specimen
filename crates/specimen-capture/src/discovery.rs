use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrowserInfo {
    pub path: PathBuf,
    pub name: String,
    pub version: Option<String>,
}

/// Locates a Chrome, Chromium, or Edge installation on the host system.
pub fn find_browser() -> Option<BrowserInfo> {
    // 1. Check override environment variables
    for var in &["CHROME_BIN", "GOOGLE_CHROME_BIN", "CHROMIUM_BIN"] {
        if let Ok(val) = env::var(var) {
            let p = PathBuf::from(val);
            if p.is_file() {
                let version = query_browser_version(&p);
                return Some(BrowserInfo {
                    name: "Custom (Env)".to_string(),
                    path: p,
                    version,
                });
            }
        }
    }

    // 2. Platform-specific search
    #[cfg(target_os = "windows")]
    {
        let candidates = [
            (
                "Google Chrome",
                env::var("ProgramFiles")
                    .map(|p| format!(r"{}\Google\Chrome\Application\chrome.exe", p)),
            ),
            (
                "Google Chrome (x86)",
                env::var("ProgramFiles(x86)")
                    .map(|p| format!(r"{}\Google\Chrome\Application\chrome.exe", p)),
            ),
            (
                "Google Chrome (User)",
                env::var("LocalAppData")
                    .map(|p| format!(r"{}\Google\Chrome\Application\chrome.exe", p)),
            ),
            (
                "Microsoft Edge",
                env::var("ProgramFiles(x86)")
                    .map(|p| format!(r"{}\Microsoft\Edge\Application\msedge.exe", p)),
            ),
            (
                "Microsoft Edge",
                env::var("ProgramFiles")
                    .map(|p| format!(r"{}\Microsoft\Edge\Application\msedge.exe", p)),
            ),
        ];

        for (name, path_opt) in candidates {
            if let Ok(path_str) = path_opt {
                let path = PathBuf::from(path_str);
                if path.is_file() {
                    let version = query_browser_version(&path);
                    return Some(BrowserInfo {
                        name: name.to_string(),
                        path,
                        version,
                    });
                }
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = [
            (
                "Google Chrome",
                "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
            ),
            (
                "Chromium",
                "/Applications/Chromium.app/Contents/MacOS/Chromium",
            ),
            (
                "Microsoft Edge",
                "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
            ),
        ];

        for (name, path_str) in candidates {
            let path = PathBuf::from(path_str);
            if path.is_file() {
                let version = query_browser_version(&path);
                return Some(BrowserInfo {
                    name: name.to_string(),
                    path,
                    version,
                });
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = [
            ("Google Chrome", "google-chrome"),
            ("Google Chrome Stable", "google-chrome-stable"),
            ("Chromium", "chromium"),
            ("Chromium Browser", "chromium-browser"),
        ];

        for (name, bin) in candidates {
            if let Ok(output) = Command::new("which").arg(bin).output() {
                if output.status.success() {
                    let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    let path = PathBuf::from(path_str);
                    if path.is_file() {
                        let version = query_browser_version(&path);
                        return Some(BrowserInfo {
                            name: name.to_string(),
                            path,
                            version,
                        });
                    }
                }
            }
        }
    }

    None
}

fn query_browser_version(path: &Path) -> Option<String> {
    let output = Command::new(path).arg("--version").output().ok()?;
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();
        if !trimmed.is_empty() {
            return Some(trimmed.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_discovery_runs() {
        let info = find_browser();
        if let Some(ref b) = info {
            assert!(b.path.exists());
        }
    }
}
