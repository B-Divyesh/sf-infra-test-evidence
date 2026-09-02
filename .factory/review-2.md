# Adversarial first-read review 2

**Product:** Infra Test Evidence  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Reviewed:** 2026-09-02  
**Candidate:** `f86ed93e79c9592874f27f216685e5c478a287a2`  
**Verdict:** **FAIL**

All 23 declared claim commands pass from a clean clone, and the live demo is
representative and isolated. This review still has six findings. Three
published behaviors are not fully asserted by their named claim tests, and an
output-name inconsistency leaves F-1-13 only half-fixed. A pass requires zero
findings and no untested claim.

## Cold first read

I opened `/` in fresh Chromium contexts at 390 × 844 and 1440 × 900 before
scrolling or reading repository copy.

- **What it does:** Converts OpenTofu or Terraform test output into evidence a
  reviewer can inspect.
- **Who it is for:** Infrastructure-module maintainers whose failed tests need
  review.
- **What to click first:** **Try it with sample data**.

All three answers are explicit on both first screens. At 390 px, the headline,
audience sentence, both actions, action explanation, and all three facts end at
`y=793`. At 1440 px, the three facts end at `y=641`.

## Findings

### Blocking

#### F-2-1 — Output names remain inconsistent (F-1-13 is half-fixed)

- **Exact quotes / location:** Landing, How it works: “Create JUnit, reviewer
  JSON, and a static HTML page.” Landing, Generated files: “The CLI creates
  three output files: a JUnit report, evidence JSON, and a reviewer page.”
- **Why this fails:** The same three files have two sets of names. “Reviewer
  JSON” conflicts with “evidence JSON”, and “static HTML page” conflicts with
  “reviewer page”. Review 1 required one output set and polish 1 reported that
  F-1-13 was closed, but this sentence remains in the live page and source.
- **Concrete fix:** Rewrite the step as “Create a JUnit report, evidence JSON,
  and a reviewer page.” Update the copy regression so it rejects “reviewer
  JSON” and “static HTML page” as names for these outputs.

#### F-2-2 — The strict-validation claim test skips published failure cases

- **Exact quotes / location:** README, Strict validation and CI: “Invalid JSON
  and incomplete records exit 2.” “The converter exits 0 for valid input, 2
  for invalid input or output failures, and 64 for incorrect usage.”
- **Claim/test:** `strict-validation`; `npm run test:frontend --
  --testNamePattern '@claim:strict-validation'`.
- **Why this fails:** The tagged test runs one valid compact record, one missing
  file, and one conflicting-options usage error. It does not run invalid JSON,
  an incomplete compact record, or a forced output-write failure. The command
  passes without proving those published exit-code cases, so the claims
  inventory contains untested behavior.
- **Concrete fix:** In the one `@claim:strict-validation` test, add a malformed
  JSON fixture, an incomplete compact-record fixture, and an unwritable or
  rejected output path. Assert exit 2 and the documented machine-readable
  error for each. Alternatively, remove the untested cases from README and
  narrow the claim.

#### F-2-3 — Named-field redaction is published without claim-test evidence

- **Exact quote / location:** README, conversion section:
  “Resource-identifier-named fields are also redacted.”
- **Claim/test:** The nearest entry is `sensitive-redaction`, whose sandbox is
  the explicit-sensitive fixture.
- **Why this fails:** That tagged test covers `sensitive: true`; it contains no
  `id`, `*_id`, `identifier`, `address`, `arn`, or `resource_ref` field holding
  an otherwise harmless value. The resource-identifier claim tests use values
  that match identifier patterns themselves. They do not prove that the field
  name triggers redaction. The sentence is also hard to parse because it does
  not name the fields.
- **Concrete fix:** Rewrite the sentence to name the supported field-name
  patterns. Add one fixture whose values do not match any resource-ID pattern,
  place those values under every promised field-name pattern, and assert that
  none appears in JUnit, evidence JSON, or the reviewer page. Give this behavior
  its own claim or extend the exact wording and tagged test of
  `sensitive-redaction`.

#### F-2-4 — The browser-demo claim test verifies only one of three output names

- **Exact quote / location:** Landing action explanation: “See a failed test,
  redaction, and output files.” `/demo/` lists `report.xml`,
  `evidence/evidence.json`, and `evidence/index.html`.
- **Claim/test:** `site-demo`; `npm run qa:browser -- --grep
  '@claim:site-demo'`.
- **Why this fails:** The tagged test asserts `report.xml` but never asserts
  `evidence/evidence.json` or `evidence/index.html`. The separate
  `demo-artifact-fidelity` test checks the failed check, path, duration,
  redaction, and hash, but also omits the output list. The live page currently
  shows all three; the named browser claim does not protect that outcome.
- **Concrete fix:** Assert all three exact output names in the sample proof for
  both 390 px and desktop. If all three are intended as first-viewport proof,
  assert each bottom edge is within the viewport; otherwise narrow the claim
  to the proof that must be above the fold.

### Minor

#### F-2-5 — “Earlier workflows” does not identify a usable source

- **Exact quote / location:** Landing, Generated files: “The local reader also
  accepts the compact record used by earlier workflows.”
- **Why this matters:** A new visitor cannot identify which workflow or whether
  their file matches it.
- **Concrete rewrite:** “The local reader also opens the compact JSON record
  shown below.”

#### F-2-6 — The reviewer-page description is a dense jargon list

- **Exact quote / location:** README, conversion section: “It records
  test-case inputs, assertion paths where emitted by the runner, redacted
  plan-change summaries, failures, and source provenance, including the input
  SHA-256.” — 22 words.
- **Why this matters:** “where emitted by the runner” and “source provenance”
  require interpretation, even though the useful details are concrete.
- **Concrete rewrite:** “The reviewer page records each test’s inputs,
  assertion path, failure, and redacted plan changes. It also records the
  input SHA-256.”

## Landing-page copy audit

Counts are whitespace-delimited; hyphenated terms count as one word. The meta
description is included. No landing sentence exceeds 22 words or uses a banned
marketing adjective.

| Sentence or factual line | Words | Flag |
| --- | ---: | --- |
| Convert infrastructure test runs into JUnit and redacted evidence that reviewers can inspect locally. | 14 | None |
| Turn infrastructure tests into reviewable evidence | 6 | None |
| For infrastructure-module maintainers who need reviewers to inspect failed OpenTofu or Terraform tests without uploading logs. | 16 | None |
| See a failed test, redaction, and output files. | 8 | F-2-4 |
| Runs in your browser | 4 | None |
| No trackers or uploads | 4 | None |
| Free under the MIT License | 5 | None |
| See the CLI create JUnit, JSON, and HTML | 8 | None |
| The real packaged `--demo` flow converts two bundled checks without setup. | 11 | None |
| Recorded from the packaged 0.1.0 binary. | 6 | None |
| Temporary directory details are shortened. | 5 | None |
| Your run details and checks will appear here. | 8 | None |
| Save OpenTofu or Terraform `test -json` output. | 7 | None |
| Create JUnit, reviewer JSON, and a static HTML page. | 9 | F-2-1 |
| Open the redacted page from disk or inspect a compact record above. | 12 | None |
| The CLI creates three output files: a JUnit report, evidence JSON, and a reviewer page. | 15 | F-2-1 |
| The reviewer page includes test inputs, assertion paths, redacted plan changes, failures, and provenance. | 14 | None |
| The local reader also accepts the compact record used by earlier workflows. | 12 | F-2-5 |
| Use `infra-test-evidence --json evidence.json` to strictly validate this compact record in CI. | 12 | None |
| It does not upload files, run infrastructure changes, or replace a reviewer. | 12 | None |
| It only converts and displays the evidence you provide. | 9 | None |
| Read the privacy details. | 4 | None |
| Infra Test Evidence turns test runs into reviewer evidence. | 9 | None |

Headings are descriptive: **Open a record**, **How it works**, **CI outputs and
compact record format**, and **What it does not do** all name their sections.
Eyebrows also name content rather than mood. Actions use result-naming verbs:
**Try it with sample data**, **Open your evidence file**, **Pause/Replay
recording**, **Choose a JSON evidence file**, and **Read the privacy details**.
Navigation nouns are not form buttons. Status text such as “Recording playing”
and “Waiting for evidence” is state, not a sentence or action.

## README copy audit

No README sentence exceeds 22 words or contains a banned marketing adjective.

| Sentence | Words | Flag |
| --- | ---: | --- |
| Infra Test Evidence converts local OpenTofu or Terraform `test -json` output into a JUnit report and a redacted reviewer page. | 20 | None |
| It is for infrastructure-module maintainers who need reviewers to inspect failed tests without uploading logs or plans. | 17 | None |
| The companion landing page is `https://infra-test-evidence.sociobot.in`. | 6 | None |
| It is a local reader for the compact record used by earlier workflows. | 13 | None; the following sentence identifies the demo |
| Open `https://infra-test-evidence.sociobot.in/?demo=1` to try it with sample data. | 8 | None |
| The landing page also includes a self-hosted recording and transcript of the packaged CLI demo. | 15 | None |
| Install the CLI from a checkout: | 6 | None |
| Or build it without installing: | 5 | None |
| Capture the JSON-lines output, then create three output files: | 9 | None |
| `report.xml` contains the converted checks in JUnit XML. | 8 | None |
| `evidence/` contains `index.html` and `evidence.json`; open `evidence/index.html` directly or serve that directory statically. | 13 | None |
| It records test-case inputs, assertion paths where emitted by the runner, redacted plan-change summaries, failures, and source provenance, including the input SHA-256. | 22 | F-2-6 |
| The reviewer page works from disk and makes no network requests. | 11 | None |
| Sensitive values and common AWS, Azure, and GCP resource identifiers are redacted before they reach the output files. | 18 | None |
| This includes AWS ARNs, EC2, subnet, and security-group IDs, Azure resource IDs, and GCP instance paths. | 16 | None |
| Resource-identifier-named fields are also redacted. | 5 | F-2-3 |
| The CLI also redacts values marked by `sensitive: true`, `before_sensitive`, `after_sensitive`, or `sensitive_values`. | 13 | None |
| Malformed sensitivity metadata rejects the input and does not produce output files. | 12 | None |
| See `examples/tofu-test.jsonl` for a complete sample. | 6 | None |
| Run a realistic sample without preparing an input file: | 9 | None |
| The command writes the bundled sample, JUnit report, evidence JSON, and reviewer page to a new temporary directory. | 18 | None |
| It prints every output path when complete. | 7 | None |
| The CLI still validates the earlier compact JSON record used by the browser reader: | 14 | None |
| Set `run`, `environment`, and `recordedAt` to non-empty strings. | 8 | None |
| Add one named check with status `pass`, `fail`, `error`, or `skip`. | 11 | None |
| If present, `durationMs` and event-stream `elapsed` values must be non-negative finite numbers. | 12 | None |
| Invalid JSON and incomplete records exit 2. | 7 | F-2-2 |
| The converter exits 0 for valid input, 2 for invalid input or output failures, and 64 for incorrect usage. | 19 | F-2-2 |
| `--help` documents every option. | 4 | None |
| `--json` prints a machine-readable validation result for either supported input form. | 11 | None |
| The CLI only reads existing test output. | 7 | None |
| It never invokes OpenTofu or Terraform and never contacts remote state or another service. | 14 | None |
| Deploy `dist/site/` as a static site. | 6 | None |
| The browser reader uses no analytics, remote fonts, CDNs, storage, or uploads. | 12 | None |
| MIT. | 1 | None |
| See LICENSE. | 2 | None |
| The converter rejects an event stream without one final supported summary. | 11 | None |
| It also rejects summary statuses that contradict completed runs, unsupported run statuses, and malformed or negative durations. | 17 | None |
| Test-plan variables, outputs, resource changes, and assertion traversals stay with their matching test run before redacted output files are written. | 20 | None |
| The CLI redacts the whole sensitive diagnostic. | 7 | None |
| This keeps unmarked values in the same diagnostic out of every output file. | 13 | None |

README headings—**Install**, **Convert OpenTofu or Terraform test output**,
**Try the bundled demo**, **Strict validation and CI**, **Develop, test, and
deploy**, **License**, and **Evidence safety**—all identify their sections.
The command blocks are commands, not prose sentences.

### Terminology check

| Concept | Terms observed | Required term |
| --- | --- | --- |
| Generated XML | JUnit; JUnit report | JUnit report |
| Generated JSON | reviewer JSON; evidence JSON | evidence JSON |
| Generated HTML | static HTML page; reviewer page | reviewer page |
| Earlier browser input | compact record; compact JSON record | compact record |
| No-setup mode | demo | demo |
| Individual result | check | check |

## Demo and sandbox behavior

- The first-screen action reaches `/demo/?demo=1` in one click and redirects to
  `/demo/?demo=1`.
- The persistent banner reads “Demo — sample data, nothing is saved” and has
  working **Reset demo** and **Start for real** controls.
- At 390 × 844, the representative proof starts at `y=627`. The failed
  `blocks_public_ingress` check, assertion path, `[REDACTED]`, source hash, and
  `report.xml` are visible in the first viewport. The same proof is visible on
  desktop.
- Replacing the sample with a one-check local file changes the in-memory demo.
  Reset restores the two-check OpenTofu sample and clears the replacement.
  Start for real returns to `/`, removes the banner, and removes sample data.
- Fresh context inspection found no cookies, localStorage, sessionStorage, or
  IndexedDB databases. All requests during landing, demo, mutation, reset, and
  exit stayed on `https://infra-test-evidence.sociobot.in`.
- The packaged CLI claim ran in an isolated temporary directory and created a
  unique demo directory containing the sample, JUnit, evidence JSON, and HTML.

Sandbox isolation and live demo behavior pass. F-2-4 concerns the incomplete
browser claim assertion, not a missing live output.

## Claims verification

I cloned commit `f86ed93e79c9592874f27f216685e5c478a287a2` with `--no-local`
into `/tmp/infra-test-evidence-review2-ElrJkJ/repo`, ran `npm ci`, and ran each
exact `.factory/claims.json` command separately.

| Claim | Result |
| --- | --- |
| `cli-demo` | PASS |
| `cli-recording` | PASS |
| `cli-conversion` | PASS |
| `sensitive-redaction` | PASS |
| `resource-identifier-redaction` | PASS |
| `cross-provider-resource-redaction` | PASS |
| `sensitivity-fail-closed` | PASS |
| `strict-validation` | PASS, but incomplete under F-2-2 |
| `malformed-duration-types` | PASS |
| `event-stream-validation` | PASS |
| `summary-consistency` | PASS |
| `run-correlation` | PASS |
| `sensitive-diagnostics` | PASS |
| `conversion-only` | PASS |
| `requested-path-writes` | PASS |
| `artifact-private` | PASS |
| `reader-private` | PASS |
| `site-demo` | PASS, but incomplete under F-2-4 |
| `demo-artifact-fidelity` | PASS |
| `browser-record-import` | PASS |
| `help-options` | PASS |
| `json-validation-output` | PASS |
| `mit-license` | PASS |

**Command total:** 23 passed, 0 failed.  
**Untested published behavior:** F-2-2, F-2-3, and F-2-4.  
**Unlisted claim-like sentences:** none beyond F-2-3's missing exact contract.

`npm run check`, `npm run build`, and `npm run package:check` also pass from the
clean clone. The built JavaScript totals 8.12 kB raw and 3.37 kB gzip.

## History verification

I read `.factory/review-1.md`, `.factory/polish-1.md`, and the current
`.factory/handoff.md`, then checked each prior finding on the live site and in
the current source/tests.

| Earlier ID | Status in review 2 | Evidence |
| --- | --- | --- |
| F-1-1 | Fixed | Live one-click demo shows a failed OpenTofu check, assertion path, redaction, hash, and output list before the chooser; Reset and exit work. |
| F-1-2 | Fixed | `browser-record-import` is listed and passes. |
| F-1-3 | Fixed | `help-options` is listed and passes. |
| F-1-4 | Fixed | Copy now says converted checks are in JUnit XML; the test checks balanced XML and both cases. |
| F-1-5 | Fixed | `json-validation-output` is listed and passes both input forms. |
| F-1-6 | Fixed | The untested response-policy sentence is absent. |
| F-1-7 | Fixed | All three facts are above the fold at 390 × 844 and 1440 × 900. |
| F-1-8 | Fixed | Forward and Back focus the route `h1` and update the polite announcement live. |
| F-1-9 | Fixed | The designed 404 has canonical, Open Graph, and Twitter metadata. |
| F-1-10 | Fixed | Heading is “See the CLI create JUnit, JSON, and HTML”. |
| F-1-11 | Fixed | Decorative label is now “Generated files”. |
| F-1-12 | Fixed | Error label is “404 · PAGE NOT FOUND”. |
| F-1-13 | **Half-fixed; blocking** | The canonical output set exists, but “reviewer JSON” and “static HTML page” remain. See F-2-1. |
| F-1-14 | Fixed | “Compact record” is the consistent base term. |
| F-1-15 | Fixed | The schema instruction is split into short sentences. |
| F-1-16 | Fixed | README names the exact sensitivity markers and redaction action. |
| F-1-17 | Fixed in its README location | README now identifies the compact JSON record and browser reader. F-2-5 covers separate vague landing copy. |
| F-1-18 | Fixed | “Restrictive browser response policies” is absent. |
| F-1-19 | Fixed | The fail-closed rule is split into two sentences. |
| F-1-20 | Fixed | The diagnostic rule now states that values stay out of every output file. |

The prior handoff's clean-build, claim-command, same-origin, storage, route,
accessibility, and bundle-size statements are confirmed. Its PASS conclusion
is not confirmed because the claim tests leave the cases in F-2-2 through
F-2-4 unproved.

## Structure, links, accessibility, and identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` load. An unknown path
  returns HTTP 404 with the designed page and a working return link.
- Every route has a route-specific title, one `h1`, one `main`, a description,
  canonical, Open Graph fields, Twitter card, favicon, and touch icon.
  `robots.txt` and `sitemap.xml` load and the sitemap lists all indexed routes.
- Every intended internal link crawled to HTTP 200. Deep links load their
  targets. Forward and Back move focus to `h1`, announce the title, and restore
  the top position.
- `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 615 ms with no errors,
  one `h1`, `lang=en`, `main`, complete alt text, and named buttons.
- Independent live Axe scans of all five routes in light and dark mode at
  390 px found zero serious or critical violations. The live root and demo
  emitted no console or page errors during normal flows.
- The field-notebook palette, coral ledger marks, offset paper shadow,
  editorial scale, and terminal recording are product-specific. The layout is
  not a generic centered SaaS hero with feature cards.

## Missed leverage

No AI feature is justified. Conversion, validation, correlation, and redaction
are deterministic evidence-handling tasks; model output would weaken the
audit boundary. Import and export are the core workflow, and sync would
conflict with the local/no-upload contract. No additional leverage finding is
recorded.

## What would make this perfect

Use the three canonical output names everywhere. Then make each tagged claim
test prove every published case: malformed and incomplete compact inputs,
output-write failure, named-field redaction, and all three browser-demo output
names. Replace the two vague or jargon-heavy sentences with the proposed
rewrites. Re-run all 23 commands from a clean clone and repeat the live cold
read, demo sandbox, route crawl, focus, and accessibility checks. Only zero
remaining findings earns PASS.
