// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Size;
use glib::translate::*;
use std::fmt;

impl Size {
    #[doc(alias = "graphene_size_init")]
    pub fn new(width: f32, height: f32) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let mut siz = Size::uninitialized();
            ffi::graphene_size_init(siz.to_glib_none_mut().0, width, height);
            siz
        }
    }

    #[doc(alias = "graphene_size_init_from_size")]
    #[doc(alias = "new_from_size")]
    pub fn from_size(src: &Size) -> Size {
        assert_initialized_main_thread!();
        unsafe {
            let mut siz = Size::uninitialized();
            ffi::graphene_size_init_from_size(siz.to_glib_none_mut().0, src.to_glib_none().0);
            siz
        }
    }

    pub fn width(&self) -> f32 {
        self.0.width
    }

    pub fn set_width(&mut self, width: f32) {
        self.0.width = width;
    }

    pub fn height(&self) -> f32 {
        self.0.height
    }

    pub fn set_height(&mut self, height: f32) {
        self.0.height = height;
    }
}

impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Size")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}
