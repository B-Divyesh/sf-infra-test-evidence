# Independent QA verification 2 — FAIL

**Candidate:** `e6d6f4881f9a272ef50868d565e4b61866da7aea`  
**Live URL:** https://infra-test-evidence.sociobot.in  
**Verified:** 2026-08-28 UTC from a clean detached worktree at the candidate SHA.

## Verdict

**FAIL.** This is not a deployment-only failure: the live root document and
both hashed production assets exactly match a fresh production build of the
candidate. The candidate is a permissive validator for an undocumented bespoke
record, not the required OpenTofu/Terraform JSON adapter that emits JUnit and a
static, redacted IaC evidence artifact. It cannot perform the smallest useful
product's job.

## Release-blocking defects

### P0 — The OpenTofu/Terraform-to-JUnit evidence product does not exist

The acceptance contract requires a CLI that converts OpenTofu/Terraform test
JSON to JUnit plus a static evidence page showing test-case inputs, assertion
paths, redacted plan summaries, failures, and provenance. The only public CLI
is:

```text
infra-test-evidence [--json] <evidence.json>
```

It only checks for raw-text occurrences of `run`, `environment`, `recordedAt`,
and an object in `checks`, then prints a validation summary. There is no
Terraform/OpenTofu input adapter, JUnit XML writer, evidence-artifact writer,
plan/assertion extraction, provenance, or recursive default redaction.

A representative `tofu test -json`-style event (`@level`, `@message`, and a
failed `test_run`) was rejected by the release-package binary with exit 2.
`--junit /tmp/report.xml` is unsupported (exit 64), and no documented or
implemented command can make either required output. This prevents the target
maintainer from diagnosing failed module tests from an artifact.

### P1 — The CI validator can falsely succeed on corrupt or unusable input

The Rust binary does not parse JSON; it searches strings and counts braces.
The clean-consumer installed binary returned exit 0 and
`{"valid":true,"checks":1,"errors":[]}` for syntactically malformed JSON:

```json
{"run":"x","environment":"prod","recordedAt":"now","checks":[{}]
```

It also returned the same successful result for empty required values and a
check with no name or status:

```json
{"run":"","environment":"","recordedAt":"","checks":[{}]}
```

This is unsafe for CI evidence: a corrupt or non-reviewable record can be
treated as a passing validation gate.

### P1 — The deployed Privacy and Terms links are broken

The candidate contains `privacy/index.html` and `terms/index.html`, but the
exact production build emits only `dist/site/index.html` and two assets. Both
local production preview and live `/privacy/` and `/terms/` respond `200` with
the root page (root title and root h1), not their policy documents.

### P2 — The file chooser is keyboard-reachable with no visible focus

At 390px, Tab reaches `#evidence-file` after the navigation links, but its
focused box is `1 × 1` CSS px with `opacity: 0` and no visible outline. The
interactive control can therefore receive keyboard focus without a discernible
focus indicator, contrary to the required visible-focus baseline. The skip
link and sample button do show visible focus.

### P2 — Content-hashed live assets are not immutably cached

Live JS and CSS use `cache-control: public, must-revalidate, max-age=30`.
They are content-hashed and should receive a long-lived immutable cache policy
under the static-product performance contract.

### P3 — Response security policy is incomplete

The live deployment sends HSTS, `nosniff`, and
`Referrer-Policy: strict-origin-when-cross-origin`, but no
Content-Security-Policy, clickjacking policy (`frame-ancestors` or
X-Frame-Options), or Permissions-Policy.

## Checks that passed

| Area | Fresh evidence |
| --- | --- |
| Clean install/toolchain | Fresh detached worktree; `npm ci` succeeded with 0 vulnerabilities. Node 22.23.2, npm 10.9.8, Rust/Cargo 1.98.0. |
| Repository checks | `npm run check` passed: ESLint, TypeScript, 2 Vitest tests, and 2 Rust unit tests. `npm audit --audit-level=high` found 0 vulnerabilities. |
| Exact build and budgets | `npm run build` succeeded. It emitted HTML 2.78 kB, CSS 5.37 kB (1.97 kB gzip), JS 3.44 kB (1.55 kB gzip): within 50 kB CSS and 200 kB initial-JS budgets. |
| Browser and axe | `npm run qa:browser` passed 4/4 and `npm run qa:a11y` passed 2/2 across desktop and the shipped iPhone-13 project; no axe serious/critical violations. |
| Independent browser exercise | At 1440px and 390px, sample load, malformed JSON error, invalid-shape feedback, recovery by selecting a valid record, and failed-check display worked. No horizontal overflow, console errors, or page errors occurred. The skip-link focus ring was visible; reduced-motion changed drop-zone transition duration to `0.01ms`. |
| Privacy/network | Browser request capture after all flows used only the local origin. Source/build inspection found no upload API, analytics/tracker, remote font, CDN script, or browser storage. Files are read locally by the viewer. |
| Pack/install consumer | `cargo package --locked` and `npm pack --dry-run` passed. The packaged crate was installed into a clean `/tmp` consumer root with `cargo install --path target/package/infra-test-evidence-0.1.0 --root … --locked`; `--help`, documented valid input, unreadable-file exit 2, and usage exit 64 behaved as documented. |
| Live identity | SHA-256 matched exactly: root HTML `f7edb0adc67dd9e8ce9b187257fdb37e097d84886591c166e89f3f30575174fe`, JS `9d2c680b84ee8732239fb6cbea857b5b6df72c81c9afe6f7e88989e5d57cd283`, CSS `e6d764b8a335f3be1cadc93e81a4d627109e28054487847ef6d21f9b92faf052`. |

## Reproduction

```sh
git clone https://github.com/B-Divyesh/sf-infra-test-evidence.git qa
cd qa && git checkout --detach e6d6f4881f9a272ef50868d565e4b61866da7aea
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
npm audit --audit-level=high
cargo install --path target/package/infra-test-evidence-0.1.0 --root /tmp/ite-consumer --locked
/tmp/ite-consumer/bin/infra-test-evidence --json examples/passing-evidence.json
curl -sS https://infra-test-evidence.sociobot.in/ | sha256sum
curl -i https://infra-test-evidence.sociobot.in/privacy/
```

## Required next steps

Implement and test a strict JSON parser and real OpenTofu/Terraform test JSON
adapter; produce standards-compatible JUnit and a static evidence artifact with
case inputs, assertions, failures, provenance, and default recursive
secret/resource-identifier redaction. Make all invalid record cases fail
nonzero. Include Privacy and Terms in `dist/site/`; restore a visible file-input
focus treatment; then set immutable caching for hashed assets and a restrictive
static-site response policy. Re-run this verification after those changes.
