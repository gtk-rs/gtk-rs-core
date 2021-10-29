// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoRectangle")]
    pub struct Rectangle(BoxedInline<ffi::PangoRectangle>);
}

impl Rectangle {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self(ffi::PangoRectangle {
            x,
            y,
            width,
            height,
        })
    }

    pub fn x(&self) -> i32 {
        self.0.x
    }

    pub fn y(&self) -> i32 {
        self.0.y
    }

    pub fn width(&self) -> i32 {
        self.0.width
    }

    pub fn height(&self) -> i32 {
        self.0.height
    }
}

impl fmt::Debug for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Rectangle")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
