# Frequently Asked Questions (FAQ)

### What browsers are supported?
Specimen works with any Chromium-based browser supporting the Chrome DevTools Protocol (CDP), including **Google Chrome**, **Microsoft Edge**, **Brave**, and **Chromium**.

### How does Specimen handle password fields?
In scripted flows and headed captures, input elements with `type="password"` or name attributes matching password heuristics are never recorded in plaintext. Evidence network logs automatically redact authorization tokens and cookies.

### Can I run Specimen on private networks / localhost?
By default, Specimen blocks all RFC 1918 private IP ranges, loopback addresses (`127.0.0.1`, `localhost`), and link-local addresses (`169.254.169.254` cloud metadata) to prevent Server-Side Request Forgery (SSRF). You can explicitly pass `--allow-private` for local staging environment development.

### How are duplicate tokens merged?
When capturing multiple pages or sources with identical values, Specimen computes a SHA-256 hash over canonical JSON. If the hash matches an existing item, Specimen updates the existing `item.json` by adding the new capture to its `provenance` array without duplicating the file.
