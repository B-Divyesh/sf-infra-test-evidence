# Polish round 3 — zero-finding repair

**Release repair commit:** `e45ce587daaeb212e7c4299c412fd68f9a578bcc`  
**Production deployment:** `2decafa3-46b9-4278-8299-743e373cfb2a`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Clean clone:** `/tmp/infra-test-evidence-polish3-clean-RxQCei/repo` (`git clone --no-local`, then `npm ci`)  
**Cold live screenshots:** `/tmp/infra-test-evidence-polish3-live-browser-S0rSti/landing-390.png`, `landing-1440.png`, `demo-390.png`, and `demo-1440.png`.

## Verification summary

- All 24 exact commands declared by `.factory/claims.json` passed independently from the clean clone.
- The clean clone passed `npm run check` (8 Rust and 28 frontend tests), `npm run build`, `npm run qa:browser` (26 tests), `npm run qa:a11y` (2 Axe projects), and `npm run package:check`.
- The built initial JavaScript is 6.78 kB raw / 2.70 kB gzip. There is no offline claim; the required private-reader claim tests the supported local-only flow instead.
- `/opt/fleet/lib/verify-url.sh` passed cold on `/` in 815 ms and on `/demo/?demo=1` in 595 ms. Both reports show a title, `lang=en`, one `h1`, one `main`, complete image alt coverage, named buttons, and no console errors. Evidence: `/tmp/infra-test-evidence-polish3-live-root-TnqRC6/` and `/tmp/infra-test-evidence-polish3-live-demo-AGD7Dw/`.
- The final cold browser check recorded only same-origin requests, no cookies, localStorage, sessionStorage, or IndexedDB; reset and exit both worked. It found no serious or critical Axe results on `/`, `/demo/?demo=1`, `/privacy/`, `/terms/`, or `/not-found`. Its structured evidence is `/tmp/infra-test-evidence-polish3-live-browser-S0rSti/live-check.json`.

## Finding map

| Finding | Change made or rechecked | Evidence |
| --- | --- | --- |
| F-1-1 | The isolated demo still opens the failed bundled OpenTofu conversion above the picker, including its assertion path, `[REDACTED]`, input SHA-256, and all three output paths. | `@claim:site-demo`, `@claim:demo-artifact-fidelity`; live `/demo/?demo=1`; `demo-390.png`, `demo-1440.png`. |
| F-1-2 | The compact-record import contract remains named and tests rendering plus validation feedback. | `@claim:browser-record-import`; live `/`; clean-clone claim run. |
| F-1-3 | Help-option coverage remains named and executed from a packaged CLI. | `@claim:help-options`; clean-clone claim run. |
| F-1-4 | README keeps the narrower JUnit XML statement and conversion parses the output cases. | `@claim:cli-conversion`; clean-clone claim run. |
| F-1-5 | Both supported input forms retain their named machine-readable JSON contract. | `@claim:json-validation-output`; clean-clone claim run. |
| F-1-6 | The unsupported deployment-policy promise remains absent from published copy. | `tests/claims.test.ts` copy regression; live root CSP, Referrer-Policy, and `nosniff` headers. |
| F-1-7 | All three product facts remain in both cold first viewports. | `keeps the landing action and all three product facts…`; live `/`; `landing-390.png`, `landing-1440.png`. |
| F-1-8 | Internal navigation and Back still focus the destination h1 and announce the route. | `moves focus and announces…`; live `/` → demo → Back check. |
| F-1-9 | The designed 404 retains canonical, Open Graph, and Twitter metadata. | `publishes distinct demo, policy, discovery, and error documents`; live `/404.html` metadata check. |
| F-1-10 | The recording heading retains all three canonical output names. | copy regression; live `/`; `landing-390.png`. |
| F-1-11 | The utility label remains `Generated files`, not decorative copy. | copy regression; live `/`; `landing-390.png`. |
| F-1-12 | The 404 names the error plainly as `404 · PAGE NOT FOUND`. | route/copy regression; live `/404.html`. |
| F-1-13 | Replaced the runtime recording status with “The JUnit report, evidence JSON, and reviewer page paths are shown.” The copy regression now reads `src/main.ts`, and the browser test waits for that exact live status. | `keeps reviewed copy…`, `@claim:cli-recording`; live `/` status check returned the exact sentence. |
| F-1-14 | Standardized all reader actions, demo chooser, privacy text, README, and runtime error to **compact record**. The regression checks HTML, TypeScript, README, demo docs, claims, recording, and privacy copy. | `keeps the compact-record name in every browser input state`; live `/` and `/demo/?demo=1`; `landing-390.png`, `demo-390.png`. |
| F-1-15 | Compact-record schema instructions stay split into short direct sentences. | copy regression and `.factory/copy-audit.md`; clean-clone `npm run check`. |
| F-1-16 | README continues to list exact sensitivity markers and the redaction action. | `@claim:sensitive-redaction`; clean-clone claim run. |
| F-1-17 | README names the browser reader’s compact record directly. | copy regression; README at release commit `e45ce58`. |
| F-1-18 | The vague response-policy sentence remains removed. | copy regression; live root header check. |
| F-1-19 | Event-stream safety wording remains short and the failure modes stay tested. | `@claim:event-stream-validation`; clean-clone claim run. |
| F-1-20 | Sensitive diagnostics remain described without metaphor and scanned across generated output. | `@claim:sensitive-diagnostics`; clean-clone claim run. |
| F-2-1 | The landing, recording status, transcript, README, claims, and generated page use the canonical set: JUnit report, evidence JSON, reviewer page. | `keeps reviewed copy…`, `@claim:cli-recording`; live `/` recording-status check. |
| F-2-2 | Strict validation continues to prove malformed JSON, incomplete compact record, rejected output path, valid input, and usage exit codes. | `@claim:strict-validation`; clean-clone claim run. |
| F-2-3 | Every documented named-field pattern remains covered with harmless sentinels across all three output files. | `@claim:named-field-redaction`; clean-clone claim run. |
| F-2-4 | The demo claim checks every output path and its bottom edge in 390 × 844 and 1440 × 900 viewports. | `@claim:site-demo`; live `demo-390.png`, `demo-1440.png`. |
| F-2-5 | The local reader now calls the input only a compact record. | `keeps reviewed copy…`; live `/`; `landing-390.png`. |
| F-2-6 | README continues to name inputs, assertion path, failures, redacted plan changes, and input SHA-256 in direct sentences. | copy regression; README at release commit `e45ce58`. |
| F-3-1 | Replaced landing-page “provenance” with the concrete **input SHA-256** and made the demo proof and generated page use plain labels too. | copy regression, `@claim:demo-artifact-fidelity`; live `/demo/?demo=1`; `demo-390.png`. |
| F-3-2 | Replaced README “assertion traversals” with **assertion paths** and reject the retired term in the copy regression. | `keeps reviewed copy…`; clean-clone `npm run check`. |

No blocking or minor finding remains.
