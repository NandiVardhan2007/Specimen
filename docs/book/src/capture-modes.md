# Capture Modes

Specimen provides four specialized capture modes to handle real-world web applications.

## 1. Single-Page Mode (`specimen capture <url>`)
The standard capture mode captures a single URL across:
- **Responsive Viewports**: Mobile (390x844 @2x) and Desktop (1920x1080 @1x).
- **Color Schemes**: Light mode and dark mode via CDP emulation (`emulateMediaFeatures`).
- **Full DOM & Style Tree**: Raw computed styles, stylesheets, and SVG elements.

## 2. Scripted Flow Mode (`specimen capture flow.yaml --mode flow`)
For multi-step web applications, dashboards, or animated UI states, flow mode executes declarative YAML scripts:

```yaml
start: "https://example.com/login"
steps:
  - action: "type"
    selector: "#username"
    text: "demo-user"
  - action: "type"
    selector: "#password"
    text: "secret-password"
  - action: "click"
    selector: "button[type='submit']"
  - action: "wait_for"
    selector: ".dashboard-container"
    timeout_ms: 10000
  - action: "capture"
    label: "dashboard-home"
```

**Security Invariant:** Password inputs (`type="password"`) are masked during recording, and authorization headers / query session tokens are scrubbed from on-disk evidence logs.

## 3. Headed Login Mode (`--headed`)
When a target application requires WebAuthn, OAuth, passkeys, or two-factor authentication:
```bash
specimen capture https://app.example.com --headed
```
Specimen launches a visible browser window, allowing you to manually complete authentication. After sign-in, the capture process resumes and extracts the design tokens without saving session credentials or cookies.

## 4. Crawl Mode (`specimen crawl <url>`)
For extracting the design tokens of an entire site:
```bash
specimen crawl https://docs.example.com --depth 2 --max-pages 15
```
- **Strict Same-Origin**: Enforces host boundary checks to prevent link hijacking.
- **Template Deduplication**: Detects structural DOM similarities to avoid capturing 100 identical blog post templates.
- **Group Summary**: Consolidates all pages into a unified `sources/<host>/crawl-<timestamp>` group with a master report.
