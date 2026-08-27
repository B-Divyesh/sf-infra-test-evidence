# Infra Test Evidence handoff

## Delivered

- Replaced the failed scaffold with the documented `0.1.0` Rust CLI and a
  local-first Vite evidence reader. The CLI accepts an evidence JSON path,
  offers useful `--help`, supports `--json` for CI, and has documented exit
  codes. The viewer reads selected/dropped JSON only in the browser; it has no
  analytics, remote assets, accounts, or uploads.
- Added reproducible `package-lock.json` and `Cargo.lock`, complete npm build,
  lint, type, test, package, consumer, and Playwright scripts, plus a sample
  evidence record.
- Added `/privacy/` and `/terms/`, MIT documentation, a product-specific
  ledger/audit visual system in `.factory/design.md`, keyboard/focus support,
  dark mode, reduced-motion handling, and desktop/mobile axe coverage.
- The static deployment target is `dist/site/`; `dist/site/index.html` is
  emitted by the exact `npm run build:site` command.

## Verification (2026-08-27 UTC)

Executed successfully from the repository root:

```sh
npm ci
npm run check
npm run build:site
test -f dist/site/index.html
npm run qa:browser
npm audit --audit-level=high
npm run package:check
npm run consumer:check
```

Results:

- `npm run check`: ESLint, TypeScript, 2 Vitest tests, and 2 Rust unit tests
  passed.
- `npm run build:site`: emitted `dist/site/index.html` (2.78 kB), CSS (5.37
  kB), and JavaScript (3.44 kB; 1.55 kB gzip), comfortably under the 200 kB
  initial-JS budget.
- `npm run qa:browser`: 4/4 Playwright checks passed in Chromium — desktop and
  iPhone-13 viewport sample-report flows had no console errors, and axe found
  no serious or critical violations in either viewport.
- `npm audit --audit-level=high`: 0 vulnerabilities.
- `npm run package:check`: `cargo package --locked` packaged and verified 26
  crate files (134.8 KiB unpacked) and `npm pack --dry-run` verified the
  JavaScript package manifest.
- `npm run consumer:check`: the documented CLI invocation returned
  `{"valid":true,"checks":2,"errors":[]}` for the shipped sample.

`cargo package --locked` is the ready-to-publish package check; do not publish
from this repository.

## Deployment

Commit `3022233a5d717cc5829063bf13d530bc78b43f7e` was pushed to `origin/main`.
This is prepared for the factory's **Standard static** deployment: the only
publish directory is `dist/site/` and the verified entry point is
`dist/site/index.html`. The deployment factory owns hosting/DNS. A direct
health probe from this container could not resolve
`infra-test-evidence.sociobot.in`, so the public hostname cannot be verified
from this execution environment.

## Known gap

The base candidate did not contain `.factory/brief.json`, despite the README
referencing it. The implementation therefore makes the closest honest product
choice inferred from the product name and mandated Rust-CLI/static-viewer
stack: portable infrastructure test evidence that remains local by default.
