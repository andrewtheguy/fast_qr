//! Converts a [`crate::QRCode`] to a simple black-and-white SVG.

use core::fmt;

use crate::QRCode;

use super::Builder;

/// Builder for SVG QR output.
pub struct SvgBuilder {
    margin: usize,
    width: Option<usize>,
    height: Option<usize>,
}

/// Possible errors when writing SVG output.
#[derive(Debug)]
pub enum SvgError {
    /// Error while writing the SVG file.
    IoError(std::io::Error),
}

impl fmt::Display for SvgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SvgError::IoError(e) => write!(f, "failed to write SVG: {}", e),
        }
    }
}

impl std::error::Error for SvgError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SvgError::IoError(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for SvgError {
    fn from(err: std::io::Error) -> Self {
        SvgError::IoError(err)
    }
}

impl Default for SvgBuilder {
    fn default() -> Self {
        Self {
            margin: 4,
            width: None,
            height: None,
        }
    }
}

impl Builder for SvgBuilder {
    fn margin(&mut self, margin: usize) -> &mut Self {
        self.margin = margin;
        self
    }
}

impl SvgBuilder {
    /// Sets an explicit `width` attribute on the root `<svg>` element.
    pub fn width(&mut self, width: usize) -> &mut Self {
        self.width = Some(width);
        self
    }

    /// Sets an explicit `height` attribute on the root `<svg>` element.
    pub fn height(&mut self, height: usize) -> &mut Self {
        self.height = Some(height);
        self
    }

    fn path(&self, qr: &QRCode) -> String {
        let mut path = String::with_capacity(10 * qr.size * qr.size);
        path.push_str(r#"<path d=""#);

        for y in 0..qr.size {
            let line = &qr[y];
            for (x, &cell) in line.iter().enumerate() {
                if cell.value() {
                    path.push_str(&format!("M{},{}h1v1h-1z", x + self.margin, y + self.margin));
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

        out.push_str("<svg");
        if let Some(w) = self.width {
            out.push_str(&format!(r#" width="{w}""#));
        }
        if let Some(h) = self.height {
            out.push_str(&format!(r#" height="{h}""#));
        }
        out.push_str(&format!(
            r#" viewBox="0 0 {0} {0}" xmlns="http://www.w3.org/2000/svg">"#,
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
