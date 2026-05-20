//! Shared helpers for end-to-end roundtrip tests.
//!
//! Pipeline:
//!   QRBuilder -> QRCode
//!     |-- render_qr_to_png_bytes -> PNG bytes -> decode_png_bytes -> Vec<Vec<u8>>
//!     `-- SvgBuilder::to_str     -> SVG str   -> rasterize_svg_to_rgba -> decode_rgba

use std::io::Cursor;
use std::path::PathBuf;

use fast_qr_reworked::convert::svg::SvgBuilder;
use fast_qr_reworked::convert::Builder;
use fast_qr_reworked::{QRBuilder, QRCode};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageReader};

/// Render `qr` to a single-channel grayscale PNG.
///
/// Each module is scaled to `scale`x`scale` pixels and a `margin`-module quiet
/// zone is added on all sides. Dark module -> 0, light module -> 255.
pub fn render_qr_to_png_bytes(qr: &QRCode, scale: u32, margin: u32) -> Vec<u8> {
    let modules = qr.size as u32;
    let side = (modules + 2 * margin) * scale;
    let side_us = side as usize;
    let mut buf = vec![255u8; side_us * side_us];

    for y in 0..modules {
        for x in 0..modules {
            if qr[y as usize][x as usize].value() {
                let px = (x + margin) * scale;
                let py = (y + margin) * scale;
                for dy in 0..scale {
                    let row = ((py + dy) as usize) * side_us;
                    for dx in 0..scale {
                        buf[row + (px + dx) as usize] = 0;
                    }
                }
            }
        }
    }

    let mut out = Vec::with_capacity(buf.len() / 4);
    PngEncoder::new(&mut out)
        .write_image(&buf, side, side, ExtendedColorType::L8)
        .expect("png encode");
    out
}

/// Rasterize an SVG string at `px_per_module` pixels per SVG user unit.
///
/// `SvgBuilder` emits a viewBox of `(qr.size + 2 * margin)` units; one unit per
/// module. The returned RGBA buffer is premultiplied (alpha is uniformly 255
/// for our opaque black-on-white renders, so this is a no-op for now).
pub fn rasterize_svg_to_rgba(svg: &str, px_per_module: u32) -> (Vec<u8>, u32, u32) {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg, &opt).expect("parse svg");
    let int_size = tree.size().to_int_size();
    let w = int_size.width() * px_per_module;
    let h = int_size.height() * px_per_module;

    let mut pixmap = tiny_skia::Pixmap::new(w, h).expect("pixmap alloc");
    // SVG's <rect> covers the canvas, but an unfilled pixmap can leave
    // alpha-zero pixels at the edges that confuse rxing's binarizer.
    pixmap.fill(tiny_skia::Color::WHITE);

    let s = px_per_module as f32;
    let xform = tiny_skia::Transform::from_scale(s, s);
    resvg::render(&tree, xform, &mut pixmap.as_mut());

    (pixmap.data().to_vec(), w, h)
}

/// Decode an in-memory PNG to RGBA and run the rxing-reader QR pipeline.
pub fn decode_png_bytes(png: &[u8]) -> Vec<Vec<u8>> {
    let img = ImageReader::new(Cursor::new(png))
        .with_guessed_format()
        .expect("guess png format")
        .decode()
        .expect("decode png")
        .into_rgba8();
    let (w, h) = (img.width(), img.height());
    decode_rgba(&img.into_raw(), w, h)
}

/// Run an RGBA buffer through the rxing-reader QR pipeline.
///
/// Matches the production rxing-wasm policy: try_harder / try_rotate /
/// try_invert all off, `max = 1` (one symbol per image).
pub fn decode_rgba(rgba: &[u8], w: u32, h: u32) -> Vec<Vec<u8>> {
    let luma = rxing_reader::rgba_to_luma(rgba, w, h).expect("rgba_to_luma");
    rxing_reader::decode_qr_codes_luma(&luma, w, h, false, false, false, 1).expect("decode")
}

/// Table-driven roundtrip: build the QR, exercise both PNG and rasterized-SVG
/// paths, assert each decodes back to the original `input` bytes.
///
/// Accepts `&[u8]` so binary payloads (with `Mode::Byte` forced) round-trip
/// the same way text does.
pub fn assert_roundtrip(label: &str, input: &[u8], build: impl Fn(&mut QRBuilder)) {
    let mut b = QRBuilder::new(input.to_vec());
    build(&mut b);
    let qr = b.build().expect("qr build");

    // PNG path: 8 px/module, 4-module quiet zone.
    let png = render_qr_to_png_bytes(&qr, 8, 4);
    let decoded = decode_png_bytes(&png);
    assert_eq!(
        decoded.len(),
        1,
        "{label}: PNG path produced {} decodes (expected 1)",
        decoded.len()
    );
    assert_eq!(decoded[0], input, "{label}: PNG payload mismatch");

    // SVG path: SvgBuilder default margin = 4 (already), rasterize at 8 px/module.
    let svg = SvgBuilder::default().margin(4).to_str(&qr);
    let (rgba, w, h) = rasterize_svg_to_rgba(&svg, 8);
    let decoded = decode_rgba(&rgba, w, h);
    assert_eq!(
        decoded.len(),
        1,
        "{label}: SVG path produced {} decodes (expected 1)",
        decoded.len()
    );
    assert_eq!(decoded[0], input, "{label}: SVG payload mismatch");

    dump_artifacts(label, &png, &svg);
}

/// Dump PNG and SVG artifacts under `CARGO_TARGET_TMPDIR/e2e/` for debugging.
pub fn dump_artifacts(label: &str, png: &[u8], svg: &str) {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("e2e");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join(format!("{label}.png")), png);
    let _ = std::fs::write(dir.join(format!("{label}.svg")), svg);
}
