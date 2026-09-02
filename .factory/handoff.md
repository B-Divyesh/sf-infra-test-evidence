# Infra Test Evidence repair 7 handoff — PASS

**Work order:** `infra-test-evidence-repair-7`

**Verifier report commit:** `f8924f0d016795c02f9e25016e22152d27a1d8d0`

**Rejected candidate:** `a3ae8f446b76a20384a30a5ffff29d0f2219c796`

**Implementation commit:** `ce5ed56`

**Live URL:** https://infra-test-evidence.sociobot.in

**Artifact/deployment class:** Rust CLI plus Vite static documentation site

## Repairs

Both release blockers in `.factory/verification-7.md` were reproduced before
editing. The landing source contained no recording element or asset. The
claims manifest had no entries for the README event-stream promises or the
privacy-page requested-path promise.

1. The landing page now plays a 658-byte, self-hosted asciinema v2 recording
   captured from the packaged `infra-test-evidence --demo` flow. It visibly
   names the JUnit, reviewer HTML, and evidence JSON outputs. The player runs
   once, supports pause and replay, includes a complete text transcript, and
   displays the complete output immediately under reduced motion. The packaged
   demo test compares the recording transcript with a real installed-binary
   run after normalizing only its unique temporary path.
2. `.factory/claims.json` now contains 15 claims. New exact claims cover the
   recording, event-stream completion and status validation, per-run plan and
   assertion correlation, sensitive diagnostic redaction, and conversion-mode
   filesystem writes. Every claim has exactly one tagged test.
3. Packaged regressions reject missing, repeated, unsupported, and non-final
   summaries, unsupported completed-run status, and negative duration. They
   assert exit 2 and no JUnit or reviewer output. The parser now enforces the
   promised final-summary ordering.
4. Interleaved plans and diagnostics are converted through a freshly installed
   crate. The test proves each case contains only its own variables, outputs,
   resource changes, assertion path, and failure. Per-case plan summaries are
   now included in reviewer JSON and HTML.
5. A Linux preload guard rejects write-open and directory-creation calls
   outside the exact `--junit` and `--evidence-dir` allowlist. The packaged CLI
   completed normally, and the sandbox contained only the input, guard, JUnit,
   evidence JSON, and reviewer HTML. Privacy copy now distinguishes conversion
   mode from the documented temporary directory created by `--demo`.

## Clean local verification

The following passed on 2026-09-02 UTC:

```sh
npm ci
# Every command emitted by: jq -r '.[].test' .factory/claims.json
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

- `npm ci` installed 182 packages and found 0 vulnerabilities.
- All 15 exact claim commands passed individually from the clean install.
- `npm run check` passed ESLint, TypeScript, 6 Rust tests, and 20 Vitest
  tests. Rust formatting and warnings-as-errors Clippy passed.
- `cargo package --locked` verified 54 files at 319.4 KiB (91.5 KiB
  compressed). `npm pack --dry-run` verified 50 files at 67.5 kB (221.2 kB
  unpacked). Nothing was published.
- The consumer check returned `{"checks":2,"errors":[],"valid":true}`.
- The production build emitted 5.31 kB JS (2.21 kB gzip) and 10.61 kB CSS
  (3.16 kB gzip), below the 200 kB JS and 50 kB CSS budgets.
- Playwright passed 14/14 tests across desktop Chromium and the 390 px Chromium
  project. The dedicated Axe run passed 2/2. Coverage includes recording
  playback and transcript, reduced motion, demo/reset/exit, malformed-input
  recovery, keyboard focus, 200% text, touch targets, mobile overflow, policy
  routes, designed 404 behavior, browser privacy, and file-URL evidence.
- Local `verify-url.sh` reported HTTP 200 in 590 ms, no console errors, one
  `h1`, one `main`, `lang=en`, no missing alt text, and no unlabelled buttons.
- Local mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO; FCP/LCP were 1.0 s, TBT 0 ms, and CLS 0.

## Privacy, offline, and update behavior

Fresh browser contexts recorded only same-origin requests through the demo,
recording, and local-file flow. Cookies, localStorage, sessionStorage, and
IndexedDB remained empty. The site has no analytics, remote assets, account,
upload endpoint, backend, payment, or AI feature.

The generated reviewer page was opened from `file:` and made no HTTP requests.
The public site makes no offline/PWA claim and registers no service worker, so
there is no update lifecycle to test. The CLI package does not contact a
service or launch OpenTofu/Terraform; its existing preload claim test passed.

## Deployment and live evidence

The factory deployment script uploaded `dist/site/` to the existing
`sf-infra-test-evidence` Azure Static Web App. Deployment
`d5972674-3b4f-45ee-a64b-044058ec17d7` succeeded, and the custom domain was
Ready. No other resource was read or modified.

- Live `verify-url.sh` returned HTTP 200 in 641 ms with the expected title,
  language, landmarks, accessible controls, and no console errors.
- Live desktop and 390 px flows passed recording playback, first-click demo,
  reset, keyboard entry, 44 px targets, reduced motion, no overflow, no
  storage, same-origin-only requests, and light/dark Axe scans on all routes.
- `/`, `/demo/`, `/privacy/`, `/terms/`, and `/cli-demo.cast` return 200. An
  unknown route returns the designed page with HTTP 404.
- The root response includes CSP with `frame-ancestors 'none'`, HSTS,
  `nosniff`, `DENY`, a strict referrer policy, and a restrictive Permissions
  Policy. Hashed assets use `public, max-age=31536000, immutable`.
- Live mobile Lighthouse scored 100 performance, 100 accessibility, 100 best
  practices, and 100 SEO; FCP/LCP were 0.8 s, TBT 0 ms, CLS 0, and transfer was
  9,788 bytes.
- Local and live SHA-256 values match: root
  `1cd7b3d5418699d118359076a8441457d1e96ccfb99ed24bbb3f56454d955f80`;
  demo `6ddf5a59abf6eeab9c7d3855d81df5f39b28f75d09bc38e0ab56ed0406a34230`;
  privacy `e0cd3f6cccd7d2589bd4d6d8a430d98ef9ce780bd715d3ae6926388dffa52d06`;
  terms `6278c575f0b94e3a9de0fe0fd8f8a99dbf991a2da4580cb6a2994bdb6b8f8078`;
  404 `216ba6079b9fc332f6795e7cd1412398b4754a00e775355944fc9bc0539fbe5d`;
  recording `9f7d82e2f37a42cd7f2c6392c25d5be6cb5be07bbb7fd9d92f2bfa74b50571f5`;
  JS `8833f0b9614ee9c796e359d65decbc82e932d4236a8ad4b3a7e85e2043d07311`;
  CSS `25e118bc8f78f3e78f6a97f7f297354309a596333e246ff13496d10565ef15fc`.

## Known gaps

No `.factory/brief.json` exists in the supplied repository. Scope was
preserved from the verifier report, README, and `.factory/design.md`. No known
release blocker remains.
