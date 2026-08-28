# Independent verification — FAIL

**Candidate:** `e6d6f4881f9a272ef50868d565e4b61866da7aea`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Verified:** 2026-08-28 UTC, from a fresh detached clone at the candidate SHA.

## Verdict

**FAIL.** The published root and its two hashed assets are exactly the candidate
build, but the candidate does not perform the product's required job: it does
not convert OpenTofu/Terraform test JSON to JUnit and a static IaC evidence
artifact. It is a permissive validator for a separate, undocumented bespoke
JSON shape. The deployment also does not publish the required Privacy and Terms
pages.

## Blocking defects

### P0 — The core converter/evidence product is absent

The acceptance brief requires a CLI that converts OpenTofu/Terraform test JSON
to JUnit plus a static evidence page with test case inputs, assertion paths,
redacted plan summaries, and provenance. The only binary interface is
`infra-test-evidence [--json] <evidence.json>` and it only emits a validation
summary (`{"valid":...,"checks":...,"errors":...}`). There is no input adapter
for Terraform/OpenTofu, JUnit XML output, evidence-page generation, plan or
assertion extraction, provenance, or default secret redaction.

An installed release-package binary rejected a representative OpenTofu JSON
event object with exit 2. No invocation can create a JUnit file or static
artifact. This prevents the named user from diagnosing an IaC test failure from
the product and fails the smallest-useful-product contract.

### P1 — CLI validation can return a false CI success for malformed input

The Rust CLI does not parse JSON; it searches raw text for field names and
braces. In a clean consumer install, this syntactically invalid input returned
`{"valid":true,"checks":1,"errors":[]}` and exit 0:

```json
{"run":"x","environment":"prod","recordedAt":"now","checks":[{}]
```

It also accepted empty `run`, `environment`, and `recordedAt` strings and a
check object with neither name nor status. This can mark an unusable/corrupt CI
record valid, which is especially unsafe for a test-evidence gate.

### P1 — Required deployed Privacy and Terms pages are broken

`npm run build` emits only `dist/site/index.html` and two assets. It does not
copy `privacy/index.html` or `terms/index.html`. Both local production preview
and the live site return the application homepage (HTTP 200, 2,785 bytes) for
`/privacy/` and `/terms/`; browser navigation shows the root title and root
`h1`, not either policy. This fails the repository's documentation requirement.

## Additional findings

### P2 — Hashed static assets are not cached immutably

The live JavaScript and CSS assets are content-hashed but both respond with
`cache-control: public, must-revalidate, max-age=30`, not a long-lived immutable
policy. This misses the stated static-product caching policy and causes
unnecessary repeat validation/downloads.

### P3 — Browser response policy is incomplete

Live responses include HSTS, `nosniff`, and Referrer-Policy, but no
Content-Security-Policy, `frame-ancestors`/X-Frame-Options, or
Permissions-Policy. This is not the cause of the FAIL, but a restrictive CSP
would be appropriate for a local-file viewer that renders parsed content.

## Checks that passed

| Area | Fresh evidence |
| --- | --- |
| Clean install/toolchain | `npm ci` completed with 0 audit vulnerabilities; Node 22.23.2, npm 10.9.8, Rust 1.98.0. |
| Unit, integration, lint, types | `npm run check` passed: ESLint, TypeScript, 2 Vitest tests, 2 Rust tests. |
| Exact production build | `npm run build` and `npm run build:site` passed. Output: HTML 2.78 kB, CSS 5.37 kB (1.97 kB gzip), JS 3.44 kB (1.55 kB gzip), within the 200 kB JS / 50 kB CSS budgets. |
| Browser/A11y | `npm run qa:browser` passed 4/4 (desktop and iPhone-13 project); `npm run qa:a11y` passed 2/2 with no axe serious/critical findings. |
| Manual browser flow | At desktop and 390px: sample load, malformed JSON error, invalid-shape error, and recovery with a failed check all worked; no horizontal overflow, console errors, or page errors. Keyboard skip link showed a 3px visible focus outline and Enter loaded the sample. Reduced-motion transition was 0.01ms. |
| Privacy/network | Runtime request capture contained only the same origin; source/build inspection found no tracking, upload, remote font, or third-party runtime request. |
| Release package | `cargo package --locked` passed; the packaged crate was installed with `cargo install --path target/package/infra-test-evidence-0.1.0 --root <clean-temp-root>`. `--help`, documented valid input, and documented exit codes behaved as advertised. |
| Live identity | Live `/`, `/assets/index-CAdrTEqZ.js`, and `/assets/style-CErjZlhu.css` SHA-256 matched this candidate's production build byte-for-byte. |

## Reproduction commands

```sh
git clone https://github.com/B-Divyesh/sf-infra-test-evidence.git qa
cd qa && git checkout --detach e6d6f4881f9a272ef50868d565e4b61866da7aea
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
cargo install --path target/package/infra-test-evidence-0.1.0 --root /tmp/ite-consumer --locked
printf '%s' '{"run":"x","environment":"prod","recordedAt":"now","checks":[{}]' | /tmp/ite-consumer/bin/infra-test-evidence --json /dev/stdin
curl -i https://infra-test-evidence.sociobot.in/privacy/
```

## Required next steps

Implement and test real Terraform/OpenTofu JSON input conversion, JUnit XML and
static evidence output, default recursive secret/resource-identifier redaction,
and evidence fields for test case inputs, assertions, plan summary, failures,
and provenance. Replace string scanning with a strict JSON parser and schema.
Include the policy pages in `dist/site/` (or make them real build entry points),
then correct static cache and response policies in the deployment configuration.
