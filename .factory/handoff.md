# Convert infrastructure tests into reviewable evidence — handoff

## Result

**PASS.** The phone first-screen defect from verification 13 is fixed in
implementation commit `b0d8ea1c6a58fa66bc082c539df0a908e50b986b` and is live
at `https://infra-test-evidence.sociobot.in`.

The implementation SHA differs from the documentation evidence commit
`4a7778ec1a5761cbd4657cebec0c92f671ee58a7`, which records this handoff and
QA 14 report.

At a fresh iPhone 13 browser viewport (`390 × 664`), the page now shows the
job, audience, first action, expected result, and all three product facts
before scrolling. Their final bottom coordinates are 247.41, 377.84, 456.13,
552.02, 589.63, 619.23, and 648.84 px respectively. The final fact has 15.16
px of room before the browser fold.

## What changed

- Reduced only the mobile landing heading scale and the gap before the facts.
  Desktop layout, copy, CLI behavior, demo data, and privacy behavior are
  unchanged.
- Added a browser regression that measures the visible result of the page at
  `390 × 664`: the job, audience, action, expected result, and each fact must
  all have bottom edges within the viewport.

## Job, audience, and first action

Before scrolling, the landing page says:

- Job: turn infrastructure tests into reviewable evidence.
- Audience: infrastructure-module maintainers reviewing failed OpenTofu or
  Terraform tests without uploading logs.
- First action: **Try it with sample data**. It opens a failed bundled test
  with redaction and the three generated output paths.

## Verification

From a fresh remote clone of implementation commit `b0d8ea1`, after `npm ci`:

- All 24 exact commands in `.factory/claims.json` passed separately.
- `npm test` and `npm run check` passed: 8 Rust tests and 28 frontend tests.
- `npm run build` produced `dist/site/`; initial JavaScript is 6.78 kB raw /
  2.70 kB gzip and CSS is 11.79 kB raw / 3.39 kB gzip.
- `npm run qa:browser` passed 28 tests and `npm run qa:a11y` passed 2 Axe
  projects. `npm run package:check`, `npm run consumer:check`, `cargo fmt
  --check`, strict Clippy, and `npm audit --audit-level=high` also passed.
- A separately installed packaged CLI passed `--demo`, normal conversion,
  invalid-duration rejection with exit 2, and a valid recovery run.
- Fresh live desktop and iPhone 13 contexts verified the demo banner, realistic
  failed evidence, reset, exit, empty browser storage, empty cookies,
  same-origin-only requests, keyboard skip link, reduced motion, Privacy,
  Terms, and the designed HTTP 404. The 404 console message is expected from
  the deliberate 404 response; normal routes had no browser errors.
- `/opt/fleet/lib/verify-url.sh` passed cold for `/` and `/demo/?demo=1` with
  title, language, one h1, main landmark, image-alt, and button-name checks.
- Live Axe found no serious or critical issues on root, demo, Privacy, or
  Terms in both contexts. Live mobile Lighthouse scored 100 for Performance,
  Accessibility, Best Practices, and SEO (LCP 829 ms, CLS 0, TBT 0 ms).

The static deployment helper reused the existing product Static Web App and
published the built `dist/site/`. The live root serves
`/assets/style-DQWrmAgu.css`, matching the verified build.

## Earlier findings

Verification 13's complete disposition table was reviewed before this change.
All earlier converter, redaction, validation, package, demo, privacy,
accessibility, routing, cache/security, copy, claim, and terminology findings
remain closed through the clean claim run, full gates, and cold live audit.
The only open item in that report, F-13-1, is closed by the new phone-fold
regression and live measurement above.

## Evidence and known gaps

Evidence is in `/work/.evidence/infra-test-evidence-repair-10/`; the catalog
description was copied to `/work/.evidence/catalog-description.txt`.

There are no current product defects known. The repository still lacks the
historical `.factory/brief.json`; the supplied researched brief remains the
acceptance source, as recorded by verification 13. The product has no backend,
accounts, payment offer, remote state access, offline promise, or service
worker, so backend persistence, billing, and offline-update checks do not
apply.
