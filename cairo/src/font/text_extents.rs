// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc(alias = "cairo_text_extents_t")]
pub struct TextExtents(ffi::cairo_text_extents_t);

impl TextExtents {
    pub fn x_bearing(&self) -> f64 {
        self.0.x_bearing
    }

    pub fn y_bearing(&self) -> f64 {
        self.0.y_bearing
    }

    pub fn width(&self) -> f64 {
        self.0.width
    }

    pub fn height(&self) -> f64 {
        self.0.height
    }

    pub fn x_advance(&self) -> f64 {
        self.0.x_advance
    }

    pub fn y_advance(&self) -> f64 {
        self.0.y_advance
    }
}

impl fmt::Debug for TextExtents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextExtents")
            .field("x_bearing", &self.x_bearing())
            .field("y_bearing", &self.y_bearing())
            .field("width", &self.width())
            .field("height", &self.height())
            .field("x_advance", &self.x_advance())
            .field("y_advance", &self.y_advance())
            .finish()
    }
}
