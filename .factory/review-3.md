# Adversarial first-read review 3

**Product:** Infra Test Evidence

**Live URL:** https://infra-test-evidence.sociobot.in

**Reviewed:** 2026-09-02

**Candidate:** `b08c37df7f28a35c4e57aef2d8ae58e20d3cb9c5`

**Verdict:** **FAIL**

All 24 declared claim commands pass independently from a clean clone. The live
demo is representative, one-click, and isolated. This review still has four
findings. Two terminology findings from review 1 remain in user-facing runtime
copy despite being marked fixed. Per the review contract, each is blocking
again under its original ID. A pass requires zero findings.

## Cold first read

I opened `/` without scrolling in fresh Chromium contexts at 390 × 844 and
1440 × 900. I had not interacted with the page in either context.

- **What it does:** It converts OpenTofu or Terraform test output into local
  evidence for a reviewer.
- **Who it is for:** Infrastructure-module maintainers whose failed tests need
  review without uploading logs.
- **What to click first:** **Try it with sample data**.

The exact first-screen text supporting those answers is “Turn infrastructure
tests into reviewable evidence”, “For infrastructure-module maintainers who
need reviewers to inspect failed OpenTofu or Terraform tests without uploading
logs”, and “Try it with sample data”. The adjacent line says “See a failed test,
redaction, and output files.”

At 390 px, the headline, audience sentence, both actions, action explanation,
and all three facts end at `y=793`. At 1440 px, the facts end at `y=641`. The
first-read gate passes in both viewports.

## Findings

### Blocking

#### F-1-13 — The reviewer page still has a fourth output name

- **Exact quote / location:** Landing-page recording status in `src/main.ts`:
  “Recording complete. JUnit, reviewer HTML, and evidence JSON paths are
  shown.”
- **History:** Review 1 required one output set: **JUnit report**, **evidence
  JSON**, and **reviewer page**. Both polish reports marked F-1-13 fixed, but
  this screen-reader status still says “reviewer HTML”.
- **Why this fails:** A visitor who hears status updates gets a different name
  from the visible heading, transcript, How it works section, README, and CLI
  output. The same output must keep one name everywhere.
- **Concrete fix:** Rewrite the status as “Recording complete. The JUnit
  report, evidence JSON, and reviewer page paths are shown.” Extend the copy
  regression to inspect user-facing strings in `src/main.ts`, not only
  `index.html`.

#### F-1-14 — The browser’s only input still changes names

- **Exact quotes / locations:** Landing page: “Open your evidence file”, “Open
  a record”, “Choose a JSON evidence file”, and “compact record”. File error in
  `src/main.ts`: “Choose an exported evidence record or correct the file and
  try again.” README: “compact JSON record”.
- **History:** Review 1 required **compact record** everywhere for this input.
  The later reviews marked F-1-14 fixed, but the live action, reader, chooser,
  and error still use six variants for the same JSON shape.
- **Why this fails:** A new visitor cannot confirm whether an “evidence file”,
  an “evidence record”, a “JSON evidence file”, and a “compact record” are the
  same accepted input.
- **Concrete fix:** Use **compact record** throughout: “Open a compact record”,
  “Choose a compact record”, and “Choose a compact record or correct the
  file and try again.” Extend the copy regression across HTML and TypeScript.

### Minor

#### F-3-1 — “Provenance” hides the concrete field the page records

- **Exact quote / location:** Landing, **CI outputs and compact record format**:
  “The reviewer page includes test inputs, assertion paths, redacted plan
  changes, failures, and provenance.”
- **Why this matters:** “Provenance” is audit jargon. The README already names
  the useful value as the input SHA-256.
- **Concrete rewrite:** “The reviewer page includes test inputs, assertion
  paths, redacted plan changes, failures, and the input SHA-256.”

#### F-3-2 — The README changes “assertion paths” to “assertion traversals”

- **Exact quote / location:** README, **Evidence safety**: “Test-plan
  variables, outputs, resource changes, and assertion traversals stay with
  their matching test run before redacted output files are written.”
- **Why this matters:** The landing page, demo, claim, and earlier README copy
  use **assertion path**. “Traversal” is unexplained jargon for the same field.
- **Concrete rewrite:** “Test-plan variables, outputs, resource changes, and
  assertion paths stay with their matching test run before redacted output
  files are written.”

## Landing-page copy audit

Counts are whitespace-delimited. Hyphenated terms and inline commands count as
one word unless the command itself contains spaces. The audit includes
reachable empty, error, recording, and reduced-motion states. No sentence is
over 22 words and no banned marketing adjective appears.

| Sentence or factual line | Words | Flag |
| --- | ---: | --- |
| Convert infrastructure test runs into JUnit and redacted evidence that reviewers can inspect locally. | 14 | None; meta description |
| Turn infrastructure tests into reviewable evidence | 6 | None; headline |
| For infrastructure-module maintainers who need reviewers to inspect failed OpenTofu or Terraform tests without uploading logs. | 16 | None |
| See a failed test, redaction, and output files. | 8 | None |
| Runs in your browser | 4 | None |
| No trackers or uploads | 4 | None |
| Free under the MIT License | 5 | None |
| See the CLI create a JUnit report, evidence JSON, and reviewer page | 12 | None |
| The real packaged `--demo` flow converts two bundled checks without setup. | 11 | None |
| Recorded from the packaged 0.1.0 binary. | 6 | None |
| Temporary directory details are shortened. | 5 | None |
| Recording complete. | 2 | None |
| JUnit, reviewer HTML, and evidence JSON paths are shown. | 9 | F-1-13 |
| Reduced motion is on. | 4 | None |
| The complete recording is shown without animation. | 7 | None |
| The visual recording could not load. | 6 | None |
| The full transcript follows. | 4 | None |
| Your run details and checks will appear here. | 8 | None |
| That file is not valid JSON. | 6 | None |
| Choose an exported evidence record or correct the file and try again. | 12 | F-1-14 |
| The file must contain one JSON object. | 7 | None |
| Add a non-empty “run” field. | 5 | None |
| Add a non-empty “environment” field. | 5 | None |
| Add a non-empty “recordedAt” field. | 5 | None |
| Add at least one object to “checks”. | 7 | None |
| Check 1 must be an object. | 6 | None |
| Check 1 needs a name. | 5 | None |
| Check 1 needs a supported status. | 6 | None |
| Check 1 needs a non-negative duration. | 6 | None |
| Save OpenTofu or Terraform `test -json` output. | 7 | None |
| Create a JUnit report, evidence JSON, and a reviewer page. | 10 | None |
| Open the redacted page from disk or inspect a compact record above. | 12 | None |
| The CLI creates three output files: a JUnit report, evidence JSON, and a reviewer page. | 15 | None |
| The reviewer page includes test inputs, assertion paths, redacted plan changes, failures, and provenance. | 14 | F-3-1 |
| The local reader also opens the compact JSON record shown below: | 11 | F-1-14 |
| Use `infra-test-evidence --json evidence.json` to strictly validate this compact record in CI. | 12 | None |
| It does not upload files, run infrastructure changes, or replace a reviewer. | 12 | None |
| It only converts and displays the evidence you provide. | 9 | None |
| Read the privacy details. | 4 | None |
| Infra Test Evidence turns test runs into reviewer evidence. | 9 | None |

### Landing headings and actions

The headings **Turn infrastructure tests into reviewable evidence**, **See the
CLI create a JUnit report, evidence JSON, and reviewer page**, **Open a
record**, **How it works**, **CI outputs and compact record format**, and **What
it does not do** name their sections. The reachable result headings **Recorded
checks** and **Make this record reviewable** also name their states. Labels such as **Packaged CLI demo**,
**Evidence reader**, **Three local steps**, **Generated files**, and
**Boundaries** carry information rather than mood or metaphor.

**Try it with sample data**, **Pause/Play/Replay recording**, **Read the
recording transcript**, **Reset demo**, **Start for real**, and **Read the
privacy details** use result-naming verbs. The input actions and heading
**Open your evidence file**, **Open a record**, and **Choose a JSON evidence
file** are actionable but fail terminology consistency under F-1-14.

## README copy audit

No README sentence exceeds 22 words or uses a banned marketing adjective.

| Sentence | Words | Flag |
| --- | ---: | --- |
| Infra Test Evidence converts local OpenTofu or Terraform `test -json` output into a JUnit report and a redacted reviewer page. | 20 | None |
| It is for infrastructure-module maintainers who need reviewers to inspect failed tests without uploading logs or plans. | 17 | None |
| The companion landing page is https://infra-test-evidence.sociobot.in. | 6 | None |
| It is a local reader for the compact JSON record shown on the landing page. | 15 | F-1-14; use “compact record” consistently |
| Open https://infra-test-evidence.sociobot.in/?demo=1 to try it with sample data. | 8 | None |
| The landing page also includes a self-hosted recording and transcript of the packaged CLI demo. | 15 | None |
| Install the CLI from a checkout: | 6 | None |
| Or build it without installing: | 5 | None |
| Capture the JSON-lines output, then create three output files: | 9 | None |
| `report.xml` contains the converted checks in JUnit XML. | 8 | None |
| `evidence/` contains `index.html` and `evidence.json`; open `evidence/index.html` directly or serve that directory statically. | 13 | None |
| The reviewer page records each test’s inputs, assertion path, failure, and redacted plan changes. | 14 | None |
| It also records the input SHA-256. | 6 | None |
| The reviewer page works from disk and makes no network requests. | 11 | None |
| Sensitive values and common AWS, Azure, and GCP resource identifiers are redacted before they reach the output files. | 18 | None |
| This includes AWS ARNs, EC2, subnet, and security-group IDs, Azure resource IDs, and GCP instance paths. | 16 | None |
| The CLI also redacts values in fields named `id`, `id_*`, `*_id`, `identifier`, `address`, `arn`, `resource_ref`, or `*_resource_ref`. | 17 | None |
| It also redacts values marked by `sensitive: true`, `before_sensitive`, `after_sensitive`, or `sensitive_values`. | 12 | None |
| Malformed sensitivity metadata rejects the input and does not produce output files. | 12 | None |
| See `examples/tofu-test.jsonl` for a complete sample. | 6 | None |
| Run a realistic sample without preparing an input file: | 9 | None |
| The command writes the bundled sample, JUnit report, evidence JSON, and reviewer page to a new temporary directory. | 18 | None |
| It prints every output path when complete. | 7 | None |
| The CLI still validates the earlier compact JSON record used by the browser reader: | 14 | F-1-14; use “compact record” consistently |
| Set `run`, `environment`, and `recordedAt` to non-empty strings. | 8 | None |
| Add one named check with status `pass`, `fail`, `error`, or `skip`. | 11 | None |
| If present, `durationMs` and event-stream `elapsed` values must be non-negative finite numbers. | 12 | None |
| Invalid JSON and incomplete records exit 2. | 7 | None |
| The converter exits 0 for valid input, 2 for invalid input or output failures, and 64 for incorrect usage. | 19 | None |
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
| Test-plan variables, outputs, resource changes, and assertion traversals stay with their matching test run before redacted output files are written. | 20 | F-3-2 |
| The CLI redacts the whole sensitive diagnostic. | 7 | None |
| This keeps unmarked values in the same diagnostic out of every output file. | 13 | None |

README headings—**Install**, **Convert OpenTofu or Terraform test output**,
**Try the bundled demo**, **Strict validation and CI**, **Develop, test, and
deploy**, **License**, and **Evidence safety**—name their sections. Command
blocks are instructions rather than prose sentences.

### Terminology check

| Concept | Terms found | Required term |
| --- | --- | --- |
| Generated XML | JUnit; JUnit report | JUnit report |
| Generated JSON | evidence JSON | evidence JSON |
| Generated HTML | reviewer page; reviewer HTML | reviewer page |
| Browser JSON input | evidence file; JSON evidence file; record; evidence record; compact JSON record; compact record | compact record |
| Assertion location | assertion path; assertion traversal | assertion path |
| No-setup mode | demo | demo |
| Individual result | check | check |

## Demo and sandbox behavior

- The landing primary action reaches `/demo/?demo=1` in one click.
- The persistent banner says “Demo — sample data, nothing is saved” and
  includes working **Reset demo** and **Start for real** controls.
- At 390 × 844, the failed sample starts at `y=628`.
  `blocks_public_ingress`, `[REDACTED]`, `report.xml`,
  `evidence/evidence.json`, and `evidence/index.html` all end within the first
  viewport. They also appear within the 1440 × 900 first viewport.
- Selecting a private one-check file replaces the sample in memory. **Reset
  demo** removes that record and restores the two-check OpenTofu sample.
  **Start for real** returns to `/`, removes the banner, and restores “Waiting
  for evidence”.
- Fresh contexts contained no cookies, localStorage, sessionStorage, or
  IndexedDB databases. Every request during landing, demo, local-file import,
  reset, and exit stayed on `https://infra-test-evidence.sociobot.in`.
- I ran the compiled CLI with `--demo` from a new temporary directory and a
  dedicated `TMPDIR`. It created one new unique directory containing the
  bundled input, `report.xml`, `evidence/evidence.json`, and
  `evidence/index.html`, then printed every path.

The browser and CLI demo requirements pass. No offline claim is published, so
an offline test does not apply.

## Claims verification

I cloned the candidate with `git clone --no-local` into
`/tmp/infra-test-evidence-review3-RU5lgz/repo`, ran `npm ci`, and then ran each
exact command from `.factory/claims.json` separately.

| Claim ID | Result |
| --- | --- |
| `cli-demo` | PASS |
| `cli-recording` | PASS |
| `cli-conversion` | PASS |
| `sensitive-redaction` | PASS |
| `named-field-redaction` | PASS |
| `resource-identifier-redaction` | PASS |
| `cross-provider-resource-redaction` | PASS |
| `sensitivity-fail-closed` | PASS |
| `strict-validation` | PASS |
| `malformed-duration-types` | PASS |
| `event-stream-validation` | PASS |
| `summary-consistency` | PASS |
| `run-correlation` | PASS |
| `sensitive-diagnostics` | PASS |
| `conversion-only` | PASS |
| `requested-path-writes` | PASS |
| `artifact-private` | PASS |
| `reader-private` | PASS |
| `site-demo` | PASS |
| `demo-artifact-fidelity` | PASS |
| `browser-record-import` | PASS |
| `help-options` | PASS |
| `json-validation-output` | PASS |
| `mit-license` | PASS |

**Declared total:** 24 passed, 0 failed.

**Untested declared claims:** none.

**Unlisted claim-like sentences:** none. The four findings concern terminology
and plain-language quality, not missing behavior tests.

The clean clone also passed `npm run check` (lint, typecheck, 8 Rust tests, and
28 frontend tests), `npm run build` (initial JavaScript 8.12 kB raw / 3.37 kB
gzip), `npm run qa:browser` (22 tests), `npm run qa:a11y` (2 tests), and
`npm run package:check`.

## History verification

I read both earlier reviews, both polish reports, and the existing handoff. I
then checked every earlier finding on the live site and in current source and
tests.

| Earlier ID | Status in review 3 | Evidence |
| --- | --- | --- |
| F-1-1 | Fixed | One-click demo shows the failed OpenTofu check, path, redaction, hash, and all outputs above the fold. |
| F-1-2 | Fixed | `browser-record-import` is listed and passes. |
| F-1-3 | Fixed | `help-options` is listed and passes. |
| F-1-4 | Fixed | Copy says JUnit XML; conversion checks balanced XML and both cases. |
| F-1-5 | Fixed | `json-validation-output` covers both input forms. |
| F-1-6 | Fixed | The unsupported deployment-policy claim remains absent. |
| F-1-7 | Fixed | Both cold viewports contain all three facts. |
| F-1-8 | Fixed | Forward and Back focus the destination `h1` and update the polite announcement. |
| F-1-9 | Fixed | The designed 404 has canonical, Open Graph, and Twitter metadata. |
| F-1-10 | Fixed | Recording heading names the JUnit report, evidence JSON, and reviewer page. |
| F-1-11 | Fixed | Decorative label is “Generated files”. |
| F-1-12 | Fixed | Error label is “404 · PAGE NOT FOUND”. |
| F-1-13 | **Not fixed; blocking** | Runtime status still says “reviewer HTML”. |
| F-1-14 | **Not fixed; blocking** | The same browser input is still called six variants. |
| F-1-15 | Fixed | Schema instructions remain short sentences. |
| F-1-16 | Fixed | README names the sensitivity markers and action. |
| F-1-17 | Fixed | README identifies the browser reader’s compact record. |
| F-1-18 | Fixed | Vague response-policy copy remains absent. |
| F-1-19 | Fixed | Event-stream rules remain split into short sentences. |
| F-1-20 | Fixed | Whole-diagnostic redaction is stated directly. |
| F-2-1 | Fixed | How it works uses the canonical three output names. |
| F-2-2 | Fixed | Strict-validation test covers malformed JSON, incomplete input, output failure, valid input, and usage. |
| F-2-3 | Fixed | Named-field claim enumerates and tests every published pattern across all outputs. |
| F-2-4 | Fixed | Demo claim asserts all three paths and their first-viewport positions. |
| F-2-5 | Fixed | “Earlier workflows” is absent. |
| F-2-6 | Fixed at its README location | README now names the inputs, path, failure, changes, and input SHA-256. F-3-1 covers separate landing jargon. |

The prior handoff’s functional, build, privacy, accessibility, and size results
are confirmed. Its PASS conclusion and “Known gaps: None” are not confirmed
because F-1-13 and F-1-14 were not actually closed.

## Structure, links, accessibility, and identity

- `/`, `/demo/?demo=1`, `/privacy/`, and `/terms/` return 200. An unknown path
  returns the designed page with HTTP 404 and a working home action.
- Every checked document has `lang="en"`, exactly one `h1`, one `main`, a
  route-specific title, description, canonical, Open Graph fields, Twitter
  card, SVG favicon, and apple-touch icon. Titles follow the required pattern.
- `robots.txt`, `sitemap.xml`, the social card, and both icons return 200. The
  sitemap lists all four indexed routes.
- Every internal link found across all routes returns 200. Fragment links land
  on existing targets. Forward and Back focus the route `h1`, update the polite
  route announcement, and return to the correct URL.
- The root URL verifier passed in 764 ms with no console errors, one `h1`,
  `lang=en`, a main landmark, complete alt text, and named buttons. Normal live
  routes emitted no console or page errors.
- Independent live Axe scans of all five documents at 390 × 844 and 1440 ×
  900, in light and dark modes, found zero violations. Keyboard, focus,
  reduced-motion, touch-target, and 200% text checks also pass in the browser
  suite.
- Live response headers include same-origin CSP, HSTS, `nosniff`, frame denial,
  strict referrer policy, and Permissions Policy. Hashed assets use immutable
  one-year caching.
- The paper-ledger palette, coral rules, navy evidence panel, offset borders,
  monospace annotations, and terminal recording form a recognisable product
  identity rather than a generic SaaS template.

## Missed leverage

No additional feature finding is justified. The product already supplies the
implied import and export path: it converts test output into three files and
opens the compact record locally. Sync would conflict with the no-upload
boundary. Model-assisted conversion or redaction would make deterministic
evidence handling less trustworthy, so an AI feature is not appropriate.

## What would make this perfect

Use **reviewer page**, **compact record**, and **assertion path** consistently
in every HTML, README, error, and runtime-status string. Replace “provenance”
with “input SHA-256”. Extend the copy regression to scan TypeScript-generated
user copy as well as static HTML. Re-run the full clean-clone claims and live
checklist; only zero remaining findings earns PASS.
