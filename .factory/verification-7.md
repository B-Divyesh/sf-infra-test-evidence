# Independent verification 7 — FAIL

**Work order:** `infra-test-evidence-verify-7`
**Candidate:** `a3ae8f446b76a20384a30a5ffff29d0f2219c796`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-09-02 UTC
**Scope:** Rust CLI, generated static reviewer artifact, and deployed static reader

## Decision

**FAIL — release blocked.** The implementation, package, deployment, privacy,
accessibility, and every listed claim test pass. Two acceptance-contract gaps
remain: this CLI product has no landing-page recording of the real binary, and
the README makes evidence-safety promises that are not listed in
`.factory/claims.json` with their own tagged observable tests.

## Mandatory first checks

### First read of the cold live page

The first screen passes. It says **“Turn infrastructure tests into reviewable
evidence,”** names **infrastructure-module maintainers**, and presents **“Try it
with sample data”** as the primary action. The adjacent sentence says the click
will show two example checks. One click opened `/demo/`, displayed the
“Demo — sample data, nothing is saved” banner, and immediately showed two
realistic checks.

### Claims from the clean checkout

After the lockfile install (`npm ci`), every command in
`.factory/claims.json` was run exactly as written, before the general quality
gates. All listed claims passed.

| Claim | Exact command | Result |
| --- | --- | --- |
| `cli-demo` | `npm run test:frontend -- --testNamePattern '@claim:cli-demo'` | PASS (1 test) |
| `cli-conversion` | `npm run test:frontend -- --testNamePattern '@claim:cli-conversion'` | PASS (1 test) |
| `sensitive-redaction` | `npm run test:frontend -- --testNamePattern '@claim:sensitive-redaction'` | PASS (1 test) |
| `sensitivity-fail-closed` | `npm run test:frontend -- --testNamePattern '@claim:sensitivity-fail-closed'` | PASS (1 test) |
| `strict-validation` | `npm run test:frontend -- --testNamePattern '@claim:strict-validation'` | PASS (1 test) |
| `conversion-only` | `npm run test:frontend -- --testNamePattern '@claim:conversion-only'` | PASS (1 test) |
| `artifact-private` | `npm run qa:browser -- --grep '@claim:artifact-private'` | PASS (desktop and mobile) |
| `reader-private` | `npm run qa:browser -- --grep '@claim:reader-private'` | PASS (desktop and mobile) |
| `site-demo` | `npm run qa:browser -- --grep '@claim:site-demo'` | PASS (desktop and mobile) |
| `mit-license` | `npm run test:frontend -- --testNamePattern '@claim:mit-license'` | PASS (1 test) |

The packaged claim tests installed the crate into fresh temporary roots. The
conversion-only test placed instrumented `tofu` and `terraform` executables on
`PATH` and used a preload guard for child processes and network sockets; no
external effect was observed.

## Clean local verification

These commands passed:

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

Evidence:

- The lockfile install added 182 packages and reported 0 vulnerabilities.
- ESLint and TypeScript passed. Rust ran 6/6 tests; Vitest ran 16/16.
- Rust formatting passed and Clippy passed with warnings denied.
- The exact production build created `dist/site/`. Initial JS is 3.73 kB
  (1.67 kB gzip); CSS is 8.64 kB (2.70 kB gzip).
- `cargo package --locked` verified 51 files at 284.2 KiB (82.6 KiB
  compressed). `npm pack --dry-run` verified 47 files at 58.8 kB (185.1 kB
  unpacked). Nothing was published.
- The repository browser suite passed 12/12 desktop/390 px tests. The Axe-only
  invocation passed 2/2 projects.

## Independent packaged CLI exercise

The crate from `target/package/infra-test-evidence-0.1.0` was installed into a
clean consumer root, then exercised through its installed binary.

- `--help` documented both modes, all options, and exit codes 0, 2, and 64.
- `--demo` created a unique directory under an isolated `TMPDIR`, converted two
  checks, and printed the sample, JUnit, JSON, and HTML paths.
- Converting `examples/opentofu-real-stream.jsonl` returned exit 0 and
  `{"checks":2,"errors":[],"valid":true}`. The reviewer JSON contained two
  cases, two assertion paths, plan summaries, failures, and a 64-character
  SHA-256 provenance digest. JUnit reported two failed test cases and 0.270 s.
- Neither `s3cr3t-sentinel` nor `never-export-this` appeared in JUnit, JSON, or
  HTML. Sensitive fields and a sensitive diagnostic were redacted.
- Empty input, malformed JSON, a negative duration, and a missing path each
  returned exit 2 with a specific machine-readable error. Combining `--demo`
  with `--json` returned the documented usage exit 64.
- The generated reviewer page opened from `file:` at 390 px with two cases,
  six keyboard-scrollable evidence regions, no overflow, no network requests,
  no browser errors, and no serious or critical Axe finding.

## Live product, privacy, and accessibility

Independent Playwright flows ran in fresh 1440×900 and 390×844 contexts.

- Demo entry, malformed-file error, corrected-file recovery, reset, and exit
  all worked. The error said what happened and what to do next.
- Every recorded request was same-origin. No cookies, localStorage,
  sessionStorage, IndexedDB databases, or service-worker controller existed.
- There were no console or page errors and no horizontal overflow.
- Keyboard Tab focused the skip link first. Enter moved to `#main`. The skip
  and file-input focus treatments were visible 3 px solid outlines. No tested
  link or button was below 44×44 CSS px.
- Under reduced motion, the drop-zone transition was `0.00001s`.
- Root, demo, privacy, terms, and 404 documents had `lang=en`, one `h1`, one
  `main`, and route-specific titles. Light and dark Axe scans found no serious
  or critical violations. Every link crawled from those documents returned
  200; an unknown URL returned the designed document with HTTP 404.
- `/opt/fleet/lib/verify-url.sh` returned HTTP 200 in 737 ms, found the correct
  title, `lang=en`, one `h1`, a main landmark, no missing alt text, no
  unlabelled buttons, and no console/page errors.

Fresh mobile Lighthouse results:

| Category/metric | Result |
| --- | ---: |
| Performance | 100 |
| Accessibility | 100 |
| Best practices | 100 |
| SEO | 100 |
| FCP / LCP | 1.1 s / 1.1 s |
| TBT / CLS | 10 ms / 0 |
| Total transfer | 7,579 bytes |

The root response uses 30-second revalidation and includes CSP with
`frame-ancestors 'none'`, HSTS, `nosniff`, `DENY`, a strict referrer policy,
and a restrictive Permissions Policy. Hashed JS has
`Cache-Control: public, max-age=31536000, immutable`.

This static product has no backend endpoint, product-unlock call, sign-in,
payment, or API rate-limit surface. It makes no offline/PWA claim and registers
no service worker, so endpoint allowance, Entra authority, persistence,
concurrency, and PWA update checks are not applicable.

## Deployment identity

The live deployment matches the candidate production build byte for byte:

| File | SHA-256 |
| --- | --- |
| `index.html` | `7f2795144556013df7c7e8ecc4f6b4b2deca7910dd1dc94a6fd703387b2747a3` |
| `demo/index.html` | `155863167f2c443429b9c33998c303068ec8e4cd9ec66b1125207050977f565c` |
| `privacy/index.html` | `f0a51f5d23c88e55632b23858a6776a35a8e1d73951d369dfe585015e9d6f1fe` |
| `terms/index.html` | `5e12022b39c8ef72d7644b937d7bc624341a1d823ddb8560b78b6f4353d3c1b7` |
| `404.html` | `5695390422f5346e58d5eb40bffd401a8b14d49f874148d2b1df13686e8c7083` |
| `assets/main-CokhCdvW.js` | `9849a365f0775392613e98de7a483ef4b2e45b13daa3a7cffbc8798eb9daf1e9` |
| `assets/style-tNWs5Hr4.css` | `bdd692b1001dd3b529cb135241900dafa194ecea1ffcaaf697af11eb0cd1dcfc` |
| `robots.txt` | `cec5c0adc25eedc6f353db0fd033747da59af67824c8df1bb62d9e8deb2f0d00` |
| `sitemap.xml` | `5759fad9e2568a277c2b937eb8ce49f2a9cb29b88c73c3dec4dc15601a808447` |

## Release-blocking defects

### High — CLI landing page does not show the real CLI demo

The attached demo-sandbox contract requires a CLI product to provide both a
real `--demo` command and a self-hosted landing-page terminal recording of that
binary performing the main job. The command exists and passes, but the site
contains no terminal recording. “Try it with sample data” opens the compact
browser reader instead; the demo page only prints the command and a sentence
about its outputs. A visitor cannot see the actual converter produce JUnit,
reviewer JSON, and HTML from the web demo.

Add a self-hosted, accessible recording generated from the packaged
`infra-test-evidence --demo` flow, with captions/text alternative and reduced-
motion handling. Keep the existing interactive reader and CLI demo.

### High — README safety promises are absent from the claims manifest

The README says an event stream must end with one supported summary, every
completed run must have a supported status, negative durations are rejected,
and test-plan data is correlated with its test run. Those are observable
safety claims, but `.factory/claims.json` has no entry for event-stream
fail-closed validation. `strict-validation` is explicitly limited to compact
records. Ordinary Rust unit tests cover parts of these behaviors, but the
claims contract requires each relied-on claim to be listed and to have exactly
one `@claim:<id>` test run through the shipped demo/package boundary.

The privacy page also says the CLI writes only to standard output and requested
paths. The existing `conversion-only` sandbox guards child processes and
network sockets, not unexpected filesystem writes, so that promise is not
proved by its named claim.

Add narrowly scoped claim entries and packaged observable tests, or remove or
qualify the promises. This is release-blocking under the attached claims
contract even though all currently listed claim commands pass.

## Other notes

No `.factory/brief.json` exists in this checkout; the researched brief supplied
in the work order was used. No product code, deployment, infrastructure, DNS,
or external resource was modified during verification.
