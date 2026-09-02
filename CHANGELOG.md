# Changelog

## 0.1.0 repair 9

- Redact AWS subnet and security-group IDs, Azure resource IDs, and GCP
  instance paths in structured values and free text.
- Scan all three generated output files in one packaged cross-provider claim.

## 0.1.0 repair 7

- Add an accessible, self-hosted recording of the packaged CLI demo.
- Enforce a final event-stream summary and retain plan evidence per test run.
- Add packaged claim tests for event validation, correlation, diagnostic redaction, and requested-path writes.

## 0.1.0 repair 6

- Make every Playwright claim command build its own production site from a clean checkout.
- Prove the converter never launches infrastructure tools or opens a network socket.
- Keep both demo-banner controls at least 44 px tall and cover them in browser QA.

## 0.1.0

- Add a packaged `--demo` flow that writes bundled sample outputs to a unique
  temporary directory.
- Add the one-click browser demo, claims suite, discovery files, and designed
  static 404 response.
- Honor explicit OpenTofu/Terraform sensitivity metadata and fail closed on
  malformed sensitivity masks before creating reviewer artifacts.
- Convert OpenTofu/Terraform JSON event streams to JUnit XML.
- Generate a self-contained reviewer page, evidence JSON, and an input hash.
- Strictly parse and validate the compact record.
