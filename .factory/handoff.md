# Infra Test Evidence repair 8 handoff

**Work order:** `infra-test-evidence-repair-8`
**Base verifier report:** `.factory/verification-9.md` at
`283838aa87005cfbb219b67eaa0ff4fe55f71cf0`
**Status:** local repair verification passed; deployment evidence is recorded
after the production upload below.

## What changed

- Redacts text containing AWS ARNs or EC2 instance IDs before a JUnit report,
  evidence JSON, or reviewer HTML page is rendered. The exact verifier-style
  fixture covers diagnostic, failure, and plan-output positions.
- Reads `assertion_path` and `assertion_paths` from completed `test_run`
  events. The bundled demo now preserves the two real assertion paths.
- Adds a sensitive test-plan output to the bundled demo and ships
  `public/demo-evidence.json`, generated from that conversion. The browser
  demo loads this artifact and derives its failed check, assertion path,
  duration, redaction, and SHA-256 proof from it. The release test compares
  the static artifact with a fresh packaged `--demo` result.
- Rejects present string, boolean, null, negative, or non-finite compact
  `durationMs` values and event-stream `elapsed`/`duration_ms` values.
  The browser reader now rejects malformed duration values and invalid entries
  in a mixed `checks` array instead of filtering them out.
- Rejects a `skip` summary unless every completed run was skipped. A failed
  run followed by a skipped summary now exits 2 with the documented mismatch
  error.
- Adds four named claims and exact counterexamples for resource redaction,
  malformed duration types, skipped-summary consistency, and CLI/demo
  artifact fidelity.

## Required reproduction before the fix

Before implementation changes, the verifier fixture returned
`{"checks":1,"errors":[],"valid":true}` and `rg` found `arn:aws`,
`i-0abc123`, and `aws_instance.web` in all three generated artifacts:

```sh
cargo run -- --json --junit /tmp/report.xml --evidence-dir /tmp/evidence \
  tests/fixtures/verification-9-resource-identifiers.jsonl
rg 'arn:aws|i-0abc123|aws_instance\.web' /tmp/report.xml /tmp/evidence/*
```

The same pre-fix run accepted compact `durationMs: "fast"`, event-stream
`elapsed: "minus one"`, and a failed `test_run` with a final `skip` summary.
The bundled demo produced empty assertion arrays despite two
`test_run.assertion_path` values.

## Verification

Clean install and complete local gates passed:

```text
npm ci                                      PASS, 0 audit vulnerabilities
npm run check                               PASS
  eslint, tsc --noEmit, cargo test (7), vitest (26)
npm run qa:browser                          PASS, 22 tests, desktop + 390 px
npm run qa:a11y                             PASS, both Playwright projects
npm run build                               PASS, dist/site/
npm run package:check                       PASS, cargo package + npm pack dry run
npm run consumer:check                      PASS
cargo fmt --check                           PASS
cargo clippy --locked --all-targets -- -D warnings
                                            PASS
npm audit --audit-level=high                PASS, 0 vulnerabilities
```

The repaired production build has 9.1 kB of JavaScript and 11.7 kB of CSS
before gzip. Its checked-in demo evidence is 1.7 kB. The browser suite covers
desktop/mobile, keyboard focus, 200% text, reduced motion, contrast/axe,
same-origin requests, no browser storage, and the disk-opened reviewer page.
This static product has no service worker, server API, data store, or payment
flow, so update/offline-server, rate-limit, persistence, and payment checks do
not apply.

Every exact command listed in `.factory/claims.json` was run separately from
the clean install: all 22 claims passed. The new counterexample commands are:

```text
@claim:resource-identifier-redaction         PASS
@claim:malformed-duration-types              PASS
@claim:summary-consistency                   PASS
@claim:demo-artifact-fidelity                PASS in desktop and mobile projects
```

## Deployment and live identity

Pending the repair commit push and static deployment. After upload, verify
`https://infra-test-evidence.sociobot.in` with the fleet URL checker, live
desktop/mobile browser checks, and byte comparison against `dist/site/`.

## Known gaps / next steps

None in the product implementation. Do not publish the crate from this worker;
the package is ready for the factory registry workflow.
