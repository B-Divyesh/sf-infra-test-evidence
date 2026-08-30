# Infra Test Evidence repair handoff — READY TO DEPLOY

**Work order:** `infra-test-evidence-repair-5`

**Verifier report:** commit `72b092b6e09b08815ee6bdf5b6b6603119502692`,
candidate `bfea264291205b960f4cbcbc18f1396a4f1ad1ab`

**Deployment target:** https://infra-test-evidence.sociobot.in

**Artifact/deployment class:** Rust CLI plus Vite static documentation site

## Repairs

All findings in `.factory/verification-5.md` were reproduced and repaired.

- Added `.factory/claims.json` with nine claims. Each claim has one exact
  `@claim:<id>` regression and a documented clean sandbox.
- Added `infra-test-evidence --demo`. The packaged binary runs the embedded
  `examples/tofu-test.jsonl` fixture, creates a unique temporary directory,
  writes JUnit plus JSON/HTML reviewer evidence, and prints every path.
- Added the real `/demo/` page and `/demo` redirect. The first-screen action is
  **Try it with sample data**. Demo data stays in memory; its persistent banner
  provides **Reset demo** and **Start for real** controls.
- Rewrote the first screen to identify infrastructure-module maintainers and
  state the review job directly. Added the required three-step explanation and
  product boundaries. `.factory/copy-audit.md` records sentence counts and
  terminology; no visible sentence exceeds 22 words.
- Added `robots.txt`, `sitemap.xml`, a designed `404.html`, and the Static Web
  Apps 404 response override. Vite now builds separate demo, privacy, terms,
  and 404 documents instead of relying on its development fallback.
- Added canonical and social metadata, a hand-authored ledger social card, an
  apple-touch icon, consistent headers/footers, stricter CSP, 44 px targets,
  and 200% text reflow. Asset provenance is in `.factory/design.md`.
- Preserved the prior explicit-sensitive redaction repair. The installed-package
  regression still scans JUnit, JSON, and HTML for the opaque sentinel.

## Local verification

The following were run from this checkout on 2026-08-30 UTC:

```sh
npm ci
npm run lint
npm run typecheck
npm test
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
cargo package --locked --allow-dirty
npm pack --dry-run
npm run consumer:check
npm run qa:browser
npm run qa:a11y
```

Results:

- Clean install: 182 packages, 0 audit vulnerabilities.
- Unit/integration: 6 Rust tests and 14 Vitest tests passed.
- All nine commands in `.factory/claims.json` passed independently.
- Packaging: Cargo packaged 49 files at 266.8 KiB (76.8 KiB compressed).
  npm packed 45 files at 52.9 kB (167.2 kB unpacked). The clean consumer
  returned `{"checks":2,"errors":[],"valid":true}`.
- Production build: JS 3.73 kB raw / 1.67 kB gzip; CSS 8.60 kB raw / 2.70 kB
  gzip. Output is under `dist/site/` and remains far below product budgets.
- Playwright: 12/12 desktop and 390 px mobile tests passed. These cover the
  first-click demo, reset/exit, malformed-file recovery, exact static routes,
  same-origin traffic, empty cookies/storage/IndexedDB, keyboard order, visible
  focus, 44 px targets, reduced motion, 200% text reflow, and generated file
  evidence opened without network access.
- Axe: 2/2 project tests passed across `/`, `/demo/`, `/privacy/`, `/terms/`,
  and `/404.html` in both light and dark modes. There were no serious or
  critical findings.
- `verify-url.sh` against the local production preview returned 200 with the
  correct title, `lang=en`, one h1, a main landmark, no missing alt text, no
  unlabelled buttons, and no console or page errors.
- Local mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1.0 s, LCP 1.0 s, TBT 0 ms, CLS 0.

## Privacy, offline, and release behavior

The site has no analytics, external scripts, remote fonts, cookies, browser
storage, upload endpoint, account, backend, payment, or AI feature. The public
reader does not claim PWA offline reload and has no service worker or update
channel, so PWA offline/update tests do not apply. The generated reviewer page
is self-contained and its file-URL offline behavior is covered by
`@claim:artifact-private`.

The factory owns registry publication; no crate or npm package was published.
The package is ready for the factory release process.

## Deployment and live evidence

Deployment is pending the repair commit push to the configured `main` static
deployment branch. Replace this section after the live endpoint matches the
local build.

## Known gaps

No `.factory/brief.json` exists in the supplied repository, so the preserved
research context comes from the verifier reports, README, and design thesis.
No known product or QA gap remains from verification 5.
