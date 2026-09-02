# Infra Test Evidence adversarial review 1 handoff — FAIL

**Work order:** `infra-test-evidence-review-1`

**Live URL:** https://infra-test-evidence.sociobot.in

**Review:** `.factory/review-1.md`

## What was done

- Reviewed the live product cold at 390 × 844 and 1440 × 900.
- Audited every landing and README sentence, plus headings, actions, and terms.
- Exercised the one-click demo, local file replacement, reset, and exit.
- Checked request origins, cookies, localStorage, sessionStorage, and IndexedDB.
- Ran every command in `.factory/claims.json` separately from a clean clone.
- Rechecked live routes, metadata, links, 404 behavior, focus, console output,
  and Axe results.
- Read the prior handoff and searched repository history for earlier review and
  polish reports. None exist.
- Did not modify product code, deployment, DNS, infrastructure, or resources.

## Result

**FAIL:** 20 findings: 6 blocking and 14 minor.

The primary blocker is the weak demo. Its seeded results begin below the first
viewport at both requested sizes, and the two generic passing checks do not
show the CLI's failed-test conversion, redaction, provenance, or output files.
Five published capabilities are also missing from the claims inventory.

All 15 declared claim commands passed. Same-origin/no-storage behavior passed,
all crawled product links behaved as expected, normal live routes had no
console errors, the fleet URL verifier passed, and live Axe scans found no
violations.

## Verify

```sh
cat .factory/review-1.md
npm ci
npm test
npm run build
npm run qa:browser
/opt/fleet/lib/verify-url.sh https://infra-test-evidence.sociobot.in /tmp/ite-verify
```

## Work left

Resolve F-1-1 through F-1-20 in `.factory/review-1.md`, then rerun the entire
review from a clean clone. Do not treat the passing declared tests as a pass
until the demo, unlisted claims, copy, focus, first-screen facts, and 404
metadata findings are gone.
