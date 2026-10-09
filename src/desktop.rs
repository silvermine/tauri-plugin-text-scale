//! The fallback for platforms that do not read a text size yet.
//!
//! macOS has no public API for its text size setting, so it always uses this fallback.
//! Linux uses it until a module reads the GTK text scale. Windows and iOS use it until
//! each platform has its own module.

/// The text scale at the platform's default text size.
pub(crate) const DEFAULT_TEXT_SCALE: f64 = 1.0;

/// Returns the default text scale. The fallback never reports a change.
pub(crate) fn text_scale() -> f64 {
   DEFAULT_TEXT_SCALE
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn returns_the_default_scale() {
      assert_eq!(text_scale(), 1.0);
   }
}
