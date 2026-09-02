# Landing copy audit

Audited 2026-09-02. Hyphenated terms and commands count as one word. Navigation,
field labels, status labels, and code samples are listed as interface terms,
not sentences. No sentence exceeds 22 words. No banned marketing word appears.

| Landing sentence | Words | Flag |
| --- | ---: | --- |
| Convert infrastructure test runs into JUnit and redacted evidence that reviewers can inspect locally. | 14 | None |
| Turn infrastructure tests into reviewable evidence | 6 | None |
| For infrastructure-module maintainers who need reviewers to inspect failed OpenTofu or Terraform tests without uploading logs. | 16 | None |
| See a failed test, redaction, and output files. | 8 | None |
| Runs in your browser | 4 | None |
| No trackers or uploads | 4 | None |
| Free under the MIT License | 5 | None |
| See the CLI create a JUnit report, evidence JSON, and reviewer page | 12 | None |
| The real packaged `--demo` flow converts two bundled checks without setup. | 10 | None |
| Recorded from the packaged 0.1.0 binary. | 6 | None |
| Temporary directory details are shortened. | 5 | None |
| Recording complete. The JUnit report, evidence JSON, and reviewer page paths are shown. | 13 | None; recording status |
| Your run details and checks will appear here. | 8 | None |
| That file is not valid JSON. Choose a compact record or correct the file and try again. | 17 | None; file error |
| Save OpenTofu or Terraform `test -json` output. | 7 | None |
| Create a JUnit report, evidence JSON, and a reviewer page. | 10 | None |
| Open the reviewer page from disk or inspect a compact record above. | 12 | None |
| The CLI creates three output files: a JUnit report, evidence JSON, and a reviewer page. | 15 | None |
| The reviewer page includes test inputs, assertion paths, redacted plan changes, failures, and the input SHA-256. | 16 | None |
| The local reader also opens the compact record shown below. | 10 | None |
| Use `infra-test-evidence --json evidence.json` to strictly validate this compact record in CI. | 11 | None |
| It does not upload files, run infrastructure changes, or replace a reviewer. | 12 | None |
| It only converts and displays the evidence you provide. | 9 | None |
| Infra Test Evidence turns test runs into reviewer evidence. | 9 | None |

## Interface terms

| Concept | One term used |
| --- | --- |
| Input from OpenTofu or Terraform | test run |
| Compact browser input | compact record |
| Generated HTML | reviewer page |
| Generated XML | JUnit report |
| No-setup mode | demo |
| Individual result | check |
| Browser playback of the CLI | recording |

Primary action: **Try it with sample data**. Real-data action: **Open a compact
record**. File action: **Choose a compact record**. The first screen names the job, the infrastructure-module
maintainer, and both available first steps.
