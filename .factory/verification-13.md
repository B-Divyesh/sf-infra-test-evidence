# Verify infrastructure test evidence conversion — QA 13

**Work order:** `infra-test-evidence-verify-13`
**Verdict:** **FAIL**
**Finding count:** **1** (`P2: 1`)
**Untested claim count:** **0**
**Implementation candidate:** `e45ce587daaeb212e7c4299c412fd68f9a578bcc`
**Documentation commit:** `01a144941610cc99a3244a98bb54f54089a6d948`
**Reported deployment:** `2decafa3-46b9-4278-8299-743e373cfb2a`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-05 UTC

## Verdict

**FAIL.** The CLI, generated artifacts, demo sandbox, claims, deployment
identity, privacy behavior, routes, accessibility, and quality gates pass.
There is one current `P2` finding: a fresh iPhone 13 browser does not show the
required click-expectation line or any of the three product facts before the
visitor scrolls. A PASS requires zero findings of every severity.

The repository does not contain `.factory/brief.json`. The researched brief
included in the work order was used as the acceptance contract.

## Finding

### F-13-1 — P2 — Required first-screen facts are below the phone browser fold

The plain-words contract requires the first screen to contain the primary
action, what happens after the click, and three short product facts. In a fresh
Playwright iPhone 13 context, the CSS viewport is `390 × 664`.

The core cold-read requirement does pass:

- Job: **Turn infrastructure tests into reviewable evidence** ends at
  `y=383.52`.
- Audience: **For infrastructure-module maintainers who need reviewers to
  inspect failed OpenTofu or Terraform tests without uploading logs** ends at
  `y=513.95`.
- First action: **Try it with sample data** ends at `y=592.23`.

The rest of the mandatory first-screen content does not pass:

| Required content | Top | Bottom | In the 664 px viewport |
| --- | ---: | ---: | --- |
| See a failed test, redaction, and output files. | 666.52 | 688.13 | No |
| Runs in your browser | 712.13 | 733.73 | No |
| No trackers or uploads | 741.73 | 763.34 | No |
| Free under the MIT License | 771.34 | 792.95 | No |

Evidence:

- `/work/.evidence/infra-test-evidence-verify-13/live/live-check.json`
- `/work/.evidence/infra-test-evidence-verify-13/live/live-root-phone-first-viewport.png`
- Public copy is at `index.html:33-36`.
- The current browser test checks the landing facts only at `1440 × 900`
  (`tests/site.spec.ts:127-135`). Other first-viewport checks force
  `390 × 844`, which is the device screen height rather than the iPhone 13
  browser viewport (`tests/site.spec.ts:11-34`).

Required repair: keep the click-expectation line and all three facts within a
fresh `390 × 664` browser viewport without hiding the job, audience, or primary
action. Add a regression at that viewport. No product code was changed during
this verification.

## Job, audience, and first action

Before scrolling on both desktop and phone, the live page states:

- Job: turn infrastructure tests into reviewable evidence.
- Audience: infrastructure-module maintainers reviewing failed OpenTofu or
  Terraform tests.
- First action: **Try it with sample data**.

The title, **Infra Test Evidence — review test runs locally**, names the job in
plain words and is under 60 characters. The headline is seven words and the
audience sentence is 17 words. No banned marketing words, mood headings, or
the previously inconsistent terms were found in public HTML, runtime copy,
README, claims, or demo documentation.

## Declared claims

A clean GitHub clone at documentation commit `01a1449` received `npm ci` with
182 packages and zero vulnerabilities. Every exact command in
`.factory/claims.json` was then run separately. All 24 exited 0.

| Claim | Result | Output |
| --- | --- | --- |
| `cli-demo` | PASS | `claims/cli-demo.log` |
| `cli-recording` | PASS | `claims/cli-recording.log` |
| `cli-conversion` | PASS | `claims/cli-conversion.log` |
| `sensitive-redaction` | PASS | `claims/sensitive-redaction.log` |
| `named-field-redaction` | PASS | `claims/named-field-redaction.log` |
| `resource-identifier-redaction` | PASS | `claims/resource-identifier-redaction.log` |
| `cross-provider-resource-redaction` | PASS | `claims/cross-provider-resource-redaction.log` |
| `sensitivity-fail-closed` | PASS | `claims/sensitivity-fail-closed.log` |
| `strict-validation` | PASS | `claims/strict-validation.log` |
| `malformed-duration-types` | PASS | `claims/malformed-duration-types.log` |
| `event-stream-validation` | PASS | `claims/event-stream-validation.log` |
| `summary-consistency` | PASS | `claims/summary-consistency.log` |
| `run-correlation` | PASS | `claims/run-correlation.log` |
| `sensitive-diagnostics` | PASS | `claims/sensitive-diagnostics.log` |
| `conversion-only` | PASS | `claims/conversion-only.log` |
| `requested-path-writes` | PASS | `claims/requested-path-writes.log` |
| `artifact-private` | PASS | `claims/artifact-private.log` |
| `reader-private` | PASS | `claims/reader-private.log` |
| `site-demo` | PASS | `claims/site-demo.log` |
| `demo-artifact-fidelity` | PASS | `claims/demo-artifact-fidelity.log` |
| `browser-record-import` | PASS | `claims/browser-record-import.log` |
| `help-options` | PASS | `claims/help-options.log` |
| `json-validation-output` | PASS | `claims/json-validation-output.log` |
| `mit-license` | PASS | `claims/mit-license.log` |

The output paths above are relative to
`/work/.evidence/infra-test-evidence-verify-13/`. Public landing, demo, policy,
runtime-error, and README statements were compared with the inventory. No
unlisted, false, incomplete, or untested public claim was found.

## CLI and generated artifacts

The crate produced by `cargo package --locked` was installed into a new
consumer root. The installed binary, not the workspace binary, passed:

- `--help`, with every documented option and process code;
- `--demo`, which made a unique directory under a dedicated `TMPDIR` and
  printed the sample, JUnit report, evidence JSON, and reviewer-page paths;
- a normal conversion of `examples/opentofu-real-stream.jsonl`, producing all
  three non-empty outputs;
- a malformed string duration, which returned exit 2 and machine-readable
  validation output;
- recovery immediately afterward with `examples/passing-evidence.json`, which
  returned `{"checks":2,"errors":[],"valid":true}`.

The claim runs additionally covered malformed JSON, incomplete records,
unsupported usage, unwritable output, malformed sensitivity metadata,
negative and wrong-type durations, incomplete/contradictory summaries,
interleaved runs, explicit secrets, named fields, sensitive diagnostics, and
AWS, Azure, and GCP identifiers. They also instrumented the process boundary
to prove no Terraform/OpenTofu invocation, network socket, or unrequested
filesystem write.

## Demo and privacy

One click from the primary landing action opened `/demo/?demo=1`. The banner
remained present and said **Demo — sample data, nothing is saved**, with
working **Reset demo** and **Start for real** actions.

The populated sample showed two checks, the failed
`blocks_public_ingress` check, `aws_security_group.web.ingress`, a 310 ms
duration, `[REDACTED]`, input SHA-256 `85bfaca…0711522`, and all three canonical
output paths. Importing a private compact record changed only the in-memory
view. Invalid JSON produced the documented alert. Reset restored the two-check
sample, and Start for real returned to the empty reader.

Across each fresh desktop and phone flow, all 69 observed browser requests
were same-origin. Cookies, localStorage, sessionStorage, IndexedDB, and service
worker registrations were empty. This proves the demo did not read or change
real product data. The generated reviewer page's separate claim test opened it
from `file:` without a network request.

There is no published offline/update claim. The site has no service worker.
Offline reload and update behavior are therefore not applicable.

## Live browser, routes, and accessibility

Fresh `1440 × 900` desktop and Playwright iPhone 13 (`390 × 664` browser
viewport) contexts exercised the live site.

- `/`, `/demo/?demo=1`, `/privacy/`, and `/terms/` returned 200 with distinct
  route titles, `lang=en`, one `h1`, one `main`, header/nav/footer landmarks,
  canonical/description/Open Graph/Twitter metadata, and working internal
  links.
- `/not-found` returned the expected HTTP 404 and the designed page titled
  **Page not found — Infra Test Evidence**, with a route home. The browser's
  expected 404 resource message is not classified as a defect.
- The first Tab focused the skip link with a designed
  `rgb(7, 90, 158) solid 3px` outline. Enter moved to main. Internal forward
  and Back navigation focused and announced the destination `h1`. Space
  operated Reset demo.
- Reduced motion changed the drop-zone transition to `0.00001s` and showed the
  completed CLI recording without animation. No horizontal overflow occurred,
  visible link/button/summary targets were at least 44 px, and the root
  tolerated 200% text.
- Live Axe checks on root, demo, Privacy, Terms, and 404 found zero serious or
  critical issues in both device contexts. The local two-project Axe suite
  also passed its light/dark coverage.
- Outside the deliberate 404, there were no console or page errors.
- `/opt/fleet/lib/verify-url.sh` passed cold on root in 856 ms and demo in
  808 ms, with title, language, `h1`, main, alt text, button names, and zero
  errors.

The live mobile Lighthouse run scored 100 Performance, 100 Accessibility,
100 Best Practices, and 100 SEO. LCP was 1.0 s, CLS 0, and total blocking time
0 ms. Initial production JavaScript is 8.12 kB raw / 3.38 kB gzip combined;
CSS is 11.74 kB raw / 3.38 kB gzip.

## Quality gates

All clean-checkout gates passed:

```text
npm run check                                  PASS (8 Rust + 28 Vitest tests)
npm run build                                  PASS (dist/site/)
npm run qa:browser                             PASS (26 tests)
npm run qa:a11y                                PASS (2 tests)
npm run package:check                          PASS
npm run consumer:check                         PASS
cargo fmt --check                              PASS
cargo clippy --locked --all-targets -- -D warnings  PASS
npm audit --audit-level=high                   PASS (0 vulnerabilities)
```

Logs are under `/work/.evidence/infra-test-evidence-verify-13/gates/`.

## Deployment identity

Commit `01a1449` changes only `.factory/handoff.md` and adds
`.factory/polish-3.md`; the last implementation candidate is `e45ce58`.
Local and live SHA-256 values match for all checked runtime files:

- root, demo, Privacy, Terms, and 404 documents;
- `main-Dju-avPR.js`, `routes-j3C_yCWu.js`, and `style-CgdVxA8l.css`;
- `cli-demo.cast`, `demo-evidence.json`, and `social-card.png`.

The live site therefore represents implementation candidate `e45ce58`.
Hash evidence is in `identity/runtime-hashes.tsv`.

This is a static site and local CLI. It has no product backend, account,
tenant, SQLite state, health API, payment path, or live request allowance.
Backend tenant isolation, restart persistence, and `429`/`Retry-After` checks
do not apply.

## Earlier finding disposition

### Earlier independent verifications

| Earlier finding | Current disposition and proof |
| --- | --- |
| Verification 1/2: converter and reviewer artifact absent | Fixed. Installed package converts a real-style event stream to JUnit, evidence JSON, and a reviewer page. |
| Verification 1/2: malformed input could return success | Fixed. Strict-validation claims pass; independent malformed duration exits 2, followed by successful recovery. |
| Verification 1/2: Privacy and Terms broken | Fixed. Both live routes return distinct 200 documents, metadata, landmarks, links, and clean Axe results. |
| Verification 2: file chooser had no visible focus | Fixed. Local browser gate and live focus audit show a designed 3 px ring. |
| Verification 1/2: hashed assets not immutable | Fixed. Live hashed assets return `public, max-age=31536000, immutable`. |
| Verification 1/2: response policy incomplete | Fixed. Live CSP includes `frame-ancestors 'none'`; `nosniff`, DENY framing, strict referrer, Permissions Policy, and HSTS are present. |
| Verification 3: real sensitive diagnostic leaked | Fixed. `sensitive-redaction` and `sensitive-diagnostics` pass across all three artifacts. |
| Verification 3: inputs/plans/assertion paths omitted or misattributed | Fixed. `cli-conversion` and `run-correlation` pass; the populated demo shows the generated assertion path and input hash. |
| Verification 3: partial/unsupported event streams succeeded | Fixed. `event-stream-validation` passes seven malformed stream cases without reviewer artifacts. |
| Verification 3: clean checkout test timed out | Fixed. Cold `npm run check` passes 8 Rust and 28 frontend tests. |
| Verification 3: generated page had serious Axe issues | Fixed. `artifact-private`, the local Axe suite, and live Axe audits pass. |
| Verification 3: negative duration, browser status, formatting, and Clippy boundaries | Fixed. Duration claims, browser import validation, fmt, and strict Clippy all pass. |
| Verification 3 minor: favicon, wordmark name, missing brief, inaccurate test count | Favicon and accessible wordmark pass Lighthouse/Axe; current evidence reports exact counts. `.factory/brief.json` remains absent, so the supplied researched brief was used as directed and is not a current runtime defect. |
| Verification 4: explicitly sensitive plan output leaked | Fixed. `sensitive-redaction` and `sensitivity-fail-closed` scan all outputs and pass. |
| Verification 5: claims manifest and isolated one-click demo absent | Fixed. The 24-entry manifest passes; one-click in-memory sample, label, reset, and exit all work. |
| Verification 5: audience, discovery routes, and designed 404 absent | Fixed at the reported conditions. Audience, robots, sitemap, legal routes, and the HTTP 404 are present. F-13-1 records the new phone-browser fold issue. |
| Verification 6: browser claims failed from a clean clone | Fixed. Every browser claim command passed independently after `npm ci`. |
| Verification 6/7: safety and requested-write promises missing from claims | Fixed. `event-stream-validation`, `run-correlation`, `sensitive-diagnostics`, `conversion-only`, and `requested-path-writes` are listed and pass. |
| Verification 7: landing page lacked a real CLI recording | Fixed. Self-hosted cast, transcript, controls, output paths, and reduced-motion view pass `cli-recording`. |
| Verification 9: AWS identifiers leaked | Fixed. AWS ARN/EC2 claim passes across XML, JSON, and HTML. |
| Verification 9: demo dropped assertion evidence and browser proof was invented | Fixed. `demo-artifact-fidelity` compares the shipped CLI artifact with the displayed path, duration, redaction, and input SHA-256. |
| Verification 9: wrong duration types accepted | Fixed. `malformed-duration-types` covers compact duration strings/booleans and string event elapsed values. |
| Verification 9: skipped summary contradicted failed run | Fixed. `summary-consistency` passes. |
| Verification 10: subnet, security-group, Azure, and GCP identifiers leaked | Fixed. `cross-provider-resource-redaction` scans all three artifacts and passes. |

### Earlier copy and claim reviews

| Finding | Current disposition and proof |
| --- | --- |
| F-1-1 | Fixed. One-click demo contains a real failed check, path, redaction, input SHA-256, and all outputs. |
| F-1-2 | Fixed. Browser compact-record import is listed and tested. |
| F-1-3 | Fixed. CLI help coverage is listed and tested. |
| F-1-4 | Fixed. Copy says JUnit XML/report; conversion tests parse both cases. |
| F-1-5 | Fixed. Machine-readable output for both forms is listed and tested. |
| F-1-6 | Fixed. The untested deployment-policy marketing sentence remains absent. |
| F-1-7 | Fixed for its reported desktop condition: all facts end at `y=641.36` in the 900 px viewport. F-13-1 covers the distinct phone-browser failure. |
| F-1-8 | Fixed. Forward and Back focus and announce the route heading. |
| F-1-9 | Fixed. 404 has canonical, Open Graph, Twitter, title, and route home. |
| F-1-10 | Fixed. CLI recording heading names all three outputs. |
| F-1-11 | Fixed. The decorative slogan was replaced by **Generated files**. |
| F-1-12 | Fixed. The error label is **404 · PAGE NOT FOUND**. |
| F-1-13 / F-2-1 / review-3 reopening | Fixed. Runtime, recording, pages, README, and claims consistently use JUnit report, evidence JSON, and reviewer page. |
| F-1-14 / review-3 reopening | Fixed. Browser input copy consistently uses compact record, including the invalid-file alert. |
| F-1-15 | Fixed. Compact-record schema instructions are short sentences. |
| F-1-16 | Fixed. README names the exact sensitivity markers and redaction result. |
| F-1-17 / F-2-5 | Fixed. Vague workflow language is absent; the browser input is identified. |
| F-1-18 | Fixed. Vague response-policy copy is absent. |
| F-1-19 | Fixed. Event-stream rules are short, direct sentences. |
| F-1-20 | Fixed. Whole-diagnostic redaction states that values stay out of every output. |
| F-2-2 | Fixed. Strict-validation tests malformed JSON, incomplete record, output rejection, valid input, and usage. |
| F-2-3 | Fixed. Named-field claim enumerates every published pattern and scans all outputs. |
| F-2-4 | Fixed. Site-demo asserts all three canonical output paths; demo fidelity proves their source. |
| F-2-6 / F-3-1 | Fixed. Public copy names inputs, paths, changes, failures, and the input SHA-256 without “provenance”. |
| F-3-2 | Fixed. README consistently says assertion paths. |

All earlier implementation, privacy, safety, accessibility, copy, claims, and
terminology findings are closed. F-13-1 is the only current finding.

## Evidence location

The complete verification evidence is copied to:

`/work/.evidence/infra-test-evidence-verify-13/`

This report is also copied to `/work/.evidence/qa-report.md`. The machine
result is `/work/.evidence/qa-result.json`.
