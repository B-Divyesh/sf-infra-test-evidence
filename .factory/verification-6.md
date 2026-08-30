# Independent verification 6 — FAIL

**Candidate:** `6cb8435321fbd6b7f0783d893df7db61b76870de`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-08-30 UTC
**Scope:** Rust CLI, generated static evidence artifact, and deployed static reader.

## Decision

**FAIL — release blocked.** The deployed product and built artifact work, but
the required browser claim commands do not run from a clean checkout as stated
in `.factory/claims.json`. The claims contract explicitly makes any such
failure release-blocking.

## First read of the cold live page

The cold landing page plainly says it **turns infrastructure tests into
reviewable evidence**, is **for infrastructure-module maintainers**, and makes
the first action **Try it with sample data**. The adjacent explanation says it
will show two example checks in the browser reader. The primary action opened
`/demo/` in one click, showed “Demo — sample data, nothing is saved” and two
realistic checks, Reset demo restored them, and Start for real returned to the
empty reader. This acceptance gate passes.

## Required claims run first from the clean checkout

After `npm ci`, every command in `.factory/claims.json` was run exactly as
written, before `npm run build`.

| Claim | Exact command | Clean-clone result |
| --- | --- | --- |
| `cli-demo` | `npm run test:frontend -- --testNamePattern '@claim:cli-demo'` | PASS |
| `cli-conversion` | `npm run test:frontend -- --testNamePattern '@claim:cli-conversion'` | PASS |
| `sensitive-redaction` | `npm run test:frontend -- --testNamePattern '@claim:sensitive-redaction'` | PASS |
| `sensitivity-fail-closed` | `npm run test:frontend -- --testNamePattern '@claim:sensitivity-fail-closed'` | PASS |
| `strict-validation` | `npm run test:frontend -- --testNamePattern '@claim:strict-validation'` | PASS |
| `artifact-private` | `npm run qa:browser -- --grep '@claim:artifact-private'` | **FAIL** |
| `reader-private` | `npm run qa:browser -- --grep '@claim:reader-private'` | **FAIL** |
| `site-demo` | `npm run qa:browser -- --grep '@claim:site-demo'` | **FAIL** |
| `mit-license` | `npm run test:frontend -- --testNamePattern '@claim:mit-license'` | PASS |

Each failed browser invocation exited 1 with:

```text
Error: Timed out waiting 60000ms from config.webServer.
```

The evidence is deterministic: `vite.config.ts` builds to `dist/site`, while
the Playwright web server is `npm run preview` (`vite preview`), which needs a
pre-existing production output. A brand-new clone has no `dist/` and cannot
serve the demo. After `npm run build`, all three commands passed (two desktop
and mobile projects each). That confirms the functionality is sound but does
not make the exact clean-clone claim commands pass.

## Other local verification

All of the following passed after the production build:

```sh
npm run check
npm run build
npm run package:check
npm run consumer:check
npm run qa:browser
npm run qa:a11y
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
```

Evidence:

- `npm run check`: ESLint, TypeScript, 6 Rust unit tests, and 14 Vitest tests
  passed.
- `npm run build`: produced `dist/site/`; initial JS is 3.73 kB raw / 1.67 kB
  gzip and CSS 8.60 kB raw / 2.70 kB gzip, well below the 200 kB / 50 kB
  budgets.
- `npm run qa:browser`: 12/12 passed across desktop and 390 px mobile,
  including sample demo, reset/exit, malformed JSON recovery, no overflow,
  keyboard skip link/focus, 44 px targets, reduced motion, 200% text, local
  storage/cookie/IndexedDB checks, and generated file artifact behavior.
- `npm run qa:a11y`: 2/2 passed. Direct live Axe scans of `/`, `/demo/`,
  `/privacy/`, `/terms/`, and `/404.html` in light and dark found no serious or
  critical issues; all had exactly one `h1` and one `main`.
- A clean consumer install of the packaged crate succeeded. `--help` describes
  the public CLI and exit codes; valid compact JSON returned
  `{"checks":2,"errors":[],"valid":true}`; empty input returned exit 2; and
  `--demo` created a new temporary directory containing `tofu-test.jsonl`,
  `report.xml`, `evidence/evidence.json`, and `evidence/index.html`.
- The bundled sample input contains a password sentinel; the generated JUnit,
  JSON, and HTML did not contain it. The packaged claims separately cover
  explicit-sensitive values and malformed sensitivity metadata fail-closed.

## Live deployment, privacy, performance, and headers

- Live root, demo, and privacy HTML plus hashed JS and CSS are byte-identical
  to the candidate production build. The deployed asset names are
  `main-CokhCdvW.js` and `style-BV_yaj-y.css`.
- Fresh desktop and 390 px Playwright contexts recorded only same-origin
  requests, no cookies, no local/session/IndexedDB storage, no console errors,
  and no page errors. Local file selection and demo use did not upload data.
- Response headers include CSP with `frame-ancestors 'none'`, HSTS,
  `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, strict referrer
  policy, and a restrictive Permissions Policy. Hashed JS has
  `Cache-Control: public, max-age=31536000, immutable`; HTML has the expected
  short revalidation policy. Unknown routes return the designed 404 with HTTP
  404. This is a static product, with no server endpoint, sign-in, PWA, or
  rate-limit surface to test.
- Live mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1.0 s, LCP 1.0 s, TBT 0 ms, CLS 0.

## Defects

### High — clean-clone browser claim commands are not self-contained (release blocker)

The three browser entries in `.factory/claims.json` fail when run exactly from
a clean clone because the configured web server only previews built files.
This violates the explicit clean-sandbox claims requirement and produces three
failed release-blocking claim tests. Make the claim command build the site
first, or configure Playwright to start a server that does so, then prove each
exact command from a new clone.

### High — an explicit safety promise has no claim entry or observable regression (release blocker)

The landing page says, “It does not upload files, run infrastructure changes,
or replace a reviewer.” `.factory/claims.json` covers uploads, but has no
claim/test for never running infrastructure changes. The researched brief
names running Terraform as a non-goal. Under the claims contract, a visitor
relied-on sentence without a listed claim fails review. Add a named claim with
an isolated test demonstrating the CLI neither invokes Terraform/OpenTofu nor
contacts a remote state/service, or remove/qualify the untestable sentence.

## Not findings

The first-screen plain-words/demo acceptance gate passes; the builder's prior
deployment-only issue is not present in this candidate. There is no evidence
of tracking, remote assets, uploads, unsafe headers, accessibility violations,
bundle-budget excess, a runtime error, or a mismatch between this deployment
and the candidate build.
