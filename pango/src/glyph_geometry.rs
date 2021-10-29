// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoGlyphGeometry")]
    pub struct GlyphGeometry(BoxedInline<ffi::PangoGlyphGeometry>);
}

impl GlyphGeometry {
    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn x_offset(&self) -> i32 {
        self.0.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.0.y_offset
    }
}

impl fmt::Debug for GlyphGeometry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GlyphGeometry")
            .field("x_offset", &self.x_offset())
            .field("y_offset", &self.y_offset())
            .field("width", &self.width())
            .finish()
    }
}
