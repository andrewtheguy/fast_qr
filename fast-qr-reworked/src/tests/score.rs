use crate::default::create_mat_from_bool;
use crate::module::Module;
use crate::score::{
    test_matrix_dark_modules, test_matrix_pattern_and_line, test_matrix_score_squares,
    test_score_line, test_score_pattern,
};
use crate::tests::default::{DATA, EMPT, F, FIND, T};

#[rustfmt::skip]
const MAT_EXAMPLE_COM: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, true, false, false, false, true, false, true, false, false, true, false, true, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, true, true, true, false, false, true, false, true, false, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, false, false, false, true, true, true, true, true, false, true, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, true, true, true, true, true, false, false, true, false, false, true, false, false, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, true, true, true, false, true, false, true, false, false, false, false, false, false, false, true, false, true, true, true, true, true, true, false, false, true, true, true],
    [false, false, false, false, false, true, false, true, true, true, true, true, true, false, true, false, true, false, false, false, false, true, true, true, true, true, true, false, true],
    [true, false, true, true, false, true, true, false, true, true, true, true, true, true, true, true, true, false, true, false, true, true, true, true, false, false, false, false, false],
    [true, false, true, false, true, true, false, true, true, true, false, true, true, false, true, false, true, true, true, false, false, true, true, true, false, true, false, true, false],
    [false, true, false, true, false, true, true, true, true, false, true, false, false, true, false, false, false, false, false, false, true, true, false, true, false, false, true, true, true],
    [false, true, true, false, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, true, true, true, true, false, true, true, false, true, true],
    [true, true, false, true, false, true, true, false, true, true, true, false, true, false, false, true, true, false, true, false, true, false, true, true, false, false, false, false, false],
    [true, false, true, true, true, false, false, false, true, true, false, false, true, false, false, false, false, true, true, false, true, false, false, false, true, true, false, true, false],
    [false, true, true, false, true, true, true, false, false, true, true, false, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, true, false],
    [true, false, true, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, true, false, true, true],
    [true, false, false, true, true, false, true, false, false, true, false, true, false, true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, false],
    [true, false, true, false, false, true, false, true, false, true, true, true, true, false, false, true, true, true, false, true, true, false, true, false, true, false, false, true, false],
    [true, false, true, true, false, true, true, false, true, false, true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, false, true, true, true, false, true, false, false, false, true, true, false, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, true, true, true, false, true, true, true, true, false, true, false, true, false, false, false, false],
    [true, false, false, false, false, false, true, false, false, true, true, true, true, true, false, true, true, false, true, true, true, false, false, false, true, true, false, false, false],
    [true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, false, true, true, true, true, false, false, false, true, true, true, true, false, true, false, true, true, false, false],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, false, true, false, true, true, true, true, true, true, true, true, true, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, true, true, true, false, false, false, false, true, false, true, true, false, true, false],
    [true, true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false]
];

#[test]
fn example_com() {
    let mat = create_mat_from_bool(&MAT_EXAMPLE_COM);
    let (line_score, col_score, pattern_score) = test_matrix_pattern_and_line(&mat);
    let dark_score = test_matrix_dark_modules(&mat);
    let square_score = test_matrix_score_squares(&mat);

    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 138, "square score, expected 138");
    assert_eq!(line_score + col_score, 117, "line col score, expected 117");
    assert_eq!(pattern_score, 80, "pattern score, expected 80");
}

#[rustfmt::skip]
const MAT_FAST_QR_COM: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, true, true, true, true, true, true, true, true, true, true, true, true, false, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, false, false, false, false, true, true, false, false, true, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, false, false, true, false, true, false, false, true, false, false, true, true, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, false, true, true, false, true, true, true, true, false, true, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false],
    [false, false, false, false, false, true, true, false, false, false, true, true, true, false, true, false, false, false, true, false, true, false, true, false, true, false, true, false, true],
    [true, true, false, true, true, true, false, true, true, true, true, false, false, false, false, false, false, false, false, false, true, false, false, true, true, false, true, true, false],
    [false, true, false, false, true, false, true, false, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, false, true, false, false, false],
    [false, false, false, true, true, false, false, false, false, false, true, true, true, false, false, true, true, true, false, false, false, false, true, true, false, true, false, false, false],
    [true, true, false, true, true, true, true, false, true, false, true, true, false, false, false, true, true, false, true, true, true, true, true, false, false, false, false, true, false],
    [true, false, true, true, false, false, false, false, false, true, false, false, false, true, false, false, true, true, true, true, true, false, true, false, true, true, false, true, true],
    [true, true, false, false, false, false, true, true, true, false, false, false, false, false, false, true, false, true, false, false, false, false, true, true, false, false, false, false, false],
    [false, false, true, false, true, false, false, false, false, false, true, true, false, false, false, true, true, false, true, true, false, true, true, false, true, true, true, true, true],
    [false, false, true, false, true, false, true, true, false, false, true, false, true, false, true, true, false, false, true, false, false, true, false, false, false, true, true, false, true],
    [true, true, true, true, false, false, false, true, false, false, true, false, false, false, true, false, true, true, false, false, true, false, true, true, true, false, false, false, true],
    [true, true, true, true, true, true, true, true, true, false, false, false, false, true, true, true, false, false, true, false, true, false, false, false, false, true, false, false, true],
    [true, false, false, false, false, true, false, false, true, false, true, true, false, true, true, false, false, false, true, true, true, true, true, false, true, false, false, false, false],
    [true, false, false, true, false, false, true, false, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, false, true, false, false],
    [false, false, false, false, false, false, false, false, true, false, true, false, false, true, true, false, true, true, true, false, true, false, false, false, true, true, true, false, false],
    [true, true, true, true, true, true, true, false, false, false, false, true, true, false, false, false, true, true, true, true, true, false, true, false, true, false, false, true, false],
    [true, false, false, false, false, false, true, false, true, true, false, true, true, true, true, false, false, false, true, true, true, false, false, false, true, true, false, false, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, true, false, false, true, true, false, true, true, true, true, true, true, true, false, false, false, true],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, true, false, true, false, true, true, true, false, false, true, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, false, false, false, false, true, true, false, true, true, true, true, true, true, true, false, true, true, false],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, true, false, true, true, true, false, false, true, true, false, false, true, true, true, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, true, false, false, true, true, true, true, true, false, false, false, true, false, true, false, true, false, false]
];

#[test]
fn fast_qr_com() {
    let mat = create_mat_from_bool(&MAT_FAST_QR_COM);
    let (line_score, col_score, pattern_score) = test_matrix_pattern_and_line(&mat);
    let dark_score = test_matrix_dark_modules(&mat);
    let square_score = test_matrix_score_squares(&mat);

    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 174, "square score, expected 174");
    assert_eq!(line_score + col_score, 108, "line col score, expected 108");
    assert_eq!(pattern_score, 40, "pattern score, expected 40");
}

#[rustfmt::skip]
const MAT_XIAOJIBA_DEV: [[bool; 29]; 29] = [
    [true, true, true, true, true, true, true, false, false, true, false, true, false, true, true, true, true, true, false, true, true, false, true, true, true, true, true, true, true],
    [true, false, false, false, false, false, true, false, false, true, true, false, true, true, true, false, true, false, true, true, false, false, true, false, false, false, false, false, true],
    [true, false, true, true, true, false, true, false, true, true, true, false, true, false, true, true, true, false, false, true, false, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, false, false, true, false, false, true, false, true, false, true, true, true, false, true],
    [true, false, true, true, true, false, true, false, false, true, false, true, true, true, false, false, true, false, true, true, false, false, true, false, true, true, true, false, true],
    [true, false, false, false, false, false, true, false, false, true, false, false, false, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, true],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true],
    [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, true, true, true, false, false, false, false, false, false, false, false],
    [false, false, false, true, true, false, true, true, false, false, false, true, false, false, false, false, true, false, false, false, false, false, false, false, false, true, true, false, false],
    [true, true, false, true, true, false, false, true, true, false, true, true, false, true, true, true, false, false, true, false, false, false, false, true, true, false, false, true, false],
    [true, true, true, false, false, true, true, true, true, false, false, true, false, true, false, true, false, true, false, true, true, true, true, true, true, true, true, false, false],
    [true, true, true, false, false, true, false, true, false, true, false, false, true, true, false, false, false, true, false, true, true, false, false, false, true, true, false, false, true],
    [false, true, true, false, false, true, true, true, true, true, true, true, true, true, false, true, true, true, true, false, true, true, true, false, false, true, false, true, false],
    [true, false, false, false, false, true, false, true, false, false, false, true, false, true, true, false, false, true, true, false, false, false, true, false, true, false, true, false, true],
    [false, true, false, false, true, true, true, true, true, true, false, true, true, false, true, true, false, true, false, true, false, false, true, true, false, true, false, false, true],
    [true, true, false, true, false, true, false, false, true, true, true, false, false, false, false, false, false, true, true, false, true, true, true, false, true, true, true, false, false],
    [false, true, false, false, false, true, true, true, false, true, true, true, true, true, true, true, true, false, true, true, true, false, false, true, false, true, false, true, true],
    [true, true, true, false, true, false, false, false, true, true, false, false, true, true, false, true, true, false, false, false, false, true, false, false, true, false, false, false, false],
    [true, true, true, false, true, true, true, false, true, false, false, true, false, true, false, false, true, true, true, true, false, false, false, true, false, true, false, false, true],
    [true, true, true, true, true, false, false, false, false, true, true, true, true, false, false, false, false, false, false, false, false, true, true, true, false, false, true, true, false],
    [true, true, false, true, true, false, true, true, true, false, false, false, false, true, false, true, false, true, false, false, true, true, true, true, true, true, true, false, true],
    [false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, true, true, false, false, false, true, true, true, false, false],
    [true, true, true, true, true, true, true, false, true, false, true, false, true, true, false, false, false, false, false, true, true, false, true, false, true, false, true, false, false],
    [true, false, false, false, false, false, true, false, false, true, true, false, false, false, false, true, false, false, false, true, true, false, false, false, true, true, false, true, false],
    [true, false, true, true, true, false, true, false, true, true, true, true, false, true, false, false, false, true, false, true, true, true, true, true, true, false, false, false, false],
    [true, false, true, true, true, false, true, false, true, false, true, false, true, true, false, true, true, true, true, false, false, false, false, true, false, true, true, true, false],
    [true, false, true, true, true, false, true, false, false, true, true, true, false, true, false, true, true, true, false, false, true, true, false, true, true, false, false, true, true],
    [true, false, false, false, false, false, true, false, false, true, false, true, true, false, false, true, true, true, true, false, false, true, true, false, true, false, true, false, true],
    [true, true, true, true, true, true, true, false, false, false, true, false, true, false, true, true, false, true, true, false, true, true, true, true, true, false, false, false, false]
];

#[rustfmt::skip]
const XIAOJIBA_MOD: [[Module; 29]; 29] = [
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(14), Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(14), Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(6), Module(7), Module(14), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3)],
    [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(8), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14)],
    [Module(8), Module(8), Module(8), Module(9), Module(9), Module(8), Module(7), Module(9), Module(8), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(8), Module(8), Module(8), Module(8), Module(9), Module(9), Module(8), Module(8)],
    [Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(6), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0)],
    [Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(7), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0)],
    [Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(6), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1)],
    [Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(7), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0)],
    [Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(6), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1)],
    [Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(7), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1)],
    [Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(6), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0)],
    [Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(7), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1)],
    [Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(6), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0)],
    [Module(1), Module(1), Module(1), Module(0), Module(1), Module(1), Module(7), Module(0), Module(1), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(1)],
    [Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(6), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0)],
    [Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(7), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(0), Module(5), Module(5), Module(5), Module(5), Module(5), Module(1), Module(1), Module(0), Module(1)],
    [Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(14), Module(13), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(1), Module(1), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(1), Module(0), Module(0)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(9), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(0), Module(1), Module(5), Module(4), Module(5), Module(4), Module(5), Module(0), Module(1), Module(0), Module(0)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(5), Module(4), Module(4), Module(4), Module(5), Module(1), Module(0), Module(1), Module(0)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(5), Module(5), Module(5), Module(5), Module(5), Module(0), Module(0), Module(0), Module(0)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(9), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0)],
    [Module(3), Module(2), Module(3), Module(3), Module(3), Module(2), Module(3), Module(14), Module(8), Module(1), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1)],
    [Module(3), Module(2), Module(2), Module(2), Module(2), Module(2), Module(3), Module(14), Module(8), Module(1), Module(0), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(1), Module(1), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1)],
    [Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(3), Module(14), Module(8), Module(0), Module(1), Module(0), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(0), Module(1), Module(1), Module(1), Module(1), Module(1), Module(0), Module(0), Module(0), Module(0)]
];

#[test]
fn xiaojiba_dev() {
    let mat = create_mat_from_bool(&MAT_XIAOJIBA_DEV);
    let (line_score, col_score, pattern_score) = test_matrix_pattern_and_line(&mat);
    let dark_score = test_matrix_dark_modules(&mat);
    let square_score = test_matrix_score_squares(&mat);

    assert_eq!(dark_score, 0, "dark score, expected 0");
    assert_eq!(square_score, 150, "square score, expected 150");
    assert_eq!(line_score + col_score, 95, "line col score, expected 95");
    assert_eq!(pattern_score, 0, "pattern score, expected 0");
}

#[test]
fn adjacent_line() {
    // Data module true, false, true, true, true, false, true
    let line = [
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
    ];

    // Data module true true true true true true true
    let line_2 = [
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
    ];

    // Data module true true true true true false true true true true true
    let line_3_not_data = [
        EMPT(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        EMPT(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
    ];

    let line_4_xiaojiba_dev = [
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        EMPT(F),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        EMPT(F),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
        FIND(T),
    ];

    assert_eq!(test_score_line(&line), 5, "line score, expected 5");
    assert_eq!(test_score_line(&line_2), 5, "line score, expected 5");
    assert_eq!(
        test_score_line(&line_3_not_data),
        3,
        "line score, expected 3"
    );
    assert_eq!(
        test_score_line(&line_4_xiaojiba_dev),
        3,
        "line score, expected 3"
    );
}

#[test]
fn line_by_line_xiaojiba() {
    let scores = [
        3, 0, 0, 0, 0, 0, 0, // 7
        6, 0, 0, 6, 0, 5, 0, // 14
        0, 4, 6, 0, 0, 9, 0, // 21
        0, 3, 0, 0, 0, 0, 0, 3,
    ]; // total: 45

    for (i, line) in XIAOJIBA_MOD.iter().enumerate() {
        let score = scores[i];
        assert_eq!(test_score_line(line), score, "line {i}, expected {score}",);
    }
}

#[test]
fn col_by_col_xiaojiba() {
    let mut transposed = [[EMPT(T); 29]; 29];
    for i in 0..29 {
        for j in i..29 {
            transposed[j][i] = XIAOJIBA_MOD[i][j];
            transposed[i][j] = XIAOJIBA_MOD[j][i];
        }
    }

    let scores = [
        0, 5, 0, 3, 0, 5, 0, // 7
        4, 0, 4, 3, 0, 0, 3, // 14
        9, 0, 4, 7, 0, 0, 0, // 21
        0, 0, 0, 0, 0, 0, 0, 3,
    ]; // total: 50

    for (i, col) in transposed.iter().enumerate() {
        let score = scores[i];
        assert_eq!(
            test_score_line(col),
            score,
            "col {i}, expected {score}\n\n{:?}",
            col
        );
    }
}

#[test]
fn pattern() {
    // Bare 7-bit core `1011101` without 4-light flanking: per spec, no penalty.
    let line = [
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
    ];

    assert_eq!(test_score_pattern(&line), 0, "no flanking, expected 0");

    // Two bare cores `1011101 1011101`: still no 4-light flanking, expected 0.
    let line = [
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
    ];

    assert_eq!(test_score_pattern(&line), 0, "no flanking, expected 0");

    // `0000 1011101`: 4 light modules then the 7-bit core → 40.
    let line = [
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
    ];

    assert_eq!(test_score_pattern(&line), 40, "pre-light flanking, expected 40");

    // `1011101 0000`: 7-bit core then 4 light modules → 40.
    let line = [
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(F),
    ];

    assert_eq!(test_score_pattern(&line), 40, "post-light flanking, expected 40");

    // `0000 1011101 0000`: matches both pre- and post-light patterns → 80.
    let line = [
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(F),
        DATA(F),
        DATA(F),
        DATA(F),
    ];

    assert_eq!(test_score_pattern(&line), 80, "both flanking, expected 80");

    // Non-data module breaks the run; only 7 contiguous data modules available → 0.
    let line = [
        EMPT(F),
        DATA(T),
        DATA(F),
        DATA(T),
        DATA(T),
        DATA(T),
        DATA(F),
        DATA(T),
        EMPT(F),
    ];

    assert_eq!(test_score_pattern(&line), 0, "broken run, expected 0");
}
