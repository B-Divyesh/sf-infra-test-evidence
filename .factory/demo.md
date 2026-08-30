# Demo sandbox

## Browser demo

- URL: `https://infra-test-evidence.sociobot.in/demo/`
- Local URL after `npm run build && npm run preview`:
  `http://127.0.0.1:4173/demo/`
- Entry action: **Try it with sample data** on the landing page.
- Sample: the `staging-2026-08-27.1` compact record with HTTP health and
  database migration checks.
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
