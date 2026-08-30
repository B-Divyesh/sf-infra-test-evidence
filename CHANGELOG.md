# Changelog

## 0.1.0

- Honor explicit OpenTofu/Terraform sensitivity metadata and fail closed on
  malformed sensitivity masks before creating reviewer artifacts.
- Convert OpenTofu/Terraform JSON event streams to JUnit XML.
- Generate self-contained redacted static evidence artifacts with provenance.
- Strictly parse and validate the legacy portable evidence record.
