package org.silvermine.textscale

import org.junit.Assert.assertEquals
import org.junit.Test

class AndroidTextScaleTest {
   @Test
   fun `test that the default font scale gives the default scale`() {
      assertEquals(1.0, AndroidTextScale.fromFontScale(1.0f), 0.0)
   }

   @Test
   fun `test that a font scale keeps its decimal value`() {
      assertEquals(0.85, AndroidTextScale.fromFontScale(0.85f), 0.0)
      assertEquals(1.15, AndroidTextScale.fromFontScale(1.15f), 0.0)
      assertEquals(1.3, AndroidTextScale.fromFontScale(1.3f), 0.0)
      assertEquals(2.0, AndroidTextScale.fromFontScale(2.0f), 0.0)
   }

   @Test
   fun `test that an undefined font scale gives the default scale`() {
      assertEquals(1.0, AndroidTextScale.fromFontScale(0f), 0.0)
   }

   @Test
   fun `test that an invalid font scale gives the default scale`() {
      assertEquals(1.0, AndroidTextScale.fromFontScale(-1.0f), 0.0)
      assertEquals(1.0, AndroidTextScale.fromFontScale(Float.NaN), 0.0)
      assertEquals(1.0, AndroidTextScale.fromFontScale(Float.POSITIVE_INFINITY), 0.0)
   }
}
