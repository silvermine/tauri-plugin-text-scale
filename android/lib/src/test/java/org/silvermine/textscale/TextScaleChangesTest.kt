package org.silvermine.textscale

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TextScaleChangesTest {
   @Test
   fun `test that the same scale is not a change`() {
      val changes = TextScaleChanges(1.0)

      assertNull(changes.update(1.0))
   }

   @Test
   fun `test that a new scale is a change`() {
      val changes = TextScaleChanges(1.0)

      assertEquals(1.3, changes.update(1.3))
   }

   @Test
   fun `test that a change becomes the scale to compare against`() {
      val changes = TextScaleChanges(1.0)

      changes.update(1.3)

      assertNull(changes.update(1.3))
      assertEquals(1.0, changes.update(1.0))
   }
}
