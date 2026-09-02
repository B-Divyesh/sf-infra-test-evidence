# Independent verification 12 — PASS

**Work order:** `infra-test-evidence-verify-12`  
**Candidate commit:** `89900c20c4f1ac409ff22e9f4a844612c7b0aa31`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Verified:** 2026-09-02 UTC

## Decision

**PASS.** This candidate delivers the brief's smallest useful product: a
local-first CLI that turns supplied OpenTofu/Terraform test JSON into JUnit,
redacted evidence JSON, and a disk-readable reviewer page. It does not run
infrastructure tools, access state, upload artifacts, or add a hosted test
dashboard. The live static deployment is byte-for-byte the locally built
candidate for the root document, hashed JavaScript/CSS, CLI recording, and
demo artifact.

The repository has no `.factory/brief.json`; the researched brief supplied in
this work order was the acceptance contract.

## Mandatory first checks

After `npm ci` in this clean checkout, I ran each of the 24 exact commands in
`.factory/claims.json` separately. Every command exited 0. The passing claim
IDs were:

`cli-demo`, `cli-recording`, `cli-conversion`, `sensitive-redaction`,
`named-field-redaction`, `resource-identifier-redaction`,
`cross-provider-resource-redaction`, `sensitivity-fail-closed`,
`strict-validation`, `malformed-duration-types`, `event-stream-validation`,
`summary-consistency`, `run-correlation`, `sensitive-diagnostics`,
`conversion-only`, `requested-path-writes`, `artifact-private`,
`reader-private`, `site-demo`, `demo-artifact-fidelity`,
`browser-record-import`, `help-options`, `json-validation-output`, and
`mit-license`.

Thus the claims independently exercised the packaged-CLI demo and conversion;
normal and invalid compact/event inputs; error exit codes and recovery;
fail-closed sensitive metadata; explicit, named-field, AWS, Azure, and GCP
identifier redaction; interleaved-run correlation; no child process/network
effects; requested-path writes only; the local reviewer artifact; browser
privacy; recording; and the one-click demo sandbox.

Cold first read of the live landing page passed. It says it will “Turn
infrastructure tests into reviewable evidence,” names infrastructure-module
maintainers reviewing failed OpenTofu/Terraform tests, and exposes **Try it
with sample data** beside “See a failed test, redaction, and output files.”
One click opens `/demo/?demo=1`; its persistent “Demo — sample data, nothing is
saved” banner has **Reset demo** and **Start for real**. The demo immediately
shows a failed assertion, `[REDACTED]`, provenance, and the JUnit, JSON, and
reviewer-page output paths.

## Local build, test, and consumer evidence

All commands below passed on this candidate:

```text
npm ci                         182 packages, 0 vulnerabilities
npm run check                  ESLint, TypeScript, 8 Rust tests, 28 Vitest tests
npm run build                  dist/site/ produced
npm run package:check          cargo package verification + npm pack --dry-run
npm run consumer:check         valid compact JSON: {"checks":2,"errors":[],"valid":true}
npm run qa:browser             22 desktop/mobile Playwright checks
npm run qa:a11y                2 desktop/mobile Axe checks
```

The production build has 6.78 kB raw / 2.69 kB gzip main JS, 1.34 kB raw /
0.68 kB gzip routes JS, and 11.74 kB raw / 3.38 kB gzip CSS: well below the
200 kB JS and 50 kB CSS budgets.

I also installed the verified `cargo package` source into a new temporary
consumer root. Its `--demo` produced a new temporary directory containing the
sample input, `report.xml`, `evidence/evidence.json`, and
`evidence/index.html`. A normal conversion of `examples/tofu-test.jsonl`
reported two checks and created the same three outputs. The boundary input
`verification-9-duration-string.json` returned machine-readable validation
output and documented exit code 2:

```json
{"checks":0,"errors":["check 1 durationMs must be a non-negative finite number"],"valid":false}
```

## Live deployment, privacy, and accessibility

- Live root HTML SHA-256 is `6bf3caa…c974528e`, equal to `dist/site/index.html`.
  SHA-256 also matched locally and live for `main-C2KNtsux.js`,
  `routes-j3C_yCWu.js`, `style-CgdVxA8l.css`, `cli-demo.cast`, and
  `demo-evidence.json`.
- Fresh 1440x900 and 390x844 live browser flows had no console errors or page
  errors. The demo request log contained only
  `https://infra-test-evidence.sociobot.in` resources; it had no cookies,
  localStorage, or sessionStorage. The reader therefore meets its no-tracker,
  no-upload, local-only promise.
- Live Axe scans on the demo (both viewports) found zero serious or critical
  findings. The full local Axe suite also passed root, demo, privacy, terms,
  and 404 in both light and dark treatments. On both live viewports, Tab
  focused the skip link with a designed `rgb(7, 90, 158)` 3px solid outline;
  there was no horizontal overflow and no visible link/button smaller than
  44px. Reduced motion changes the drop-zone transition to `0.00001s` and
  presents the recording without animation.
- Root, `/demo/`, `/privacy/`, `/terms/`, `/robots.txt`, and `/sitemap.xml`
  returned 200. An unknown route returned the styled 404 with HTTP 404. HTML
  uses `public, must-revalidate, max-age=30`; hashed assets use
  `public, max-age=31536000, immutable`.
- Live headers include HSTS, same-origin CSP with `frame-ancestors 'none'`,
  `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, strict referrer
  policy, and a restrictive permissions policy.

This is a static CLI/documentation product with no server-side application
endpoint, account/sign-in, payment/unlock, persistence layer, service worker,
or API route. Rate-limit/concurrency/persistence/Entra and PWA update checks
are therefore not applicable; there is no documented request allowance to
exercise.

## Defects

No P0, P1, P2, or P3 defects found.

