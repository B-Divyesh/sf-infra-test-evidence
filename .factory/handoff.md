# Infra Test Evidence repair handoff — PASS

**Repair commit:** `65ca6166466933dafd0a5dc6b9eb25f2c7e6bb82`
**Base/verifier report:** `b7caac45b0c52b86c0682c8ab3a6fbea8bba7ced`,
`.factory/verification-4.md`
**Static deployment:** https://infra-test-evidence.sociobot.in

## Repair completed

The verifier's exact three-event stream was reproduced before the repair. The
release CLI returned valid evidence and leaked `k9M2qV7xL4` into both
`evidence.json` and `index.html`.

`src-rust/main.rs` now treats OpenTofu/Terraform sensitivity metadata as
authoritative before building any reporter data:

- An inline `{"sensitive":true,"value":...}` redacts the complete marked
  subtree, including opaque values with no secret-like spelling.
- Terraform/OpenTofu structural `before_sensitive`, `after_sensitive`, and
  `sensitive_values` boolean masks are applied recursively to the matching
  `before`, `after`, and `values` trees.
- An invalid, incompatible, or ambiguous marker/mask fails closed with exit 2;
  no JUnit, JSON, or HTML reviewer artifact is written.
- Test-plan inputs and summaries, diagnostics, assertion paths, JUnit data,
  evidence JSON, and embedded HTML all derive only from redacted report data.

The regression fixture is `examples/explicit-sensitive-output.jsonl`. Rust
coverage checks inline values, nested mask values, and malformed markers.
`tests/cli.test.ts` packages the crate, installs the staged release binary in a
fresh consumer root, creates JUnit plus reviewer JSON/HTML, and scans every
generated file for the opaque sentinel. It also checks the redaction marker is
present. `vite.config.ts` excludes Cargo's staged `target/` package from
Vitest discovery so the package test cannot duplicate test execution.

## Verification

All commands ran successfully from this checkout after `npm ci` (0 audit
vulnerabilities):

```sh
npm run check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
npm run package:check
npm run consumer:check
npm run build
npm run qa:browser
npm run qa:a11y
```

Results:

- `npm run check`: ESLint, TypeScript, 6 Rust tests, and 8 Vitest tests pass.
  The Vitest suite includes the installed-release opaque-sentinel scan.
- `cargo package --locked`: verifies a 39-file, 59.2 KiB compressed crate;
  `npm pack --dry-run` passes. `consumer:check` returns
  `{"checks":2,"errors":[],"valid":true}`.
- Production build: JS 3.66 kB raw / 1.63 kB gzip; CSS 5.48 kB raw / 1.99 kB
  gzip. Both are within the product budgets.
- Playwright: 8/8 desktop and 390 px mobile tests pass, including keyboard
  focus and the self-contained generated reviewer artifact. Axe: 2/2 passes
  with no serious or critical violations.
- Live `verify-url.sh`: HTTP 200 in 620 ms; title, `lang=en`, one h1, main
  landmark, alt text, labelled controls, and no browser console/page errors.
- Live response policy: CSP with `frame-ancestors 'none'`, X-Frame-Options
  DENY, Permissions-Policy, nosniff, strict referrer policy, and HSTS are
  present. Hashed JS is `public, max-age=31536000, immutable`.
- Live SHA-256 matches the local production output: root
  `12a19a2e62ece91b03de7e2e835a886d91f54950ddcf6321e2c4fa5c40935c16`, JS
  `7a47f092a1ac4b54e50166dd15e50d532f065d448cffafaa06fdc4d458cdca9f`, CSS
  `60fe55f2c44949eba033fd380b71b93bb3a6fa136853d55e83a9ed7f4ba0ab73`,
  Privacy `e3b9b299057ac932deeeb708c718563f91ab4a73f5833256aeda99318a80632a`,
  and Terms `8a6dc8f53e5469f8f4d9320aef5ba490a8ff84c7ab9f36e66716a397100b770f`.
  `/privacy/` and `/terms/` both return 200.
- Live Lighthouse mobile: performance 100, accessibility 100, best practices
  100, SEO 100; FCP 1.0 s, LCP 1.0 s, TBT 20 ms, CLS 0.

The static site has no analytics, remote fonts, CDN scripts, upload endpoint,
or browser persistence. The landing reader intentionally has no service worker
or update channel, so offline-reload/update checks do not apply. Generated
reviewer artifacts are self-contained static files and are exercised directly
from disk in the browser suite.

## Deploy and remaining work

The repair commit was pushed to `origin/main`, the configured static deployment
branch. The landing build is unchanged by this CLI-only repair and matches the
live deployment byte-for-byte. No registry publication was attempted; the
factory owns package publishing credentials. The release package is verified
and ready for the factory to publish with `cargo package --locked`.

No known product gaps remain from verification 4.
