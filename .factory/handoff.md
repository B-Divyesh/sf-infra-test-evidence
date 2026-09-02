# Infra Test Evidence — polish 3 handoff

## Shipped result

- **Release repair commit:** `e45ce587daaeb212e7c4299c412fd68f9a578bcc`
- **Production deployment:** `2decafa3-46b9-4278-8299-743e373cfb2a`
- **Live URL:** https://infra-test-evidence.sociobot.in
- **Result:** PASS — review-3’s two blocking and two minor findings, plus every earlier finding, are closed and rechecked.

The repair standardizes the three output names in runtime status text, uses
**compact record** for the browser input everywhere, replaces vague
“provenance” with the input SHA-256, and replaces README “assertion
traversals” with assertion paths. It also expands the copy regression to cover
runtime TypeScript, demo/privacy pages, README, claims, recording, and demo
documentation; adds a browser check for direct `?demo=1`; and verifies the
canonical labels and dynamic file error in a browser.

The catalog description is now a verb-first 90-character sentence in
`.factory/catalog-description.txt`.

## Exact verification evidence

Clean clone: `/tmp/infra-test-evidence-polish3-clean-RxQCei/repo`, created by
`git clone --no-local /work/repo` at `e45ce587daaeb212e7c4299c412fd68f9a578bcc`.
After `npm ci` (zero audit vulnerabilities), every one of the 24 exact
commands in `.factory/claims.json` passed separately. The clone also passed:

```sh
npm run check        # 8 Rust tests + 28 frontend tests
npm run build        # dist/site/, 6.78 kB raw / 2.70 kB gzip initial JS
npm run qa:browser   # 26 tests
npm run qa:a11y      # 2 Axe projects
npm run package:check
```

No offline claim is published, so no offline behavior is promised. The
applicable privacy contract is tested by `@claim:reader-private`: no upload,
external request, cookies, localStorage, sessionStorage, or IndexedDB during
the demo/import flow.

The deployment used:

```sh
/opt/fleet/lib/deploy-static.sh infra-test-evidence dist/site
```

Cold factory URL verification passed without console errors:

- Root: 815 ms; evidence `/tmp/infra-test-evidence-polish3-live-root-TnqRC6/`
- Demo: 595 ms; evidence `/tmp/infra-test-evidence-polish3-live-demo-AGD7Dw/`
- Browser, route, privacy, focus, demo-reset, and live Axe evidence:
  `/tmp/infra-test-evidence-polish3-live-browser-S0rSti/live-check.json`
- Mobile/desktop screenshots:
  `/tmp/infra-test-evidence-polish3-live-browser-S0rSti/landing-390.png`,
  `landing-1440.png`, `demo-390.png`, and `demo-1440.png`

The final cold check found all routes healthy: `/`, `/demo/?demo=1`,
`/privacy/`, and `/terms/` return 200; `/not-found` returns the designed 404.
Each has one h1 and main landmark, route title, canonical metadata, and zero
serious or critical Axe findings. The live root, demo, privacy, terms, and 404
were checked for titles/routing; all internal links resolve. The live root and
demo emitted no console or page errors. The expected 404 document response is
not treated as an application console error.

`/demo/?demo=1` remains an isolated one-click sample: it shows the failed
OpenTofu proof, redaction, assertion path, input hash, and three output paths
above the fold. Reset restores only the in-memory sample; Start for real
returns to the empty real reader.

## Run and deploy

```sh
npm ci
npm run check
npm run build
npm run qa:browser
npm run qa:a11y
npm run package:check
```

Deploy `dist/site/` with the factory static deploy command above. Build the
CLI locally with `cargo build --release --locked`; do not publish it from this
worktree.

## Known gaps and next steps

None. The product intentionally has no AI or sync feature: deterministic local
conversion and redaction are the audit boundary, and remote model or sync
traffic would weaken its stated privacy behavior.
