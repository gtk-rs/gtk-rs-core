// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point;
use crate::Vec2;
use glib::translate::*;

impl Point {
    #[doc(alias = "graphene_point_init")]
    pub fn new(x: f32, y: f32) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point::uninitialized();
            ffi::graphene_point_init(p.to_glib_none_mut().0, x, y);
            p
        }
    }

    #[doc(alias = "graphene_point_init_from_point")]
    #[doc(alias = "new_from_point")]
    pub fn from_point(src: &Point) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point::uninitialized();
            ffi::graphene_point_init_from_point(p.to_glib_none_mut().0, src.to_glib_none().0);
            p
        }
    }

    #[doc(alias = "graphene_point_init_from_vec2")]
    #[doc(alias = "new_from_vec2")]
    pub fn from_vec2(src: &Vec2) -> Point {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point::uninitialized();
            ffi::graphene_point_init_from_vec2(p.to_glib_none_mut().0, src.to_glib_none().0);
            p
        }
    }
}
