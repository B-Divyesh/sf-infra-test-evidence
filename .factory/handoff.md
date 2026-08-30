# Infra Test Evidence verification handoff — FAIL

**Verified candidate:** `bfea264291205b960f4cbcbc18f1396a4f1ad1ab`
**Live deployment:** https://infra-test-evidence.sociobot.in
**Report:** `.factory/verification-5.md`

## Result

**FAIL — do not release.** Fresh evidence proves that the live deployment is
the candidate, not an older/deployment-only build. The code checks, conversion,
packaging, browser tests, local-first network behavior, and repaired sensitive
redaction path pass. Two mandatory product-contract gates fail:

1. `.factory/claims.json` is missing, so no required claim tests exist or can
   be run from a clean demo entry point.
2. There is no contract-compliant sample demo: the landing page has only “Load
   a sample report,” `/demo` is 404, and `infra-test-evidence --demo` exits 64.

The cold first screen also does not plainly identify infrastructure-module
maintainers as its audience. Robots, sitemap, and a real 404 page are absent.

## What was verified

```sh
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
npm run consumer:check
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
npm audit --audit-level=high
```

All commands above pass. The production build is 1.63 kB gzip JS and 1.99 kB
gzip CSS. The live root, JS, CSS, Privacy, and Terms documents are
byte-identical to the candidate build. Live Playwright traffic stayed same
origin only; no console/page errors or axe serious/critical findings were
observed on desktop or 390px mobile. The explicit-sensitive fixture produced
redacted artifacts without the opaque sentinel.

## Next steps

Add the required claims manifest and tagged tests; implement the supplied
sample as `--demo` plus an isolated, bannered `/demo`/`?demo=1` site flow;
change the primary action to “Try it with sample data”; and name the target
maintainer audience on the first screen. Add robots, sitemap, and a designed
404 page, then request re-verification. No product source was changed during
this QA.
