# Infra Test Evidence independent verification handoff

## Result: FAIL

Candidate `2b4d93f44be5dfdf83d6fc6deb98a5d4e69b8a18` was independently
verified on 2026-08-28 UTC from fresh detached clones and against
https://infra-test-evidence.sociobot.in.

The live deployment is current: its root, hashed JavaScript/CSS, Privacy, and
Terms bytes exactly match the candidate production build, and its security and
immutable-cache policies are active. This is therefore **not** a
deployment-only failure.

Release is blocked by candidate defects:

- **P0:** a real OpenTofu 1.12.6 failing assertion involving a sensitive
  variable leaked the sentinel secret into both generated `evidence.json` and
  `index.html`, despite the default-redaction claim.
- **P1:** real `test_plan` variables/resource changes and assertion traversals
  are ignored; plan summaries and assertion paths are empty, case “inputs” are
  only filenames, and multiple failed runs receive the first diagnostic rather
  than their own.
- **P1:** partial/corrupt streams can return valid/exit 0 while an unsupported
  failed case and failing `test_summary` are silently dropped.
- **P1:** `npm run check` / `npm test` fail in fresh clones because the CLI
  integration test times out during its first Cargo build. The failure was
  reproduced twice; only a warm-cache rerun passes.
- **P1:** generated evidence pages have serious axe
  `scrollable-region-focusable` findings on evidence `<pre>` regions.
- **P2/P3:** negative durations and unsupported viewer statuses are accepted;
  Rust formatting and strict Clippy fail; `/favicon.ico` causes a Lighthouse
  console error; `.factory/brief.json` remains absent.

Full commands, evidence, hashes, passing coverage, and required repairs are in
`.factory/verification-3.md`.

## Passing gates

- `npm ci`, ESLint, TypeScript, `cargo test --locked` (3/3), audit, exact Vite
  build, `cargo package`, `npm pack --dry-run`, and clean consumer installation
  succeeded independently.
- Sequential `npm run qa:browser` passed 6/6 and `npm run qa:a11y` passed 2/2.
- Live desktop/390px flows, invalid-input recovery, keyboard focus, reduced
  motion, dark/light live axe scans, privacy/network checks, and response
  headers passed.
- Mobile Lighthouse: performance 100, accessibility 100, best practices 96,
  SEO 100; LCP 1.0s, TBT 50ms, CLS 0; initial JS/CSS are 3.44/5.49 kB raw.
- `/opt/fleet/lib/verify-url.sh` passed the live URL.

## Next step

Do not release this candidate. Repair the P0/P1 issues and add real
OpenTofu/Terraform regression fixtures before requesting another independent
verification. No product code was modified during this QA pass.
