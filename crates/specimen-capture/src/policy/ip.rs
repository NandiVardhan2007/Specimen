use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// Checks if an IP address falls into private, loopback, link-local, or reserved ranges.
pub fn is_private_or_restricted(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(ipv4) => is_private_v4(ipv4),
        IpAddr::V6(ipv6) => is_private_v6(ipv6),
    }
}

fn is_private_v4(ip: Ipv4Addr) -> bool {
    let octets = ip.octets();

    // 0.0.0.0/8 (Broadcast/Current network)
    if octets[0] == 0 {
        return true;
    }

    // 10.0.0.0/8 (Private)
    if octets[0] == 10 {
        return true;
    }

    // 100.64.0.0/10 (Carrier-Grade NAT)
    if octets[0] == 100 && (octets[1] & 0b1100_0000) == 0b0100_0000 {
        return true;
    }

    // 127.0.0.0/8 (Loopback)
    if octets[0] == 127 {
        return true;
    }

    // 169.254.0.0/16 (Link-local / Cloud metadata service)
    if octets[0] == 169 && octets[1] == 254 {
        return true;
    }

    // 172.16.0.0/12 (Private)
    if octets[0] == 172 && (octets[1] >= 16 && octets[1] <= 31) {
        return true;
    }

    // 192.0.0.0/24 (IETF Protocol Assignments)
    if octets[0] == 192 && octets[1] == 0 && octets[2] == 0 {
        return true;
    }

    // 192.0.2.0/24 (TEST-NET-1)
    if octets[0] == 192 && octets[1] == 0 && octets[2] == 2 {
        return true;
    }

    // 192.168.0.0/16 (Private)
    if octets[0] == 192 && octets[1] == 168 {
        return true;
    }

    // 198.18.0.0/15 (Benchmarking)
    if octets[0] == 198 && (octets[1] == 18 || octets[1] == 19) {
        return true;
    }

    // 198.51.100.0/24 (TEST-NET-2)
    if octets[0] == 198 && octets[1] == 51 && octets[2] == 100 {
        return true;
    }

    // 203.0.113.0/24 (TEST-NET-3)
    if octets[0] == 203 && octets[1] == 0 && octets[2] == 113 {
        return true;
    }

    // 224.0.0.0/4 (Multicast)
    if octets[0] >= 224 && octets[0] <= 239 {
        return true;
    }

    // 240.0.0.0/4 (Reserved)
    if octets[0] >= 240 {
        return true;
    }

    false
}

fn is_private_v6(ip: Ipv6Addr) -> bool {
    // Check for IPv4-mapped IPv6 address (::ffff:x.x.x.x)
    if let Some(v4) = ip.to_ipv4_mapped() {
        return is_private_v4(v4);
    }

    let segments = ip.segments();

    // ::1 (Loopback)
    if ip.is_loopback() {
        return true;
    }

    // :: (Unspecified)
    if ip.is_unspecified() {
        return true;
    }

    // fe80::/10 (Link-local)
    if (segments[0] & 0xffc0) == 0xfe80 {
        return true;
    }

    // fc00::/7 (Unique Local Address)
    if (segments[0] & 0xfe00) == 0xfc00 {
        return true;
    }

    // ff00::/8 (Multicast)
    if (segments[0] & 0xff00) == 0xff00 {
        return true;
    }

    // 2001:db8::/32 (Documentation)
    if segments[0] == 0x2001 && segments[1] == 0x0db8 {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_private_ips() {
        assert!(is_private_or_restricted(
            IpAddr::from_str("127.0.0.1").unwrap()
        ));
        assert!(is_private_or_restricted(
            IpAddr::from_str("10.0.1.5").unwrap()
        ));
        assert!(is_private_or_restricted(
            IpAddr::from_str("172.20.0.1").unwrap()
        ));
        assert!(is_private_or_restricted(
            IpAddr::from_str("192.168.1.1").unwrap()
        ));
        assert!(is_private_or_restricted(
            IpAddr::from_str("169.254.169.254").unwrap()
        ));
        assert!(is_private_or_restricted(IpAddr::from_str("::1").unwrap()));
        assert!(is_private_or_restricted(
            IpAddr::from_str("fe80::1").unwrap()
        ));
        assert!(is_private_or_restricted(
            IpAddr::from_str("fc00::1").unwrap()
        ));
    }

    #[test]
    fn test_public_ips() {
        assert!(!is_private_or_restricted(
            IpAddr::from_str("8.8.8.8").unwrap()
        ));
        assert!(!is_private_or_restricted(
            IpAddr::from_str("1.1.1.1").unwrap()
        ));
        assert!(!is_private_or_restricted(
            IpAddr::from_str("2607:f8b0:4005:808::200e").unwrap()
        ));
    }
}
