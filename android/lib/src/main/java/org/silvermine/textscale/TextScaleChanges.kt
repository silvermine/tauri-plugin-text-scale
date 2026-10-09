package org.silvermine.textscale

// Keeps the last reported scale. Android reports every configuration change, such as a
// rotation, to the same callback, and only a new scale is a change to report.
class TextScaleChanges(private var lastScale: Double) {
   // Returns the new scale when it differs from the last one, or null when it does not.
   fun update(scale: Double): Double? {
      if (scale == lastScale) {
         return null
      }

      lastScale = scale
      return scale
   }
}
