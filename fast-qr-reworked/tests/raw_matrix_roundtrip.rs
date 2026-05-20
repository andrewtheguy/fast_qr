//! Raw-matrix roundtrip tests: generate a QR, hand the module matrix straight
//! to `rxing-reader` (no PNG codec, no SVG rasterizer), assert the decoded
//! payload matches the input. Isolates encoder correctness from rendering.

mod common;

use common::assert_raw_matrix_roundtrip;
use fast_qr_reworked::{ECL, Mode, Version};

#[test]
fn short_ascii() {
    assert_raw_matrix_roundtrip("short_ascii", b"HELLO", |_| {});
}

#[test]
fn url_with_query() {
    assert_raw_matrix_roundtrip(
        "url_with_query",
        b"https://example.com/search?q=fast_qr&lang=en",
        |b| {
            b.ecl(ECL::M);
        },
    );
}

#[test]
fn numeric_only() {
    assert_raw_matrix_roundtrip("numeric_only", b"01234567890123456789", |b| {
        b.ecl(ECL::L);
    });
}

#[test]
fn long_text() {
    let s = "abcdefghij".repeat(15);
    assert_raw_matrix_roundtrip("long_text", s.as_bytes(), |b| {
        b.ecl(ECL::Q);
    });
}

#[test]
fn forced_version_and_high_ecl() {
    assert_raw_matrix_roundtrip("forced_v05_ecl_h", b"fast-qr-reworked roundtrip", |b| {
        b.ecl(ECL::H).version(Version::V05);
    });
}

#[test]
fn force_byte_mode_binary_short() {
    let payload: &[u8] = &[0x00, 0xFF, 0x80, 0x41, 0x42, 0x43, 0x7F, 0x10];
    assert_raw_matrix_roundtrip("force_byte_mode_binary_short", payload, |b| {
        b.mode(Mode::Byte).ecl(ECL::Q);
    });
}

#[test]
fn force_byte_mode_binary_full_range() {
    let payload: Vec<u8> = (0u8..=255u8).collect();
    assert_raw_matrix_roundtrip("force_byte_mode_binary_full_range", &payload, |b| {
        b.mode(Mode::Byte).ecl(ECL::L);
    });
}

#[test]
fn force_byte_mode_on_numeric_string() {
    assert_raw_matrix_roundtrip(
        "force_byte_mode_on_numeric_string",
        b"01234567890123456789",
        |b| {
            b.mode(Mode::Byte).ecl(ECL::M);
        },
    );
}
