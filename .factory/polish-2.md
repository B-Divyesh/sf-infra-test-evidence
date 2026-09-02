# Polish round 2 — review findings closed

**Repair commit:** `17462deb7815a31ca1a37e82b0b49399167b478c`  
**Production URL:** https://infra-test-evidence.sociobot.in  
**Deployment:** Azure Static Web Apps production deployment from `dist/site/` on 2026-09-02.  
**Cold-browser evidence:** `/tmp/infra-test-evidence-polish2-live/landing-390.png`,
`landing-1440.png`, `demo-390.png`, and `demo-1440.png`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | The demo opens a failed bundled OpenTofu conversion before the file picker, with redaction, assertion path, hash, and all output paths. | `@claim:site-demo`, `@claim:demo-artifact-fidelity`; live `demo-390.png`, `demo-1440.png`. |
| F-1-2 | The compact-record import contract and invalid-record feedback are retained. | `@claim:browser-record-import`. |
| F-1-3 | CLI help lists every accepted option. | `@claim:help-options`. |
| F-1-4 | The README accurately calls `report.xml` JUnit XML, and conversion verifies balanced XML and both sample cases. | `@claim:cli-conversion`. |
| F-1-5 | Both compact records and event streams have a parseable `--json` response. | `@claim:json-validation-output`. |
| F-1-6 | The unsupported response-policy claim remains removed from published copy. | copy regression in `tests/claims.test.ts`; live headers checked by `curl -I`. |
| F-1-7 | The landing action and all three product facts remain above the first viewport. | browser test `keeps the landing action and all three product facts…`; live `landing-390.png`, `landing-1440.png`. |
| F-1-8 | Route changes and Back focus the destination h1 and announce it. | browser test `moves focus and announces…`. |
| F-1-9 | The designed 404 keeps canonical, Open Graph, and Twitter metadata. | browser route test; live `/not-found` Axe check. |
| F-1-10 | The recording heading names the three output types. | copy regression; live landing check. |
| F-1-11 | The decorative eyebrow remains `Generated files`. | copy regression. |
| F-1-12 | The 404 names the error plainly as `PAGE NOT FOUND`. | route/copy regression. |
| F-1-13 | The final stray alternate output names were removed. The site now uses **JUnit report**, **evidence JSON**, and **reviewer page**. | copy regression rejects `reviewer JSON` and `static HTML page`; live landing check. |
| F-1-14 | Browser input terminology remains `compact record`. | copy regression and `@claim:browser-record-import`. |
| F-1-15 | Compact-record schema instructions stay split into short direct sentences. | README copy audit and regression. |
| F-1-16 | The README names the exact sensitivity markers and redaction action. | `@claim:sensitive-redaction`. |
| F-1-17 | Compact-record wording now identifies the visible JSON example. | copy regression; live landing check. |
| F-1-18 | The vague response-policy statement remains absent. | copy regression and live headers. |
| F-1-19 | Event-stream fail-closed behavior remains described in short direct sentences. | `@claim:event-stream-validation`. |
| F-1-20 | Sensitive diagnostics are explained without metaphor. | `@claim:sensitive-diagnostics`. |
| F-2-1 | Rewrote the How it works step to “Create a JUnit report, evidence JSON, and a reviewer page.” | copy regression rejects both former names; cold live landing check. |
| F-2-2 | The strict-validation claim now executes malformed JSON, incomplete compact JSON, a rejected output path, valid input, and incorrect usage. | `@claim:strict-validation` from clean clone. |
| F-2-3 | The README names each field-name pattern, and a new claim supplies harmless sentinels under every pattern and scans all outputs. | `@claim:named-field-redaction` from clean clone. |
| F-2-4 | The demo claim now asserts all three exact output paths and their bottom edges at 390×844 and 1440×900. | `@claim:site-demo`; live `demo-390.png`, `demo-1440.png`. |
| F-2-5 | Rewrote the vague landing line to “The local reader also opens the compact JSON record shown below.” | copy regression; cold live landing check. |
| F-2-6 | Rewrote the reviewer-page description as two direct sentences naming inputs, assertion path, failure, redacted plan changes, and SHA-256. | README copy regression and clean-clone checks. |

## Verification

From a `git clone --no-local` of `17462deb7815a31ca1a37e82b0b49399167b478c`,
`npm ci` completed with zero audit vulnerabilities. All 24 commands listed in
`.factory/claims.json` passed separately. The same clean clone passed `npm run
check` (8 Rust and 28 frontend tests), `npm run build`, `npm run qa:browser`
(22 browser tests), `npm run qa:a11y` (2 Axe tests), and `npm run
package:check`.

After deployment, cold Chromium checks at 390×844 and 1440×900 found no console
errors and verified the revised landing copy plus all three demo output paths
above the fold. Live Axe checks reported zero violations on `/`, `/demo/?demo=1`,
`/privacy/`, `/terms/`, and `/not-found`.
