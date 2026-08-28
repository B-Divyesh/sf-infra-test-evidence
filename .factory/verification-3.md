# Independent QA verification 3 — FAIL

**Candidate:** `2b4d93f44be5dfdf83d6fc6deb98a5d4e69b8a18`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Verified:** 2026-08-28 UTC from fresh detached clones at the candidate SHA.

## Verdict

**FAIL.** The deployment is current and healthy, so this is not the previously
reported deployment-only failure. The live HTML, JavaScript, CSS, Privacy, and
Terms documents match a fresh candidate build byte-for-byte. The candidate
itself is not releasable: a real OpenTofu 1.12.6 run demonstrated that the CLI
copies a sensitive test value into both generated evidence artifacts, omits the
real test inputs, assertion paths, and plan summary, and assigns the wrong
diagnostic to later failed cases. The required clean-checkout test command also
fails consistently.

## Defects

### P0 — Default redaction leaks a real OpenTofu sensitive value

A local OpenTofu 1.12.6 module defined a sensitive `db_password` variable with
the sentinel value `s3cr3t-sentinel`. A failing assertion used
`nonsensitive(output.db_password)`; OpenTofu's real JSON diagnostic included
the sentinel in `diagnostic.difference.before`. Running the clean-consumer
installed binary produced `evidence/evidence.json` and `evidence/index.html`
that both contained the sentinel verbatim. The CLI redacted the assertion code
because its text contained `password`, but did not redact the corresponding
diagnostic value.

This violates the brief's default-redaction constraint and the artifact's own
provenance claim that recursive secret redaction was applied. The generated
HTML is intended to be shared with reviewers, so this is a direct secret
disclosure path.

### P1 — Real IaC context is omitted and failures are misattributed

The same real `tofu test -json -verbose` stream contained three `test_plan`
events with variables, outputs, and a planned `terraform_data.build` creation,
plus two distinct assertion diagnostics. The generated artifact reported:

```json
{
  "inputs": [["test_file=tests/basic.tftest.hcl"]],
  "assertionPaths": [],
  "planSummary": []
}
```

It ignores OpenTofu's `test_plan` payload instead of creating the required
redacted input and plan summaries. It also does not derive the assertion path
from `diagnostic.snippet.values[].traversal`. Worse, both failed cases were
given the first failure's `expected production environment` diagnostic; the
second case's actual `the protected value did not match` diagnostic appeared
only in the unassociated diagnostic list. A reviewer would receive missing and
incorrect evidence for the core job-to-be-done.

### P1 — A partial or corrupt event stream can return success

The installed package returned exit 0 and
`{"checks":1,"errors":[],"valid":true}` for a stream containing one valid
passing `test_run`, a second `test_run` with unsupported status `mystery`, and
a `test_summary` reporting one failure. The unsupported case and failing
summary were silently ignored. A single completed case without any final
`test_summary` was also accepted. Truncated or forward-incompatible CI output
can therefore be presented as valid while dropping failures.

### P1 — The documented clean-checkout test gate fails

`npm run check` failed in a fresh clone because
`tests/cli.test.ts > writes JUnit and a redacted, self-contained reviewer
artifact` timed out at Vitest's 5-second default while Cargo performed its
first build (12.3 seconds). A second independent fresh clone reproduced the
failure (11.3 seconds). A warm-cache `npm test` passed, which explains the
builder's result but does not satisfy the clean-clone contract.

### P1 — Generated evidence page has serious axe findings

The self-contained artifact was opened directly from disk at 1440px and 390px.
It made no network requests and had no console/page errors, but axe reported
`scrollable-region-focusable` with serious impact on `#provenance`,
`#failures`, and the per-case `<pre>` regions. Keyboard users cannot reach and
scroll this core evidence content.

### P2 — Validation is inconsistent at other boundaries

- The CLI accepts `durationMs: -100` and emits a negative JUnit duration.
- The live local reader accepts an unsupported `mystery` status and `-1 ms` as
  “1 checks ready,” although the CLI and README define a closed status set.
- `cargo fmt --check` fails throughout `src-rust/main.rs`; strict Clippy fails
  on `clippy::collapsible-if`.

### P3 — Minor release/documentation issues

- Lighthouse records a console network error because `/favicon.ico` returns
  404; best-practices scores 96 rather than 100.
- Lighthouse also flags the visible `ITE•` wordmark because its accessible
  name (`Infra Test Evidence home`) does not contain the visible label.
- The source-of-truth `.factory/brief.json` is absent. The injected researched
  brief was used for this verification.
- The prior handoff claims 4 Vitest files / 10 tests, but this candidate runs 2
  Vitest files / 5 tests plus 3 Rust tests.

## Passing evidence

| Area | Result |
| --- | --- |
| Toolchain/install | Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0; `npm ci` succeeded with 0 audit vulnerabilities. |
| Independent checks | ESLint, TypeScript, `cargo test --locked` (3/3), and a warm-cache Vitest run (5/5) passed. `npm audit --audit-level=high` reported 0 vulnerabilities. |
| Production build | `npm run build` passed. Output: HTML 3,355 B, JS 3,442 B (1.55 kB gzip), CSS 5,486 B (1.99 kB gzip), plus Privacy, Terms, and hosting config. Budgets pass. |
| Packaging/consumer | `cargo package --locked` passed (50.9 KiB compressed); `npm pack --dry-run` passed. The staged crate installed into a clean consumer root and its public CLI/help and documented 0/2/64 paths worked. |
| Normal conversion | The shipped sample produced parseable JUnit with 2 cases/1 failure, a SHA-256 provenance match, `evidence.json`, and a self-contained HTML page. Empty, malformed, unknown-object, invalid-middle-line, unreadable-input, unwritable-output, unknown-option, and missing-option-value paths failed with the documented codes. |
| Live identity | Live/local SHA-256 matched for root `7b86dc1c…`, JS `9d2c680b…`, CSS `60fe55f2…`, Privacy `e3b9b299…`, and Terms `8a6dc8f5…`. |
| Live browser | At 1440px and 390px, sample load, malformed JSON, invalid shape, recovery, and escaped markup worked. No page overflow, third-party requests, cookies, local/session storage, service worker, console error, or page error was observed in Playwright. Keyboard order and visible focus passed; targets were at least 44px; reduced motion produced a 0.01ms transition. Live axe had 0 serious/critical findings in desktop/mobile light and mobile dark modes. |
| Privacy/policies | Browser traffic stayed on the product origin and file processing caused no requests. Source inspection found no telemetry/network client. CSP, `frame-ancestors 'none'`, X-Frame-Options DENY, Permissions-Policy, `nosniff`, strict referrer policy, and HSTS are live. Hashed JS/CSS use `max-age=31536000, immutable`. |
| Lighthouse | Mobile: performance 100, accessibility 100, best practices 96, SEO 100; FCP 0.9s, LCP 1.0s, TBT 50ms, CLS 0, total transfer about 8 KiB. |
| Factory URL smoke test | `/opt/fleet/lib/verify-url.sh` passed: HTTP 200, 810ms load, title/lang/one h1/main present, no missing alt or unlabeled buttons, and no captured console/page errors. |

The landing page is intentionally not a PWA (no manifest or service worker),
so service-worker update/offline-reload checks are not applicable. There is no
backend, remote persistence, or health endpoint; CLI output is local files.

## Commands used

```sh
git clone https://github.com/B-Divyesh/sf-infra-test-evidence.git qa
cd qa && git checkout --detach 2b4d93f44be5dfdf83d6fc6deb98a5d4e69b8a18
npm ci
npm run check
npm audit --audit-level=high
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
cargo install --path target/package/infra-test-evidence-0.1.0 --root <clean-root> --locked
<clean-root>/bin/infra-test-evidence --json --junit report.xml --evidence-dir evidence tofu-real.jsonl
/opt/fleet/lib/verify-url.sh https://infra-test-evidence.sociobot.in <evidence-dir>
```

OpenTofu 1.12.6 generated `tofu-real.jsonl` from a local, provider-free test
module; no fixture or generated evidence is committed.

## Required before re-verification

Implement sensitive-aware redaction over diagnostics and plan/test-plan data;
associate diagnostics and plan context by `@testfile`/`@testrun`; emit real,
redacted case inputs, assertion paths, and plan summaries; reject incomplete or
unsupported event streams; make the clean test command deterministic; and fix
keyboard access in generated artifacts. Then add regression fixtures captured
from real OpenTofu/Terraform output and re-run the full clean verification.
