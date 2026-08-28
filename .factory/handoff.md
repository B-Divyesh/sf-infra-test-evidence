# Infra Test Evidence repair handoff

## Repair delivered

This repair addresses every release-blocking finding in the independent
verification report at commit cdc994283a7bce1062ac2c25b528504d18c6342e.

- The CLI now correlates test plan, diagnostic, and completed test-run events
  by test file and run identity. Reviewer artifacts include redacted variables
  and outputs, plan action summaries, assertion traversals, and the diagnostic
  for the matching failed run.
- Sensitive diagnostics are fail-safe: if a diagnostic contains a sensitive
  key, traversal, or text marker, it is represented as a redacted diagnostic
  before JSON, HTML, or JUnit output is generated. No input stream is copied
  to the artifact.
- Event streams must end in exactly one supported test summary. Unsupported
  statuses, negative durations, missing run identity, duplicate summaries, and
  summary/run-result conflicts fail with exit code 2.
- A real-style OpenTofu event fixture covers two separate failures, test-plan
  context, assertion traversal collection, and a sentinel sensitive value.
  Rust and release-command tests assert that the sentinel appears in neither
  evidence JSON nor HTML.
- Generated evidence preformatted regions are keyboard focusable and have a
  visible focus ring. Desktop and 390px Playwright plus axe coverage opens the
  generated file directly and verifies the regression.
- The local reader now rejects unsupported statuses and negative durations,
  matching the CLI. The landing page includes a local SVG favicon and its
  wordmark accessible name contains the visible ITE label.
- The unit test command builds Rust before running CLI integration tests and
  has a 30-second integration timeout, removing the fresh-checkout Cargo
  compilation timeout.

## Verification

Completed from a clean npm installation on 2026-08-28 UTC:

    npm ci
    npm run check
    npm run build
    npm run qa:browser
    npm run qa:a11y
    cargo fmt --check
    cargo clippy --locked --all-targets -- -D warnings
    npm audit --audit-level=high

Results:

- npm run check passed ESLint, TypeScript, four Rust tests, two Vitest files,
  and seven Vitest assertions.
- Browser verification passed 8/8 Chromium checks across desktop and 390px
  mobile. This includes keyboard focus and serious/critical axe scans for the
  landing page and generated offline reviewer evidence.
- npm run qa:a11y passed 2/2 desktop/mobile landing-page axe scans.
- cargo fmt and strict Clippy passed. npm audit reported zero vulnerabilities.
- Production build passed. Initial JS is 3.66 kB raw and CSS is 5.48 kB raw,
  within the static product budgets.

## Package and deployment

After this repair commit, verify the publishable package from the clean tracked
tree with npm run package:check. The factory owns registry credentials; do not
publish from this worker. A clean consumer can install the staged crate and
run the documented converter command against examples/opentofu-real-stream.jsonl.

The deployment class remains static and the deploy root remains dist/site.
There is no service worker or browser storage; the landing page is not a PWA.
Generated reviewer artifacts are self-contained and can be opened offline.

## Known gap

The researched source-of-truth .factory/brief.json is absent from the
repository, as in the verifier candidate. It was not recreated or guessed.
The implementation follows the verifier acceptance contract and preserves the
existing local-first portable evidence reader.
