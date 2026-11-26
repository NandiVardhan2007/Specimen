# Local Web Interface

Specimen provides a built-in server-rendered web UI for browsing your design library without requiring Node.js or any external web servers.

To start the interface:

```bash
specimen serve --port 4000 --open
```

## Security Invariants (Loopback Only)

The web interface is built strictly around **Invariant 8**:
- **Loopback-Only Bind**: Binds exclusively to `127.0.0.1`. Never listens on `0.0.0.0` or external network adapters.
- **DNS Rebinding Defense**: Every HTTP request is checked by an Axum security middleware. Requests with non-loopback `Host` headers (e.g. `attacker.com`) are rejected with `403 Forbidden`.
- **Strict Content-Security-Policy (CSP)**: `default-src 'self'; script-src 'self' 'unsafe-inline'; frame-ancestors 'none';`.
- **Anti-Clickjacking**: Emits `X-Frame-Options: DENY` and `X-Content-Type-Options: nosniff`.

## Features
- **Overview Dashboard**: High-level counts across all 7 token collections and capture source history.
- **Collection Browsers**: Specialized grid views for Palettes, Typography, Components, Icons, Effects, Animations, and Layouts.
- **Item Detail**: Full canonical JSON payloads, WCAG contrast evaluations, and similarity links.
- **Side-by-Side Comparison & Merge Review**: Navigate to `/compare?a=<id>&b=<id>` to inspect differences between two tokens side-by-side.
- **Export Sandbox**: Instant live previews of CSS, Tailwind, SCSS, and DTCG exports.
