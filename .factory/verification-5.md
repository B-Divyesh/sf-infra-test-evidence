# Independent QA verification 5 — FAIL

**Candidate:** `bfea264291205b960f4cbcbc18f1396a4f1ad1ab`
**Live URL:** https://infra-test-evidence.sociobot.in
**Verified:** 2026-08-30 UTC, from the supplied clean checkout at the candidate SHA.

## Verdict

**FAIL.** The live deployment is healthy and is exactly this candidate build,
so this is **not** a deployment-only failure. The repaired CLI itself performs
its main conversion and redacts the supplied explicit-sensitive fixture. It is
nevertheless unreleasable against the factory acceptance contract: the required
claims manifest is absent, and the CLI/site supply neither the required
one-click demo sandbox nor the CLI `--demo` command.

## Release-blocking defects

### P0 — `.factory/claims.json` is missing; no claim tests can be run

The verifier's first action was to resolve `.factory/claims.json` in the clean
candidate checkout. It is absent (the only top-level `.factory` files are
`design.md`, `handoff.md`, and prior verification reports). Therefore there are
no declared tests to run from the demo entry point. The claims contract makes a
missing manifest release-blocking, independent of the otherwise passing suite.

The landing copy makes testable privacy claims, including “nothing is uploaded”
and “No trackers. No uploads.”, but those claims have no manifest entries or
claim-tagged sandbox tests.

### P0 — Required one-click, isolated sample demo is absent

The cold live first screen has a **“Load a sample report”** button. It is not
the required visible **“Try it with sample data”** action and does not enter a
demo URL/namespace, show a persistent “Demo — sample data, nothing is saved”
banner, or provide Reset demo/Start for real. `GET /demo` returns HTTP 404.

As a CLI product, it must additionally provide `infra-test-evidence --demo`
against shipped `examples/` data. The candidate returns exit 64 and “Unknown
option: --demo”. This means a new maintainer cannot try the product by the
contractual no-setup path, and no isolated demo entry point exists for claim
verification.

### P1 — Cold landing copy does not plainly name its intended user

The first screen says what it accepts and contains a sample button, but it does
not say that it is for infrastructure-module maintainers/reviewers. Its
subhead describes “a portable evidence record” and “the companion CLI”; those
terms do not identify the person in the researched brief. Thus the first-read
test does not answer “for whom?” in plain words, which is independently a
stated failure condition.

### P2 — Required static-site discovery/error routes are missing

Live `/robots.txt`, `/sitemap.xml`, and `/404.html` each return 404. There is
also no `/demo` route. The site-structure contract requires robots, a sitemap,
and a designed 404 route. These are not the reason for the P0 verdict but
remain release work.

## Passing evidence

| Area | Result |
| --- | --- |
| Clean install | `npm ci` succeeded: 182 packages installed, 0 audit vulnerabilities. Node 22.23.2, npm 10.9.8, Cargo 1.98.0. |
| Code checks | `npm run check` passed ESLint, `tsc --noEmit`, 6 Rust tests, and 8 Vitest tests. `cargo fmt --check`, strict Clippy, and `npm audit --audit-level=high` also passed. |
| Build/budgets | `npm run build` passed. Initial JS is 3.66 kB raw / 1.63 kB gzip; CSS is 5.48 kB raw / 1.99 kB gzip, within the static budgets. |
| Browser/a11y tests | `npm run qa:browser` passed 8/8 (desktop + 390px-mobile); `npm run qa:a11y` passed 2/2. Independent live axe scans after the sample/error flows found no serious or critical violations. |
| Manual live flow | On desktop and 390px: sample report loads two checks; malformed JSON reports a clear recovery message; keyboard Tab reaches all controls; the file control has a visible `rgb(7, 90, 158)` 3px focus outline. Normal 390px layout has no horizontal overflow. Reduced-motion transition measured `0.00001s`. No console or page errors occurred. |
| CLI/API exercise | A normal `examples/tofu-test.jsonl` conversion exited 0 and wrote `report.xml`, `evidence/evidence.json`, and `evidence/index.html`. Invalid compact input returned exit 2 with validation errors. The package check and clean-consumer exercise passed. |
| Redaction regression | `examples/explicit-sensitive-output.jsonl` converted successfully. `rg` found no `k9M2qV7xL4` sentinel in any generated file and found `[REDACTED]` in JSON/HTML. |
| Privacy/network | Playwright request capture across initial load, sample load, and invalid-file recovery contained only the product origin (HTML, JS, CSS). No cookies or local/session-storage keys were observed; no third-party scripts, fonts, analytics, or uploads were requested. |
| Live headers/cache | Live responses have CSP with `frame-ancestors 'none'`, HSTS, `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, referrer and Permissions policies. Hashed JS/CSS return `public, max-age=31536000, immutable`. |
| Live identity | SHA-256 is byte-identical between live and `npm run build`: root `12a19a2e62ece91b03de7e2e835a886d91f54950ddcf6321e2c4fa5c40935c16`; JS `7a47f092a1ac4b54e50166dd15e50d532f065d448cffafaa06fdc4d458cdca9f`; CSS `60fe55f2c44949eba033fd380b71b93bb3a6fa136853d55e83a9ed7f4ba0ab73`; Privacy and Terms also match their built documents. |

No backend, account, sign-in, paid-unlock, or service-worker endpoint exists,
so rate-limit, Entra tenant, persistence, and PWA update/offline checks are not
applicable. The static reviewer artifact itself is local/offline; the public
landing reader is not presented as a PWA/offline app.

## Reproduction

```sh
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
npm run consumer:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high

target/debug/infra-test-evidence --junit /tmp/report.xml \
  --evidence-dir /tmp/evidence examples/tofu-test.jsonl
target/debug/infra-test-evidence --demo                 # currently exits 64
curl -i https://infra-test-evidence.sociobot.in/demo     # currently 404
```

## Required next steps

Add `.factory/claims.json` and one tagged, observable demo-entry test for
every landing/README claim. Implement the product-class demo contract:
`infra-test-evidence --demo` must run bundled realistic input in a temporary
directory and print its outputs; the first screen must offer “Try it with
sample data” and the site needs an isolated `/demo` (or `?demo=1`) flow with
the persistent demo controls. Rewrite the first-screen subhead to name
infrastructure-module maintainers, then add robots, sitemap, and a real 404
page. Re-run independent verification after those changes.
