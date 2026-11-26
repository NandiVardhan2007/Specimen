use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;
use thiserror::Error;

use crate::discovery::find_browser;

#[derive(Error, Debug)]
pub enum BrowserError {
    #[error("No supported browser found on system. Install Chrome, Chromium, or Edge")]
    NotFound,

    #[error("Failed to spawn browser process: {0}")]
    SpawnFailed(#[from] std::io::Error),

    #[error("Timed out waiting for DevToolsActivePort: {0}")]
    PortTimeout(String),

    #[error("Browser process exited prematurely with code: {0}")]
    ProcessExited(String),
}

#[derive(Debug, Clone)]
pub struct LaunchOptions {
    pub headed: bool,
    pub custom_binary: Option<PathBuf>,
    pub window_width: u32,
    pub window_height: u32,
}

impl Default for LaunchOptions {
    fn default() -> Self {
        Self {
            headed: false,
            custom_binary: None,
            window_width: 1920,
            window_height: 1080,
        }
    }
}

/// A RAII guard for the Chrome browser process and its temporary user profile.
///
/// On Drop, the browser process is killed and the temporary profile directory is wiped,
/// guaranteeing that no orphaned processes or session data remain on disk.
pub struct BrowserGuard {
    child: Option<Child>,
    temp_dir: TempDir,
    port: u16,
    ws_url: String,
    http_endpoint: String,
}

impl BrowserGuard {
    /// Launches a fresh browser instance with a temporary profile.
    pub fn launch(options: &LaunchOptions) -> Result<Self, BrowserError> {
        let binary_path = match &options.custom_binary {
            Some(p) => p.clone(),
            None => find_browser()
                .map(|b| b.path)
                .ok_or(BrowserError::NotFound)?,
        };

        let temp_dir = tempfile::Builder::new()
            .prefix("specimen-browser-")
            .tempdir()
            .map_err(BrowserError::SpawnFailed)?;

        let mut cmd = Command::new(&binary_path);

        // Core flags for headless operation and temporary profile isolation
        cmd.arg(format!("--user-data-dir={}", temp_dir.path().display()))
            .arg("--remote-debugging-port=0")
            .arg("--no-first-run")
            .arg("--no-default-browser-check")
            .arg("--disable-background-networking")
            .arg("--disable-sync")
            .arg("--disable-default-apps")
            .arg("--mute-audio")
            .arg(format!(
                "--window-size={},{}",
                options.window_width, options.window_height
            ));

        if !options.headed {
            cmd.arg("--headless=new").arg("--disable-gpu");
        }

        // Run detached and suppress console noise
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());

        let mut child = cmd.spawn().map_err(BrowserError::SpawnFailed)?;

        // Wait for DevToolsActivePort to be written
        let port_file = temp_dir.path().join("DevToolsActivePort");
        let start = Instant::now();
        let timeout = Duration::from_secs(10);

        let mut port_info = None;
        while start.elapsed() < timeout {
            // Check if process crashed
            if let Ok(Some(status)) = child.try_wait() {
                return Err(BrowserError::ProcessExited(format!("{}", status)));
            }

            if port_file.exists() {
                if let Ok(content) = fs::read_to_string(&port_file) {
                    let lines: Vec<&str> = content.lines().collect();
                    if lines.len() >= 2 {
                        if let Ok(port) = lines[0].trim().parse::<u16>() {
                            let ws_path = lines[1].trim();
                            port_info = Some((port, ws_path.to_string()));
                            break;
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(50));
        }

        let (port, ws_path) = port_info.ok_or_else(|| {
            BrowserError::PortTimeout("DevToolsActivePort was not created within 10s".to_string())
        })?;

        let http_endpoint = format!("http://127.0.0.1:{}", port);
        let ws_url = format!("ws://127.0.0.1:{}{}", port, ws_path);

        Ok(BrowserGuard {
            child: Some(child),
            temp_dir,
            port,
            ws_url,
            http_endpoint,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn ws_url(&self) -> &str {
        &self.ws_url
    }

    pub fn http_endpoint(&self) -> &str {
        &self.http_endpoint
    }

    pub fn temp_dir(&self) -> &Path {
        self.temp_dir.path()
    }

    /// Checks if the browser child process is currently alive.
    pub fn is_alive(&mut self) -> bool {
        if let Some(child) = self.child.as_mut() {
            matches!(child.try_wait(), Ok(None))
        } else {
            false
        }
    }
}

impl Drop for BrowserGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        // self.temp_dir drops automatically here and deletes the folder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_launch_and_guard_drop() {
        if find_browser().is_none() {
            eprintln!("Skipping browser launch test: no browser installed");
            return;
        }

        let opts = LaunchOptions::default();
        let guard_res = BrowserGuard::launch(&opts);
        assert!(
            guard_res.is_ok(),
            "Failed to launch browser: {:?}",
            guard_res.err()
        );

        let mut guard = guard_res.unwrap();
        assert!(guard.is_alive());
        assert!(guard.port() > 0);
        assert!(guard.ws_url().starts_with("ws://127.0.0.1:"));
        let temp_dir_path = guard.temp_dir().to_path_buf();
        assert!(temp_dir_path.exists());

        // When guard is dropped, process should be terminated and directory cleaned up
        drop(guard);
        // Temporary profile directory should now be gone
        assert!(!temp_dir_path.exists());
    }
}
