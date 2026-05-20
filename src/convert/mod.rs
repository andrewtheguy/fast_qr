//! SVG conversion support used by the wasm wrapper.

pub mod svg;

/// Shared builder controls for QR renderers.
pub trait Builder {
    /// Updates the quiet-zone margin in module units.
    fn margin(&mut self, margin: usize) -> &mut Self;
}
