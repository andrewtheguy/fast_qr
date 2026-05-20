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

fn root_tag(svg: &str) -> &str {
    let end = svg.find('>').expect("SVG should have a root tag");
    &svg[..=end]
}

fn sample_qrcode() -> crate::QRCode {
    use crate::{QRBuilder, Version, ECL};
    QRBuilder::new("dim-test")
        .ecl(ECL::M)
        .version(Version::V01)
        .build()
        .unwrap()
}

#[test]
fn svg_default_omits_width_and_height() {
    use crate::convert::svg::SvgBuilder;

    let svg = SvgBuilder::default().to_str(&sample_qrcode());
    let tag = root_tag(&svg);
    assert!(
        !tag.contains(r#"width=""#),
        "root <svg> tag should not contain width by default: {tag}"
    );
    assert!(
        !tag.contains(r#"height=""#),
        "root <svg> tag should not contain height by default: {tag}"
    );
}

#[test]
fn svg_sets_explicit_width() {
    use crate::convert::svg::SvgBuilder;

    let svg = SvgBuilder::default().width(200).to_str(&sample_qrcode());
    let tag = root_tag(&svg);
    assert!(
        tag.contains(r#"width="200""#),
        "root <svg> tag should contain width: {tag}"
    );
    assert!(
        !tag.contains(r#"height=""#),
        "root <svg> tag should not contain height: {tag}"
    );
}

#[test]
fn svg_sets_explicit_height() {
    use crate::convert::svg::SvgBuilder;

    let svg = SvgBuilder::default().height(300).to_str(&sample_qrcode());
    let tag = root_tag(&svg);
    assert!(
        tag.contains(r#"height="300""#),
        "root <svg> tag should contain height: {tag}"
    );
    assert!(
        !tag.contains(r#"width=""#),
        "root <svg> tag should not contain width: {tag}"
    );
}

#[test]
fn svg_sets_both_width_and_height() {
    use crate::convert::svg::SvgBuilder;

    let svg = SvgBuilder::default()
        .width(200)
        .height(300)
        .to_str(&sample_qrcode());
    let tag = root_tag(&svg);
    assert!(
        tag.contains(r#"width="200""#),
        "root <svg> tag should contain width: {tag}"
    );
    assert!(
        tag.contains(r#"height="300""#),
        "root <svg> tag should contain height: {tag}"
    );
}
