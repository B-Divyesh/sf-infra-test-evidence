# Verify infrastructure tests become reviewable evidence — QA 14

**Work order:** `infra-test-evidence-verify-14`
**Verdict:** **PASS**
**Finding count:** **0**
**Untested claim count:** **0**
**Implementation candidate:** `b0d8ea1c6a58fa66bc082c539df0a908e50b986b`
**Documentation evidence:** `4a7778ec1a5761cbd4657cebec0c92f671ee58a7`, cited by `98d80c921b07e4cdf99d88df7faeb405d4929303`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-06 UTC

## Verdict

**PASS.** The live site, packaged CLI, generated artifacts, sample sandbox,
privacy behavior, routes, accessibility, and every declared claim pass. There
are zero findings of every severity and zero untested claims.

The repository has no `.factory/brief.json`. The researched brief supplied in
the work order was used as the acceptance source. This is a static site and a
local CLI. It has no backend, account, tenant, payment path, remote state
access, health API, request allowance, or persistent product state. Backend
tenant isolation, restart persistence, and `429`/`Retry-After` checks do not
apply. There is no offline or update promise and no service worker.

## Job, audience, and first action

Before scrolling in fresh desktop and iPhone 13 browsers, the live page says:

- Job: **Turn infrastructure tests into reviewable evidence**.
- Audience: infrastructure-module maintainers reviewing failed OpenTofu or
  Terraform tests without uploading logs.
- First action: **Try it with sample data**.
- Result of the action: **See a failed test, redaction, and output files.**

The title is **Infra Test Evidence — review test runs locally**. It names the
job in 46 characters. The public copy contains no banned marketing words or
mood headings.

At the live `390 × 664` phone viewport, the first-screen bottom coordinates
are:

| Content | Bottom (px) |
| --- | ---: |
| Job | 247.41 |
| Audience | 377.84 |
| First action | 456.13 |
| Expected result | 552.02 |
| Runs in your browser | 589.63 |
| No trackers or uploads | 619.23 |
| Free under the MIT License | 648.84 |

The final fact has 15.16 px of room before the fold. This closes F-13-1 and is
also covered by the outcome-based browser regression added in `b0d8ea1`.

## One-click sample and privacy

The primary action opened `/demo/?demo=1` in one click. The persistent label
said **Demo — sample data, nothing is saved** and retained working **Reset
demo** and **Start for real** controls through normal and invalid input.

The populated sample showed two checks, failed check
`blocks_public_ingress`, assertion path
`aws_security_group.web.ingress`, 310 ms duration, `[REDACTED]`, input SHA-256
`85bfaca…0711522`, and `report.xml`, `evidence/evidence.json`, and
`evidence/index.html`. Importing a private compact record changed only the
in-memory view. Malformed JSON produced a direct error. Space activated Reset
demo and restored the sample. Start for real returned to an empty reader.

Fresh desktop and phone flows made 20 requests each. Every request was
same-origin. Cookies, localStorage, sessionStorage, IndexedDB, and service
worker registrations remained empty before and after leaving the sample. No
real data was read or changed.

## Declared claims

A fresh remote checkout at the implementation candidate received `npm ci`.
All 24 exact commands in `.factory/claims.json` ran separately and exited 0.
Every claim ID appears in exactly one source test.

| Claim | Result | Evidence log |
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

Landing, sample, policy, error, README, and runtime-error copy were compared
with the manifest. No missing, false, incomplete, unlisted, or untested public
claim was found. An AI step is not missed leverage here: deterministic,
local redaction and standards conversion are the job, and optional model use
would weaken the product's privacy boundary without improving that job.

## Installed CLI and generated artifacts

The crate produced by `cargo package --locked` was installed into a new
consumer root. That installed binary, not the workspace binary, passed:

- `--help`, including every supported option and documented exit code;
- `--demo`, which created a unique temporary directory and printed paths for
  the bundled input, JUnit report, evidence JSON, and reviewer page;
- normal conversion of `examples/opentofu-real-stream.jsonl`, producing three
  non-empty reviewer artifacts;
- a string duration counterexample, returning exit 2 with machine-readable
  validation output;
- immediate recovery with `examples/passing-evidence.json`, returning
  `{"checks":2,"errors":[],"valid":true}`.

The claim runs also covered malformed JSON, incomplete compact records,
incorrect usage, rejected output paths, malformed sensitivity metadata,
negative and wrong-type durations, incomplete and contradictory event
streams, interleaved runs, sensitive diagnostics, named fields, and AWS,
Azure, and GCP identifiers. Guarded process tests proved no Terraform or
OpenTofu invocation, network socket, or write outside requested paths. The
generated reviewer page opened from `file:` with no network request and no
serious or critical Axe issue.

## Live browser, routes, and accessibility

Fresh desktop and iPhone 13 contexts checked normal, invalid, zero-check
boundary, and recovery paths.

- `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200 with distinct route
  titles, `lang=en`, one `h1`, one `main`, required metadata, and working
  navigation. `robots.txt` and `sitemap.xml` returned 200.
- An unknown URL returned the deliberate HTTP 404 and the designed **Page not
  found — Infra Test Evidence** page with a route home. Its expected browser
  404 resource message is not a defect.
- The first Tab reached the skip link. Its focus outline was a visible 3 px
  solid ring. Enter moved to main. Forward and Back navigation focused and
  announced the destination heading. Space operated Reset demo.
- Reduced motion changed the transition to `0.00001s` and showed the complete
  recording without animation. The site had no horizontal overflow, retained
  content at 200% text, and the local browser suite verified 44 px targets.
- Live Axe found zero serious or critical issues on root, sample, Privacy,
  Terms, and 404 in desktop and phone contexts across light and dark modes.
- `/opt/fleet/lib/verify-url.sh` passed root in 645 ms and sample in 822 ms,
  with correct title, language, one heading, main landmark, image-alt and
  button-name checks, and no browser errors.

## Quality, performance, and deployment identity

All clean-checkout gates passed:

| Command | Result |
| --- | --- |
| `npm test` | PASS — 8 Rust and 28 frontend tests |
| `npm run check` | PASS |
| `npm run build` | PASS — produced `dist/site/` |
| `npm run qa:browser` | PASS — 28 browser tests |
| `npm run qa:a11y` | PASS — 2 browser projects |
| `npm run package:check` | PASS |
| `npm run consumer:check` | PASS |
| `cargo fmt --check` | PASS |
| `cargo clippy --locked --all-targets -- -D warnings` | PASS |
| `npm audit --audit-level=high` | PASS — 0 vulnerabilities |

The build contains 6.78 kB raw / 2.70 kB gzip main JavaScript, 1.34 kB raw /
0.68 kB gzip route JavaScript, and 11.79 kB raw / 3.39 kB gzip CSS. A fresh
mobile Lighthouse run scored 100 Performance, 100 Accessibility, 100 Best
Practices, and 100 SEO. LCP was 1.54 s, CLS was 0, and TBT was 0 ms.

Live security headers include CSP with response-header `frame-ancestors
'none'`, HSTS, `nosniff`, DENY framing, strict referrer policy, and a
Permissions Policy. Hashed assets return `public, max-age=31536000,
immutable`.

Local and live SHA-256 values match for root, sample, Privacy, Terms, the 404,
all built JS/CSS, `cli-demo.cast`, `demo-evidence.json`, and the social card.
The deployed runtime therefore matches `b0d8ea1`. Commits `4a7778e` and
`98d80c9` change only handoff and QA documentation, so they do not require a
new product image.

## Earlier finding disposition

All earlier reports and reviews, including minor findings, were inspected.

| Earlier finding | Current proof |
| --- | --- |
| Verification 1/2: converter and reviewer output absent | Closed. Installed package converts a real-style event stream into JUnit, evidence JSON, and a reviewer page. |
| Verification 1/2: malformed input could return success | Closed. Strict-validation claims pass; independent malformed duration exits 2, then valid recovery succeeds. |
| Verification 1/2: Privacy and Terms broken | Closed. Both live routes return 200 with distinct titles, structure, metadata, links, and clean Axe results. |
| Verification 2: file chooser had no visible focus | Closed. Keyboard checks show the designed 3 px focus ring. |
| Verification 1/2: hashed assets not immutable | Closed. Live hashed assets return the one-year immutable policy. |
| Verification 1/2: response policy incomplete | Closed. Current CSP and security headers pass live inspection. |
| Verification 3: sensitive diagnostic leaked | Closed. Sensitive-value and whole-diagnostic claims scan all outputs and pass. |
| Verification 3: IaC context omitted or misattributed | Closed. Conversion and run-correlation claims pass; sample displays path, duration, redaction, and hash from the bundled artifact. |
| Verification 3: partial or unsupported streams succeeded | Closed. Seven malformed event-stream cases fail without artifacts. |
| Verification 3: clean test timeout and generated-page Axe issues | Closed. Cold check passes; generated and live pages have no serious or critical Axe issue. |
| Verification 3 boundary and minor release items | Closed. Duration/status validation, formatting, Clippy, favicon, accessible wordmark, and exact test counts pass. The absent historical brief is handled from the supplied work-order brief. |
| Verification 4: explicitly sensitive plan values leaked | Closed. Sensitive redaction and fail-closed claims pass across all outputs. |
| Verification 5: manifest and isolated sample absent | Closed. The 24-entry manifest and one-click in-memory sample pass. |
| Verification 5: audience, discovery routes, and designed 404 absent | Closed. Audience, robots, sitemap, legal routes, and designed HTTP 404 are live. |
| Verification 6: browser claims failed in a clean clone | Closed. Every exact browser claim command passes after `npm ci`. |
| Verification 6/7: safety and requested-write claims absent | Closed. Event validation, correlation, diagnostic, no-external-effect, and requested-write claims are listed and pass. |
| Verification 7: real CLI recording absent | Closed. Self-hosted cast, transcript, controls, outputs, and reduced-motion state pass. |
| Verification 9: AWS identifiers leaked | Closed. ARN and EC2 redaction passes across XML, JSON, and HTML. |
| Verification 9: sample dropped assertion evidence | Closed. Sample fidelity compares the displayed fields with the shipped CLI artifact. |
| Verification 9: wrong duration types and contradictory summary accepted | Closed. Both dedicated claims pass. |
| Verification 10: subnet, security-group, Azure, and GCP identifiers leaked | Closed. Cross-provider redaction scans all artifacts and passes. |
| F-13-1: required phone facts below fold | Closed. All required content ends by 648.84 px in the 664 px live viewport. |

| Copy/review finding | Current proof |
| --- | --- |
| F-1-1 | Closed. One click shows the failed check, path, redaction, hash, and three outputs. |
| F-1-2 | Closed. Browser compact-record import is listed and tested. |
| F-1-3 | Closed. CLI help is listed and tested. |
| F-1-4 | Closed. Copy and tests use and parse JUnit XML/report. |
| F-1-5 | Closed. Machine-readable validation covers both input forms. |
| F-1-6 | Closed. The untested deployment-policy sentence remains absent. |
| F-1-7 | Closed. Desktop and phone first screens contain all three facts. |
| F-1-8 | Closed. Forward and Back focus and announce the route heading. |
| F-1-9 | Closed. The 404 has canonical, Open Graph, Twitter, title, and route home. |
| F-1-10 | Closed. Recording heading names all three outputs. |
| F-1-11 | Closed. The decorative slogan is replaced by **Generated files**. |
| F-1-12 | Closed. The error label is **404 · PAGE NOT FOUND**. |
| F-1-13 / F-2-1 / review-3 reopening | Closed. Runtime, pages, README, and claims use JUnit report, evidence JSON, and reviewer page. |
| F-1-14 / review-3 reopening | Closed. Browser input copy consistently says compact record. |
| F-1-15 | Closed. Compact-record schema instructions are short sentences. |
| F-1-16 | Closed. README names exact sensitivity markers and the redaction result. |
| F-1-17 / F-2-5 | Closed. Vague workflow wording is absent and the browser input is named. |
| F-1-18 | Closed. Vague response-policy wording remains absent. |
| F-1-19 | Closed. Event-stream rules are short, direct sentences. |
| F-1-20 | Closed. Whole-diagnostic redaction says values stay out of every output. |
| F-2-2 | Closed. Strict validation covers normal, malformed, incomplete, output-error, and usage cases. |
| F-2-3 | Closed. Named-field claim enumerates and tests every published pattern. |
| F-2-4 | Closed. Sample claim asserts all three output paths and their source. |
| F-2-6 / F-3-1 | Closed. Public copy names inputs, paths, changes, failures, and input SHA-256 without “provenance”. |
| F-3-2 | Closed. README consistently says assertion paths. |

No prior finding has reopened. No new finding was observed.

## Evidence

Detailed logs and browser artifacts are under
`/work/.evidence/infra-test-evidence-verify-14/`. The report is copied to
`/work/.evidence/qa-report.md`; the machine result is
`/work/.evidence/qa-result.json`.
