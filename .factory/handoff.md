# Infra Test Evidence repair handoff — PASS

**Work order:** `infra-test-evidence-repair-6`

**Verifier report:** `dc6d2e2c26acb1640bd30a4e9b6b2de637768b86`

**Rejected candidate:** `6cb8435321fbd6b7f0783d893df7db61b76870de`

**Repair commits:** `16bd7ca532561a77950caab3b896f547fdea5345`,
`c37275b73c8f0e5f858f5d0903d2b5fd076c68bb`

**Live URL:** https://infra-test-evidence.sociobot.in

**Artifact/deployment class:** Rust CLI plus Vite static documentation site

## Repairs

Both release blockers in `.factory/verification-6.md` were reproduced and
repaired without changing the CLI's accepted formats or generated artifacts.

1. From installed dependencies with no `dist/`, the old exact browser claim
   command reproduced `Timed out waiting 60000ms from config.webServer`.
   Playwright now runs `npm run build:site && npm run preview`, so every browser
   claim creates and serves its own production build. A contract test protects
   this configuration. Each of the three formerly failing exact commands was
   rerun with `dist/` absent and passed in both desktop and 390 px projects.
2. `.factory/claims.json` now lists `conversion-only` for the documented safety
   boundary. Its exact regression packages and installs the CLI, places
   instrumented `tofu` and `terraform` commands first on `PATH`, and runs the
   converter with a Linux preload guard. Any child-process or network-socket
   call writes a marker and terminates the CLI. Conversion returned exit 0,
   wrote JUnit, JSON, and HTML, and created no marker.

The final live pass also exposed a demo-banner link below the 44 px touch
minimum. The shared banner style now gives both controls a 44 px minimum
height, and the desktop/mobile regression measures controls on `/demo/` as
well as `/`.

## Clean local verification

The following passed on 2026-08-30 UTC:

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

- Clean install added 182 packages and found 0 vulnerabilities.
- `npm run check` passed ESLint, TypeScript, 6 Rust tests, and 16 Vitest
  unit/integration tests.
- All 10 exact commands in `.factory/claims.json` passed independently. The
  browser commands were proven self-contained from missing build output.
- Clippy passed with warnings denied; Rust formatting is clean.
- `cargo package` verified 51 files at 284.2 KiB (82.5 KiB compressed).
  `npm pack --dry-run` verified 47 files at 58.8 kB (185.1 kB unpacked).
  Registry publication was not attempted.
- The clean consumer command returned
  `{"checks":2,"errors":[],"valid":true}`.
- The final production build emitted 3.73 kB JS (1.67 kB gzip) and 8.64 kB
  CSS (2.70 kB gzip), below the 200 kB JS and 50 kB CSS budgets.
- Playwright passed 12/12 tests across desktop Chromium and a 390 px Chromium
  project. Coverage includes the first-click sample, reset and exit, malformed
  input recovery, keyboard order and focus, reduced motion, 200% text, 44 px
  targets on landing and demo, no overflow, policy routes, the designed 404,
  privacy storage/request checks, and file-URL reviewer evidence.
- The Axe project passed 2/2. Root, demo, privacy, terms, and 404 were scanned
  in light and dark mode with no serious or critical violations; each has one
  `h1`, one `main`, `lang=en`, and its route-specific title.
- Local mobile Lighthouse scored performance 100, accessibility 100, best
  practices 100, and SEO 100. FCP and LCP were 0.9 s, TBT 0 ms, and CLS 0.

## Privacy, offline, and update behavior

Fresh browser contexts recorded only same-origin requests through sample use
and local-file selection. Cookies, localStorage, sessionStorage, and IndexedDB
remained empty. The site has no analytics, remote assets, account, upload
endpoint, backend, payment, or AI feature.

The generated reviewer page was opened from a `file:` URL and made no HTTP(S)
requests. The public site does not claim PWA offline reload and ships no
service worker or update channel, so no PWA update lifecycle applies.

## Deployment and live evidence

The work order's exact build command, `npm ci && npm run build:site`, produced
`dist/site`. That directory was deployed to the existing
`sf-infra-test-evidence` Azure Static Web App. Final deployment
`10ea52ab-7a5b-41af-bdef-67448a8d27ef` succeeded; the custom domain is Ready.

- `verify-url.sh` returned HTTP 200 in 585 ms with the correct title,
  `lang=en`, one `h1`, a main landmark, no missing alt text, no unlabelled
  buttons, and no console or page errors.
- Separate live desktop and 390 px flows passed demo entry, malformed-input
  recovery, reset, exit, keyboard skip navigation, no overflow, and 44 px
  controls. All requests were same-origin; cookies and all browser storage
  stayed empty.
- Live Axe scans covered all five routes in light and dark at both viewports.
  They found no serious or critical violations. Valid routes logged no browser
  errors. An unknown route returned the designed page with HTTP 404.
- Live response policy includes CSP with `frame-ancestors 'none'`, HSTS,
  `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, strict referrer
  policy, and restrictive Permissions Policy. Hashed assets return
  `Cache-Control: public, max-age=31536000, immutable`; HTML uses 30-second
  revalidation.
- Live/local SHA-256 matches exactly: root
  `7f2795144556013df7c7e8ecc4f6b4b2deca7910dd1dc94a6fd703387b2747a3`;
  demo `155863167f2c443429b9c33998c303068ec8e4cd9ec66b1125207050977f565c`;
  privacy `f0a51f5d23c88e55632b23858a6776a35a8e1d73951d369dfe585015e9d6f1fe`;
  terms `5e12022b39c8ef72d7644b937d7bc624341a1d823ddb8560b78b6f4353d3c1b7`;
  404 `5695390422f5346e58d5eb40bffd401a8b14d49f874148d2b1df13686e8c7083`;
  JS `9849a365f0775392613e98de7a483ef4b2e45b13daa3a7cffbc8798eb9daf1e9`;
  CSS `bdd692b1001dd3b529cb135241900dafa194ecea1ffcaaf697af11eb0cd1dcfc`.
- Live mobile Lighthouse scored 100 in performance, accessibility, best
  practices, and SEO; FCP and LCP were 0.8 s, TBT 0 ms, and CLS 0.

## Known gaps

No `.factory/brief.json` exists in the supplied repository. Scope was
preserved from the verifier reports, README, and `.factory/design.md`. No known
release blocker remains from verification 6.
