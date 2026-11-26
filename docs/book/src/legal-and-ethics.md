# Legal, Ethics & Licensing

Specimen is built with a principled, ethics-first architecture to respect web standards, copyright law, and website operator rights.

## 1. Respect for Web Standards & `robots.txt`
- Specimen checks `robots.txt` rules by default before crawling.
- Specimen enforces configurable rate limits (`rate_limit_rps` in `specimen.toml`) to prevent excessive server load.

## 2. No Evasion Guarantee (Invariant 7)
Specimen has **zero evasion capabilities**:
- No CAPTCHA solvers or bypass mechanisms.
- No anti-bot obfuscation or fingerprint spoofing.
- If a target site blocks automated access, Specimen reports the status code and halts.

## 3. License Tracking (Invariant 6)
Every item extracted into a Specimen library contains an explicit `license` declaration:
- **Reusable**: Functional, uncopyrightable values such as raw hex color values, spacing scales, and standard CSS rules.
- **Reference-Only**: Proprietary brand icons, logos, typography files, or copyrighted layout graphics.
- **Unknown**: Items whose license basis cannot be automatically determined and require human audit.

## 4. No Secrets Stored (Invariant 5)
Specimen never stores user credentials:
- Password input fields (`type="password"`) are redacted.
- `Authorization`, `Cookie`, and `Set-Cookie` headers are stripped from network evidence.
- Session tokens in query parameters (`token=`, `session=`, `auth=`) are replaced with `[REDACTED]`.
