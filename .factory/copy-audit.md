# Landing copy audit

Audited 2026-08-30. Hyphenated terms and commands count as one word. Navigation,
field labels, status labels, and code samples are listed as interface terms,
not sentences. No sentence exceeds 22 words. No banned marketing word appears.

| Landing sentence | Words | Flag |
| --- | ---: | --- |
| Convert infrastructure test runs into JUnit and redacted evidence that reviewers can inspect locally. | 14 | None |
| Turn infrastructure tests into reviewable evidence | 6 | None |
| For infrastructure-module maintainers who need reviewers to inspect failed OpenTofu or Terraform tests without uploading logs. | 16 | None |
| See two example checks in the browser reader. | 8 | None |
| Runs in your browser | 4 | None |
| No trackers or uploads | 4 | None |
| Free under the MIT License | 5 | None |
| Your run details and checks will appear here. | 8 | None |
| Save OpenTofu or Terraform `test -json` output. | 7 | None |
| Create JUnit, reviewer JSON, and a static HTML page. | 9 | None |
| Open the redacted page from disk or inspect a compact record above. | 12 | None |
| Run the converter against `tofu test -json` or `terraform test -json` output to create a JUnit report and a self-contained reviewer page. | 22 | None |
| The page includes test inputs, assertion paths, redacted plan changes, failures, and provenance. | 12 | None |
| The local reader also accepts the compact portable record used by earlier workflows. | 12 | None |
| Use `infra-test-evidence --json evidence.json` to strictly validate this compact form in CI. | 10 | None |
| It does not upload files, run infrastructure changes, or replace a reviewer. | 12 | None |
| It only converts and displays the evidence you provide. | 9 | None |
| Infra Test Evidence turns test runs into reviewer evidence. | 9 | None |

## Interface terms

| Concept | One term used |
| --- | --- |
| Input from OpenTofu or Terraform | test run |
| Compact browser input | evidence record |
| Generated HTML | reviewer page |
| Generated XML | JUnit report |
| No-setup mode | demo |
| Individual result | check |

Primary action: **Try it with sample data**. Real-data action: **Open your
evidence file**. The first screen names the job, the infrastructure-module
maintainer, and both available first steps.
