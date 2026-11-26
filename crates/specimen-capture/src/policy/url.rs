use std::net::{IpAddr, ToSocketAddrs};
use std::str::FromStr;
use url::Url;

use crate::policy::error::{PolicyError, Result};
use crate::policy::ip::is_private_or_restricted;

/// Validates scheme and direct IP target of a URL without performing network I/O.
pub fn validate_url_syntax(url: &Url, allow_private: bool) -> Result<()> {
    match url.scheme() {
        "http" | "https" => {}
        other => return Err(PolicyError::DisallowedScheme(other.to_string())),
    }

    if let Some(host_str) = url.host_str() {
        // If the host is a raw IP literal (e.g. http://127.0.0.1 or http://[::1])
        if let Ok(ip) = IpAddr::from_str(host_str.trim_start_matches('[').trim_end_matches(']')) {
            if !allow_private && is_private_or_restricted(ip) {
                return Err(PolicyError::DisallowedPrivateIp(ip.to_string()));
            }
        }
    } else {
        return Err(PolicyError::InvalidUrl("Missing host".to_string()));
    }

    Ok(())
}

/// Resolves the hostname of the URL via DNS and verifies that no resolved address
/// belongs to a private or restricted network range (SSRF / DNS rebinding protection).
pub fn resolve_and_validate_host(url: &Url, allow_private: bool) -> Result<Vec<IpAddr>> {
    validate_url_syntax(url, allow_private)?;

    let host = url
        .host_str()
        .ok_or_else(|| PolicyError::InvalidUrl("Missing host".to_string()))?;
    let port = url.port_or_known_default().unwrap_or(80);
    let socket_str = format!("{}:{}", host, port);

    let addrs: Vec<IpAddr> = socket_str
        .to_socket_addrs()
        .map_err(|e| PolicyError::DnsResolutionFailed(host.to_string(), e.to_string()))?
        .map(|sa| sa.ip())
        .collect();

    if addrs.is_empty() {
        return Err(PolicyError::DnsResolutionFailed(
            host.to_string(),
            "No IP addresses resolved".to_string(),
        ));
    }

    if !allow_private {
        for &ip in &addrs {
            if is_private_or_restricted(ip) {
                return Err(PolicyError::ResolvedToPrivateIp(
                    host.to_string(),
                    ip.to_string(),
                ));
            }
        }
    }

    Ok(addrs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_validation() {
        assert!(validate_url_syntax(&Url::parse("https://example.com").unwrap(), false).is_ok());
        assert!(
            validate_url_syntax(&Url::parse("http://example.com/test").unwrap(), false).is_ok()
        );
        assert!(validate_url_syntax(&Url::parse("ftp://example.com").unwrap(), false).is_err());
        assert!(validate_url_syntax(&Url::parse("file:///etc/passwd").unwrap(), false).is_err());
        assert!(validate_url_syntax(&Url::parse("javascript:alert(1)").unwrap(), false).is_err());
    }

    #[test]
    fn test_raw_ip_validation() {
        let private_url = Url::parse("http://127.0.0.1:8080/").unwrap();
        assert_eq!(
            validate_url_syntax(&private_url, false),
            Err(PolicyError::DisallowedPrivateIp("127.0.0.1".to_string()))
        );
        assert!(validate_url_syntax(&private_url, true).is_ok());

        let public_url = Url::parse("http://8.8.8.8/").unwrap();
        assert!(validate_url_syntax(&public_url, false).is_ok());
    }

    #[test]
    fn test_localhost_resolution_blocked() {
        let local_url = Url::parse("http://localhost:3000/").unwrap();
        let res = resolve_and_validate_host(&local_url, false);
        assert!(res.is_err());
        // With allow_private = true, it should succeed
        assert!(resolve_and_validate_host(&local_url, true).is_ok());
    }
}
