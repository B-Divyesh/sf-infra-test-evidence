# Infra Test Evidence

Infra Test Evidence converts local OpenTofu or Terraform `test -json` output
into a JUnit report and a redacted reviewer page. It is for
infrastructure-module maintainers who need reviewers to inspect failed tests
without uploading logs or plans.

The companion landing page is https://infra-test-evidence.sociobot.in. It is a
local reader for the compact record used by earlier workflows. Open
https://infra-test-evidence.sociobot.in/?demo=1 to try it with sample data.
The landing page also includes a self-hosted recording and transcript of the
packaged CLI demo.

## Install

Install the CLI from a checkout:

```sh
cargo install --path . --locked
```

Or build it without installing:

```sh
cargo build --release --locked
```

## Convert OpenTofu or Terraform test output

Capture the JSON-lines output, then create three output files:

```sh
tofu test -json > tofu-test.jsonl
infra-test-evidence --junit report.xml --evidence-dir evidence tofu-test.jsonl
# Terraform works the same way:
# terraform test -json > terraform-test.jsonl
```

`report.xml` contains the converted checks in JUnit XML. `evidence/` contains
`index.html` and `evidence.json`; open `evidence/index.html` directly or serve
that directory statically. It records test-case inputs, assertion paths where
emitted by the runner, redacted plan-change summaries, failures, and source
provenance, including the input SHA-256. The reviewer page works from disk and
makes no network requests.

Sensitive values and common AWS, Azure, and GCP resource identifiers are
redacted before they reach the output files. This includes AWS ARNs, EC2,
subnet, and security-group IDs, Azure resource IDs, and GCP instance paths.
Resource-identifier-named fields are also redacted. The CLI also redacts values marked by
`sensitive: true`, `before_sensitive`, `after_sensitive`, or
`sensitive_values`. Malformed sensitivity metadata rejects the input and does
not produce output files. See `examples/tofu-test.jsonl` for a complete sample.

## Try the bundled demo

Run a realistic sample without preparing an input file:

```sh
infra-test-evidence --demo
```

The command writes the bundled sample, JUnit report, evidence JSON, and reviewer
page to a new temporary directory. It prints every output path when complete.

## Strict validation and CI

The CLI still validates the earlier compact JSON record used by the browser
reader:

```sh
infra-test-evidence --json examples/passing-evidence.json
# {"checks":2,"errors":[],"valid":true}
```

Set `run`, `environment`, and `recordedAt` to non-empty strings. Add one named
check with status `pass`, `fail`, `error`, or `skip`. If present, `durationMs`
and event-stream `elapsed` values must be non-negative finite numbers. Invalid
JSON and incomplete records exit 2.

The converter exits 0 for valid input, 2 for invalid input or output failures,
and 64 for incorrect usage. `--help` documents every option. `--json` prints a
machine-readable validation result for either supported input form. The CLI
only reads existing test output. It never invokes OpenTofu or Terraform and
never contacts remote state or another service.

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

Deploy `dist/site/` as a static site. The browser reader uses no analytics,
remote fonts, CDNs, storage, or uploads.

## License

MIT. See [LICENSE](LICENSE).

## Evidence safety

The converter rejects an event stream without one final supported summary. It
also rejects summary statuses that contradict completed runs, unsupported run
statuses, and malformed or negative durations. Test-plan
variables, outputs, resource changes, and assertion traversals stay with their
matching test run before redacted output files are written. The CLI
redacts the whole sensitive diagnostic. This keeps unmarked values in the same
diagnostic out of every output file.
