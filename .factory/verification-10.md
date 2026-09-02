# Independent verification 10 — FAIL

**Work order:** `infra-test-evidence-verify-10`
**Candidate commit:** `b6ffdd41e6962486522a54402ce26840e7b6ab54`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-02 UTC

## Decision

**FAIL.** The deployed site matches the candidate's public build, all 22 declared claims pass, and the local, browser, privacy, accessibility, and performance checks pass. A fresh installed release CLI nevertheless leaves common cloud resource identifiers in all shareable evidence outputs. That violates the brief's requirement to redact resource identifiers by default and the product's own redaction statement.

The checkout does not contain `.factory/brief.json`; the researched brief in the work order was used as the acceptance contract.

## P1 release blocker — incomplete default resource-identifier redaction

From the crate produced by `cargo package --locked`, I installed the CLI into a fresh consumer root and converted a valid failed Terraform-style event stream. The plan output and failure message contained these representative values:

```text
subnet-0123456789abcdef0
sg-0123456789abcdef0
/subscriptions/11111111-2222-3333-4444-555555555555/resourceGroups/prod/providers/Microsoft.Compute/virtualMachines/api-01
projects/acme-prod/zones/us-central1-a/instances/api-01
```

The release command completed successfully and printed:

```json
{"checks":1,"errors":[],"valid":true}
```

Every value was present verbatim in all of these generated artifacts:

- `report.xml`
- `evidence/evidence.json`
- `evidence/index.html`

The leak includes plan-output fields and failure text. The generated page states, “Sensitive values and resource identifiers are redacted by default.” The landed source's free-text detector only identifies AWS ARNs and EC2 instance IDs, and its key-based filtering does not identify `resource_ref`. The declared `resource-identifier-redaction` fixture covers only the AWS ARN and EC2 forms, so it passes without exercising this multi-cloud case.

This is P1/release-blocking: the intended output is a shareable CI artifact, and the product must redact cloud resource identifiers by default.

Reproduction evidence remains in this verifier container at `/tmp/ite-verify10-3Y5jLz`.

## Required first checks

### Declared claims — PASS

After `npm ci` from the candidate checkout, each exact command in `.factory/claims.json` was run separately. All 22 passed:

`cli-demo`, `cli-recording`, `cli-conversion`, `sensitive-redaction`, `resource-identifier-redaction`, `sensitivity-fail-closed`, `strict-validation`, `malformed-duration-types`, `event-stream-validation`, `summary-consistency`, `run-correlation`, `sensitive-diagnostics`, `conversion-only`, `requested-path-writes`, `artifact-private`, `reader-private`, `site-demo`, `demo-artifact-fidelity`, `browser-record-import`, `help-options`, `json-validation-output`, and `mit-license`.

Each executed test had the required single `@claim:<id>` tag. The successful AWS-only claim does not override the independently reproduced behavior above.

### Cold first read — PASS

At a cold live desktop load the first screen says, “Turn infrastructure tests into reviewable evidence.” It identifies infrastructure-module maintainers and failed OpenTofu or Terraform tests. The visible first action is “Try it with sample data,” with adjacent text saying it will show a failed test, redaction, and output files. The one-click route opens `/demo/?demo=1`; its persistent banner says “Demo — sample data, nothing is saved” and provides Reset demo and Start for real.

## Local and packaged checks

The following completed successfully against the candidate:

```text
npm ci
npm run check                 # ESLint, tsc, 7 Rust tests, 26 Vitest tests
npm run build                 # dist/site/
npm run package:check
npm run consumer:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high  # 0 vulnerabilities
npm run qa:a11y               # 2 projects passed
```

`npm run qa:browser` exercised the 22 desktop/390 px scenarios; the individually required browser claim runs also all passed. The isolated package installation used the public CLI, not the workspace binary. The normal bundled demo created JUnit, JSON, and static HTML artifacts; malformed input, invalid durations, unknown options, and missing input returned the documented failure codes, and a subsequent valid conversion succeeded.

The production build contains 8.12 kB JavaScript (3.37 kB gzip combined) and 11.72 kB CSS (3.37 kB gzip), well within the static bundle budget.

## Live deployment, privacy, and accessibility

Fresh Playwright contexts exercised the live landing page, demo, malformed file error and valid-file recovery, policy pages, and 404 page.

- Only `https://infra-test-evidence.sociobot.in` was requested during the reader/demo flow. There were no cookies, localStorage keys, sessionStorage keys, IndexedDB databases, console errors, or page errors.
- Root, demo, privacy, terms, and unknown-route documents had `lang=en`, one `main`, and one `h1`; the unknown route returned HTTP 404.
- Keyboard Tab reaches the skip link with a visible `rgb(7, 90, 158) solid 3px` focus outline. At 390 px there was no horizontal overflow, no measured link/button below 44 px, and reduced motion set the reader transition to `0.00001s`.
- Axe on root, demo, privacy, terms, and 404 found zero serious or critical findings. The repository accessibility suite also passed in both authored browser projects.
- `/opt/fleet/lib/verify-url.sh` passed: 845 ms cold load, no errors, title, `lang`, `h1`, `main`, image alt text, and button names all present. Evidence: `/tmp/ite-verify-url.Qb7MJc`.

Live headers include same-origin CSP with `frame-ancestors 'none'`, HSTS, `nosniff`, `X-Frame-Options: DENY`, strict referrer policy, and restrictive Permissions Policy. HTML uses `max-age=30`; hashed assets use `public, max-age=31536000, immutable`.

Lighthouse (mobile default) scored 100 Performance, 100 Accessibility, 100 Best Practices, and 100 SEO. FCP/LCP were 1.0 s, TBT 50 ms, CLS 0, and total transfer 11 KiB. Report: `/tmp/ite-lighthouse-verify10.json`.

All 16 public files from `dist/site/` match live response bytes. The only non-comparable local file is `staticwebapp.config.json`, which is deployment configuration and is correctly not served as a public asset.

This is a static reader and CLI: it has no product API, sign-in, payment, service worker, backend state, or product-unlock endpoint. API allowance/429, authentication authority, persistence/concurrency, and PWA update/offline checks are not applicable.

## Required repair

Implement a provider-appropriate default identifier-redaction policy that covers common AWS VPC identifiers, Azure resource IDs, and GCP resource paths in both structured fields and free text. Add one cross-provider claim fixture that scans JUnit, JSON, and HTML outputs for all sentinels, then rerun the full gate and this independent probe.
