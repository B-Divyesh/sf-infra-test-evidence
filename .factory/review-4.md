# Review infrastructure tests as local evidence — review 4

**Work order:** `infra-test-evidence-review-4`  
**Verdict:** **PASS**  
**Finding count:** **0**  
**Untested claim count:** **0**  
**Implementation candidate:** `b0d8ea1c6a58fa66bc082c539df0a908e50b986b`  
**Documentation base:** `fe90c1bce60c9577809c3d057fea508e3a48f2cd`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Reviewed:** 2026-09-06 UTC

## Verdict

**PASS.** The live site, packaged CLI, generated reviewer artifacts, demo
sandbox, privacy behavior, routes, accessibility, and all 24 declared claims
pass. There are zero findings at every severity and zero untested claims.

The committed full QA report, `.factory/verification-14.md`, was read before
testing. The separate factory-evidence path named in the work order was not
mounted in this disposable worker, so this review regenerated its own evidence
from a fresh candidate checkout. The repository has no `.factory/brief.json`;
the researched brief in the work order remains the acceptance source.

## Job, audience, and first action

Before scrolling in fresh desktop and `390 × 664` phone browsers, the live
page states:

- Job: **Turn infrastructure tests into reviewable evidence**.
- Audience: infrastructure-module maintainers reviewing failed OpenTofu or
  Terraform tests without uploading logs.
- First action: **Try it with sample data**.
- Expected result: **See a failed test, redaction, and output files.**

The route title is **Infra Test Evidence — review test runs locally**. It is 46
characters and names the job. The public copy uses plain headings and contains
no banned marketing words or metaphorical section names.

At the phone viewport, the required first-screen content ends at:

| Content | Bottom (px) |
| --- | ---: |
| Job | 247.41 |
| Audience | 377.84 |
| First action | 456.13 |
| Expected result | 552.02 |
| Runs in your browser | 589.63 |
| No trackers or uploads | 619.23 |
| Free under the MIT License | 648.84 |

Everything fits within 664 px. This independently confirms the closure of
F-13-1.

## Demo and privacy

The primary action opened `/demo/?demo=1` in one click. The persistent banner
said **Demo — sample data, nothing is saved** and retained **Reset demo** and
**Start for real** throughout the flow.

The populated sample showed two checks, failed check
`blocks_public_ingress`, assertion path
`aws_security_group.web.ingress`, 310 ms, `[REDACTED]`, input SHA-256
`85bfaca…0711522`, and all three output paths: `report.xml`,
`evidence/evidence.json`, and `evidence/index.html`.

A private compact record replaced the sample only in memory. Malformed JSON
produced the documented direct error. Space activated **Reset demo** and
restored the bundled sample. **Start for real** returned to the empty reader.
A zero-check boundary record produced specific repair guidance, and a valid
record loaded immediately afterward.

The desktop and phone flows each recorded 45 requests across all navigation,
sample, error, reset, and recovery steps. Every request was same-origin.
Cookies, localStorage, sessionStorage, IndexedDB, and service-worker
registrations remained empty. The sample did not read or change real data.

## Declared claims

A clean remote checkout detached at the implementation candidate received
`npm ci`. Every exact command in `.factory/claims.json` then ran separately.
Every claim ID occurs in exactly one source test.

| Claim | Result |
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

Landing, demo, policy, README, empty, error, and runtime-status copy was
compared with the claim inventory. No missing, false, incomplete, unlisted, or
untested public claim was found.

AI is not missed leverage for this product. Conversion, validation,
correlation, and redaction need deterministic local behavior. A model would
weaken the audit boundary without improving the job. Import and export are
already the core workflow; remote sync would conflict with the no-upload
boundary.

## Packaged CLI and reviewer artifact

The crate produced by `cargo package --locked` was installed into a new
consumer root. The installed binary, rather than the workspace binary, passed:

- `--help`, including supported options and process codes;
- `--demo`, creating a unique temporary directory and printing the bundled
  input, JUnit report, evidence JSON, and reviewer-page paths;
- normal conversion of `examples/opentofu-real-stream.jsonl` into three
  non-empty artifacts;
- a string-duration counterexample with exit 2 and machine-readable output;
- immediate recovery with `examples/passing-evidence.json` and
  `{"checks":2,"errors":[],"valid":true}`;
- conflicting `--demo --json` usage with exit 64.

The complete claim runs additionally cover malformed JSON, incomplete compact
records, rejected output paths, wrong duration types, incomplete and
contradictory event streams, malformed sensitivity metadata, interleaved runs,
sensitive diagnostics, named fields, and AWS, Azure, and GCP identifiers.
Guarded process tests prove no Terraform/OpenTofu invocation, network socket,
or write outside requested paths.

The installed binary's reviewer page opened from `file:` at 390 px with one
`h1`, one `main`, six keyboard-scrollable evidence regions, no network
request, no console error, no overflow, and no serious or critical Axe issue.

## Live routes, keyboard, accessibility, and links

- `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200 with distinct titles,
  `lang=en`, one `h1`, one `main`, header/nav/footer landmarks, complete route
  metadata, and working navigation.
- `/not-found-review-4` deliberately returned HTTP 404 and the designed
  **Page not found — Infra Test Evidence** document with a route home. This
  expected response is not a defect.
- All crawled internal links and `robots.txt`, `sitemap.xml`, the social card,
  icons, recording, and demo artifact returned 200.
- The first Tab focused the skip link. Its focus treatment was a visible 3 px
  solid blue ring. Enter moved to main. Forward and Back navigation focused
  and announced the destination heading. Space operated Reset demo.
- Reduced motion changed the transition to `0.00001s` and exposed the complete
  recording without animation. The site retained its content at 200% text
  without horizontal overflow. The browser suite confirmed 44 px targets.
- Independent live Axe scans found zero serious or critical issues on root,
  demo, Privacy, Terms, and 404 in both light and dark modes. In fact, the
  scans reported zero violations of any impact.
- `/opt/fleet/lib/verify-url.sh` passed root in 618 ms and demo in 587 ms with
  correct title, language, heading, main, alt text, button names, and no
  browser errors.

## Quality, performance, security, and runtime identity

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

The production build contains 6.79 kB raw / 2.71 kB gzip main JavaScript,
1.34 kB raw / 0.70 kB gzip route JavaScript, and 11.79 kB raw / 3.39 kB gzip
CSS. A fresh mobile Lighthouse run scored 100 Performance, 100 Accessibility,
100 Best Practices, and 100 SEO. LCP was 0.9 s, CLS was 0, and total blocking
time was 30 ms.

Live responses include CSP with response-header `frame-ancestors 'none'`,
HSTS, `nosniff`, DENY framing, strict referrer policy, and a Permissions
Policy. Hashed assets return `public, max-age=31536000, immutable`.

Local and live SHA-256 values match for root, demo, Privacy, Terms, 404, every
built JS/CSS asset, `cli-demo.cast`, `demo-evidence.json`, and the social card.
The live runtime is therefore the implementation candidate. The commits from
`b0d8ea1` through documentation base `fe90c1b` modify only
`.factory/handoff.md` and `.factory/verification-14.md`; they do not require a
new product image.

This is a static site and local CLI. It has no backend, account, tenant,
payment path, remote-state access, health API, request allowance, or stored
product state. Tenant isolation, restart persistence, health, and
`429`/`Retry-After` checks do not apply. There is no offline or update promise
and no service worker.

## Earlier finding disposition

Every earlier review, polish report, verification, and handoff was inspected.

| Earlier finding | Current proof |
| --- | --- |
| Verification 1/2: required converter and reviewer output absent | Closed. The installed package converts a real-style event stream into JUnit, evidence JSON, and a reviewer page. |
| Verification 1/2: malformed input could succeed | Closed. Strict-validation claims pass; an independent wrong-type duration exits 2, followed by valid recovery. |
| Verification 1/2: Privacy and Terms broken | Closed. Both live routes return distinct 200 documents with metadata, landmarks, links, and clean Axe results. |
| Verification 2: file chooser lacked visible focus | Closed. Keyboard coverage shows the designed 3 px focus ring. |
| Verification 1/2: immutable caching and response policy absent | Closed. Live cache and security headers pass inspection. |
| Verification 3: sensitive diagnostics leaked | Closed. Sensitive-value and whole-diagnostic claims scan all outputs and pass. |
| Verification 3: inputs, plans, paths omitted or misattributed | Closed. Conversion and run-correlation claims pass; the sample shows its generated assertion path and hash. |
| Verification 3: incomplete streams could succeed | Closed. Seven malformed event-stream cases fail without artifacts. |
| Verification 3: cold timeout and generated-page Axe issues | Closed. Cold gates pass; generated and live pages have no serious or critical Axe issue. |
| Verification 3 boundary and minor release issues | Closed. Duration/status validation, formatting, Clippy, favicon, accessible wordmark, and exact test counts pass. The absent historical brief is handled from the work order. |
| Verification 4: explicitly sensitive values leaked | Closed. Sensitive redaction and fail-closed claims pass across all outputs. |
| Verification 5: claims manifest and isolated demo absent | Closed. The 24-entry manifest and both browser/CLI demos pass. |
| Verification 5: audience, discovery routes, and designed 404 absent | Closed. Audience, robots, sitemap, legal routes, and designed HTTP 404 are live. |
| Verification 6: browser claims failed in a clean clone | Closed. Every exact browser claim command passed after only `npm ci`. |
| Verification 6/7: external-effect and requested-write claims absent | Closed. The relevant guarded claims are listed and pass. |
| Verification 7: real CLI recording absent | Closed. The self-hosted cast, transcript, controls, output labels, and reduced-motion state pass. |
| Verification 8/11/12: prior PASS conditions | Reconfirmed by clean claims, package, live, privacy, accessibility, and performance checks. |
| Verification 9: AWS identifiers leaked | Closed. ARN and EC2 redaction passes across XML, JSON, and HTML. |
| Verification 9: sample dropped assertion evidence | Closed. Demo fidelity compares displayed fields with the bundled CLI artifact. |
| Verification 9: wrong duration types and contradictory summary accepted | Closed. Both dedicated claims pass. |
| Verification 10: subnet, security-group, Azure, and GCP identifiers leaked | Closed. Cross-provider redaction scans all artifacts and passes. |
| F-13-1: required phone facts below fold | Closed. All required content ends by 648.84 px in the 664 px live viewport. |
| F-1-1 through F-1-12 | Closed. Representative one-click proof, named claim coverage, first-screen facts, route focus/metadata, and plain headings all pass. |
| F-1-13 / F-2-1 / review-3 reopening | Closed. Runtime, pages, README, claims, and recording use JUnit report, evidence JSON, and reviewer page. |
| F-1-14 / review-3 reopening | Closed. Browser input copy consistently says compact record, including errors. |
| F-1-15 through F-1-20 | Closed. Instructions are short and concrete; sensitivity markers and event-stream rules are explicit; vague policy and workflow wording stays absent. |
| F-2-2 through F-2-4 | Closed. Strict failure paths, every named-field pattern, and all three sample output paths have dedicated passing evidence. |
| F-2-5 / F-2-6 / F-3-1 / F-3-2 | Closed. Copy names the input, output details, input SHA-256, and assertion paths consistently without jargon. |

No earlier finding reopened. No new finding was observed.

## Evidence

Detailed logs and artifacts are under
`/work/.evidence/infra-test-evidence-review-4/`. This report is also copied to
`/work/.evidence/qa-report.md`; the machine verdict is
`/work/.evidence/qa-result.json`.
