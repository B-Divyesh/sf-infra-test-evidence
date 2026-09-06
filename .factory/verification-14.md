# Verify phone first-screen evidence — QA 14

**Work order:** `infra-test-evidence-repair-10`  
**Verdict:** **PASS**  
**Implementation candidate:** `b0d8ea1c6a58fa66bc082c539df0a908e50b986b`  
**Live URL:** https://infra-test-evidence.sociobot.in

## Finding closed

Verification 13 found that the click expectation and three product facts fell
below a fresh `390 × 664` iPhone 13 browser viewport. The mobile heading now
uses a proportionate 30 px size and the fact list has a 16 px preceding gap.
This keeps the landing decision complete without removing any required content.

The outcome-based Playwright regression visits `/` at `390 × 664` and checks
the bottom edge of the job, audience, primary action, expected result, and all
three facts. It does not inspect source strings or CSS rules.

## Cold live evidence

Fresh desktop and iPhone 13 contexts loaded the deployed site after the static
deployment. The phone bottom coordinates were:

| Content | Bottom (px) |
| --- | ---: |
| Job | 247.41 |
| Audience | 377.84 |
| First action | 456.13 |
| Expected result | 552.02 |
| Runs in your browser | 589.63 |
| No trackers or uploads | 619.23 |
| Free under the MIT License | 648.84 |

All content is within the 664 px viewport. The live page also passed the
one-click demo, populated failed evidence, persistent sample label, reset,
exit, no-storage/no-cookie check, same-origin request check, keyboard skip
link, reduced-motion check, legal routes, designed HTTP 404, and Axe scans.

## Clean verification

A fresh remote clone at `b0d8ea1`, with `npm ci`, passed all 24 declared claim
commands separately. It also passed `npm test`, `npm run check`, `npm run
build`, `npm run qa:browser` (28 tests), `npm run qa:a11y` (2 tests), package
and consumer checks, Rust formatting, strict Clippy, and npm audit.

The installed packaged CLI passed its demo, normal conversion, invalid input,
and recovery paths. Live Lighthouse scored 100 in all four audited categories.

## Earlier history

The full verification/review history and verification 13's earlier-finding
table were read before repair. The clean claim and live checks reconfirm the
previously closed safety, accessibility, routing, copy, and artifact findings.
F-13-1 was the only open finding and is now closed.
