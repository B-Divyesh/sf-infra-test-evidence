# Convert infrastructure tests into reviewable evidence — handoff

## Result

**PASS.** Independent QA 14 found zero defects and zero untested claims.

- Implementation candidate:
  `b0d8ea1c6a58fa66bc082c539df0a908e50b986b`
- Documentation evidence:
  `4a7778ec1a5761cbd4657cebec0c92f671ee58a7`, cited by
  `98d80c921b07e4cdf99d88df7faeb405d4929303`
- Live URL: https://infra-test-evidence.sociobot.in
- Full report: `.factory/verification-14.md`

The implementation and live deployment were not changed during verification.
The later commits are documentation-only.

## Job, audience, and first action

Before scrolling, fresh desktop and iPhone 13 browsers show:

- Job: turn infrastructure tests into reviewable evidence.
- Audience: infrastructure-module maintainers reviewing failed OpenTofu or
  Terraform tests without uploading logs.
- First action: **Try it with sample data**.
- Expected result: a failed sample test, redaction, and output files.

At `390 × 664`, the last required fact ends at 648.84 px. All required
first-screen content fits without scrolling.

## What was verified

- All 24 exact claim commands passed separately from the fresh candidate
  checkout after `npm ci`. Every claim has exactly one tagged source test.
- `npm test`, `npm run check`, `npm run build`, `npm run qa:browser`, `npm run
  qa:a11y`, package and consumer checks, Rust formatting, strict Clippy, and
  npm audit passed. The suites contain 8 Rust tests, 28 frontend tests, 28
  browser tests, and two accessibility projects.
- A separately installed packaged CLI passed help, bundled demo, normal
  conversion, invalid input with exit 2, and immediate valid recovery.
- Fresh live desktop and phone contexts passed the one-click populated sample,
  persistent sample label, keyboard reset, exit to an empty real reader,
  normal/invalid/boundary/recovery input, keyboard and route focus, reduced
  motion, 200% text, legal routes, and the designed HTTP 404.
- Live browser traffic was same-origin only. Cookies, localStorage,
  sessionStorage, IndexedDB, and service workers stayed empty. Sample actions
  did not read or change real data.
- Live Axe found no serious or critical issues across root, sample, Privacy,
  Terms, and 404 on desktop and phone in light and dark modes.
- The standard URL verifier passed root and sample with no console errors.
- Mobile Lighthouse scored 100 Performance, 100 Accessibility, 100 Best
  Practices, and 100 SEO. LCP was 1.54 s, CLS 0, and TBT 0 ms.
- Built and live root, sample, policy, 404, JS/CSS, recording, sample artifact,
  and social-card hashes match. The live runtime is the reviewed candidate.
- Every earlier verification and review finding, including minor copy,
  terminology, accessibility, safety, and phone-fold issues, is closed.

## Build size

`npm run build` creates `dist/site/`.

- Main JavaScript: 6.78 kB raw / 2.70 kB gzip
- Route JavaScript: 1.34 kB raw / 0.68 kB gzip
- CSS: 11.79 kB raw / 3.39 kB gzip

## Run and verify

```sh
npm ci
npm test
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
npm run consumer:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
```

Install the packaged CLI from a checkout with:

```sh
cargo install --path . --locked
infra-test-evidence --demo
```

## Known gaps and next steps

There are no known product defects. The repository lacks the historical
`.factory/brief.json`; the researched brief supplied in the work order remains
the acceptance source.

The product is a static site and local CLI. It has no backend, account,
payment, offline promise, remote state access, or stored product state, so
backend persistence, tenant, rate-limit, billing, and offline-update checks do
not apply. No deployment work is needed for this documentation-only handoff.
