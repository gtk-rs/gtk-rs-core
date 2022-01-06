// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point;
use crate::Vec2;
use glib::translate::*;
use std::fmt;

impl Point {
    #[doc(alias = "graphene_point_init")]
    pub fn new(x: f32, y: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Self::uninitialized();
            ffi::graphene_point_init(p.to_glib_none_mut().0, x, y);
            p
        }
    }

    #[doc(alias = "graphene_point_init_from_vec2")]
    #[doc(alias = "init_from_vec2")]
    pub fn from_vec2(src: &Vec2) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Self::uninitialized();
            ffi::graphene_point_init_from_vec2(p.to_glib_none_mut().0, src.to_glib_none().0);
            p
        }
    }

    pub fn x(&self) -> f32 {
        self.inner.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.inner.x = x;
    }

    pub fn y(&self) -> f32 {
        self.inner.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.inner.y = y;
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x())
            .field("y", &self.y())
            .finish()
    }
}
