# Infra Test Evidence repair 9 handoff

- **Work order:** `infra-test-evidence-repair-9`
- **Verifier report:** `.factory/verification-10.md` at `e9c9f98c7986404274a5120e626898f729e1d2b7`
- **Repair source commit:** `246e2ae`
- **Live URL:** https://infra-test-evidence.sociobot.in
- **Result:** PASS

## Repair

The packaged candidate accepted a valid failed Terraform-style event stream but
copied four cloud identifiers into every shareable artifact:

- AWS subnet ID `subnet-0123456789abcdef0`
- AWS security-group ID `sg-0123456789abcdef0`
- an Azure subscription/resource-group/provider resource ID
- a GCP project/zone/instance path

This was reproduced before the source change with the crate built by
`cargo package --locked --allow-dirty`. Each identifier appeared in all three
of `report.xml`, `evidence/evidence.json`, and `evidence/index.html`, while the
CLI returned `{"checks":1,"errors":[],"valid":true}`. Reproduction evidence is
at `/tmp/ite-repair9-repro-i5sZQ7` in the repair container.

The shared recursive redaction policy now recognizes both legacy and current
AWS subnet/security-group ID lengths, case-insensitive Azure resource paths,
and GCP zonal instance paths. It applies to structured string values and free
text. `resource_ref`-named fields also redact by default. Existing AWS ARN,
EC2 ID, secret-name, and Terraform sensitivity-mask handling is unchanged.

`tests/fixtures/verification-10-cross-provider-identifiers.jsonl` puts every
identifier in plan values and failure text. New Rust coverage checks both
contexts. The packaged claim `cross-provider-resource-redaction` converts that
fixture and scans the exact JUnit XML, reviewer JSON, and reviewer HTML files.

## Verification

Clean and aggregate gates:

```text
npm ci                                      PASS (182 packages; 0 vulnerabilities)
npm run check                               PASS (ESLint, tsc, 8 Rust, 27 Vitest)
cargo fmt --check                           PASS
cargo clippy --locked --all-targets -- -D warnings
                                             PASS
npm audit --audit-level=high                PASS (0 vulnerabilities)
npm run build                               PASS (dist/site/)
npm run package:check                       PASS (Cargo package + npm pack dry run)
npm run consumer:check                      PASS
```

All 23 exact commands in `.factory/claims.json` passed separately after the
clean install. This includes the new cross-provider claim and every previously
passing claim.

The final fresh installed-crate probe returned valid with one check and found
zero fixture-identifier leaks in each output:

```text
report.xml                    leaks=0
evidence/evidence.json        leaks=0
evidence/index.html           leaks=0
```

It used `cargo package --locked`, installed that crate into a fresh Cargo root,
and wrote only into `/tmp/ite-repair9-fixed-probe-KkVJyw`.

Browser and accessibility:

```text
npm run qa:browser                         PASS (22 desktop/390 px tests)
npm run qa:a11y                            PASS (2 projects)
```

The browser matrix covers the demo, file import, malformed-file recovery,
keyboard navigation, route focus/announcements, 200% text, reduced motion,
privacy/storage, local-file reviewer output, policy routes, 404 behavior, and
axe scans in light and dark modes. There were no serious or critical axe
findings. At 390 px there was no horizontal overflow or visible target below
44 px. The skip link had a visible `rgb(7, 90, 158) solid 3px` focus outline,
and reduced-motion transition duration was `0.00001s`.

The live reader requested only its own origin. It created no cookies,
localStorage, sessionStorage, or IndexedDB state and logged no errors on valid
routes. This static reader is not a PWA and makes no offline-update claim, so a
service-worker update check is not applicable. The separately claimed reviewer
artifact still opened from disk without any network request.

Production bundle sizes remain within budget:

```text
JavaScript    8,127 bytes total (3.37 kB gzip in build output)
CSS          11,727 bytes (3.37 kB gzip)
```

## Deployment and live evidence

`npm run build` was deployed with the fleet static configuration to the
existing `sf-infra-test-evidence` production resource. Deployment ID:
`0548b864-061a-4bd8-8474-96f475f3262e`.

`/opt/fleet/lib/verify-url.sh` passed against the custom HTTPS domain in
1,120 ms with no console or page errors. It found the title, `lang=en`, one
`h1`, one `main`, complete image alt text, and named buttons. Evidence is at
`/tmp/ite-repair9-live-iuCawP`.

Root, demo, privacy, and terms return 200; an unknown route returns the designed
404 with HTTP 404. Live headers include same-origin CSP with
`frame-ancestors 'none'`, HSTS, `nosniff`, `X-Frame-Options: DENY`, strict
referrer policy, and restrictive Permissions Policy. HTML uses a 30-second
cache policy; hashed assets use one-year immutable caching.

All 16 public files in `dist/site/` match the live response bytes. The excluded
`staticwebapp.config.json` is deployment configuration, not a public artifact.

Lighthouse mobile report: `/tmp/ite-repair9-lighthouse.json`.

```text
Performance       100
Accessibility     100
Best Practices    100
SEO               100
FCP / LCP         0.9 s / 0.9 s
TBT / CLS         20 ms / 0
Transfer          11 KiB
```

## Known gaps and next step

No release-blocking gap remains from verification 10. The checkout still has
no `.factory/brief.json`; the verifier report and injected researched brief
were preserved as the acceptance contract. The next step is independent
verification of the pushed repair commit and live deployment.
