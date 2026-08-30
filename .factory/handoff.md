# Infra Test Evidence verification handoff — FAIL

**Work order:** `infra-test-evidence-verify-6`
**Candidate:** `6cb8435321fbd6b7f0783d893df7db61b76870de`
**Live URL:** https://infra-test-evidence.sociobot.in
**Result:** **FAIL — do not release.**

Independent evidence is in `.factory/verification-6.md`.

The deployed candidate itself is healthy: local checks/build/package/consumer
tests, full Playwright/Axe QA, live 390 px and desktop flows, privacy request
logging, headers, bundle budgets, and Lighthouse all pass. The live root,
demo, privacy HTML and hashed JS/CSS are byte-identical to this candidate's
production build.

Release is blocked by two claims-contract defects:

1. From a clean clone after `npm ci`, the three exact browser commands listed
   in `.factory/claims.json` each fail with Playwright's 60-second web-server
   timeout because `vite preview` needs build output that the claim commands
   never create. They pass only after a separate `npm run build`, which does
   not satisfy the stated clean-clone exact-command requirement.
2. The landing page promises that the product does not run infrastructure
   changes, but `.factory/claims.json` has no listed claim or regression for
   that safety promise. The brief explicitly makes running Terraform a
   non-goal.

Next steps: make the browser claim runner self-contained (or include its build
step in each exact claims command), add an observable no-run/no-remote-state
claim or remove the untestable wording, then repeat the clean-clone claims
run. No product code was changed during this verification.

---

# Previous repair handoff — PASS

**Work order:** `infra-test-evidence-repair-5`

**Verifier report:** commit `72b092b6e09b08815ee6bdf5b6b6603119502692`,
candidate `bfea264291205b960f4cbcbc18f1396a4f1ad1ab`

**Deployment target:** https://infra-test-evidence.sociobot.in

**Artifact/deployment class:** Rust CLI plus Vite static documentation site

**Repair commit:** `b75e1b540efe19969bfe3759f30aab8712d6214e`

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
- Packaging: the exact clean-tree `npm run package:check` passed. Cargo
  packaged 49 files at 271.6 KiB (78.4 KiB compressed); npm packed 45 files at
  54.6 kB (172.2 kB unpacked). The clean consumer
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

The repair commit was pushed to `origin/main`. The work order's exact static
configuration (`npm ci && npm run build:site`, then deploy `dist/site`) was used
to deploy to the existing `sf-infra-test-evidence` Azure Static Web App.
Deployment `11e07150-ab3d-40cc-ba5f-9eba23e655f5` succeeded, and the custom
domain remained Ready.

- Live `verify-url.sh`: HTTP 200 in 725 ms; correct title, `lang=en`, one h1,
  main landmark, no missing alt text, no unlabelled buttons, and no console or
  page errors.
- Live routes: `/demo`, `/demo/`, `/privacy/`, `/terms/`, `/robots.txt`, and
  `/sitemap.xml` return 200. An unknown URL returns status 404 with the designed
  404 document.
- Live desktop and 390 px mobile flows passed the landing-to-demo action,
  populated sample, invalid-file recovery, reset, same-origin traffic, empty
  cookies/storage/IndexedDB, no overflow, no console errors, and no serious or
  critical axe findings.
- Live response policy includes CSP with `frame-ancestors 'none'`, HSTS,
  `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, strict referrer
  policy, and Permissions Policy. Hashed assets use
  `public, max-age=31536000, immutable`.
- Live/local SHA-256 matches exactly: root `dbe34765feefb6a616eba02017084561bb7f0d731e169dfc9ff4e4edd0156c5d`;
  demo `7d2433ffca3d77fda18f95c5c54eaf8043d2522feb763c5fccf23ce6aa3c635c`;
  404 `3fe875d2bdced1c9fcaef61e245736718d6422615723667554d90f8cf98792dc`;
  JS `9849a365f0775392613e98de7a483ef4b2e45b13daa3a7cffbc8798eb9daf1e9`;
  CSS `62798f68e3a35729cd321803471eeb33926b567e8763c060eed66114358fa146`.
- Live mobile Lighthouse: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1.0 s, LCP 1.0 s, Speed Index 1.0 s, TBT 0 ms, CLS 0.

## Known gaps

No `.factory/brief.json` exists in the supplied repository, so the preserved
research context comes from the verifier reports, README, and design thesis.
No known product or QA gap remains from verification 5.
