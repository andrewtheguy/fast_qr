//! End-to-end roundtrip tests: generate deterministic QR matrices, render to
//! PNG and SVG, decode both with the current `rxing-reader`, and assert the
//! decoded payload and metadata match the forced generation settings.

mod common;

use common::{assert_roundtrip, roundtrip_cases};

#[test]
fn png_and_svg_roundtrip_cases_decode_with_forced_metadata() {
    for roundtrip_case in roundtrip_cases() {
        assert_roundtrip(&roundtrip_case);
    }
}
