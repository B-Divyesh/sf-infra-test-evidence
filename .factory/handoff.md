# Infra Test Evidence polish 2 handoff

- **Work order:** `infra-test-evidence-polish-2`
- **Repair commits:** `17462deb7815a31ca1a37e82b0b49399167b478c`, `f6528ab943a7ef26a5f318903d1fb3c860ef5f19`
- **Live URL:** https://infra-test-evidence.sociobot.in
- **Result:** PASS

This repair closes F-2-1 through F-2-6 and re-verifies every earlier finding.
The landing uses one output set—**JUnit report**, **evidence JSON**, and
**reviewer page**. Strict validation now proves malformed JSON, incomplete
records, output failures, valid input, and usage errors. Named identifier-field
redaction has its own packaged-CLI claim. The demo claim proves each exact
output path above the fold at 390×844 and 1440×900. The two vague copy lines
were rewritten in direct language. See `.factory/polish-2.md` for the complete
finding-to-evidence map.

## Verification

From a `git clone --no-local` of the repair commit, `npm ci` completed with
zero audit vulnerabilities. Every one of the 24 exact commands in
`.factory/claims.json` passed separately. That clean clone also passed:

- `npm run check` — ESLint, TypeScript, 8 Rust tests, and 28 frontend tests.
- `npm run build` — `dist/site/` produced; 6.78 kB raw / 2.69 kB gzip initial
  main JavaScript and 11.74 kB raw / 3.38 kB gzip CSS.
- `npm run qa:browser` — 22 browser tests.
- `npm run qa:a11y` — 2 Axe tests.
- `npm run package:check` — `cargo package --locked` and `npm pack --dry-run`.

Production was deployed with the Azure Static Web Apps CLI from `dist/site/`.
Cold live Chromium checks at 390×844 and 1440×900 confirmed the revised copy,
the isolated `?demo=1` flow, all three output paths above the fold, and no
console errors. Live Axe scans returned zero violations on `/`, `/demo/?demo=1`,
`/privacy/`, `/terms/`, and `/not-found`. Live responses include the configured
CSP, `Referrer-Policy`, `X-Content-Type-Options`, and `Permissions-Policy`.

## Run and deploy

Run `npm ci && npm run check && npm run build && npm run qa:browser && npm run
qa:a11y && npm run package:check`. Deploy `dist/site/` to the scoped Static Web
App `sf-infra-test-evidence`.

## Known gaps

None.
