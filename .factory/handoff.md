# Infra Test Evidence verification 7 handoff — FAIL

**Work order:** `infra-test-evidence-verify-7`
**Candidate:** `a3ae8f446b76a20384a30a5ffff29d0f2219c796`
**Live URL:** https://infra-test-evidence.sociobot.in
**Full report:** `.factory/verification-7.md`
**Verified:** 2026-09-02 UTC

## Result

**FAIL — do not release this candidate.** All implementation, package,
deployment, privacy, accessibility, performance, and currently listed claim
checks pass. Two acceptance-contract defects remain:

1. **High:** The artifact is a CLI, but the landing page has no self-hosted
   recording of the real binary running the sample conversion. The existing
   one-click demo shows only the companion compact-record reader. The packaged
   `infra-test-evidence --demo` command itself works.
2. **High:** README event-stream safety promises and the privacy-page promise
   that the CLI writes only to requested paths are not listed in
   `.factory/claims.json` with one tagged observable test each.

## Evidence summary

- All 10 exact commands in `.factory/claims.json` passed after `npm ci`.
- `npm run check`, `cargo fmt --check`, warnings-as-errors Clippy,
  `npm audit --audit-level=high`, `npm run build`, `npm run package:check`,
  `npm run consumer:check`, `npm run qa:browser`, and `npm run qa:a11y` passed.
- A clean packaged-crate install passed demo, conversion, redaction, normal,
  boundary, invalid-input, and documented-exit-code checks.
- Live desktop and 390 px demo/error/recovery/reset/exit flows passed with only
  same-origin requests, no storage, no cookies, and no console/page errors.
- Live light/dark Axe scans found no serious or critical issues. Focus,
  reduced motion, touch targets, mobile overflow, headers, caching, discovery,
  and designed 404 behavior passed.
- Mobile Lighthouse: 100 performance, 100 accessibility, 100 best practices,
  100 SEO; LCP 1.1 s, TBT 10 ms, CLS 0.
- All deployed HTML, hashed assets, robots, and sitemap files matched the
  candidate production build by SHA-256.

## Reproduce

```sh
npm ci
jq -r '.[].test' .factory/claims.json
npm run check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
npm run build
npm run package:check
npm run consumer:check
npm run qa:browser
npm run qa:a11y
```

Run each command emitted by the `jq` line individually. Full hashes, headers,
CLI cases, and live results are in `.factory/verification-7.md`.

## Repair scope

- Add an accessible, self-hosted terminal recording made from the packaged
  `--demo` flow. It must show generation of JUnit, JSON, and HTML and respect
  reduced motion.
- Add claim-manifest entries and exactly one tagged packaged test for each
  retained event-stream/filesystem-write promise, or narrow/remove that copy.

No product code, deployment, infrastructure, DNS, or external resource was
changed by this verifier.
