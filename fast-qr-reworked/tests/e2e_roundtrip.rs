//! End-to-end roundtrip tests: generate a QR, render to PNG and SVG, decode
//! both with `rxing-reader` v0.0.7, and assert the decoded payload matches.

mod common;

use common::assert_roundtrip;
use fast_qr_reworked::{ECL, Mode, Version};

#[test]
fn short_ascii() {
    assert_roundtrip("short_ascii", b"HELLO", |_| {});
}

#[test]
fn url_with_query() {
    assert_roundtrip(
        "url_with_query",
        b"https://example.com/search?q=fast_qr&lang=en",
        |b| {
            b.ecl(ECL::M);
        },
    );
}

#[test]
fn numeric_only() {
    assert_roundtrip("numeric_only", b"01234567890123456789", |b| {
        b.ecl(ECL::L);
    });
}

#[test]
fn long_text() {
    let s = "abcdefghij".repeat(15); // ~150 chars
    assert_roundtrip("long_text", s.as_bytes(), |b| {
        b.ecl(ECL::Q);
    });
}

#[test]
fn forced_version_and_high_ecl() {
    assert_roundtrip(
        "forced_v05_ecl_h",
        b"fast-qr-reworked roundtrip",
        |b| {
            b.ecl(ECL::H).version(Version::V05);
        },
    );
}

/// Force Mode::Byte on a small non-UTF-8 payload: includes 0x00, 0xFF, 0x80,
/// some ASCII, 0x7F, and 0x10. Same fixture used by fast-qr-wasm's binary
/// tests.
#[test]
fn force_byte_mode_binary_short() {
    let payload: &[u8] = &[0x00, 0xFF, 0x80, 0x41, 0x42, 0x43, 0x7F, 0x10];
    assert_roundtrip("force_byte_mode_binary_short", payload, |b| {
        b.mode(Mode::Byte).ecl(ECL::Q);
    });
}

/// Force Mode::Byte on the full 0..=255 byte range to confirm every byte
/// value round-trips through encode + render + decode.
#[test]
fn force_byte_mode_binary_full_range() {
    let payload: Vec<u8> = (0u8..=255u8).collect();
    assert_roundtrip("force_byte_mode_binary_full_range", &payload, |b| {
        b.mode(Mode::Byte).ecl(ECL::L);
    });
}

/// Digits-only string with Mode::Byte forced: without the override, the
/// encoder would pick Mode::Numeric. This verifies the force takes effect
/// (Byte-mode header) and the bytes still decode back to the ASCII digits.
#[test]
fn force_byte_mode_on_numeric_string() {
    assert_roundtrip(
        "force_byte_mode_on_numeric_string",
        b"01234567890123456789",
        |b| {
            b.mode(Mode::Byte).ecl(ECL::M);
        },
    );
}
