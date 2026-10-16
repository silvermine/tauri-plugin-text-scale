//! The fallback for platforms that do not read a text size yet.
//!
//! macOS has no public API for its text size setting, so it always uses this fallback.
//! Linux uses it until a module reads the GTK text scale. Windows has its own module.

use std::marker::PhantomData;

use tauri::Runtime;

/// The text scale at the platform's default text size.
pub(crate) const DEFAULT_TEXT_SCALE: f64 = 1.0;

/// Returns the default text scale. The fallback never reports a change.
pub(crate) fn text_scale() -> f64 {
   DEFAULT_TEXT_SCALE
}

/// Returns the fallback.
pub(crate) fn init<R: Runtime>() -> TextScale<R> {
   TextScale(PhantomData)
}

/// Access to the fallback text scale. The type parameter gives [`crate::TextScale`] the
/// same signature on every platform.
pub(crate) struct TextScale<R: Runtime>(PhantomData<fn() -> R>);

impl<R: Runtime> TextScale<R> {
   /// Returns the default text scale.
   pub(crate) fn scale(&self) -> crate::Result<f64> {
      Ok(text_scale())
   }

   /// Returns the default text scale. The fallback does not wait for a reply.
   pub(crate) async fn scale_async(&self) -> crate::Result<f64> {
      self.scale()
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn returns_the_default_scale() {
      assert_eq!(text_scale(), 1.0);
   }
}
