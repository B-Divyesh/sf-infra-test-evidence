# Infra Test Evidence visual system

## Direction

**Field notebook / audit ledger.** Infrastructure evidence is often a noisy
log; this interface makes it read like a deliberate paper trail. Warm paper,
ink blue, and an offset "binder shadow" make the result feel reviewed rather
than dashboard-generic. This is a static utility, so its quiet editorial
layout defers to the record rather than inventing a chart-heavy control room.

## Tokens

| Role | Light | Dark |
| --- | --- | --- |
| Paper / background | `#f7f2e9` | `#111b22` |
| Ink / primary text | `#1d2933` | `#eef3f5` |
| Audit navy | `#193b58` | `#d9edf7` |
| Review coral | `#b63f31` | `#ff9e8f` |
| Pass moss | `#28654c` | `#9ee1bb` |

The system font stack is intentional: a tool that checks evidence should not
need to download typography before a user can inspect it. Monospace is reserved
for the wordmark, labels, and JSON.

## Layout and interaction

The page uses a 4/8px-derived rhythm, a wide editorial headline, then one
obvious action: choose or drop a JSON file. The mobile version deliberately
stacks the metadata and check duration; controls keep a 44px target. Result
status uses words in addition to color. A visible focus ring and skip link
serve keyboard users.

The only transition is the drop-zone state at 200ms. Under reduced motion all
transitions are effectively instant. There is no looping or decorative motion.

## Asset provenance

There are no images, icon fonts, remote fonts, trackers, or generated assets.
The down arrow and ledger marks are hand-authored text/CSS, keeping this local
utility fast and auditable.
