# Infra Test Evidence polish round 1 handoff — PASS

**Work order:** `infra-test-evidence-polish-1`  
**Repair commit:** `98308867579a4a7a554e5a2ff7ff4a874838360c` (pushed to `main`)  
**Deployment:** `0eb1babf-6257-499a-bf4f-735867e5eec5`  
**Live URL:** https://infra-test-evidence.sociobot.in

## Done

- Closed all 20 findings in `.factory/review-1.md`; the detailed finding map
  is in `.factory/polish-1.md`.
- Made `/?demo=1` redirect to the isolated demo. Its persistent banner offers
  reset and real-mode exit, and its first viewport shows representative failed
  conversion evidence, a redaction, source provenance, and output files.
- Added the three missing claim contracts and exact tests for compact-record
  import, help coverage, and two-input JSON validation.
- Corrected first-screen copy, output terminology, 404 metadata/copy, desktop
  fact visibility, and forward/back focus announcements.
- Deployed `dist/site/` through the scoped Static Web App configuration.

## Verification

From a fresh clone of the pushed repair commit, after `npm ci`, every command
in `.factory/claims.json` passed individually. The same clean clone then passed
`npm test`, `npm run check`, `npm run build`, `npm run qa:browser`, `npm run
qa:a11y`, and `npm run package:check`.

The final local browser suite passed 20 tests. Build output is 5.56 KB gzip JS
and 3.37 KB gzip CSS. The live fleet verifier passed cold for `/` and
`/?demo=1` with no console errors; both pages have title, `lang`, one h1,
main, no missing alt text, and no unlabelled buttons. Live Axe scans for `/`,
`/demo/`, `/privacy/`, `/terms/`, and `/404.html` have no serious or critical
issues. The live 390 × 844 and 1440 × 900 demo checks keep the failed check,
redaction, and output-file name above the fold.

Evidence lives in `/tmp/infra-test-evidence-live-9830886/` in this worker:
fleet verifier reports and desktop/mobile screenshots for root and demo, plus
focused `demo-390.png` and `demo-1440.png` first-viewport captures.

## Run and deploy

```sh
npm ci
npm test
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
/opt/fleet/lib/deploy-static.sh infra-test-evidence dist/site
```

No known gaps remain.
