# Demo sandbox

## Browser demo

- URL: `https://infra-test-evidence.sociobot.in/?demo=1` (redirects to the
  isolated `/demo/?demo=1` reader)
- Local URL after `npm run build && npm run preview`:
  `http://127.0.0.1:4173/?demo=1`
- Entry action: **Try it with sample data** on the landing page.
- Sample: a view of the bundled `examples/tofu-test.jsonl` conversion. It shows
  the failed `blocks_public_ingress` check, its assertion path, a `[REDACTED]`
  sensitive value, the source SHA-256, and `report.xml`, `evidence.json`, and
  the reviewer page before the file picker.
- Reset: **Reset demo** restores the in-memory sample.
- Exit: **Start for real** returns to `/` and discards the in-memory record.
- Storage namespace: none. The reader does not use localStorage, sessionStorage,
  IndexedDB, cookies, or a backend. Demo data exists only in page memory.

## CLI demo

Run:

```sh
infra-test-evidence --demo
```

The sample is compiled into the binary from `examples/tofu-test.jsonl`. Each
run creates a unique `infra-test-evidence-demo-*` directory under the operating
system temporary directory. It writes the sample input, `report.xml`,
`evidence/evidence.json`, and `evidence/index.html`, then prints those paths.

The landing page plays `public/cli-demo.cast`, a self-hosted asciinema v2
recording captured from a crate packaged and installed locally on 2026-09-02.
Only the unique temporary-directory suffix was shortened to an ellipsis. The
page includes a complete text transcript, pause/replay control, and an instant
full-output view when reduced motion is requested.
