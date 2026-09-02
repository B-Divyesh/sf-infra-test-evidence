# Infra Test Evidence verification 10 handoff

**Candidate:** `b6ffdd41e6962486522a54402ce26840e7b6ab54`
**Live URL:** https://infra-test-evidence.sociobot.in
**Result:** **FAIL**

Independent QA is recorded in `.factory/verification-10.md`.

## Release blocker

The freshly packaged and installed CLI does not redact common cloud resource identifiers by default. Valid input containing an AWS subnet ID, security-group ID, Azure resource ID, and GCP instance path leaked every value into JUnit XML, reviewer JSON, and reviewer HTML, while the command exited 0. This contradicts the researched brief and the output's “resource identifiers are redacted by default” statement.

The declared redaction claims pass, but their resource fixture is limited to AWS ARN and EC2 instance forms. Verification evidence is at `/tmp/ite-verify10-3Y5jLz` in this container.

## What passed

- All 22 exact `.factory/claims.json` commands after `npm ci`.
- `npm run check`, production build, package/consumer checks, formatting, strict Clippy, and high-severity npm audit.
- Browser demo/import/error/recovery, privacy request/storage checks, keyboard focus, 390 px layout, reduced motion, and accessibility checks.
- Live `verify-url.sh`, axe serious/critical scans, security/cache headers, and Lighthouse 100/100/100/100 (1.0 s LCP; 11 KiB transfer).
- All 16 public production artifacts match the deployed bytes.

## Next step

Expand default redaction across common AWS, Azure, and GCP identifiers in structured values and free text. Add a cross-provider claim that scans all three generated artifact types, then rerun the complete gate and verification.

No product code was changed by this verifier. To reproduce the ordinary checks:

```text
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
```
