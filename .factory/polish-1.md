# Polish round 1 — all review findings closed

**Repair commit:** `98308867579a4a7a554e5a2ff7ff4a874838360c`  
**Deployment:** `0eb1babf-6257-499a-bf4f-735867e5eec5`  
**Live check:** `https://infra-test-evidence.sociobot.in/?demo=1`

Local screenshots and verifier output are under
`/tmp/infra-test-evidence-live-9830886/`. `demo-390.png` and `demo-1440.png`
are the first-viewport evidence checks; `root/` and `demo/` contain the fleet
URL verifier screenshots and JSON.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Rebuilt the demo around the bundled failed OpenTofu run. It shows `blocks_public_ingress`, its assertion path, `[REDACTED]`, SHA-256 provenance, and all three output files before the file picker. | `@claim:site-demo`; `demo-390.png`, `demo-1440.png`; live check reports final coordinates 725/775/825 and 745/796/846. |
| F-1-2 | Added the compact-record import claim and regression flow, including validation errors. | `@claim:browser-record-import`; live root reader. |
| F-1-3 | Added a help-options claim that checks every accepted option and exit 0. | `@claim:help-options`. |
| F-1-4 | Narrowed the README wording to JUnit XML and assert balanced XML plus both converted cases. | `@claim:cli-conversion`. |
| F-1-5 | Added a two-input JSON-validation claim. | `@claim:json-validation-output`. |
| F-1-6 | Removed the untested deployment-policy sentence. Live headers were separately inspected. | README copy regression; live `curl -I` shows CSP, Referrer-Policy, nosniff, and immutable hashed-asset caching. |
| F-1-7 | Reduced hero vertical scale and added a 1440 × 900 first-viewport fact check. | `keeps the landing action and all three product facts…`; `root/screenshot-desktop.png`. |
| F-1-8 | Added page-route focus and polite announcement handling for internal navigation and Back. | `moves focus and announces…`; live forward/back check `{forward:true,back:true}`. |
| F-1-9 | Added canonical, Open Graph, and Twitter metadata to the designed 404. | `publishes distinct demo…`; live `/404.html` metadata check. |
| F-1-10 | Renamed the recording heading to name JUnit, JSON, and HTML. | copy regression test; landing screenshot. |
| F-1-11 | Replaced the decorative eyebrow with `Generated files`. | copy regression test. |
| F-1-12 | Replaced the ledger lore with `404 · PAGE NOT FOUND`. | route test and live 404 response. |
| F-1-13 | Defined one output set: JUnit report, evidence JSON, reviewer page. | copy regression test; `@claim:cli-conversion`. |
| F-1-14 | Standardized browser input wording to `compact record`. | copy regression test; `@claim:browser-record-import`. |
| F-1-15 | Split the long schema sentence into two direct instructions. | copy regression test and `.factory/copy-audit.md`. |
| F-1-16 | Replaced “authoritative” with the actual marker-redaction rule. | copy regression test; `@claim:sensitive-redaction`. |
| F-1-17 | Replaced vague portable-workflow wording with the browser-reader use. | copy regression test. |
| F-1-18 | Removed unexplained restrictive-policy marketing. | copy regression test; live header inspection. |
| F-1-19 | Split the fail-closed sentence into final-summary and status/duration rules. | copy regression test; `@claim:event-stream-validation`. |
| F-1-20 | Explained whole-diagnostic redaction without metaphor. | copy regression test; `@claim:sensitive-diagnostics`. |

## Verification

From a fresh clone of the pushed repair commit, `npm ci` ran first. Every
command named by `.factory/claims.json` passed separately, followed by `npm
test`, `npm run check`, `npm run build`, `npm run qa:browser`, `npm run qa:a11y`,
and `npm run package:check`. The final local browser suite passed 20 tests;
the Axe suite passed both desktop and mobile projects.

Cold live verification passed after deployment: both fleet URL-verifier runs
reported title, language, one h1, main landmark, no missing alt text, no
unlabelled button, and no console errors. Live Axe scans across `/`, `/demo/`,
`/privacy/`, `/terms/`, and `/404.html` found no serious or critical issues.
