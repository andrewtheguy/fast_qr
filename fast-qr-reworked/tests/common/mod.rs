//! Shared helpers for end-to-end roundtrip tests.
//!
//! Pipeline:
//!   QRBuilder -> QRCode
//!     |-- render_qr_to_luma     -> luma bytes -> decode_luma         -> Vec<Vec<u8>>
//!     |-- render_qr_to_png_bytes -> PNG bytes -> decode_png_bytes    -> Vec<Vec<u8>>
//!     `-- SvgBuilder::to_str     -> SVG str   -> rasterize_svg_to_rgba -> decode_rgba

// Each integration test binary pulls in this module, so any helper used by
// only one of them looks dead to the others.
#![allow(dead_code)]

use std::io::Cursor;
use std::path::PathBuf;

use fast_qr_reworked::convert::svg::SvgBuilder;
use fast_qr_reworked::convert::Builder;
use fast_qr_reworked::{ECL, Mask, Mode as EncodeMode, QRBuilder, QRCode, Version};

use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageReader};
use rxing_reader::{
    AIFlag, Eci, ErrorCorrectionLevel, Mode as DecodeMode, QrSymbol,
};

const ROUNDTRIP_SCALE: usize = 4;
const ROUNDTRIP_MARGIN: usize = 4;

pub struct RoundTripCase {
    pub label: &'static str,
    pub input: Vec<u8>,
    mode: EncodeMode,
    ecl: ECL,
    version: Version,
    mask: Mask,
}

impl RoundTripCase {
    fn new(
        label: &'static str,
        input: impl Into<Vec<u8>>,
        mode: EncodeMode,
        ecl: ECL,
        version: Version,
        mask: Mask,
    ) -> Self {
        Self {
            label,
            input: input.into(),
            mode,
            ecl,
            version,
            mask,
        }
    }

    fn build_qr(&self) -> QRCode {
        let mut builder = QRBuilder::new(self.input.clone());
        builder
            .mode(self.mode)
            .ecl(self.ecl)
            .version(self.version)
            .mask(self.mask);
        builder.build().expect("qr build")
    }

    fn expected_version(&self) -> u32 {
        self.version as u32 + 1
    }

    fn expected_module_count(&self) -> usize {
        self.expected_version() as usize * 4 + 17
    }

    fn expected_error_correction_level(&self) -> ErrorCorrectionLevel {
        match self.ecl {
            ECL::L => ErrorCorrectionLevel::L,
            ECL::M => ErrorCorrectionLevel::M,
            ECL::Q => ErrorCorrectionLevel::Q,
            ECL::H => ErrorCorrectionLevel::H,
        }
    }

    fn expected_mode(&self) -> DecodeMode {
        match self.mode {
            EncodeMode::Numeric => DecodeMode::Numeric,
            EncodeMode::Alphanumeric => DecodeMode::Alphanumeric,
            EncodeMode::Byte => DecodeMode::Byte,
        }
    }

    fn expected_eci(&self) -> Eci {
        match self.mode {
            EncodeMode::Numeric | EncodeMode::Alphanumeric => Eci::ISO8859_1,
            EncodeMode::Byte => Eci::Unknown,
        }
    }
}

pub fn roundtrip_cases() -> Vec<RoundTripCase> {
    let full_range_payload: Vec<u8> = (0u8..=255u8).collect();
    vec![
        RoundTripCase::new(
            "numeric_l_v01_mask0",
            b"01234567".as_slice(),
            EncodeMode::Numeric,
            ECL::L,
            Version::V01,
            Mask::Checkerboard,
        ),
        RoundTripCase::new(
            "alphanumeric_m_v02_mask1",
            b"FAST QR-42".as_slice(),
            EncodeMode::Alphanumeric,
            ECL::M,
            Version::V02,
            Mask::HorizontalLines,
        ),
        RoundTripCase::new(
            "byte_q_v03_mask2",
            b"byte/q/v03/\x00\xff".as_slice(),
            EncodeMode::Byte,
            ECL::Q,
            Version::V03,
            Mask::VerticalLines,
        ),
        RoundTripCase::new(
            "byte_h_v04_mask3",
            b"byte h v04 mask3".as_slice(),
            EncodeMode::Byte,
            ECL::H,
            Version::V04,
            Mask::DiagonalLines,
        ),
        RoundTripCase::new(
            "numeric_m_v05_mask4",
            b"3141592653589793238462643383279".as_slice(),
            EncodeMode::Numeric,
            ECL::M,
            Version::V05,
            Mask::LargeCheckerboard,
        ),
        RoundTripCase::new(
            "alphanumeric_q_v06_mask5",
            b"MASK 5 ALPHANUMERIC QR".as_slice(),
            EncodeMode::Alphanumeric,
            ECL::Q,
            Version::V06,
            Mask::Fields,
        ),
        RoundTripCase::new(
            "byte_l_v07_mask6",
            b"byte/l/v07/mask6\x10\x11".as_slice(),
            EncodeMode::Byte,
            ECL::L,
            Version::V07,
            Mask::Diamonds,
        ),
        RoundTripCase::new(
            "byte_h_v08_mask7",
            b"byte h v08 mask7".as_slice(),
            EncodeMode::Byte,
            ECL::H,
            Version::V08,
            Mask::Meadow,
        ),
        RoundTripCase::new(
            "byte_m_v09_mask4_url",
            b"https://example.com/search?q=fast_qr&lang=en".as_slice(),
            EncodeMode::Byte,
            ECL::M,
            Version::V09,
            Mask::LargeCheckerboard,
        ),
        RoundTripCase::new(
            "byte_l_v10_mask7_full_range",
            full_range_payload,
            EncodeMode::Byte,
            ECL::L,
            Version::V10,
            Mask::Meadow,
        ),
        RoundTripCase::new(
            "alphanumeric_m_v27_mask1",
            b"VERSION 27 ALPHA MODE".as_slice(),
            EncodeMode::Alphanumeric,
            ECL::M,
            Version::V27,
            Mask::HorizontalLines,
        ),
        RoundTripCase::new(
            "byte_q_v40_mask2",
            b"version 40 byte fixture from fast_qr".as_slice(),
            EncodeMode::Byte,
            ECL::Q,
            Version::V40,
            Mask::VerticalLines,
        ),
    ]
}

/// Build a single-channel luma buffer (dark module -> 0, light -> 255) directly
/// from the QR module matrix.
///
/// Each module is scaled to `scale`x`scale` pixels and a `margin`-module quiet
/// zone is added on all sides. Used by the raw-matrix roundtrip test to bypass
/// PNG/SVG rendering entirely.
pub fn render_qr_to_luma(qr: &QRCode, scale: usize, margin: usize) -> (Vec<u8>, usize, usize) {
    let modules = qr.size;
    let side = (modules + 2 * margin) * scale;
    let mut buffer = vec![255u8; side * side];

    for y in 0..modules {
        for x in 0..modules {
            if qr[y][x].value() {
                let px = (x + margin) * scale;
                let py = (y + margin) * scale;
                for dy in 0..scale {
                    let row = (py + dy) * side;
                    for dx in 0..scale {
                        buffer[row + px + dx] = 0;
                    }
                }
            }
        }
    }

    (buffer, side, side)
}

/// Render `qr` to a single-channel grayscale PNG.
///
/// Each module is scaled to `scale`x`scale` pixels and a `margin`-module quiet
/// zone is added on all sides. Dark module -> 0, light module -> 255.
pub fn render_qr_to_png_bytes(qr: &QRCode, scale: usize, margin: usize) -> Vec<u8> {
    let (buffer, width, height) = render_qr_to_luma(qr, scale, margin);
    let mut out = Vec::new();
    let width = u32::try_from(width).expect("png width fits in u32");
    let height = u32::try_from(height).expect("png height fits in u32");
    PngEncoder::new(&mut out)
        .write_image(&buffer, width, height, ExtendedColorType::L8)
        .expect("png encode");
    out
}

/// Rasterize an SVG string at `px_per_module` pixels per SVG user unit.
///
/// `SvgBuilder` emits a viewBox of `(qr.size + 2 * margin)` units; one unit per
/// module. The returned RGBA buffer is premultiplied (alpha is uniformly 255
/// for our opaque black-on-white renders, so this is a no-op for now).
pub fn rasterize_svg_to_rgba(svg: &str, px_per_module: usize) -> (Vec<u8>, usize, usize) {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg, &opt).expect("parse svg");
    let int_size = tree.size().to_int_size();
    let scale = u32::try_from(px_per_module).expect("svg scale fits in u32");
    let width = int_size.width() * scale;
    let height = int_size.height() * scale;

    let mut pixmap = tiny_skia::Pixmap::new(width, height).expect("pixmap alloc");
    // SVG's <rect> covers the canvas, but an unfilled pixmap can leave
    // alpha-zero pixels at the edges that confuse rxing's binarizer.
    pixmap.fill(tiny_skia::Color::WHITE);

    let scale = px_per_module as f32;
    let xform = tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, xform, &mut pixmap.as_mut());

    (pixmap.data().to_vec(), width as usize, height as usize)
}

/// Decode an in-memory PNG to RGBA and run the rxing-reader QR pipeline.
pub fn decode_png_bytes(png: &[u8]) -> Vec<QrSymbol> {
    let img = ImageReader::new(Cursor::new(png))
        .with_guessed_format()
        .expect("guess png format")
        .decode()
        .expect("decode png")
        .into_rgba8();
    let (width, height) = (img.width() as usize, img.height() as usize);
    decode_rgba(&img.into_raw(), width, height)
}

/// Run an RGBA buffer through the rxing-reader QR pipeline.
///
/// Matches the production rxing-wasm policy: try_harder / try_rotate /
/// try_invert all off, `max = 1` (one symbol per image).
pub fn decode_rgba(rgba: &[u8], width: usize, height: usize) -> Vec<QrSymbol> {
    let luma = rxing_reader::rgba_to_luma(rgba, width, height).expect("rgba_to_luma");
    rxing_reader::decode_qr_codes_luma(&luma, width, height, false, false, false, 1)
        .expect("decode")
}

/// Run a luma buffer directly through the rxing-reader QR pipeline.
pub fn decode_luma(luma: &[u8], width: usize, height: usize) -> Vec<QrSymbol> {
    rxing_reader::decode_qr_codes_luma(luma, width, height, false, false, false, 1)
        .expect("decode")
}

fn assert_decoded_symbol(symbols: &[QrSymbol], roundtrip_case: &RoundTripCase, path: &str) {
    assert_eq!(
        symbols.len(),
        1,
        "{} {path}: produced {} decodes (expected 1)",
        roundtrip_case.label,
        symbols.len()
    );
    let symbol = &symbols[0];
    assert_eq!(
        symbol.bytes, roundtrip_case.input,
        "{} {path}: payload mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.version,
        roundtrip_case.expected_version(),
        "{} {path}: version mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.error_correction_level,
        roundtrip_case.expected_error_correction_level(),
        "{} {path}: EC level mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.mask,
        roundtrip_case.mask as u8,
        "{} {path}: mask mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.modes,
        vec![roundtrip_case.expected_mode()],
        "{} {path}: mode metadata mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.ecis,
        vec![roundtrip_case.expected_eci()],
        "{} {path}: ECI metadata mismatch",
        roundtrip_case.label
    );
    assert_eq!(
        symbol.structured_append, None,
        "{} {path}: unexpected structured append metadata",
        roundtrip_case.label
    );
    assert_eq!(symbol.symbology.code, b'Q');
    assert_eq!(symbol.symbology.modifier, b'1');
    assert_eq!(symbol.symbology.eci_modifier_offset, 1);
    assert_eq!(symbol.symbology.ai_flag, AIFlag::None);
}

fn assert_qr_geometry(qr: &QRCode, roundtrip_case: &RoundTripCase) {
    assert_eq!(
        qr.size,
        roundtrip_case.expected_module_count(),
        "{}: QR module size should match forced version",
        roundtrip_case.label
    );
}

/// Build the QR, exercise both PNG and rasterized-SVG paths, and assert each
/// decode returns the original payload plus the forced QR metadata.
pub fn assert_roundtrip(roundtrip_case: &RoundTripCase) {
    let qr = roundtrip_case.build_qr();
    assert_qr_geometry(&qr, roundtrip_case);

    let png = render_qr_to_png_bytes(&qr, ROUNDTRIP_SCALE, ROUNDTRIP_MARGIN);
    let decoded = decode_png_bytes(&png);
    assert_decoded_symbol(&decoded, roundtrip_case, "PNG");

    let svg = SvgBuilder::default().margin(ROUNDTRIP_MARGIN).to_str(&qr);
    let (rgba, width, height) = rasterize_svg_to_rgba(&svg, ROUNDTRIP_SCALE);
    let decoded = decode_rgba(&rgba, width, height);
    assert_decoded_symbol(&decoded, roundtrip_case, "SVG");

    dump_artifacts(roundtrip_case.label, &png, &svg);
}

/// Table-driven roundtrip that decodes the raw QR module matrix directly,
/// bypassing PNG/SVG rendering. Isolates encoder correctness from any
/// rendering bugs.
pub fn assert_raw_matrix_roundtrip(roundtrip_case: &RoundTripCase) {
    let qr = roundtrip_case.build_qr();
    assert_qr_geometry(&qr, roundtrip_case);

    let (luma, width, height) = render_qr_to_luma(&qr, ROUNDTRIP_SCALE, ROUNDTRIP_MARGIN);
    let decoded = decode_luma(&luma, width, height);
    assert_decoded_symbol(&decoded, roundtrip_case, "raw-matrix");
}

/// Dump PNG and SVG artifacts under `CARGO_TARGET_TMPDIR/e2e/` for debugging.
pub fn dump_artifacts(label: &str, png: &[u8], svg: &str) {
    let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("e2e");
    let _ = std::fs::create_dir_all(&dir);
    let _ = std::fs::write(dir.join(format!("{label}.png")), png);
    let _ = std::fs::write(dir.join(format!("{label}.svg")), svg);
}
