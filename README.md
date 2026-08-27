# Infra Test Evidence

Infra Test Evidence is a small Rust CLI and static, local-first viewer for a
portable record of an infrastructure test run. It is for the engineer who needs
to hand a reviewer an inspectable summary without uploading test output to a
third party.

The viewer is available at https://infra-test-evidence.sociobot.in.

## CLI usage

Build the binary, then validate a record:

```sh
cargo run -- --json examples/passing-evidence.json
# {"valid":true,"checks":2,"errors":[]}
```

The record needs string `run`, `environment`, and `recordedAt` fields and one
or more objects in `checks`. `--json` makes the result suitable for CI. The
CLI exits 0 for a valid record, 2 for an invalid or unreadable record, and 64
for incorrect usage. It never sends data over the network.

## Develop and verify

```sh
npm ci
npm run check
npm run build:site       # creates dist/site/index.html
npm run qa:browser       # desktop + mobile and axe smoke test
cargo package --locked   # ready-to-publish Rust package check
```

Run `npm run dev` for the viewer. Deploy `dist/site/` as a static site.

## License

MIT. See [LICENSE](LICENSE).
