# fast_qr

QR code generator for Rust, the browser, and the command line. Encodes
text or arbitrary bytes and renders to SVG, PNG, a raw module matrix, or
a terminal-friendly string.

## Crates

- `fast-qr-reworked`: the QR matrix builder and SVG renderer. Use this
  from Rust.
- `fast-qr-wasm`: `wasm-bindgen` wrapper that exposes SVG, PNG, and raw
  module-matrix output to JavaScript. Distributed as a `.tgz` attached
  to each [GitHub release](https://github.com/andrewtheguy/fast_qr/releases)
  (not published to the npm registry).
- `fast-qr-cli`: command-line encoder that writes SVG, PNG, or a
  terminal rendering to a file or stdout.

## Using `fast-qr-reworked` from Rust

`Cargo.toml`:

```toml
[dependencies]
fast-qr-reworked = { git = "https://github.com/andrewtheguy/fast_qr" }
```

`src/main.rs`:

```rust
use fast_qr_reworked::convert::svg::SvgBuilder;
use fast_qr_reworked::convert::Builder;
use fast_qr_reworked::QRBuilder;

fn main() {
    let qr = QRBuilder::new("https://example.com/").build().unwrap();
    let svg = SvgBuilder::default().margin(4).to_str(&qr);
    println!("{svg}");
}
```

## Using `@andrewtheguy/fast-qr-wasm` from JavaScript

The wasm package is not on the npm registry. Install from the `.tgz`
asset attached to a
[GitHub release](https://github.com/andrewtheguy/fast_qr/releases):

```sh
npm install https://github.com/andrewtheguy/fast_qr/releases/download/v0.13.1/andrewtheguy-fast-qr-wasm-0.13.1.tgz
```

Or pin it in `package.json` so `npm ci` is reproducible:

```json
{
  "dependencies": {
    "@andrewtheguy/fast-qr-wasm": "https://github.com/andrewtheguy/fast_qr/releases/download/v0.13.1/andrewtheguy-fast-qr-wasm-0.13.1.tgz"
  }
}
```

The tarball is the exact `wasm-pack` output (the contents of
`fast-qr-wasm/pkg/`); npm unpacks it under the scoped package name from
its internal `package.json`, so imports keep using
`@andrewtheguy/fast-qr-wasm`:

```js
import init, {
  generate_qr_svg,
  generate_qr_png,
  generate_qr_matrix,
} from "@andrewtheguy/fast-qr-wasm";

await init();

const data = new TextEncoder().encode("https://fast-qr.com");
const svg    = generate_qr_svg(data, 4, "M", "auto", 256, 256);
const png    = generate_qr_png(data, 512, 4, "M", "auto");
const matrix = generate_qr_matrix(data, 4, "M", "auto");
```

`ecl` accepts `"L"`, `"M"`, `"Q"`, or `"H"`. `mode` accepts `"auto"`,
`"numeric"`, `"alphanumeric"`, or `"byte"`. `auto` picks the most compact
encoding for the payload; pass `"byte"` for arbitrary binary data. See
`fast-qr-wasm/src/lib.rs` for the full argument semantics.

## Using `fast-qr-cli` from the shell

Build and run the CLI from this workspace:

```sh
cargo run --release -p fast-qr-cli -- "https://example.com/" -o qr.svg
cargo run --release -p fast-qr-cli -- "https://example.com/" -o qr.png --scale 10
cargo run --release -p fast-qr-cli -- "https://example.com/" --format terminal
```

The format is inferred from the `--output` extension (`.svg` or `.png`)
and defaults to SVG when writing to stdout. Pass `--input -` to read raw
bytes from stdin, `--ecl {L,M,Q,H}` to set the error correction level,
`--mode {auto,numeric,alphanumeric,byte}` to pin the encoding mode
(defaults to `auto`; pass `byte` for arbitrary binary payloads), and
`--qr-version N` (1-40) to force a specific version. Run
`fast-qr-cli --help` for the full argument list.

## Build and test

```sh
cargo clippy --workspace --all-targets
cargo test  --workspace --release

cd fast-qr-wasm
npm install
npm run build
```

`npm run build` invokes the `wasm-pack` version pinned in
`fast-qr-wasm/package-lock.json` and writes package artifacts to
`fast-qr-wasm/pkg/`. Always invoke `wasm-pack` through `npm`/`npx` so the
pinned version is used; do not call a system-wide cargo-installed binary.

## Upstream

Forked from [`erwanvivien/fast_qr`](https://github.com/erwanvivien/fast_qr).
The encoder paths and rendering targets unused by `fast-qr-wasm` have
been removed; everything else descends from upstream.

## License

MIT. See `LICENSE`. Originally Copyright (c) 2023 Erwan VIVIEN; modifications
by Andrew under the same MIT terms.
