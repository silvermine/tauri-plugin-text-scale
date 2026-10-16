//! The text scale that the Windows module keeps for readers on any thread.
//!
//! The module lives apart from the Windows API, so its tests run on every host.

use std::sync::atomic::{AtomicU64, Ordering};

/// Returns `raw` when it can be a text scale: a finite number above zero.
///
/// Windows documents the scale as a value from `1` to `2.25`. The check does not enforce
/// that range, so a later Windows version that widens it still reaches the app. It only
/// rejects a value that would break an app's text sizes, such as `NaN` or `0`.
pub(crate) fn valid_scale(raw: f64) -> Option<f64> {
   (raw.is_finite() && raw > 0.0).then_some(raw)
}

/// The latest text scale, shared between the threads that read it and the thread that
/// receives the changes.
#[derive(Debug)]
pub(crate) struct ScaleCache(AtomicU64);

impl ScaleCache {
   /// Creates a cache that holds `initial`.
   pub(crate) fn new(initial: f64) -> Self {
      Self(AtomicU64::new(initial.to_bits()))
   }

   /// Returns the latest scale.
   pub(crate) fn get(&self) -> f64 {
      f64::from_bits(self.0.load(Ordering::Acquire))
   }

   /// Stores `scale` and returns whether it differs from the scale that the cache held.
   pub(crate) fn replace(&self, scale: f64) -> bool {
      self.0.swap(scale.to_bits(), Ordering::AcqRel) != scale.to_bits()
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn accepts_a_finite_scale_above_zero() {
      assert_eq!(valid_scale(1.0), Some(1.0));
      assert_eq!(valid_scale(2.25), Some(2.25));
      assert_eq!(valid_scale(0.5), Some(0.5));
   }

   #[test]
   fn rejects_a_scale_that_breaks_text_sizes() {
      assert_eq!(valid_scale(0.0), None);
      assert_eq!(valid_scale(-1.5), None);
      assert_eq!(valid_scale(f64::NAN), None);
      assert_eq!(valid_scale(f64::INFINITY), None);
   }

   #[test]
   fn holds_the_initial_scale() {
      assert_eq!(ScaleCache::new(1.5).get(), 1.5);
   }

   #[test]
   fn replaces_the_scale_and_reports_a_change() {
      let cache = ScaleCache::new(1.0);

      assert!(cache.replace(2.25));
      assert_eq!(cache.get(), 2.25);
   }

   #[test]
   fn reports_no_change_for_the_same_scale() {
      let cache = ScaleCache::new(1.5);

      assert!(!cache.replace(1.5));
      assert_eq!(cache.get(), 1.5);
   }
}
