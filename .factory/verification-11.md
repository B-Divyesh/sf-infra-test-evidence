# Independent verification 11 — PASS

**Work order:** `infra-test-evidence-verify-11`  
**Candidate commit:** `ddfc263e91b2ff27a4713455ce6b10fedefbb40b`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Verified:** 2026-09-02 UTC

## Decision

**PASS.** The candidate is a working local-first CLI and static evidence reader
for infrastructure-module maintainers. It converts supplied Terraform/OpenTofu
test output to JUnit plus redacted JSON/HTML reviewer evidence, without running
infrastructure tools or contacting a service. The prior multi-cloud identifier
redaction failure is covered by a new packaged claim and independently passed.

The checkout has no `.factory/brief.json`; the researched brief injected into
this work order was used as the acceptance contract.

## Required first checks

All 23 commands declared in `.factory/claims.json` were run from the clean
checkout after `npm ci`, using their declared package/demo sandboxes. They all
passed. The aggregate reruns also passed: 17 CLI/Vitest claims and 12
desktop/mobile browser-project executions (the six browser claims in both
projects).

This includes packaged-CLI demo/conversion, explicit sensitive-value redaction,
AWS ARN/EC2 and cross-provider AWS subnet/security-group, Azure resource-ID and
GCP instance-path redaction; malformed sensitivity metadata; validation and
exit codes; run correlation; conversion isolation; requested write paths;
local reviewer artifacts; reader privacy; one-click browser demo; recording;
and MIT licensing.

Cold live first read: the headline says, “Turn infrastructure tests into
reviewable evidence.” The following sentence names infrastructure-module
maintainers and failed OpenTofu/Terraform tests. The visible primary action is
“Try it with sample data,” with adjacent text explaining that it shows a failed
test, redaction, and output files. One click opens `/demo/?demo=1`, whose
persistent banner says “Demo — sample data, nothing is saved” and exposes both
**Reset demo** and **Start for real**. This satisfies the plain-words and demo
entry requirements.

## Clean local, package, and CLI checks

```text
npm ci                                           PASS (182 packages; 0 vulnerabilities)
npm test                                         PASS (8 Rust + 27 Vitest tests)
npm run lint                                     PASS
npm run typecheck                                PASS
npm run build                                    PASS (dist/site/)
npm run package:check                            PASS (cargo package + npm pack dry run)
npm run consumer:check                           PASS
cargo fmt --check                                PASS
cargo clippy --locked --all-targets -- -D warnings PASS
npm audit --audit-level=high                     PASS (0 vulnerabilities)
npm run qa:a11y                                  PASS (2 browser projects)
```

A fresh consumer install was made from the crate produced by
`cargo package --locked --allow-dirty`. Its public binary passed `--help`,
`--demo`, and a normal conversion of `examples/tofu-test.jsonl`, producing
`report.xml`, `evidence/evidence.json`, and `evidence/index.html`. A compact
record with a string duration returned the documented machine-readable error
and exit code 2. This independently covers normal, boundary/error, and
recovery use rather than relying only on workspace execution.

The production build contains 6.78 kB JS (3.37 kB gzip for the main bundle,
0.68 kB routes) and 11.72 kB CSS (3.37 kB gzip), comfortably below the static
budget. A fresh `verify-url.sh` browser pass took 784 ms and found no console
or page errors, a title, `lang=en`, exactly one `h1`, a `main`, no missing image
alt attributes, and no unnamed buttons.

## Live, privacy, accessibility, and deployment checks

- Candidate `dist/site/` public files match the live responses byte-for-byte;
  `staticwebapp.config.json` is deployment configuration and correctly excluded.
- Root, demo, privacy, and terms return 200. An unknown route returns the
  designed 404 with HTTP 404. HTML has 30-second revalidation caching; hashed
  assets have `public, max-age=31536000, immutable`.
- Desktop and 390 px mobile Playwright checks found no console/page errors and
  no serious or critical axe findings. Keyboard Tab reaches the skip link,
  navigation, demo action, recording controls, and file input with visible
  focus; reduced-motion mode exposes the recording transcript and avoids the
  recording animation.
- During the live demo and a local file import, every outgoing request stayed
  on `https://infra-test-evidence.sociobot.in`; there were no cookies,
  localStorage/sessionStorage keys, or IndexedDB databases. The reader has no
  analytics, uploads, backend endpoint, sign-in, payment, or service worker.
  Consequently rate-limit, Entra authority, persistence/concurrency, and PWA
  update checks are not applicable.
- Live headers include HSTS, same-origin CSP with `frame-ancestors 'none'`,
  `nosniff`, `X-Frame-Options: DENY`, strict referrer policy, and restrictive
  Permissions Policy.

## Defects

No P0, P1, P2, or P3 defects found.

## Evidence locations

- Fresh URL verifier: `/tmp/ite-verify-url-dIGyPt/verify.json`
- Lighthouse was attempted but its direct launcher could not start Chromium as
  root in this container despite the available Playwright browser; this is an
  environment limitation, not a site failure. The bundle-budget, browser,
  accessibility, and URL-verifier checks above passed from fresh live loads.
