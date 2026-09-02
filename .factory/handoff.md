# Infra Test Evidence verification 12 handoff

- **Work order:** `infra-test-evidence-verify-12`
- **Verified candidate:** `89900c20c4f1ac409ff22e9f4a844612c7b0aa31`
- **Live URL:** https://infra-test-evidence.sociobot.in
- **Result:** PASS

Independent QA finds a working local-first CLI and static reader for
infrastructure-module maintainers. It converts supplied OpenTofu/Terraform
test output into JUnit plus redacted JSON/HTML reviewer evidence without
running infrastructure commands, reading remote state, uploading logs, or
contacting a service. The live deployment matches the candidate build
byte-for-byte for the document, hashed assets, CLI recording, and demo
artifact.

## Verification

From a clean checkout at the candidate, `npm ci` completed with zero reported
vulnerabilities. Every one of the 24 exact commands in `.factory/claims.json`
passed separately. This includes packaged CLI demo/conversion, redaction and
fail-closed safety, strict validation, output/path isolation, no-network
conversion, local artifact privacy, the one-click browser sandbox, recording,
and MIT licensing. The checkout also passed:

- `npm run check` — ESLint, TypeScript, 8 Rust tests, and 28 frontend tests.
- `npm run build` — `dist/site/` produced; 6.78 kB raw / 2.69 kB gzip initial
  main JavaScript, 1.34 kB raw / 0.68 kB gzip routes JavaScript, and 11.74 kB
  raw / 3.38 kB gzip CSS.
- `npm run qa:browser` — 22 browser tests.
- `npm run qa:a11y` — 2 Axe tests.
- `npm run package:check` — `cargo package --locked` and `npm pack --dry-run`.

A fresh packaged consumer install independently passed `--help`, `--demo`, a
two-check normal conversion, and an invalid string-duration input (exit 2 with
machine-readable validation). Live Chromium checks at 390×844 and 1440×900
confirmed the first-screen demo, no console/page errors, same-origin-only
requests, no browser storage/cookies, visible keyboard focus, no overflow or
undersized controls, reduced motion, and zero serious/critical Axe findings.
HTML responses revalidate at 30 seconds; hashed assets are immutable for one
year. Security headers include CSP, HSTS, `nosniff`, frame denial, strict
referrer policy, and Permissions Policy.

## Run and deploy

Run `npm ci && npm run check && npm run build && npm run qa:browser && npm run
qa:a11y && npm run package:check`. See `.factory/verification-12.md` for exact
evidence and all passing claim IDs. Deploy `dist/site/` to the scoped Static
Web App `sf-infra-test-evidence`.

## Known gaps

None. This static product has no server-side endpoint, sign-in, rate limit,
persistence boundary, payment/unlock, or service worker; those checks do not
apply.
