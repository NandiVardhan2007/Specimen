pub mod types;

pub use types::*;
use url::Url;

/// The Session trait defines the low-level interaction surface between Specimen
/// and a browser tab (or recorded mock session).
pub trait Session: Send {
    fn navigate(
        &mut self,
        url: &Url,
    ) -> impl std::future::Future<Output = Result<NavOutcome>> + Send;
    fn settle(
        &mut self,
        policy: &SettlePolicy,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn emulate(
        &mut self,
        vp: &Viewport,
        scheme: ColorScheme,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn snapshot(&mut self) -> impl std::future::Future<Output = Result<DomSnapshot>> + Send;
    fn screenshot(
        &mut self,
        opts: &ShotOpts,
    ) -> impl std::future::Future<Output = Result<Image>> + Send;
    fn force_pseudo(
        &mut self,
        node_id: u32,
        states: &[Pseudo],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn animations(
        &mut self,
    ) -> impl std::future::Future<Output = Result<Vec<AnimationRecord>>> + Send;
    fn sample_frames(&mut self) -> impl std::future::Future<Output = Result<Trace>> + Send;
    fn network_log(&self) -> NetworkLog;
    fn scroll_by(&mut self, delta_y: i32) -> impl std::future::Future<Output = Result<()>> + Send;
    fn hover(&mut self, selector: &str) -> impl std::future::Future<Output = Result<()>> + Send;
    fn click(
        &mut self,
        selector: Option<&str>,
        text: Option<&str>,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn type_text(
        &mut self,
        selector: &str,
        text: &str,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn press_key(&mut self, key: &str) -> impl std::future::Future<Output = Result<()>> + Send;
    fn wait_for_selector(
        &mut self,
        selector: &str,
        timeout_ms: u64,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
    fn evaluate(&mut self, script: &str) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// In-memory mock session for deterministic testing of drivers and extractors.
#[derive(Debug, Clone, Default)]
pub struct MockSession {
    pub current_url: Option<Url>,
    pub current_viewport: Option<Viewport>,
    pub current_scheme: Option<ColorScheme>,
    pub nav_outcome: Option<NavOutcome>,
    pub snapshot_data: Option<DomSnapshot>,
    pub screenshot_data: Option<Image>,
    pub animations_data: Vec<AnimationRecord>,
    pub network_entries: Vec<NetworkEntry>,
    pub scroll_positions: Vec<i32>,
    pub recorded_actions: Vec<String>,
}

impl Session for MockSession {
    async fn navigate(&mut self, url: &Url) -> Result<NavOutcome> {
        self.current_url = Some(url.clone());
        let outcome = self.nav_outcome.clone().unwrap_or_else(|| NavOutcome {
            status: 200,
            url: url.to_string(),
            timing_ms: 150,
        });
        Ok(outcome)
    }

    async fn settle(&mut self, _policy: &SettlePolicy) -> Result<()> {
        Ok(())
    }

    async fn emulate(&mut self, vp: &Viewport, scheme: ColorScheme) -> Result<()> {
        self.current_viewport = Some(vp.clone());
        self.current_scheme = Some(scheme);
        Ok(())
    }

    async fn snapshot(&mut self) -> Result<DomSnapshot> {
        Ok(self.snapshot_data.clone().unwrap_or_else(|| DomSnapshot {
            raw_json: "{\"documents\":[]}".to_string(),
            node_count: 1,
        }))
    }

    async fn screenshot(&mut self, _opts: &ShotOpts) -> Result<Image> {
        Ok(self.screenshot_data.clone().unwrap_or_else(|| Image {
            bytes: vec![137, 80, 78, 71], // PNG header
            mime: "image/png".to_string(),
            width: 1920,
            height: 1080,
        }))
    }

    async fn force_pseudo(&mut self, _node_id: u32, _states: &[Pseudo]) -> Result<()> {
        Ok(())
    }

    async fn animations(&mut self) -> Result<Vec<AnimationRecord>> {
        Ok(self.animations_data.clone())
    }

    async fn sample_frames(&mut self) -> Result<Trace> {
        Ok(Trace::default())
    }

    fn network_log(&self) -> NetworkLog {
        NetworkLog {
            entries: self.network_entries.clone(),
        }
    }

    async fn scroll_by(&mut self, delta_y: i32) -> Result<()> {
        self.scroll_positions.push(delta_y);
        Ok(())
    }

    async fn hover(&mut self, selector: &str) -> Result<()> {
        self.recorded_actions.push(format!("hover:{}", selector));
        Ok(())
    }

    async fn click(&mut self, selector: Option<&str>, text: Option<&str>) -> Result<()> {
        self.recorded_actions
            .push(format!("click:{:?}:{:?}", selector, text));
        Ok(())
    }

    async fn type_text(&mut self, selector: &str, text: &str) -> Result<()> {
        self.recorded_actions
            .push(format!("type:{}:{}", selector, text));
        Ok(())
    }

    async fn press_key(&mut self, key: &str) -> Result<()> {
        self.recorded_actions.push(format!("press:{}", key));
        Ok(())
    }

    async fn wait_for_selector(&mut self, selector: &str, timeout_ms: u64) -> Result<()> {
        self.recorded_actions
            .push(format!("wait_selector:{}:{}ms", selector, timeout_ms));
        Ok(())
    }

    async fn evaluate(&mut self, script: &str) -> Result<()> {
        self.recorded_actions.push(format!("eval:{}", script));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_session_workflow() {
        let mut session = MockSession::default();
        let target_url = Url::parse("https://example.com/test").unwrap();

        let outcome = session.navigate(&target_url).await.unwrap();
        assert_eq!(outcome.status, 200);

        let vp = Viewport {
            name: "desktop".to_string(),
            width: 1920,
            height: 1080,
            dpr: 1.0,
        };
        session.emulate(&vp, ColorScheme::Dark).await.unwrap();
        assert_eq!(session.current_scheme, Some(ColorScheme::Dark));

        session.scroll_by(400).await.unwrap();
        assert_eq!(session.scroll_positions, vec![400]);

        let snap = session.snapshot().await.unwrap();
        assert_eq!(snap.node_count, 1);
    }
}
