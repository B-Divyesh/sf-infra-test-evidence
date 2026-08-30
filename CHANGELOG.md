# Changelog

## 0.1.0

- Add a packaged `--demo` flow that writes bundled sample outputs to a unique
  temporary directory.
- Add the one-click browser demo, claims suite, discovery files, and designed
  static 404 response.
- Honor explicit OpenTofu/Terraform sensitivity metadata and fail closed on
  malformed sensitivity masks before creating reviewer artifacts.
- Convert OpenTofu/Terraform JSON event streams to JUnit XML.
- Generate self-contained redacted static evidence artifacts with provenance.
- Strictly parse and validate the legacy portable evidence record.
