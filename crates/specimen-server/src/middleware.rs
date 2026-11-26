#![forbid(unsafe_code)]

use axum::extract::Request;
use axum::http::{header, HeaderValue, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

/// Security middleware enforcing loopback Host header checks (DNS rebinding defense)
/// and injecting strict Content-Security-Policy headers (Invariant 8 & SPEC §10).
pub async fn loopback_security_middleware(req: Request, next: Next) -> Response {
    // 1. Host header validation
    if let Some(host_val) = req.headers().get(header::HOST) {
        if let Ok(host_str) = host_val.to_str() {
            let host_part = host_str.split(':').next().unwrap_or(host_str);
            if host_part != "127.0.0.1" && host_part != "localhost" {
                return (
                    StatusCode::FORBIDDEN,
                    "Access denied: Host header does not match loopback interface (DNS rebinding protection)",
                )
                    .into_response();
            }
        }
    }

    // 2. Proceed with handler execution
    let mut resp = next.run(req).await;

    // 3. Attach strict security headers
    let headers = resp.headers_mut();
    headers.insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static(
            "default-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline'; img-src 'self' data:; frame-ancestors 'none';",
        ),
    );
    headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    headers.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));

    resp
}
