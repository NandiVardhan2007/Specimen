# Security Policy & Threat Model

Specimen treats security as a core architectural invariant. All crates enforce `#![forbid(unsafe_code)]`.

## Threat Model

### 1. Target Website SSRF & IP Validation
- **Risk**: An attacker supplies a URL targeting internal network services (e.g. `http://169.254.169.254/latest/meta-data` or `http://localhost:8080/admin`).
- **Mitigation**: Every URL is resolved via DNS before connection. Resolved IP addresses are checked against IPv4/IPv6 private ranges, loopbacks, broadcast, and link-local subnets. Any private match halts execution unless `--allow-private` is explicitly enabled. Raw IP literal hosts are blocked.

### 2. Browser Sandbox & Ephemeral Execution
- **Risk**: Browser persistence leaking cookies or executing malicious scripts that persist across runs.
- **Mitigation**: Specimen launches headless Chrome with a freshly created isolated temporary user profile directory (`--user-data-dir`). On exit, the entire temporary directory is securely destroyed.

### 3. Local Web Server (Invariant 8)
- **Risk**: Malicious websites executing cross-site requests or DNS rebinding against the local web UI (`specimen serve`).
- **Mitigation**:
  - The server binds strictly to `127.0.0.1`.
  - All requests pass through `loopback_security_middleware`. Any request with a `Host` header not matching `127.0.0.1:<port>` or `localhost:<port>` is immediately rejected with `403 Forbidden`.
  - Strict `Content-Security-Policy` and `X-Frame-Options: DENY` prevent clickjacking and arbitrary script inclusion.

### 4. Malicious SVG Icons
- **Risk**: SVG vector assets containing `<script>` tags, inline event handlers (`onload=`), or external entity injection (XXE).
- **Mitigation**: All extracted SVG elements pass through `SvgSanitizer::sanitize`, which recursively strips scripts, styles, foreignObjects, iframes, and `on*` event attributes before saving to disk.

### 5. Stored Secrets & Credentials (Invariant 5)
- **Risk**: Auth tokens or session cookies written to git repository files.
- **Mitigation**: Evidence writers filter headers (`Cookie`, `Set-Cookie`, `Authorization`) and URL query parameters (`token=`, `session=`, `auth=`).
