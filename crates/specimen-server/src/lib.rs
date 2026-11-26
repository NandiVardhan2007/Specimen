#![forbid(unsafe_code)]

//! Specimen Server: local web UI for browsing design libraries.

pub mod handlers;
pub mod middleware;
pub mod state;
pub mod views;

use axum::middleware as axum_mw;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use std::path::PathBuf;

pub use middleware::loopback_security_middleware;
pub use state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::handle_home))
        .route("/collections/{coll_type}", get(handlers::handle_collection))
        .route("/items/{item_id}", get(handlers::handle_item_detail))
        .route("/sources", get(handlers::handle_sources))
        .route("/search", get(handlers::handle_search))
        .route("/export", get(handlers::handle_export))
        .route("/api/status", get(handlers::handle_api_status))
        .layer(axum_mw::from_fn(loopback_security_middleware))
        .with_state(state)
}

/// Runs the local web UI strictly on 127.0.0.1:<port> (Invariant 8).
pub async fn run_server(
    library_root: PathBuf,
    port: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let state = AppState::new(library_root);
    let app = build_router(state);

    // INVARIANT 8: Loopback only bind
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Specimen web app running at http://127.0.0.1:{}", port);

    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{header, Request, StatusCode};
    use tower::ServiceExt; // for oneshot

    #[tokio::test]
    async fn test_server_status_and_security_headers() {
        let temp = tempfile::tempdir().unwrap();
        let state = AppState::new(temp.path().to_path_buf());
        let app = build_router(state);

        let req = Request::builder()
            .uri("/api/status")
            .header(header::HOST, "127.0.0.1:4000")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(resp.headers().contains_key(header::CONTENT_SECURITY_POLICY));
        assert_eq!(resp.headers().get(header::X_FRAME_OPTIONS).unwrap(), "DENY");
    }

    #[tokio::test]
    async fn test_dns_rebinding_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let state = AppState::new(temp.path().to_path_buf());
        let app = build_router(state);

        let req = Request::builder()
            .uri("/api/status")
            .header(header::HOST, "attacker.com")
            .body(Body::empty())
            .unwrap();

        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
    }
}
