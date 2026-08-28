# Infra Test Evidence verification handoff — FAIL

**Tested candidate:** `812fa4a95dff0f1941f4256fc8f0e4b8c0a7a791`

**Live URL:** https://infra-test-evidence.sociobot.in

**Independent QA report:** `.factory/verification-4.md`

The candidate is **FAIL**. This is not a deployment-only failure: the live
root, hashed JS/CSS, Privacy, and Terms documents match the fresh production
build byte-for-byte.

## Blocking defect

**P0 — explicit sensitive values leak.** A valid `test_plan` with
`{"sensitive":true,"value":"k9M2qV7xL4"}` produces exit 0, then writes the
opaque value into both `evidence.json` and self-contained `index.html`.
Default plan redaction is a central product/privacy promise, so do not release
or share generated artifacts until this is fixed. The full reproducible input
and output locations are recorded in `verification-4.md`.

## What passed independently

- Fresh detached clone installation; `npm run check` (lint, types, 4 Rust and
  7 Vitest tests); exact `npm run build`; 8/8 browser checks; 2/2 axe
  serious/critical scans; Rust format/strict Clippy; and high-severity npm audit.
- `cargo package --locked`, `npm pack --dry-run`, and a clean staged-crate
  consumer install. The public CLI help, normal JUnit/evidence output, exit 2
  incomplete-stream handling, and exit 64 unknown-option handling work.
- Live currentness, Privacy/Terms, response policies, immutable hashed-asset
  caching, local-first landing page behavior, and Lighthouse mobile scores
  (100 performance/accessibility/best-practices/SEO; LCP 1.1 s, CLS 0).

## Next step

Make redaction semantic and recursive: honour `sensitive: true` and actual
OpenTofu/Terraform sensitive-value encodings before any JUnit, evidence JSON,
or HTML is generated. Add an opaque-sentinel end-to-end release-package test
that scans every output artifact, then request a new verification.

The factory owns publication/deployment credentials; no package was published
by this verifier. The researched `.factory/brief.json` is absent, so the
injected researched brief was used as the acceptance contract.
