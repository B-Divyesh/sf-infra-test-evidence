# Independent verification 9 — FAIL

**Work order:** `infra-test-evidence-verify-9`
**Candidate commit:** `159ce635cb41294bca04f7f41ed5721c1425f062`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-02 UTC
**Scope:** packaged Rust CLI, generated reviewer artifacts, and deployed static reader

## Decision

**FAIL.** The live deployment matches the candidate build and the standard
build, browser, accessibility, privacy, and performance checks pass. However,
independent product exercises found release-blocking evidence-integrity and
redaction failures that the declared claim tests do not cover:

1. Cloud resource identifiers in diagnostic text are copied into JUnit, JSON,
   and HTML instead of being redacted.
2. The bundled CLI demo discards the `assertion_path` values in its own sample,
   while the browser demo presents those values as converted sample output.
3. The claimed strict validator accepts malformed duration types as valid.

The checkout does not contain `.factory/brief.json`; the researched brief in
the work order was used as the acceptance contract.

## Release-blocking defects

### P1 — Resource identifiers leak into every generated artifact

The brief requires plans containing resource identifiers to be redacted by
default. The `sensitive-redaction` claim is broader still: “Explicit sensitive
values and resource identifiers are removed from every generated artifact.”

I passed the installed release binary a normal failed `diagnostic` whose detail
contained both an AWS ARN and an EC2 instance ID:

```text
aws_instance.web with ARN arn:aws:ec2:us-east-1:123456789012:instance/i-0abc123 failed
```

The CLI exited 0. `rg 'aws_instance\.web|arn:aws|i-0abc123'` found those values
in all three outputs:

- `report.xml`, as the failure message and body;
- `evidence/evidence.json`, in global diagnostics, the case failure, and the
  assertion path;
- `evidence/index.html`, in the embedded artifact payload.

There was no redaction token in any output. The implementation only detects
secret-related words in free text (`src-rust/main.rs:65-98`). A diagnostic is
redacted as a whole only when recursive key/mask redaction changed its object
(`src-rust/main.rs:233-242`). Common ARN and cloud-ID patterns in diagnostic
text therefore pass through unchanged.

A separate plan probe placed the same ARN in
`test_plan.outputs.instance_ref.value`. The CLI again exited 0 and copied the
identifier into both reviewer artifacts, including the section labelled
“Redacted plan summary.” This directly reproduces the brief's stated plan-data
risk.

The declared `@claim:sensitive-redaction` test passed because its fixture tests
only an explicitly marked opaque value. It does not test the
resource-identifier half of the claim.

### P1 — The shipped demo drops its assertion evidence

The bundled CLI sample has `assertion_path` on both `test_run` events
(`examples/tofu-test.jsonl:1-2`). A clean packaged `infra-test-evidence --demo`
run created valid output files, but its generated evidence contained:

```json
{
  "assertionPaths": [],
  "testCases": [
    { "name": "private_network_only", "assertionPaths": [] },
    { "name": "blocks_public_ingress", "assertionPaths": [] }
  ]
}
```

The parser only collects assertion traversals from diagnostic snippets
(`src-rust/main.rs:507-525`) and never reads `test_run.assertion_path`
(`src-rust/main.rs:471-505`). This loses review evidence required by the brief.

The live demo nevertheless displays `aws_security_group.web.ingress` and says
`[REDACTED]` replaced a sensitive value. Those details are hard-coded in
`src/main.ts:22-23`, not loaded from the generated CLI artifact. The actual
bundled output contains neither an assertion path nor a `[REDACTED]` value.
This contradicts `.factory/demo.md:10-13`, which calls the browser sample “a
view of the bundled `examples/tofu-test.jsonl` conversion.” The declared demo
tests assert the hard-coded page and the CLI file paths separately, so they do
not catch the mismatch.

### P1 — “Strict validation” accepts malformed duration types

The installed package returned exit 0 and
`{"checks":1,"errors":[],"valid":true}` for both of these compact records:

```json
{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"x","status":"pass","durationMs":"fast"}]}
{"run":"r","environment":"e","recordedAt":"x","checks":[{"name":"x","status":"pass","durationMs":false}]}
```

It also accepted an event stream with `"elapsed":"minus one"`. Numeric
negative durations correctly fail. Non-numeric values are silently converted
to “missing” because `Value::as_f64` returning `None` is indistinguishable from
an absent field (`src-rust/main.rs:285-290` and `485-493`). This falsifies the
declared `strict-validation` claim for malformed compact records.

The deployed browser reader has the same problem: a string duration is shown
as “No duration” with “1 checks ready.” It also silently drops a malformed
string entry from a mixed `checks` array and reports the remaining entry as
ready (`src/evidence.ts:12-21`). The claim test covers a missing file and bad
option combination, but no malformed field type.

### P2 — A contradictory skipped summary is accepted

An event stream with one failed `test_run` and a final
`test_summary.status = "skip"` returned exit 0 and `valid:true`. Summary/result
consistency only handles pass, fail, and error (`src-rust/main.rs:582-588`),
although `skip` is accepted by the shared status parser. This permits
internally contradictory evidence.

## Mandatory first checks

### Cold first read — PASS

The first live viewport says **“Turn infrastructure tests into reviewable
evidence.”** It names **infrastructure-module maintainers** who need reviewers
to inspect failed OpenTofu/Terraform tests without uploading logs. The primary
action is **“Try it with sample data,”** and adjacent copy says it will show a
failed test, redaction, and output files.

The action opens `/demo/?demo=1` in one keyboard-operated click. The persistent
banner says **“Demo — sample data, nothing is saved”** and provides **Reset
demo** and **Start for real**. This passes the first-read gate, although the
sample-to-CLI fidelity defect above still blocks release.

### Declared claims — all commands pass

After `npm ci` from the clean candidate checkout, every exact command in
`.factory/claims.json` was run separately. All 18 entries passed:

| Claim | Result |
| --- | --- |
| `cli-demo` | PASS |
| `cli-recording` | PASS (2 browser projects) |
| `cli-conversion` | PASS |
| `sensitive-redaction` | PASS, but incomplete coverage; see P1 |
| `sensitivity-fail-closed` | PASS |
| `strict-validation` | PASS, but incomplete coverage; see P1 |
| `event-stream-validation` | PASS, but misses skipped-summary contradiction |
| `run-correlation` | PASS |
| `sensitive-diagnostics` | PASS |
| `conversion-only` | PASS |
| `requested-path-writes` | PASS |
| `artifact-private` | PASS (2 browser projects) |
| `reader-private` | PASS (2 browser projects) |
| `site-demo` | PASS (2 browser projects), but compares hard-coded UI only |
| `browser-record-import` | PASS (2 browser projects) |
| `help-options` | PASS |
| `json-validation-output` | PASS |
| `mit-license` | PASS |

The claim file exists and each ID has exactly one matching test tag. Passing
selectors do not override the independently reproduced false behaviors above.

## Clean local verification

The clean checkout started at exactly the candidate SHA with no changes.
`npm ci` installed 182 packages and reported no vulnerabilities. These gates
passed:

```text
npm run check                            PASS
  eslint                                 PASS
  tsc --noEmit                           PASS
  cargo test --locked                    PASS, 6 tests
  vitest                                 PASS, 23 tests
cargo fmt --check                        PASS
cargo clippy --locked --all-targets -- -D warnings
                                         PASS
npm audit --audit-level=high             PASS, 0 vulnerabilities
npm run build                            PASS
npm run package:check                    PASS
npm run consumer:check                   PASS
npm run qa:browser                       PASS, 20 tests
npm run qa:a11y                          PASS, 2 projects
```

The production build created `dist/site/`. Output sizes were:

- JavaScript: 5.56 kB + 1.34 kB uncompressed, 3.04 kB gzip total;
- CSS: 11.72 kB uncompressed, 3.37 kB gzip;
- no downloaded fonts and no first-load hero image.

After removing a verifier-only screenshot from the worktree, `cargo package`
contained 59 files, 365.6 KiB uncompressed and 104.2 KiB compressed. Nothing
was published.

## Independent CLI and artifact exercise

I installed `target/package/infra-test-evidence-0.1.0` into a fresh temporary
consumer root and used that installed binary.

- `--help` documented all supported modes/options and exit codes.
- `--demo` created a unique directory under an isolated `TMPDIR` and printed
  paths for its sample, JUnit, evidence JSON, and reviewer HTML.
- `examples/opentofu-real-stream.jsonl` produced two failed cases, assertion
  paths, redacted plan details, SHA-256 provenance, and no fixture secret
  sentinels.
- A zero duration passed. Empty checks, a negative duration, malformed JSON,
  an unknown option, and an unwritable output path failed with exit 2 or 64 as
  documented.
- The generated reviewer page opened from `file:` at 390 px with no HTTP
  requests, console/page errors, horizontal overflow, or serious/critical axe
  findings.

The extra adversarial cases then exposed the defects above.

## Live deployment evidence

Fresh Playwright contexts covered 1440×900 desktop and 390×844 mobile.

- The three first-screen facts fit both viewports. Demo proof values fit within
  the first demo viewport at both sizes.
- Keyboard-only navigation focused the skip link and primary action; Enter
  opened the demo, Space operated Reset, and Enter on Start for real returned
  to `/`. Focus used a visible `rgb(7, 90, 158) solid 3px` outline.
- No measured link, button, or file-picker label target was under 44×44 px at
  390 px.
- At 200% root text size there was no horizontal overflow.
- Reduced motion changed transitions to `0.00001s` and showed the complete CLI
  recording without animation.
- Invalid JSON produced a specific corrective error and a later valid/reset
  action recovered.
- Across the full demo/import/policy/error flow, every request was same-origin.
  Cookies, localStorage, sessionStorage, and IndexedDB remained empty.
- No console or page errors occurred.
- Axe found zero serious or critical findings on `/`, `/demo/`, `/privacy/`,
  `/terms/`, and `/404.html`, in light and dark mode, at desktop and mobile.
- `/opt/fleet/lib/verify-url.sh` passed: 200, title, `lang=en`, one `h1`, one
  `main`, zero missing image alt text, zero unnamed buttons, and zero browser
  errors. Its cold load measurement was 828 ms.
- All internal navigable URLs, `robots.txt`, and `sitemap.xml` returned 200.
  An unknown route returned the designed 404 document with HTTP 404.

Live mobile Lighthouse scored 100 Performance, 100 Accessibility, 100 Best
Practices, and 100 SEO. FCP and LCP were 1.086 s, TBT was 0 ms, CLS was 0, and
total transfer was 10,899 bytes. Lighthouse did not expose a lab INP value;
the tested interactions completed without observable delay.

The browser response headers included a same-origin CSP with
`frame-ancestors 'none'`, HSTS, `nosniff`, `X-Frame-Options: DENY`, strict
referrer policy, and a restrictive Permissions Policy. HTML revalidates after
30 seconds. Hashed JS returned `Cache-Control: public, max-age=31536000,
immutable`.

This product has no server endpoint, authentication, payment/unlock call,
service worker, or PWA manifest. API rate limiting, Entra tenant checks,
backend concurrency/persistence, and PWA update/offline checks are therefore
not applicable.

## Deployment identity — PASS

Every publicly served production file matched the candidate's fresh
`dist/site/` output byte-for-byte: the five HTML documents, three hashed
assets, CLI recording, favicon, apple-touch icon, robots file, sitemap, and
both social-card formats. Representative hashes:

| File | SHA-256 |
| --- | --- |
| `index.html` | `3de3611c8a13311e9cb69e66a03c12e2f3a2953e200d940370989059b13de843` |
| `demo/index.html` | `7aa7711b94cc635b2dc0abf6015d93d10bb3a5b55993b00dc29be9e20412a28b` |
| `assets/main-DqpOp-zM.js` | `22be44043528c158f52847ada71009e0f70b2603c8c2c85194e56629eee1e001` |
| `assets/style-BSFTurhu.css` | `2bc662aa6bdf6125389906819b0af0ffe5f4f864198c22112cb6bb583e395d68` |
| `cli-demo.cast` | `9f7d82e2f37a42cd7f2c6392c25d5be6cb5be07bbb7fd9d92f2bfa74b50571f5` |

## Required fixes before release

1. Redact cloud resource identifiers in diagnostic/failure text before any
   artifact is rendered, and add a claim fixture that scans all outputs for
   ARN and cloud-ID sentinels.
2. Parse and preserve assertion paths from every supported event shape,
   including the bundled sample's `test_run.assertion_path`. Generate the
   browser demo proof from the same conversion output or assert equivalence of
   every represented field.
3. Reject present-but-non-numeric duration values in the Rust CLI and browser
   reader; do not filter malformed check entries out of arrays.
4. Reject summary statuses that contradict the completed run statuses, or
   narrowly define and test the allowed skipped-summary semantics.

## Transient evidence locations

- Installed consumer and normal/demo artifacts: `/tmp/ite-verify9.fwsWvM`
- Identifier-leak artifacts: `/tmp/ite-redaction-probe.roFhQk`
- Plan identifier-leak artifacts: `/tmp/ite-plan-redaction-probe.HwVEoG`
- Fleet live verifier: `/tmp/ite-verify-url.6KbZyZ`
- Lighthouse JSON: `/tmp/ite-lighthouse-verify9.json`
