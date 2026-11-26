use url::Url;

use crate::policy::RateLimiter;
use crate::session::{
    ColorScheme, DomSnapshot, Image, NavOutcome, NetworkLog, Result, Session, SettlePolicy,
    ShotOpts, Viewport,
};

#[derive(Debug, Clone)]
pub struct CapturedEvidence {
    pub url: Url,
    pub nav_outcome: NavOutcome,
    pub snapshots: Vec<(Viewport, ColorScheme, DomSnapshot)>,
    pub screenshots: Vec<(Viewport, ColorScheme, Image, bool)>, // (vp, scheme, image, is_full_page)
    pub network_log: NetworkLog,
    pub captured_at: String,
}

pub struct DriverOptions {
    pub viewports: Vec<Viewport>,
    pub schemes: Vec<ColorScheme>,
    pub settle_policy: SettlePolicy,
    pub scroll_steps: usize,
    pub scroll_step_px: i32,
}

impl Default for DriverOptions {
    fn default() -> Self {
        Self {
            viewports: vec![
                Viewport {
                    name: "mobile".to_string(),
                    width: 390,
                    height: 844,
                    dpr: 2.0,
                },
                Viewport {
                    name: "desktop".to_string(),
                    width: 1920,
                    height: 1080,
                    dpr: 1.0,
                },
            ],
            schemes: vec![ColorScheme::Light, ColorScheme::Dark],
            settle_policy: SettlePolicy::default(),
            scroll_steps: 3,
            scroll_step_px: 400,
        }
    }
}

/// SinglePage driver executes the capture sequence for a single URL across configured
/// viewports and color schemes (SPEC §4.1).
pub struct SinglePageDriver;

impl SinglePageDriver {
    pub async fn run<S: Session>(
        session: &mut S,
        url: &Url,
        options: &DriverOptions,
        rate_limiter: Option<&RateLimiter>,
    ) -> Result<CapturedEvidence> {
        // 1. Rate limiting per host
        if let Some(limiter) = rate_limiter {
            if let Some(host) = url.host_str() {
                limiter.wait_turn(host).await;
            }
        }

        // 2. Initial navigation
        let nav_outcome = session.navigate(url).await?;

        let mut snapshots = Vec::new();
        let mut screenshots = Vec::new();

        // 3. Loop over viewports × color schemes
        for vp in &options.viewports {
            for &scheme in &options.schemes {
                // Emulate viewport and color scheme
                session.emulate(vp, scheme).await?;
                session.settle(&options.settle_policy).await?;

                // Settle and scroll through page to trigger lazy-loaded assets and reveals (SPEC §4.1)
                for _ in 0..options.scroll_steps {
                    session.scroll_by(options.scroll_step_px).await?;
                }
                // Scroll back to top
                let total_scrolled = (options.scroll_steps as i32) * options.scroll_step_px;
                if total_scrolled > 0 {
                    session.scroll_by(-total_scrolled).await?;
                }
                session.settle(&options.settle_policy).await?;

                // Capture DOM Snapshot
                let snap = session.snapshot().await?;
                snapshots.push((vp.clone(), scheme, snap));

                // Above-the-fold / Viewport screenshot
                let vp_shot = session
                    .screenshot(&ShotOpts {
                        full_page: false,
                        quality: 90,
                    })
                    .await?;
                screenshots.push((vp.clone(), scheme, vp_shot, false));

                // Full-page screenshot
                let full_shot = session
                    .screenshot(&ShotOpts {
                        full_page: true,
                        quality: 90,
                    })
                    .await?;
                screenshots.push((vp.clone(), scheme, full_shot, true));
            }
        }

        let network_log = session.network_log();
        let captured_at = chrono_iso_now();

        Ok(CapturedEvidence {
            url: url.clone(),
            nav_outcome,
            snapshots,
            screenshots,
            network_log,
            captured_at,
        })
    }
}

fn chrono_iso_now() -> String {
    // Generate ISO 8601 UTC timestamp without relying on external crates
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    // Format YYYY-MM-DDTHH:MM:SSZ
    format_timestamp_secs(secs)
}

fn format_timestamp_secs(total_secs: u64) -> String {
    let days = total_secs / 86400;
    let rem_secs = total_secs % 86400;
    let hours = rem_secs / 3600;
    let minutes = (rem_secs % 3600) / 60;
    let seconds = rem_secs % 60;

    // Simple Gregorian calendar calculation from 1970-01-01
    let mut year = 1970;
    let mut d = days;

    loop {
        let leap = is_leap_year(year);
        let days_in_year = if leap { 366 } else { 365 };
        if d < days_in_year {
            break;
        }
        d -= days_in_year;
        year += 1;
    }

    let leap = is_leap_year(year);
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

fn is_leap_year(year: u64) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::MockSession;

    #[tokio::test]
    async fn test_single_page_driver_run() {
        let mut session = MockSession::default();
        let target_url = Url::parse("https://example.com/").unwrap();
        let options = DriverOptions {
            viewports: vec![Viewport {
                name: "desktop".to_string(),
                width: 1920,
                height: 1080,
                dpr: 1.0,
            }],
            schemes: vec![ColorScheme::Light],
            settle_policy: SettlePolicy::default(),
            scroll_steps: 2,
            scroll_step_px: 300,
        };

        let evidence = SinglePageDriver::run(&mut session, &target_url, &options, None)
            .await
            .unwrap();

        assert_eq!(evidence.url, target_url);
        assert_eq!(evidence.snapshots.len(), 1);
        assert_eq!(evidence.screenshots.len(), 2); // 1 viewport + 1 full-page
        assert_eq!(session.scroll_positions, vec![300, 300, -600]);
    }
}
