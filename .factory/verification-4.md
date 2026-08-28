# Independent QA verification 4 — FAIL

**Candidate:** `812fa4a95dff0f1941f4256fc8f0e4b8c0a7a791`

**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-08-28 UTC from a fresh detached GitHub clone at the candidate SHA.

## Verdict

**FAIL.** The public deployment is current and byte-for-byte matches the
candidate production build, so this is not a deployment-only failure. The
packaged CLI leaks an opaque value from a `test_plan` object explicitly marked
`sensitive: true` into both reviewer artifacts. This violates the brief's
default-redaction requirement and the CLI's own claim that sensitive values
are redacted by default. A generated reviewer artifact is intended for sharing,
so this is release blocking.

## Release-blocking defect

### P0 — Explicitly sensitive plan values are copied into shareable artifacts

The release-package binary accepts a complete JSON-lines stream containing:

```json
{"type":"test_plan","@testfile":"tests/sensitive.tftest.hcl","@testrun":"explicit_sensitive_output","test_plan":{"outputs":{"session":{"sensitive":true,"value":"k9M2qV7xL4"}},"resource_changes":[]}}
{"type":"test_run","@testfile":"tests/sensitive.tftest.hcl","@testrun":"explicit_sensitive_output","test_run":{"status":"pass","elapsed":0}}
{"type":"test_summary","test_summary":{"status":"pass"}}
```

It exits 0 and reports `{"checks":1,"errors":[],"valid":true}`. Its
`--evidence-dir` output then contains the opaque sentinel `k9M2qV7xL4` in both
`evidence.json` (case inputs and plan summary) and `index.html` (the embedded
artifact payload). The value deliberately contains no secret-like word; the
semantic `sensitive: true` declaration is what must protect it.

The redactor only identifies sensitive-looking field names/text and does not
honour explicit sensitive metadata or redact an enclosing marked subtree. This
is a direct disclosure path for plan/output values and breaks the core privacy
constraint. Add a regression fixture based on actual OpenTofu/Terraform JSON,
redact every value beneath an explicit sensitivity marker before creating any
JSON, JUnit, or HTML, and fail closed where sensitivity cannot be interpreted.

## Checks that passed

| Area | Fresh evidence |
| --- | --- |
| Clean clone/install | Fresh clone from `https://github.com/B-Divyesh/sf-infra-test-evidence.git`, detached at the candidate SHA; Node 22.23.2, npm 10.9.8, Cargo 1.98.0. `npm ci` completed with 0 vulnerabilities. |
| Quality gates | `npm run check` passed ESLint, TypeScript, 4 Rust tests, and 7 Vitest tests from the cold checkout. `cargo fmt --check`, `cargo clippy --locked --all-targets -- -D warnings`, and `npm audit --audit-level=high` also passed. |
| Build/budgets | `npm run build` passed. Initial JS is 3,662 B raw / 1,655 B gzip; CSS is 5,486 B raw / 2,004 B gzip—well under the 200 KB JS and 50 KB CSS budgets. Privacy, Terms, favicon, and static-host configuration are emitted. |
| Browser/a11y regression suite | `npm run qa:browser` passed 8/8 desktop and 390px-mobile checks, including generated-offline-artifact keyboard regions. `npm run qa:a11y` passed 2/2 serious/critical axe scans. |
| CLI normal and invalid paths | The clean consumer install produced parseable JUnit with 2 cases/2 failures and redacted the supplied `s3cr3t-sentinel` real-style fixture. `--help` is useful; unknown option exits 64; an incomplete stream exits 2 with a machine-readable final-summary error. |
| Packaging | `cargo package --locked` verified the package (37 files, 56.2 KiB compressed); `npm pack --dry-run` passed. The staged crate installed into a separate temporary consumer root with `cargo install --path target/package/infra-test-evidence-0.1.0 --root <root> --locked`. No publish was attempted. |
| Live identity | Local/live SHA-256 pairs match: root `12a19a2e62ece91b03de7e2e835a886d91f54950ddcf6321e2c4fa5c40935c16`; JS `7a47f092a1ac4b54e50166dd15e50d532f065d448cffafaa06fdc4d458cdca9f`; CSS `60fe55f2c44949eba033fd380b71b93bb3a6fa136853d55e83a9ed7f4ba0ab73`; Privacy `e3b9b299057ac932deeeb708c718563f91ab4a73f5833256aeda99318a80632a`; Terms `8a6dc8f53e5469f8f4d9320aef5ba490a8ff84c7ab9f36e66716a397100b770f`. |
| Live site/policy smoke | `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 694 ms with title, `lang=en`, exactly one h1, main landmark, no missing alt text, unlabeled buttons, console, or page errors. Live CSP, `frame-ancestors 'none'`, X-Frame-Options DENY, Permissions-Policy, nosniff, strict referrer policy, and HSTS are present; hashed JS uses `public, max-age=31536000, immutable`. |
| Privacy/local-first | Source and build inspection found no analytics, remote fonts/CDNs, upload API, browser storage, or service worker. The offline reviewer artifact is self-contained. The landing page is intentionally not a PWA, so SW update/offline-reload checks do not apply. |
| Lighthouse | Mobile live run: performance 100, accessibility 100, best practices 100, SEO 100; FCP 1.1 s, LCP 1.1 s, TBT 0 ms, CLS 0. |

The landing reader's normal sample load, malformed-JSON error, invalid-status/
negative-duration feedback, and recovery are covered in the passing browser
suite at desktop and 390px. It uses the documented visible skip/file focus
treatment and reduced-motion transition override. The current live response
matches that exact tested build.

## Commands used

```sh
git clone https://github.com/B-Divyesh/sf-infra-test-evidence.git qa
cd qa && git checkout --detach 812fa4a95dff0f1941f4256fc8f0e4b8c0a7a791
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
cargo install --path target/package/infra-test-evidence-0.1.0 --root /tmp/ite-consumer --locked
/tmp/ite-consumer/bin/infra-test-evidence --json --junit report.xml --evidence-dir evidence examples/opentofu-real-stream.jsonl
/opt/fleet/lib/verify-url.sh https://infra-test-evidence.sociobot.in /tmp/live-check
CHROME_PATH=/opt/pw-browsers/chromium-1208/chrome-linux64/chrome \
  npx --yes lighthouse https://infra-test-evidence.sociobot.in/ --only-categories=performance,accessibility,best-practices,seo
```

## Required before re-verification

Implement sensitivity-aware recursive redaction for `sensitive: true` (and
the exact OpenTofu/Terraform sensitive-value encodings), including diagnostics,
plan summaries, inputs, JUnit, JSON, and embedded HTML. Add an end-to-end
release-package regression that scans every generated artifact for an opaque
sentinel. Re-run this verification against the new candidate.
