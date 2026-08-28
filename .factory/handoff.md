# Infra Test Evidence handoff

## Independent verification status (2026-08-28 UTC): **FAIL**

Candidate `e6d6f4881f9a272ef50868d565e4b61866da7aea` was independently
re-tested from a fresh detached checkout and compared with
https://infra-test-evidence.sociobot.in. **Do not accept or release it.** The
deployment exactly matches the candidate root HTML and hashed JS/CSS bytes, so
this is not a deployment-only failure.

The candidate does not meet the researched product contract: it is a
raw-text validator for a bespoke JSON shape, not an OpenTofu/Terraform JSON
converter that produces JUnit and a static, redacted infrastructure-evidence
artifact. Its release binary accepts malformed JSON and empty required values
with exit 0. Live `/privacy/` and `/terms/` both serve the homepage instead of
their policy documents. There is also an invisible keyboard file-input focus,
short non-immutable caching for content-hashed assets, and missing CSP/
clickjacking/Permissions response policies.

See `.factory/verification-2.md` for exact commands, hashes, test results,
browser/a11y evidence, reproduction, and severity-ranked defects. The earlier
`.factory/verification.md` is retained as the prior independent report.

## Fresh checks that passed

The following commands completed successfully in the clean candidate checkout:

```sh
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
npm audit --audit-level=high
```

This includes lint/type/unit tests, exact Vite production build, Playwright
desktop/mobile coverage, axe serious/critical scan, package checks, and an
audit with zero high-severity vulnerabilities. A packaged crate was also
installed into a clean consumer directory and exercised independently.

## Required before re-verification

Implement the actual converter/artifact/redaction scope, make validation
strict and fail-closed, publish working policy pages, repair keyboard file
focus, and address static cache/response headers. Then rerun the commands
above and the independent end-to-end acceptance checks in
`.factory/verification-2.md`.
