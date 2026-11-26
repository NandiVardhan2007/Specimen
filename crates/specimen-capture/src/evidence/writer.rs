use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::driver::{CapturedEvidence, FlowEvidence};
use crate::session::{NetworkEntry, NetworkLog, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureManifest {
    pub schema: String,
    pub url: String,
    pub captured_at: String,
    pub version: String,
    pub viewports: Vec<String>,
    pub schemes: Vec<String>,
    pub timing_ms: u64,
    pub warnings: Vec<String>,
}

pub struct EvidenceWriter;

impl EvidenceWriter {
    /// Writes the captured evidence to `sources/<host>/<timestamp>/` atomically.
    ///
    /// Constructs all directories and files within a temporary `.partial` folder first,
    /// then renames the folder upon completion. If an error occurs, the `.partial`
    /// folder is wiped to guarantee library integrity.
    pub fn write_atomic(library_root: &Path, evidence: &CapturedEvidence) -> Result<PathBuf> {
        let host = evidence
            .url
            .host_str()
            .unwrap_or("unknown")
            .to_ascii_lowercase();
        let port_suffix = match evidence.url.port() {
            Some(p) if p != 80 && p != 443 => format!("_{}", p),
            _ => String::new(),
        };
        let host_folder = format!("{}{}", host, port_suffix);

        // Sanitize timestamp: YYYY-MM-DDTHHMMSSZ
        let ts_folder = evidence.captured_at.replace(':', "");
        let final_source_dir = library_root
            .join("sources")
            .join(host_folder)
            .join(&ts_folder);
        let partial_dir = final_source_dir.with_extension("partial");

        if partial_dir.exists() {
            let _ = fs::remove_dir_all(&partial_dir);
        }

        if let Err(e) = Self::write_internal(&partial_dir, evidence) {
            let _ = fs::remove_dir_all(&partial_dir);
            return Err(e);
        }

        // Atomic rename of .partial -> final
        if let Some(parent) = final_source_dir.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&partial_dir, &final_source_dir)?;

        Ok(final_source_dir)
    }

    fn write_internal(dest: &Path, evidence: &CapturedEvidence) -> Result<()> {
        let dom_dir = dest.join("evidence").join("dom");
        let screenshots_dir = dest.join("screenshots");
        fs::create_dir_all(&dom_dir)?;
        fs::create_dir_all(&screenshots_dir)?;

        // 1. Write DOM snapshots
        for (vp, scheme, snap) in &evidence.snapshots {
            let snap_file = dom_dir.join(format!("{}-{}.json", vp.name, scheme.as_str()));
            fs::write(snap_file, &snap.raw_json)?;
        }

        // 2. Write screenshots
        for (vp, scheme, img, is_full_page) in &evidence.screenshots {
            let vp_dir = screenshots_dir.join(&vp.name);
            fs::create_dir_all(&vp_dir)?;
            let kind = if *is_full_page { "full" } else { "viewport" };
            let filename = format!("{}-{}.png", scheme.as_str(), kind);
            fs::write(vp_dir.join(filename), &img.bytes)?;
        }

        // 3. Write network.json (with sensitive query tokens redacted)
        let clean_network_log = Self::sanitize_network_log(&evidence.network_log);
        let net_file = dest.join("evidence").join("network.json");
        let net_json = serde_json::to_string_pretty(&clean_network_log)?;
        fs::write(net_file, net_json)?;

        // 4. Write manifest.json
        let viewports: Vec<String> = evidence
            .snapshots
            .iter()
            .map(|(vp, _, _)| vp.name.clone())
            .collect();
        let schemes: Vec<String> = evidence
            .snapshots
            .iter()
            .map(|(_, sc, _)| sc.as_str().to_string())
            .collect();

        let manifest = CaptureManifest {
            schema: "specimen.manifest/1".to_string(),
            url: evidence.url.to_string(),
            captured_at: evidence.captured_at.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            viewports,
            schemes,
            timing_ms: evidence.nav_outcome.timing_ms,
            warnings: Vec::new(),
        };
        let manifest_file = dest.join("manifest.json");
        let manifest_json = serde_json::to_string_pretty(&manifest)?;
        fs::write(manifest_file, manifest_json)?;

        // 5. Write report.md
        let report_file = dest.join("report.md");
        let report_content = format!(
            "# Capture Report: {}\n\n- **Captured at**: {}\n- **Status**: {}\n- **Snapshots**: {}\n- **Screenshots**: {}\n\n*Notice: No credentials, cookies, or secrets were stored.*\n",
            evidence.url,
            evidence.captured_at,
            evidence.nav_outcome.status,
            evidence.snapshots.len(),
            evidence.screenshots.len(),
        );
        fs::write(report_file, report_content)?;

        Ok(())
    }

    /// Writes flow-captured evidence into `sources/<host>/<timestamp>/` atomically.
    pub fn write_flow_atomic(library_root: &Path, flow: &FlowEvidence) -> Result<PathBuf> {
        let host = flow
            .start_url
            .host_str()
            .unwrap_or("unknown")
            .to_ascii_lowercase();
        let port_suffix = match flow.start_url.port() {
            Some(p) if p != 80 && p != 443 => format!("_{}", p),
            _ => String::new(),
        };
        let host_folder = format!("{}{}", host, port_suffix);

        // Current UTC timestamp
        let ts_folder = chrono_iso_now().replace(':', "");
        let final_source_dir = library_root
            .join("sources")
            .join(host_folder)
            .join(&ts_folder);
        let partial_dir = final_source_dir.with_extension("partial");

        if partial_dir.exists() {
            let _ = fs::remove_dir_all(&partial_dir);
        }

        if let Err(e) = Self::write_flow_internal(&partial_dir, flow) {
            let _ = fs::remove_dir_all(&partial_dir);
            return Err(e);
        }

        if let Some(parent) = final_source_dir.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(&partial_dir, &final_source_dir)?;

        Ok(final_source_dir)
    }

    fn write_flow_internal(dest: &Path, flow: &FlowEvidence) -> Result<()> {
        let captures_dir = dest.join("evidence").join("captures");
        let screenshots_dir = dest.join("screenshots");
        fs::create_dir_all(&captures_dir)?;
        fs::create_dir_all(&screenshots_dir)?;

        for cap in &flow.captures {
            let cap_dir = captures_dir.join(&cap.label);
            fs::create_dir_all(&cap_dir)?;
            fs::write(cap_dir.join("dom.json"), &cap.snapshot.raw_json)?;
            fs::write(cap_dir.join("screenshot.png"), &cap.screenshot.bytes)?;
        }

        for (name, img) in &flow.screenshots {
            let file = screenshots_dir.join(format!("{}.png", name));
            fs::write(file, &img.bytes)?;
        }

        let manifest = CaptureManifest {
            schema: "specimen.manifest/1".to_string(),
            url: flow.start_url.to_string(),
            captured_at: chrono_iso_now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            viewports: vec!["flow".to_string()],
            schemes: vec!["default".to_string()],
            timing_ms: flow.nav_outcome.timing_ms,
            warnings: Vec::new(),
        };
        fs::write(
            dest.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)?,
        )?;

        let report_content = format!(
            "# Flow Capture Report: {}\n\n- **Status**: {}\n- **Steps Executed**: {}\n- **Captures**: {}\n- **Screenshots**: {}\n\n*Notice: No credentials, cookies, or secrets were stored.*\n",
            flow.start_url,
            flow.nav_outcome.status,
            flow.steps_executed,
            flow.captures.len(),
            flow.screenshots.len(),
        );
        fs::write(dest.join("report.md"), report_content)?;

        Ok(())
    }

    /// Redacts tokens and credentials from query parameters and headers (Invariant 5).
    pub fn sanitize_network_log(log: &NetworkLog) -> NetworkLog {
        let sensitive_keys = [
            "token",
            "access_token",
            "auth",
            "secret",
            "password",
            "key",
            "session",
            "api_key",
        ];
        let mut sanitized = NetworkLog::default();

        for entry in &log.entries {
            let mut clean_url = entry.url.clone();
            if let Ok(mut parsed) = url::Url::parse(&clean_url) {
                let pairs: Vec<(String, String)> = parsed
                    .query_pairs()
                    .map(|(k, v)| {
                        if sensitive_keys
                            .iter()
                            .any(|&sk| k.to_ascii_lowercase().contains(sk))
                        {
                            (k.to_string(), "[REDACTED]".to_string())
                        } else {
                            (k.to_string(), v.to_string())
                        }
                    })
                    .collect();
                parsed.query_pairs_mut().clear().extend_pairs(pairs);
                clean_url = parsed.to_string();
            }
            sanitized.entries.push(NetworkEntry {
                url: clean_url,
                method: entry.method.clone(),
                status: entry.status,
                mime: entry.mime.clone(),
                size_bytes: entry.size_bytes,
            });
        }
        sanitized
    }
}

fn chrono_iso_now() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let total_secs = duration.as_secs();
    let days = total_secs / 86400;
    let rem_secs = total_secs % 86400;
    let hours = rem_secs / 3600;
    let minutes = (rem_secs % 3600) / 60;
    let seconds = rem_secs % 60;

    let mut year = 1970;
    let mut d = days;
    loop {
        let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
        let days_in_year = if leap { 366 } else { 365 };
        if d < days_in_year {
            break;
        }
        d -= days_in_year;
        year += 1;
    }

    let leap = (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0);
    let month_days = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1;
    for &md in &month_days {
        if d < md {
            break;
        }
        d -= md;
        month += 1;
    }
    let day = d + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::{ColorScheme, DomSnapshot, Image, NavOutcome, NetworkLog, Viewport};
    use url::Url;

    #[test]
    fn test_evidence_writer_atomic() {
        let temp = tempfile::tempdir().unwrap();
        let library_root = temp.path().join("lib");

        let evidence = CapturedEvidence {
            url: Url::parse("https://example.com/pricing").unwrap(),
            nav_outcome: NavOutcome {
                status: 200,
                url: "https://example.com/pricing".to_string(),
                timing_ms: 220,
            },
            snapshots: vec![(
                Viewport {
                    name: "desktop".to_string(),
                    width: 1920,
                    height: 1080,
                    dpr: 1.0,
                },
                ColorScheme::Light,
                DomSnapshot {
                    raw_json: "{\"dom\":1}".to_string(),
                    node_count: 1,
                },
            )],
            screenshots: vec![(
                Viewport {
                    name: "desktop".to_string(),
                    width: 1920,
                    height: 1080,
                    dpr: 1.0,
                },
                ColorScheme::Light,
                Image {
                    bytes: vec![1, 2, 3],
                    mime: "image/png".to_string(),
                    width: 1920,
                    height: 1080,
                },
                false,
            )],
            network_log: NetworkLog::default(),
            captured_at: "2026-11-26T07:45:00Z".to_string(),
        };

        let final_dir = EvidenceWriter::write_atomic(&library_root, &evidence).unwrap();
        assert!(final_dir.exists());
        assert!(final_dir.join("manifest.json").exists());
        assert!(final_dir.join("report.md").exists());
        assert!(final_dir.join("evidence/dom/desktop-light.json").exists());
        assert!(final_dir.join("evidence/network.json").exists());
        assert!(final_dir
            .join("screenshots/desktop/light-viewport.png")
            .exists());

        // Ensure no leftover .partial directory exists
        let partial_dir = final_dir.with_extension("partial");
        assert!(!partial_dir.exists());
    }
}
