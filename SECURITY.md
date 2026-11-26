# Security Policy

## Principles

1. **Local and Private**: Specimen operates locally on your machine with no external telemetry or cloud dependencies.
2. **Strict Loopback Binding**: Local web and MCP servers bind exclusively to `127.0.0.1` and validate Host headers.
3. **Path Validation**: All file access paths are validated via `RelPath` to prevent directory traversal and arbitrary file reads/writes.
4. **Asset Sanitization**: Downloaded and extracted SVGs are strictly sanitized before rendering or saving.
5. **No Secrets Retention**: Specimen redacts sensitive authorization headers and cookies from captured network evidence.

## Reporting Vulnerabilities

If you identify a security issue, please contact the maintainers directly through a private security advisory.
