#[test]
fn check_svg_is_not_inverted() {
    use crate::convert::svg::SvgBuilder;
    use crate::convert::Builder;
    use crate::{QRBuilder, Version, ECL};

    let qrcode = QRBuilder::new("Test")
        .ecl(ECL::M)
        .version(Version::V01)
        .build()
        .unwrap();

    const MARGIN: usize = 4;
    let svg = SvgBuilder::default().margin(MARGIN).to_str(&qrcode);

    let size = qrcode.size;
    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            let expected = qrcode.data[index];
            if expected.value() {
                let expected = format!(r#"M{x},{y}"#, x = x + MARGIN, y = y + MARGIN);
                assert!(svg.contains(&expected));
            }
        }
    }
}
