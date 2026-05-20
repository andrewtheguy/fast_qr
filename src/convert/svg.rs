//! Converts a [`crate::QRCode`] to a simple black-and-white SVG.

use crate::QRCode;

use super::Builder;

/// Builder for SVG QR output.
pub struct SvgBuilder {
    margin: usize,
}

/// Possible errors when writing SVG output.
#[derive(Debug)]
pub enum SvgError {
    /// Error while writing the SVG file.
    IoError(std::io::Error),
}

impl Default for SvgBuilder {
    fn default() -> Self {
        Self { margin: 4 }
    }
}

impl Builder for SvgBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }
}

impl SvgBuilder {
    fn path(&self, qr: &QRCode) -> String {
        let mut path = String::with_capacity(10 * qr.size * qr.size);
        path.push_str(r#"<path d=""#);

        for y in 0..qr.size {
            let line = &qr[y];
            for (x, &cell) in line.iter().enumerate() {
                if cell.value() {
                    path.push_str(&format!("M{},{}h1v1h-1", x + self.margin, y + self.margin));
                }
            }
        }

        path.push_str(r##"" fill="#000000"/>"##);
        path
    }

    /// Returns a string containing the SVG for a QR code.
    #[must_use]
    pub fn to_str(&self, qr: &QRCode) -> String {
        let size = self.margin * 2 + qr.size;
        let mut out = String::with_capacity(11 * qr.size * qr.size / 2);

        out.push_str(&format!(
            r#"<svg viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
            size
        ));
        out.push_str(&format!(
            r##"<rect width="{0}px" height="{0}px" fill="#ffffff"/>"##,
            size
        ));
        out.push_str(&self.path(qr));
        out.push_str("</svg>");

        out
    }

    /// Saves the SVG for a QR code to a file.
    pub fn to_file(&self, qr: &QRCode, file: &str) -> Result<(), SvgError> {
        use std::fs::File;
        use std::io::Write;

        let out = self.to_str(qr);
        let mut file = File::create(file).map_err(SvgError::IoError)?;
        file.write_all(out.as_bytes()).map_err(SvgError::IoError)
    }
}
