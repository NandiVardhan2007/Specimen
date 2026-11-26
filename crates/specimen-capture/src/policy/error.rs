use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PolicyError {
    #[error("Disallowed scheme '{0}'. Only 'http' and 'https' are permitted")]
    DisallowedScheme(String),

    #[error("Disallowed private or local IP address: {0} (use --allow-private to bypass)")]
    DisallowedPrivateIp(String),

    #[error("DNS resolution failed for '{0}': {1}")]
    DnsResolutionFailed(String, String),

    #[error("Resolved host '{0}' points to private/local IP: {1} (use --allow-private to bypass)")]
    ResolvedToPrivateIp(String, String),

    #[error("Target URL disallowed by robots.txt rule")]
    RobotsDisallowed,

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

pub type Result<T> = std::result::Result<T, PolicyError>;
