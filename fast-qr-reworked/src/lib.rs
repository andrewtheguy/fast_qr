#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
//! Minimal QR generation backend for `fast-qr-wasm`.
//!
//! The parent crate exposes the QR matrix builder plus the simple SVG renderer
//! required by the wasm wrapper.
//!
//! ```rust
//! use fast_qr_reworked::convert::svg::SvgBuilder;
//! use fast_qr_reworked::convert::Builder;
//! use fast_qr_reworked::QRBuilder;
//!
//! let qrcode = QRBuilder::new("https://example.com/").build().unwrap();
//! let svg = SvgBuilder::default().margin(4).to_str(&qrcode);
//! assert!(svg.starts_with("<svg "));
//! ```

pub use crate::datamasking::Mask;
pub use crate::ecl::ECL;
pub use crate::encode::Mode;
pub use crate::module::{Module, ModuleType};
pub use crate::qr::{QRBuilder, QRCode};
pub use crate::version::Version;

mod compact;
#[doc(hidden)]
pub mod datamasking;

pub mod convert;
mod default;
mod ecl;
mod encode;
mod hardcode;
mod helpers;
mod module;
mod placement;
mod polynomials;
pub mod qr;
mod score;
mod version;

#[cfg(test)]
mod tests;
