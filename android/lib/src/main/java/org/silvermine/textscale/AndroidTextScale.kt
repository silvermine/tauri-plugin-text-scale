package org.silvermine.textscale

object AndroidTextScale {
   const val DEFAULT_SCALE = 1.0

   fun fromFontScale(fontScale: Float): Double {
      // `Configuration.fontScale` is `0` when the configuration does not define it. A
      // scale that is not a positive number would hide or invert the app's text, so it
      // falls back to the default text size.
      if (!fontScale.isFinite() || fontScale <= 0f) {
         return DEFAULT_SCALE
      }

      // A plain `toDouble()` turns 1.15f into 1.149999976158142. The decimal text of the
      // Float gives the value that Android shows in its settings.
      return fontScale.toString().toDouble()
   }
}
