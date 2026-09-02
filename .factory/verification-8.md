# Independent verification 8 — PASS

**Work order:** `infra-test-evidence-verify-8`
**Candidate commit:** `f53f1949e56370f4646c52e5d64d589e8f3f54d4`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-02 UTC
**Scope:** packaged Rust CLI, generated static reviewer artifact, and deployed static reader

## Decision

**PASS.** The candidate meets the researched brief: it converts existing
OpenTofu/Terraform test output into JUnit and a self-contained reviewer
artifact, redacts sensitive content by default, and provides a compact local
reader and one-click sandbox. All declared claims, local gates, independent
CLI checks, and live checks passed. No release-blocking defects were found.

`.factory/brief.json` is absent in this checkout, so the work-order researched
brief was used as the acceptance contract.

## Mandatory first checks

### Cold first read

The first live screen says **“Turn infrastructure tests into reviewable
evidence.”** It names **infrastructure-module maintainers** who need reviewers
to inspect failed OpenTofu/Terraform tests without uploading logs. Its primary
action is **“Try it with sample data,”** with the adjacent plain explanation
“See two example checks in the browser reader.”

The primary action opened `/demo/` in one click and immediately showed the
banner **“Demo — sample data, nothing is saved”** plus realistic HTTP health
and database-migration checks. Reset restored the sample; Start for real
returned to the empty local reader. This passes the plain-words and
demo-sandbox requirements.

### Claims from the clean checkout

After `npm ci` (182 packages; 0 vulnerabilities), every tagged claim test
declared in `.factory/claims.json` ran through its clean package/demo entry
point. The two claim runs (`npm run test:frontend -- --testNamePattern
'@claim:'` and `npm run qa:browser -- --grep '@claim:'`) passed **11/11**
packaged-CLI tests and **8/8** browser project tests, respectively. The
individual declared selectors for `cli-demo`, `cli-conversion`,
`sensitive-redaction`, and `sensitivity-fail-closed` were also rerun directly
and each passed.

| Claim | Declared test | Result |
| --- | --- | --- |
| `cli-demo` | `npm run test:frontend -- --testNamePattern '@claim:cli-demo'` | PASS |
| `cli-recording` | `npm run qa:browser -- --grep '@claim:cli-recording'` | PASS |
| `cli-conversion` | `npm run test:frontend -- --testNamePattern '@claim:cli-conversion'` | PASS |
| `sensitive-redaction` | `npm run test:frontend -- --testNamePattern '@claim:sensitive-redaction'` | PASS |
| `sensitivity-fail-closed` | `npm run test:frontend -- --testNamePattern '@claim:sensitivity-fail-closed'` | PASS |
| `strict-validation` | `npm run test:frontend -- --testNamePattern '@claim:strict-validation'` | PASS |
| `event-stream-validation` | `npm run test:frontend -- --testNamePattern '@claim:event-stream-validation'` | PASS |
| `run-correlation` | `npm run test:frontend -- --testNamePattern '@claim:run-correlation'` | PASS |
| `sensitive-diagnostics` | `npm run test:frontend -- --testNamePattern '@claim:sensitive-diagnostics'` | PASS |
| `conversion-only` | `npm run test:frontend -- --testNamePattern '@claim:conversion-only'` | PASS |
| `requested-path-writes` | `npm run test:frontend -- --testNamePattern '@claim:requested-path-writes'` | PASS |
| `artifact-private` | `npm run qa:browser -- --grep '@claim:artifact-private'` | PASS |
| `reader-private` | `npm run qa:browser -- --grep '@claim:reader-private'` | PASS |
| `site-demo` | `npm run qa:browser -- --grep '@claim:site-demo'` | PASS |
| `mit-license` | `npm run test:frontend -- --testNamePattern '@claim:mit-license'` | PASS |

## Clean local verification

All of these commands passed:

```sh
npm ci
npm run check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
npm run package:check
npm run consumer:check
npm run qa:browser
npm run qa:a11y
```

- `npm run check`: ESLint, TypeScript, 6 Rust tests, and 20 Vitest tests all
  passed.
- `npm run qa:browser` and `npm run qa:a11y` passed; Playwright’s recorded
  final status was `passed` with no failed tests.
- Production build created `dist/site/`: initial JS is 5.31 kB (2.21 kB gzip)
  and CSS is 10.61 kB (3.16 kB gzip), well below the 200 kB / 50 kB budgets.
- `cargo package --locked` verified 54 files (319.4 KiB, 91.5 KiB compressed).
  `npm pack --dry-run` verified 50 files (67.5 kB package, 221.2 kB unpacked).
  Nothing was published.
- `npm run consumer:check` returned
  `{"checks":2,"errors":[],"valid":true}`.

## Independent CLI and artifact exercise

I installed the crate from `target/package/infra-test-evidence-0.1.0` into a
fresh temporary consumer root and used the installed binary.

- `--help` documents the two modes, options, and exits 0, 2, and 64.
- `--demo` created a unique directory under an isolated `TMPDIR`, then printed
  the bundled input, JUnit report, reviewer HTML, and evidence JSON paths.
- `--json examples/opentofu-real-stream.jsonl` returned
  `{"checks":2,"errors":[],"valid":true}`.
- A malformed compact record returned
  `{"checks":0,"errors":["missing a non-empty checks array"],"valid":false}`
  and exit 2; the claim suite additionally covers malformed streams,
  non-final/repeated summaries, bad statuses, negative durations, and bad
  usage.
- Conversion of the real OpenTofu stream produced JUnit with two failures,
  0.270 s total duration, a sensitive diagnostic replaced by
  `[REDACTED SENSITIVE DIAGNOSTIC]`, assertion paths, plan summaries, and a
  SHA-256 provenance value. No tested secret sentinel was present.
- The generated reviewer `index.html` opened directly from `file:` at 390 px
  with no HTTP requests, browser errors, or horizontal overflow.

## Live deployment, privacy, and accessibility

Fresh independent Playwright contexts checked desktop (1440×900) and mobile
(390×844).

- On both viewports the one-click demo, reset, exit, keyboard entry, and sample
  rendering worked. The first Tab focused the skip link, with a visible
  `rgb(7, 90, 158) solid 3px` outline. There was no horizontal overflow.
- Each flow made 11 same-origin requests; there were no external origins,
  console/page errors, localStorage entries, or sessionStorage entries.
- Axe found no serious or critical violation on `/`, `/demo/`, `/privacy/`,
  `/terms/`, or `/404.html` in both light and dark schemes. Each had exactly
  one `h1` and one `main`.
- `/opt/fleet/lib/verify-url.sh` passed against the live root in 618 ms. It
  found the expected title, `lang=en`, one `h1`, `main`, zero images lacking
  `alt`, zero unnamed buttons, and no page/console errors.
- Live mobile Lighthouse: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP/LCP 1.1 s, TBT 0 ms, CLS 0, transfer 10 kB.

The deployment has a restrictive response CSP with `frame-ancestors 'none'`,
HSTS, `nosniff`, `DENY`, strict referrer policy, and Permissions Policy. HTML
uses 30-second revalidation; hashed JS is
`Cache-Control: public, max-age=31536000, immutable`.

This static product has no server-side endpoint, sign-in, payment,
product-unlock call, service worker, or API rate-limit surface. Therefore
rate-limit allowance, Entra authority, backend persistence/concurrency, and
PWA update tests are not applicable. It makes no offline claim.

## Deployment identity

The live deployment is the candidate production build byte-for-byte for all
checked files:

| File | SHA-256 |
| --- | --- |
| `index.html` | `1cd7b3d5418699d118359076a8441457d1e96ccfb99ed24bbb3f56454d955f80` |
| `demo/index.html` | `6ddf5a59abf6eeab9c7d3855d81df5f39b28f75d09bc38e0ab56ed0406a34230` |
| `privacy/index.html` | `e0cd3f6cccd7d2589bd4d6d8a430d98ef9ce780bd715d3ae6926388dffa52d06` |
| `terms/index.html` | `6278c575f0b94e3a9de0fe0fd8f8a99dbf991a2da4580cb6a2994bdb6b8f8078` |
| `404.html` | `216ba6079b9fc332f6795e7cd1412398b4754a00e775355944fc9bc0539fbe5d` |
| `cli-demo.cast` | `9f7d82e2f37a42cd7f2c6392c25d5be6cb5be07bbb7fd9d92f2bfa74b50571f5` |
| `assets/main-DuGlsbQ9.js` | `8833f0b9614ee9c796e359d65decbc82e932d4236a8ad4b3a7e85e2043d07311` |
| `assets/style-CXJJGfdM.css` | `25e118bc8f78f3e78f6a97f7f297354309a596333e246ff13496d10565ef15fc` |

## Defects by severity

None found.

## Verification artifacts

Transient local evidence retained during this verification:

- live basic verification: `/tmp/infra-test-evidence-live-verify.7sOhNA`
- Lighthouse JSON: `/tmp/infra-test-evidence-lighthouse.json`
- generated static reviewer artifact: `/tmp/infra-test-evidence-artifact.ydBo7F`
