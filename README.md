# fast_qr wasm workspace

This workspace is trimmed around the `fast-qr-wasm` crate. The parent `fast_qr`
crate now provides only the QR matrix builder and simple SVG renderer needed by
that wrapper; PNG encoding and JavaScript bindings live in `fast-qr-wasm`.

## Build

```bash
cargo test --workspace
cargo build --target wasm32-unknown-unknown -p fast-qr-wasm --release
```

For packaged web output, build the wasm crate with `wasm-pack`:

```bash
cd fast-qr-wasm
wasm-pack build --target web --release
```

## JavaScript API

```js
import init, {
  generate_qr_svg,
  generate_qr_png,
  generate_qr_matrix,
} from './pkg/fast_qr_wasm.js';

await init();

const data = new TextEncoder().encode('https://fast-qr.com');
const svg = generate_qr_svg(data, 4, 'M', false, 256, 256);
const png = generate_qr_png(data, 512, 4, 'M', false);
const matrix = generate_qr_matrix(data, 4, 'M', false);
```

`ecl` accepts `"L"`, `"M"`, `"Q"`, or `"H"`. Set `force_byte_mode` to `true`
when encoding arbitrary binary payloads.
