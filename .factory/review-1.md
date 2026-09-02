# Adversarial first-read review 1

**Product:** Infra Test Evidence

**Live URL:** https://infra-test-evidence.sociobot.in

**Reviewed:** 2026-09-02

**Verdict:** **FAIL**

The declared tests pass, but this review has findings. The one-click browser
demo does not show sample results in the first viewport and does not
demonstrate the CLI's main reviewer output. Five claim-like README or landing
sentences are also absent from `.factory/claims.json`. A pass requires zero
findings and no unlisted claim.

## Cold first read

I opened `/` in fresh Chromium contexts at 390 × 844 and 1440 × 900 without
scrolling.

- **What it does:** It turns OpenTofu or Terraform test output into evidence a
  reviewer can inspect without uploading the source logs.
- **Who it is for:** Infrastructure-module maintainers who need another person
  to review failed tests.
- **What to click first:** **Try it with sample data**.

All three answers are available on the first screen, so the landing-page
first-read gate itself passes. At 1440 × 900, the primary action starts at
`y=825`, but the three product facts start at `y=933` and require scrolling.

## Findings

### Blocking

#### F-1-1 — The one-click demo hides weak, non-representative sample results

- **Location / exact text:** `/demo/`; “2 checks ready”, “HTTP health endpoint
  PASS”, and “Database migration PASS”.
- **Evidence:** At 390 × 844, the sample result begins at `y=870`. At
  1440 × 900, it begins at `y=1019`. The first viewport shows a large intro and
  an empty-looking file chooser, not the seeded result. After scrolling, the
  result is a legacy compact record with two generic passing checks. It does
  not show a failed OpenTofu/Terraform test, a redaction, an assertion path,
  provenance, JUnit output, or the generated reviewer page. Those are the
  product's stated job and differentiators.
- **Why this fails a first visit:** The demo-sandbox requirement says the
  screen after the click must already show the product being used with
  realistic sample data. This screen requires another scroll and demonstrates
  the secondary compact reader rather than the CLI conversion workflow.
- **Concrete fix:** Put the seeded result above the file chooser and within
  the first 844 px. Seed it from `examples/tofu-test.jsonl`, including the
  failed `blocks_public_ingress` run, a visible `[REDACTED]` value, assertion
  path, source hash, and JUnit/JSON/HTML outputs. Keep the current demo banner,
  **Reset demo**, and **Start for real** controls. Add a claim test that asserts
  those representative details are visible without scrolling at 390 × 844
  and 1440 × 900.

#### F-1-2 — The browser import capability is an unlisted claim

- **Location / exact text:** Landing page: “The local reader also accepts the
  compact portable record used by earlier workflows.” README: “It is a local
  reader for the compact evidence record used by older workflows.”
- **Why this fails:** No claim in `.factory/claims.json` states that the normal
  browser reader imports and renders a compact record. The site-demo and
  reader-private tests exercise file selection for other claims, but the
  promised import behavior has no named claim contract.
- **Concrete fix:** Add a `browser-record-import` claim and one tagged test that
  opens `/`, selects the shipped compact sample, and verifies its run,
  environment, checks, statuses, and validation errors. Use “compact record”
  in both sentences.

#### F-1-3 — The `--help` coverage claim is unlisted and untested

- **Location / exact text:** README, Strict validation and CI: “`--help`
  documents every option.”
- **Why this fails:** There is no claims entry or tagged test that runs
  `--help` and compares the documented options with the accepted CLI options.
- **Concrete fix:** Add a `help-options` claim and test that asserts every
  supported flag is present and exits 0, or delete the sentence.

#### F-1-4 — CI compatibility is claimed without a compatibility test

- **Location / exact text:** README, conversion section: “`report.xml` is a
  JUnit test suite suitable for CI consumers.”
- **Why this fails:** `cli-conversion` checks only that the file contains
  `<testsuite`; it does not parse the XML or verify a stated CI consumer or
  JUnit schema.
- **Concrete fix:** Narrow the sentence to “`report.xml` contains the converted
  checks in JUnit XML” and test well-formed XML plus the expected cases, or
  add a named compatibility claim and parse the report with the supported CI
  consumer's JUnit parser.

#### F-1-5 — The two-input `--json` behavior is not in the claim inventory

- **Location / exact text:** README, Strict validation and CI: “`--json`
  prints a machine-readable validation result for either supported input
  form.”
- **Why this fails:** Separate tests happen to inspect JSON output, but no
  claim entry names and tests this exact two-input promise once, as required by
  the claims contract.
- **Concrete fix:** Add a `json-validation-output` claim with one tagged test
  that runs both a compact record and an event stream, parses standard output,
  and checks the documented shape.

#### F-1-6 — The deployment-policy sentence is an unlisted claim

- **Location / exact text:** README, Develop, test, and deploy: “The emitted
  `staticwebapp.config.json` sets restrictive browser response policies and
  immutable caching for hashed assets.”
- **Why this fails:** No claim entry verifies the built file or the live
  response headers and cache policy.
- **Concrete fix:** Replace the vague sentence with the exact policies, then
  add a `site-response-headers` claim that builds the site and checks the live
  CSP, Referrer-Policy, X-Content-Type-Options, and hashed-asset Cache-Control.
  Otherwise remove the sentence.

### Minor

#### F-1-7 — The desktop first screen omits all three product facts

- **Location / exact text:** Landing hero: “Runs in your browser”, “No trackers
  or uploads”, and “Free under the MIT License”.
- **Evidence:** At 1440 × 900 the facts begin at `y=933`.
- **Why this matters:** The mandatory first-screen shape includes the three
  privacy/offline/price facts. The very large headline and vertical spacing
  push them below the fold on a common desktop viewport.
- **Concrete fix:** Reduce the desktop headline size or hero spacing so the
  complete fact row ends above 900 px. Add a viewport assertion for all three
  facts.

#### F-1-8 — Full-page route changes leave focus on the document body

- **Location / exact behavior:** Follow **Demo** from `/`, then use Back.
  `document.activeElement` is `BODY` after both navigations, not the new `h1`.
- **Why this matters:** The site-structure contract requires focus on the new
  `h1` and a polite route announcement. The pages have no route-focus code or
  live route announcer.
- **Concrete fix:** Give each `h1` `tabindex="-1"`, focus it after a page load
  caused by internal navigation, and provide a polite announcement. Add a
  keyboard test for forward and back navigation.

#### F-1-9 — The designed 404 omits required canonical and social metadata

- **Location:** Live unknown route and `404.html`.
- **Evidence:** The 404 has a title, description, favicon, one `h1`, and a way
  home, but no canonical link, Open Graph fields, or Twitter card fields.
- **Concrete fix:** Add canonical and product social metadata to `404.html`,
  then assert them in the route test.

#### F-1-10 — “Watch the three outputs appear” is not a self-contained heading

- **Location / exact text:** Landing CLI recording heading.
- **Why this matters:** “Three outputs” makes sense only after reading nearby
  copy. It does not name the outputs or the section when headings are listed by
  a screen reader.
- **Concrete rewrite:** “See the CLI create JUnit, JSON, and HTML”.

#### F-1-11 — “Small by design” is a decorative slogan

- **Location / exact text:** Landing eyebrow above “CI outputs and record
  format”.
- **Why this matters:** It carries no usable information and could appear on
  an unrelated product.
- **Concrete fix:** Delete it, or replace it with “Generated files”.

#### F-1-12 — The 404 uses ledger metaphor instead of the error name

- **Location / exact text:** `404.html`: “404 · NOT IN THE LEDGER”.
- **Why this matters:** “Ledger” is visual-theme lore, not a clearer
  description of the error.
- **Concrete rewrite:** “404 · PAGE NOT FOUND”.

#### F-1-13 — Output counting and names are inconsistent

- **Locations / exact text:** Landing: “Watch the three outputs appear.”
  README: “create both review outputs.” Elsewhere the files are called a
  “static evidence page”, “reviewer page”, “reviewer evidence”, and “evidence
  artifact”.
- **Why this matters:** A first-time user cannot tell whether the product
  creates two outputs, three files, or one evidence bundle.
- **Concrete fix:** Define the output set once: “The CLI creates three files: a
  JUnit report, evidence JSON, and a reviewer page.” Use **reviewer page** only
  for the HTML file and **output files** for the set.

#### F-1-14 — The compact-input name changes across the landing page and README

- **Locations / exact text:** “compact evidence record”, “compact portable
  record”, “compact record”, and “compact form”.
- **Why this matters:** The plain-words contract requires one word for one
  concept.
- **Concrete fix:** Use **compact record** everywhere.

#### F-1-15 — The README schema sentence exceeds 22 words

- **Location / exact text:** README: “That record requires non-empty `run`,
  `environment`, and `recordedAt` strings and at least one check with a
  non-empty `name` and supported `status` (`pass`, `fail`, `error`, or
  `skip`).” — **26 words**.
- **Concrete rewrite:** “Set `run`, `environment`, and `recordedAt` to non-empty
  strings. Add one named check with status `pass`, `fail`, `error`, or `skip`.”

#### F-1-16 — “Authoritative” hides the actual redaction rule

- **Location / exact text:** README: “Explicit OpenTofu/Terraform `sensitive:
  true` values and `before_sensitive`, `after_sensitive`, and
  `sensitive_values` masks are also authoritative.”
- **Why this matters:** “Authoritative” does not tell the reader what the CLI
  does when it sees a mask.
- **Concrete rewrite:** “The CLI also redacts values marked by `sensitive:
  true`, `before_sensitive`, `after_sensitive`, or `sensitive_values`.”

#### F-1-17 — “Portable workflows” is vague

- **Location / exact text:** README: “The existing compact record remains
  supported for portable workflows.”
- **Why this matters:** It does not name a workflow or explain why the format
  is useful.
- **Concrete rewrite:** “The CLI still validates the earlier compact JSON
  record used by the browser reader.”

#### F-1-18 — “Restrictive browser response policies” is unexplained jargon

- **Location / exact text:** README deployment section.
- **Why this matters:** It does not tell a deployer which protections are
  present. It also contributes to the unlisted claim in F-1-6.
- **Concrete rewrite:** “The deployed site blocks third-party connections and
  caches versioned assets.” Add the test required by F-1-6 before publishing
  this rewrite.

#### F-1-19 — The README fail-closed sentence exceeds 22 words

- **Location / exact text:** README: “The converter fails closed: an event
  stream must finish with one supported test summary, every completed run must
  use a supported status, and negative durations are rejected.” — **27
  words**.
- **Concrete rewrite:** “The converter rejects an event stream without one
  final supported summary. It also rejects unsupported run statuses and
  negative durations.”

#### F-1-20 — “Cannot escape the artifact” is metaphorical and hard to parse

- **Location / exact text:** README: “A sensitive diagnostic is redacted as a
  whole, so an unlabelled value in a diagnostic diff cannot escape the
  artifact.”
- **Concrete rewrite:** “The CLI redacts the whole sensitive diagnostic. This
  keeps unmarked values in the same diagnostic out of every output file.”

## Landing-page copy audit

Counts use whitespace-delimited words. Hyphenated terms and URLs count as one
word. Code separated by a space, such as `test -json`, counts as two words.

| Sentence or factual line | Words | Flag |
| --- | ---: | --- |
| Convert infrastructure test runs into JUnit and redacted evidence that reviewers can inspect locally. | 14 | None; meta description |
| Turn infrastructure tests into reviewable evidence | 6 | None; headline |
| For infrastructure-module maintainers who need reviewers to inspect failed OpenTofu or Terraform tests without uploading logs. | 16 | None |
| See two example checks in the browser reader. | 8 | None |
| Runs in your browser | 4 | None |
| No trackers or uploads | 4 | None |
| Free under the MIT License | 5 | None |
| Watch the three outputs appear | 5 | F-1-10, F-1-13 |
| The real packaged `--demo` flow converts two bundled checks without setup. | 11 | None |
| Recorded from the packaged 0.1.0 binary. | 6 | None |
| Temporary directory details are shortened. | 5 | None |
| Recording complete. | 2 | None; runtime status |
| JUnit, reviewer HTML, and evidence JSON paths are shown. | 9 | F-1-13: use the chosen output names |
| Your run details and checks will appear here. | 8 | None |
| Save OpenTofu or Terraform `test -json` output. | 7 | None |
| Create JUnit, reviewer JSON, and a static HTML page. | 9 | F-1-13 |
| Open the redacted page from disk or inspect a compact record above. | 12 | None |
| Run the converter against `tofu test -json` or `terraform test -json` output to create a JUnit report and a self-contained reviewer page. | 22 | None; at the hard cap |
| The page includes test inputs, assertion paths, redacted plan changes, failures, and provenance. | 12 | None |
| The local reader also accepts the compact portable record used by earlier workflows. | 12 | F-1-2, F-1-14 |
| Use `infra-test-evidence --json evidence.json` to strictly validate this compact form in CI. | 12 | F-1-14 |
| It does not upload files, run infrastructure changes, or replace a reviewer. | 12 | None |
| It only converts and displays the evidence you provide. | 9 | None |
| Infra Test Evidence turns test runs into reviewer evidence. | 9 | None |

### Landing headings and actions

| Text | Kind | Flag |
| --- | --- | --- |
| Demo | navigation | None |
| How it works | navigation / heading | None |
| Privacy | navigation | None |
| Infrastructure test evidence | eyebrow | None |
| Try it with sample data | primary action | None; names the result |
| Open your evidence file | secondary action | None; names the result |
| Packaged CLI demo | eyebrow | None |
| Watch the three outputs appear | heading | F-1-10, F-1-13 |
| Pause / Play / Replay recording | buttons | None; result-naming verbs |
| Read the recording transcript | disclosure action | None |
| Evidence reader | eyebrow | None |
| Open a record | heading | None |
| Choose a JSON evidence file | file action | None |
| Three local steps | eyebrow | None |
| Capture the run / Convert the evidence / Review the result | step headings | None |
| Small by design | eyebrow | F-1-11 |
| CI outputs and record format | heading | None |
| Boundaries / What it does not do | eyebrow / heading | None |
| Read the privacy details | link | None; names the result |

No banned marketing word appears in the landing copy.

## README copy audit

| Sentence | Words | Flag |
| --- | ---: | --- |
| Infra Test Evidence converts local OpenTofu or Terraform `test -json` output into a JUnit report and a redacted static evidence page. | 21 | F-1-13: use “reviewer page” |
| It is for infrastructure-module maintainers who need reviewers to inspect failed tests without uploading logs or plans. | 17 | None |
| The companion landing page is `https://infra-test-evidence.sociobot.in`. | 6 | None |
| It is a local reader for the compact evidence record used by older workflows. | 14 | F-1-2, F-1-14 |
| Open `https://infra-test-evidence.sociobot.in/demo/` to try it with sample data. | 8 | None |
| The landing page also includes a self-hosted recording and transcript of the packaged CLI demo. | 15 | None |
| Install the CLI from a checkout: | 6 | None |
| Or build it without installing: | 5 | None |
| Capture the JSON-lines output, then create both review outputs: | 9 | F-1-13 |
| `report.xml` is a JUnit test suite suitable for CI consumers. | 10 | F-1-4 |
| `evidence/` contains `index.html` and `evidence.json`; open `evidence/index.html` directly or serve that directory statically. | 13 | None |
| It records test-case inputs, assertion paths where emitted by the runner, redacted plan-change summaries, failures, and source provenance, including the input SHA-256. | 22 | None; at the hard cap |
| The reviewer page works from disk and makes no network requests. | 11 | None |
| Secret- and resource-identifier-named fields are recursively redacted before they reach the evidence artifact. | 13 | F-1-13: use “output files” |
| Explicit OpenTofu/Terraform `sensitive: true` values and `before_sensitive`, `after_sensitive`, and `sensitive_values` masks are also authoritative. | 14 | F-1-16 |
| Malformed sensitivity metadata rejects the input and does not produce reviewer artifacts. | 12 | F-1-13: use “output files” |
| See `examples/tofu-test.jsonl` for a complete sample. | 6 | None |
| Run a realistic sample without preparing an input file: | 9 | None |
| The command writes the bundled sample, JUnit report, evidence JSON, and reviewer page to a new temporary directory. | 18 | None |
| It prints every output path when complete. | 7 | None |
| The existing compact record remains supported for portable workflows. | 9 | F-1-17 |
| That record requires non-empty `run`, `environment`, and `recordedAt` strings and at least one check with a non-empty `name` and supported `status` (`pass`, `fail`, `error`, or `skip`). | 26 | F-1-15 |
| Invalid JSON and incomplete records exit 2. | 7 | None |
| The converter exits 0 for valid input, 2 for invalid input or output failures, and 64 for incorrect usage. | 19 | None |
| `--help` documents every option. | 4 | F-1-3 |
| `--json` prints a machine-readable validation result for either supported input form. | 11 | F-1-5 |
| The CLI only reads existing test output. | 7 | None |
| It never invokes OpenTofu or Terraform and never contacts remote state or another service. | 14 | None |
| Deploy `dist/site/` as a static site. | 6 | None |
| The emitted `staticwebapp.config.json` sets restrictive browser response policies and immutable caching for hashed assets. | 14 | F-1-6, F-1-18 |
| The browser reader uses no analytics, remote fonts, CDNs, storage, or uploads. | 12 | None |
| MIT. | 1 | None |
| See LICENSE. | 2 | None |
| The converter fails closed: an event stream must finish with one supported test summary, every completed run must use a supported status, and negative durations are rejected. | 27 | F-1-19 |
| Test-plan variables, outputs, resource changes, and assertion traversals are correlated with their test run before redacted reviewer evidence is written. | 20 | F-1-13: use “output files” |
| A sensitive diagnostic is redacted as a whole, so an unlabelled value in a diagnostic diff cannot escape the artifact. | 20 | F-1-20 |

README headings are descriptive. The install and verification command blocks
are commands rather than sentences. No banned marketing word appears.

### Terminology check

| Concept | Current terms | Required term |
| --- | --- | --- |
| Generated HTML | static evidence page; reviewer page; redacted page | reviewer page |
| All generated files | both review outputs; three outputs; reviewer artifacts; reviewer evidence; evidence artifact | output files |
| Earlier browser JSON | compact evidence record; compact portable record; compact record; compact form | compact record |
| Individual result | check; test case | Keep “check” for compact records and “test run” for OpenTofu/Terraform input; state that distinction once |

## Demo and sandbox evidence

- The landing primary action reaches `/demo/` in one click.
- The persistent banner reads “Demo — sample data, nothing is saved” and
  includes **Reset demo** and **Start for real**.
- Replacing the sample with a one-check local file changes only the in-memory
  result. **Reset demo** restores the two seeded checks and clears the error.
  **Start for real** returns to an empty reader.
- Fresh mobile and desktop contexts had no cookies, localStorage,
  sessionStorage, or IndexedDB databases.
- Every observed request during landing, demo, reset, and exit was to
  `https://infra-test-evidence.sociobot.in`.
- The CLI `--demo` claim test ran from an isolated temporary directory and
  confirmed fresh JUnit, JSON, and HTML files.

Sandbox isolation passes. Demo presentation fails under F-1-1.

## Claims verification

I cloned the committed repository into `/tmp/ite-review-1-CoYuYF/repo`, ran
`npm ci`, and ran the exact `test` command from each claims entry separately.

| Claim ID | Result | Observable evidence |
| --- | --- | --- |
| `cli-demo` | PASS | Packaged binary created sample input plus JUnit, JSON, and HTML in isolated `TMPDIR` |
| `cli-recording` | PASS | Both Playwright projects loaded the local cast, output paths, transcript, and reduced-motion view |
| `cli-conversion` | PASS | JUnit and reviewer files contained cases, inputs, assertion paths, redacted plans, and provenance |
| `sensitive-redaction` | PASS | Sentinel absent from all three generated files |
| `sensitivity-fail-closed` | PASS | Malformed marker exited 2 and wrote no reviewer files |
| `strict-validation` | PASS | Compact valid, invalid, and usage cases returned documented results |
| `event-stream-validation` | PASS | Six malformed stream cases exited 2 without reviewer files |
| `run-correlation` | PASS | Interleaved alpha/beta details remained with their runs |
| `sensitive-diagnostics` | PASS | Sensitive diagnostic sentinel absent from JSON and HTML |
| `conversion-only` | PASS | Preload guard observed no child process or socket |
| `requested-path-writes` | PASS | Preload guard allowed only named output paths |
| `artifact-private` | PASS | Generated file URL made no HTTP request in both projects |
| `reader-private` | PASS | Same-origin requests only; no cookies or browser storage |
| `site-demo` | PASS | Entry, mutation, reset, and exit behavior passed in both projects |
| `mit-license` | PASS | Cargo metadata and LICENSE contain MIT terms |

**Declared total:** 15 passed, 0 failed.

**Unlisted claims:** F-1-2 through F-1-6. These prevent a PASS even though all
listed commands pass.

## History verification

There are no earlier `.factory/review-*.md` or `.factory/polish-*.md` files in
the repository or its reachable history. `.factory/brief.json` is also absent,
so scope was checked against the product contract, README, and shipped behavior.
I read `.factory/handoff.md`, which
reports verification 8 as PASS with no known gaps. Its claim-pass, same-origin
privacy, browser-storage, route, Axe, build-size, and live-load statements were
rechecked. The tests pass, but the handoff did not identify F-1-1 through
F-1-20, so its “No release-blocking gaps found” conclusion is not confirmed.

## Structure, links, and accessibility

- `/`, `/demo/`, `/privacy/`, and `/terms/` return 200. An unknown path returns
  the designed page with HTTP 404 and a working home link.
- Every tested route has `lang="en"`, one `h1`, one `main`, a descriptive
  title, a description, and a favicon. The four indexed routes have canonical,
  Open Graph, and Twitter metadata. F-1-9 records the 404 exception.
- `robots.txt` and `sitemap.xml` load. The sitemap lists all four indexed
  routes.
- All crawled internal links returned 200. The deliberate unknown route
  returned 404 as expected. Deep links `/#how`, `/#reader-title`, and
  `/#format` load their targets. Browser Back returns to `/` and restores the
  top position. F-1-8 records missing route focus.
- The live page produced no console or page errors on normal routes.
  `/opt/fleet/lib/verify-url.sh` passed with title, language, one `h1`, `main`,
  alt, button-label, and console checks.
- Live Axe scans found no violations on `/`, `/demo/`, `/privacy/`, `/terms/`,
  or the designed 404 at 390 px. The full repository browser suite also passed
  its light/dark accessibility tests in both Playwright projects.
- No horizontal overflow appeared at 390 px. The field-notebook palette,
  offset ledger shadows, monospace annotations, and editorial scale form a
  distinct identity rather than a generic SaaS template.

## Missed leverage

No missing AI feature is justified. Conversion, redaction, and validation are
deterministic security-sensitive work, so model output would reduce trust.
Import and export are already the product's core. No sync feature is implied
by the local, no-upload brief. The missed leverage is the representative
one-click artifact preview described in F-1-1.

## What would make this perfect

Resolve every finding. The decisive change is a first-viewport demo built from
the real failing OpenTofu sample that visibly proves redaction, provenance, and
all three outputs. Then inventory or remove every claim, make the naming
consistent, shorten the two long README sentences, replace the three vague
headings or phrases, restore route focus, include the desktop facts above the
fold, and complete the 404 metadata. Re-run the full checklist from a clean
clone; only zero remaining findings earns PASS.
