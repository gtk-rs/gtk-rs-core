// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc(alias = "cairo_font_extents_t")]
pub struct FontExtents(ffi::cairo_font_extents_t);

impl FontExtents {
    pub fn ascent(&self) -> f64 {
        self.0.ascent
    }

    pub fn descent(&self) -> f64 {
        self.0.descent
    }

    pub fn height(&self) -> f64 {
        self.0.height
    }

    pub fn max_x_advance(&self) -> f64 {
        self.0.max_x_advance
    }

    pub fn max_y_advance(&self) -> f64 {
        self.0.max_y_advance
    }
}

impl fmt::Debug for FontExtents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FontExtents")
            .field("ascent", &self.ascent())
            .field("descent", &self.descent())
            .field("height", &self.height())
            .field("max_x_advance", &self.max_x_advance())
            .field("max_y_advance", &self.max_y_advance())
            .finish()
    }
}
