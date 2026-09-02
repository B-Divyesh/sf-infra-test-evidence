# Infra Test Evidence

Infra Test Evidence converts local OpenTofu or Terraform `test -json` output
into a JUnit report and a redacted static evidence page. It is for
infrastructure-module maintainers who need reviewers to inspect failed tests
without uploading logs or plans.

The companion landing page is https://infra-test-evidence.sociobot.in. It is a
local reader for the compact evidence record used by older workflows. Open
https://infra-test-evidence.sociobot.in/demo/ to try it with sample data.
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
and source provenance, including the input SHA-256. The reviewer page works
from disk and makes no network requests.

Secret- and resource-identifier-named fields are recursively redacted before
they reach the evidence artifact. Explicit OpenTofu/Terraform `sensitive: true`
values and `before_sensitive`, `after_sensitive`, and `sensitive_values` masks
are also authoritative. Malformed sensitivity metadata rejects the input and
does not produce reviewer artifacts. See `examples/tofu-test.jsonl` for a
complete sample.

## Try the bundled demo

Run a realistic sample without preparing an input file:

```sh
infra-test-evidence --demo
```

The command writes the bundled sample, JUnit report, evidence JSON, and reviewer
page to a new temporary directory. It prints every output path when complete.

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

Deploy `dist/site/` as a static site. The emitted `staticwebapp.config.json`
sets restrictive browser response policies and immutable caching for hashed
assets. The browser reader uses no analytics, remote fonts, CDNs, storage, or
uploads.

## License

MIT. See [LICENSE](LICENSE).

## Evidence safety

The converter fails closed: an event stream must finish with one supported test
summary, every completed run must use a supported status, and negative
durations are rejected. Test-plan variables, outputs, resource changes, and
assertion traversals are correlated with their test run before redacted
reviewer evidence is written. A sensitive diagnostic is redacted as a whole,
so an unlabelled value in a diagnostic diff cannot escape the artifact.
