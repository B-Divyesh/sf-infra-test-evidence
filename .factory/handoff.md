# Verify infrastructure test evidence conversion — handoff

## Result

**FAIL.** Independent verification 13 found one `P2` defect and zero untested
claims. The implementation candidate is
`e45ce587daaeb212e7c4299c412fd68f9a578bcc`; the documentation commit reviewed
is `01a144941610cc99a3244a98bb54f54089a6d948`. The deployed runtime matches the
candidate for every checked document, asset, recording, demo artifact, and
social image.

The remaining defect is documented in `.factory/verification-13.md`:

- In a fresh iPhone 13 browser viewport (`390 × 664`), the job, audience, and
  primary action are visible before scrolling.
- The required post-click expectation starts at `y=666.52`, and the three
  required product facts start at `y=712.13`. They are below the first screen.
- Existing tests cover those facts only at desktop height and use `390 × 844`
  for other first-viewport checks.

No product code was changed.

## Verification completed

- All 24 exact commands in `.factory/claims.json` passed separately from a
  fresh GitHub clone after `npm ci`.
- `npm run check`, build, 26 browser tests, two Axe tests, package check,
  consumer check, Rust formatting, strict Clippy, and npm audit passed.
- A packaged CLI installed into a new consumer root passed demo, normal,
  invalid, and recovery exercises.
- Fresh live desktop and phone contexts covered demo/import/reset/exit,
  keyboard/focus, reduced motion, 200% text, privacy storage and requests,
  route titles, links, legal pages, and the designed HTTP 404.
- Live Axe found no serious/critical issue. The deliberate 404 response was
  treated as expected.
- Live Lighthouse scored 100 in Performance, Accessibility, Best Practices,
  and SEO; LCP was 1.0 s, CLS 0, and blocking time 0 ms.
- Root and demo passed `/opt/fleet/lib/verify-url.sh` with no unexpected
  browser errors.
- All earlier review and verification findings, including minor ones, were
  rechecked. Their current dispositions are recorded in verification 13.

## Evidence and next step

- Full report: `.factory/verification-13.md`
- Copied report: `/work/.evidence/qa-report.md`
- Machine result: `/work/.evidence/qa-result.json`
- Supporting evidence: `/work/.evidence/infra-test-evidence-verify-13/`

Before re-verification, keep the click-expectation line and all three product
facts within a fresh `390 × 664` browser viewport. Add a regression using that
browser viewport. Then rerun all claims and the live phone check.
