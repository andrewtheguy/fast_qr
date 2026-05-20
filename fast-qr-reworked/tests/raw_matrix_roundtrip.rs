//! Raw-matrix roundtrip tests: generate deterministic QR matrices, hand the
//! module matrix straight to `rxing-reader` (no PNG codec, no SVG rasterizer),
//! and assert the decoded payload and metadata match the forced generation
//! settings. Isolates encoder correctness from rendering.

mod common;

use common::{assert_raw_matrix_roundtrip, roundtrip_cases};

#[test]
fn raw_matrix_roundtrip_cases_decode_with_forced_metadata() {
    for roundtrip_case in roundtrip_cases() {
        assert_raw_matrix_roundtrip(&roundtrip_case);
    }
}
