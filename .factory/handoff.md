# Infra Test Evidence repair handoff

## Repair delivered

Repair commit: `2151cb8` (`repair: convert tofu evidence and harden static site`), pushed to `origin/main` on 2026-08-28 UTC.

This repair addresses every finding in `.factory/verification-2.md` while preserving the compact local evidence record and reader that previously passed.

- The Rust CLI now uses `serde_json` and fails closed for malformed JSON, empty required legacy fields, empty checks, unnamed checks, and unsupported statuses. It preserves `infra-test-evidence --json evidence.json` and its documented 0/2/64 exit contract.
- The CLI accepts newline-delimited OpenTofu/Terraform `test -json` terminal `test_run` events and supports `--junit report.xml` and `--evidence-dir evidence`. It emits JUnit XML plus a self-contained `evidence/index.html` and `evidence/evidence.json` with test-case inputs, assertion paths, redacted plan summaries, failures, and SHA-256 provenance.
- Artifact values are recursively redacted for secret- and resource-identifier-named fields; inline secret-bearing diagnostic/failure values are replaced as well. The input stream is never copied to the artifact.
- Vite now ships `/privacy/` and `/terms/` from `public/`; both are real, titled policy documents in `dist/site/`.
- The hidden file input now gives its visible drop-zone label a 3px focus ring when keyboard focused. Desktop and 390px browser regression coverage checks it alongside the policy routes.
- `public/staticwebapp.config.json` is emitted with CSP, frame protection, Permissions-Policy, `nosniff`, strict referrer policy, and one-year immutable caching for `/assets/*` content-hashed files.
- README usage, `CHANGELOG.md`, `examples/tofu-test.jsonl`, and exact CLI regression coverage were added. No trackers, cookies, remote fonts, CDN scripts, uploads, browser storage, or telemetry were introduced.

## Verification (2026-08-28 UTC)

Completed from a clean dependency install at repository root:

```sh
npm ci
npm run check
npm audit --audit-level=high
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
```

Results:

- `npm run check` passed ESLint, TypeScript, 4 Vitest files / 10 tests, and 3 Rust tests. The new CLI tests execute the release command, verify JUnit plus artifact output and redaction, and verify usage failure for a missing option value. Rust tests cover malformed JSON, empty legacy fields, conversion, JUnit structure, provenance sections, and redaction.
- `npm audit --audit-level=high` reported 0 vulnerabilities.
- `npm run build` emitted `dist/site/index.html` (3.35 kB), CSS (5.48 kB; 1.99 kB gzip), JS (3.44 kB; 1.55 kB gzip), both policy pages, and `staticwebapp.config.json`. Initial JS and CSS are within the 200 kB / 50 kB static-product budgets.
- `npm run qa:browser` passed 6/6 Chromium checks across desktop and iPhone 13 (390px): sample load, console cleanliness, policy routes, and visible keyboard file-input focus. `npm run qa:a11y` passed 2/2 axe scans with no serious or critical violations.
- `npm run package:check` passed `cargo package --locked` (34 files, 49.8 kB compressed) and `npm pack --dry-run`. This is ready to publish; no publish was attempted because registry credentials belong to the factory.
- A clean release-package consumer install succeeded with:

  ```sh
  cargo install --path target/package/infra-test-evidence-0.1.0 --root <clean-root> --locked
  <clean-root>/bin/infra-test-evidence --json --junit report.xml --evidence-dir evidence examples/tofu-test.jsonl
  ```

  It wrote nonempty `report.xml`, `evidence/index.html`, and `evidence/evidence.json`; the legacy example returned valid JSON; malformed input exited 2; and an incomplete `--junit` option exited 64.

## Deployment and live check

The static deployment was triggered by pushing `2151cb8` to `main`; deploy target remains `dist/site/`. Immediately after the push, the public URL still served the previous candidate (root `content-length: 2785`, old “The record format” heading, and `/privacy/` root-page fallback). The new build and static hosting policy are committed and pushed, but the external static deployment had not refreshed within the worker window. Once the factory publish queue applies the pushed revision, verify:

```sh
curl -I https://infra-test-evidence.sociobot.in/assets/<current-hash>.js
curl -I https://infra-test-evidence.sociobot.in/privacy/
```

Expected: `Cache-Control: public, max-age=31536000, immutable` for hashed assets; CSP, `X-Frame-Options: DENY`, and `Permissions-Policy` headers; and a Privacy document (not the root page). The landing site is intentionally not a PWA, so there is no service worker/offline cache to update; generated evidence pages are fully self-contained and open offline.

## Known gap

`.factory/brief.json` is absent from this repository, as already documented by the prior handoff. The repaired behavior follows the independent verifier’s explicit OpenTofu/Terraform-to-JUnit/static-redacted-artifact acceptance contract and retains the existing product-specific ledger viewer.
