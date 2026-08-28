# Infra Test Evidence

Infra Test Evidence converts local OpenTofu or Terraform `test -json` output
into a standards-compatible JUnit report and a redacted static evidence page.
It is for module maintainers who need a reviewer to inspect failed IaC tests
without uploading logs or plans to another service.

The companion landing page is https://infra-test-evidence.sociobot.in. It is a
local-only reader for the compact evidence record used by older workflows.

## Install

```sh
cargo install infra-test-evidence
```

Or build it from a checkout:

```sh
cargo build --release --locked
```

## Convert OpenTofu or Terraform test output

Capture the JSON-lines output, then create both review outputs:

```sh
tofu test -json > tofu-test.jsonl
infra-test-evidence --junit report.xml --evidence-dir evidence tofu-test.jsonl
# Terraform works the same way:
# terraform test -json > terraform-test.jsonl
```

`report.xml` is a JUnit test suite suitable for CI consumers. `evidence/`
contains `index.html` and `evidence.json`; open `evidence/index.html` directly
or serve that directory statically. It records test-case inputs, assertion
paths where emitted by the runner, redacted plan-change summaries, failures,
and source provenance (including the input SHA-256). It never sends data over
the network.

Secret- and resource-identifier-named fields are recursively redacted before
they reach the evidence artifact. The source input is never copied to the
artifact. See `examples/tofu-test.jsonl` for a complete sample.

## Strict validation and CI

The existing compact record remains supported for portable workflows:

```sh
infra-test-evidence --json examples/passing-evidence.json
# {"checks":2,"errors":[],"valid":true}
```

That record requires non-empty `run`, `environment`, and `recordedAt` strings
and at least one check with a non-empty `name` and supported `status` (`pass`,
`fail`, `error`, or `skip`). Invalid JSON and incomplete records exit 2.

The converter exits 0 for valid input, 2 for invalid input or output failures,
and 64 for incorrect usage. `--help` documents every option. `--json` prints a
machine-readable validation result for either supported input form.

## Develop, test, and deploy

```sh
npm ci
npm test
npm run check
npm run build:site       # creates dist/site/
npm run qa:browser       # desktop + 390px mobile + policy routes
npm run qa:a11y
npm run package:check    # cargo package and npm pack dry run
```

Deploy `dist/site/` as a static site. The emitted `staticwebapp.config.json`
sets restrictive browser response policies and immutable caching for hashed
assets. No analytics, remote fonts, CDNs, browser storage, or uploads are used.

## License

MIT. See [LICENSE](LICENSE).

## Evidence safety

The converter fails closed: an event stream must finish with one supported test
summary, every completed run must use a supported status, and negative
durations are rejected. Test-plan variables, outputs, resource changes, and
assertion traversals are correlated with their test run before redacted
reviewer evidence is written. A sensitive diagnostic is redacted as a whole,
so an unlabelled value in a diagnostic diff cannot escape the artifact.
