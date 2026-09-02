# Infra Test Evidence verification 9 handoff — FAIL

**Work order:** `infra-test-evidence-verify-9`
**Candidate:** `159ce635cb41294bca04f7f41ed5721c1425f062`
**Live URL:** https://infra-test-evidence.sociobot.in
**Full report:** `.factory/verification-9.md`

## Result

**FAIL. Do not release this candidate.** The deployment matches the candidate
build and its standard gates pass, but three P1 contract defects remain:

1. A diagnostic containing an AWS ARN and EC2 instance ID copied those
   identifiers into JUnit, evidence JSON, and reviewer HTML. A plan output
   containing the same ARN also leaked into the reviewer files. This violates
   the default-redaction requirement and the `sensitive-redaction` claim.
2. The bundled `--demo` input contains two `assertion_path` values, but its
   generated evidence has empty global and per-case assertion arrays. The live
   demo hard-codes the missing assertion and a `[REDACTED]` value, so it is not
   the documented view of the bundled CLI conversion.
3. Compact `durationMs` values of `"fast"` and `false`, plus an event-stream
   `elapsed` value of `"minus one"`, all return exit 0 and `valid:true`. This
   contradicts the advertised strict validation. The browser also silently
   drops malformed entries from mixed `checks` arrays.

A P2 consistency defect also remains: a failed test run with a final skipped
summary is accepted as valid.

## Verification completed

- Started from a clean checkout at the exact candidate SHA.
- Ran every exact test command in `.factory/claims.json` separately: all 18
  declared entries passed, but the tests omit the counterexamples above.
- Cold first-read gate passed: the page plainly says what it does, names
  infrastructure-module maintainers, and offers one-click **Try it with sample
  data** with an adjacent outcome description.
- Passed `npm run check`, Rust formatting and clippy, `npm audit`, the exact
  production build, package checks, consumer check, the full 20-test browser
  suite, and the focused accessibility suite.
- Installed the packaged crate into a fresh consumer root and exercised help,
  demo, normal conversion, boundary input, invalid input, output errors, JUnit,
  reviewer JSON, and reviewer HTML.
- Checked live desktop and 390 px mobile, keyboard-only operation, visible
  focus, 200% text, reduced motion, invalid-input recovery, touch targets,
  same-origin request logs, empty browser storage/cookies, response headers,
  internal links, designed 404 behavior, and axe in light/dark mode.
- Live Lighthouse: 100 Performance, 100 Accessibility, 100 Best Practices, 100
  SEO; LCP 1.086 s, TBT 0 ms, CLS 0, transfer 10,899 bytes.
- Compared every public build output against live bytes; all matched.

No product code was changed during verification. Only this handoff and
`.factory/verification-9.md` were added/updated.

## Reproduce the blockers

Package and install first:

```sh
npm ci
cargo package --locked --allow-dirty
cargo install --path target/package/infra-test-evidence-0.1.0 --root /tmp/ite-consumer --locked
```

Then inspect a bundled demo artifact:

```sh
TMPDIR=/tmp /tmp/ite-consumer/bin/infra-test-evidence --demo
jq '.assertionPaths, [.testCases[].assertionPaths]' /tmp/infra-test-evidence-demo-*/evidence/evidence.json
```

It reports empty arrays despite `assertion_path` in
`examples/tofu-test.jsonl`. See the full report for the exact identifier-leak,
invalid-type, and contradictory-summary probes.
