// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GlyphGeometry;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoGlyphInfo")]
    pub struct GlyphInfo(BoxedInline<ffi::PangoGlyphInfo>);
}

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        self.inner.glyph
    }

    pub fn geometry(&self) -> &GlyphGeometry {
        unsafe { &*(&self.inner.geometry as *const _ as *const GlyphGeometry) }
    }

    pub fn geometry_mut(&mut self) -> &mut GlyphGeometry {
        unsafe { &mut *(&mut self.inner.geometry as *mut _ as *mut GlyphGeometry) }
    }
}

impl fmt::Debug for GlyphInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GlyphInfo")
            .field("glyph", &self.glyph())
            .field("geometry", &self.geometry())
            .finish()
    }
}
