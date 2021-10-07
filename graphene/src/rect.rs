// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Rect;
use crate::Vec2;
use glib::translate::*;

impl Rect {
    #[doc(alias = "graphene_rect_get_vertices")]
    #[doc(alias = "get_vertices")]
    pub fn vertices(&self) -> [Vec2; 4] {
        unsafe {
            let mut out: [ffi::graphene_vec2_t; 4] = std::mem::zeroed();
            ffi::graphene_rect_get_vertices(self.to_glib_none().0, &mut out as *mut _);
            [
                from_glib_none(&out[0] as *const _),
                from_glib_none(&out[1] as *const _),
                from_glib_none(&out[2] as *const _),
                from_glib_none(&out[3] as *const _),
            ]
        }
    }

    #[doc(alias = "graphene_rect_init")]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_rect_alloc();
            ffi::graphene_rect_init(alloc, x, y, width, height);
            from_glib_full(alloc)
        }
    }

    #[doc(alias = "graphene_rect_init_from_rect")]
    #[doc(alias = "new_from_rect")]
    pub fn from_rect(src: &Rect) -> Rect {
        assert_initialized_main_thread!();
        unsafe {
            let alloc = ffi::graphene_rect_alloc();
            ffi::graphene_rect_init_from_rect(alloc, src.to_glib_none().0);
            from_glib_full(alloc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Point;

    #[test]
    fn contains_point() {
        let rect = Rect::new(100., 100., 100., 100.);

        let right = Point::new(250., 150.);
        let below = Point::new(150., 50.);
        let left = Point::new(50., 150.);
        let above = Point::new(150., 250.);

        assert!(!rect.contains_point(&right));
        assert!(!rect.contains_point(&below));
        assert!(!rect.contains_point(&left));
        assert!(!rect.contains_point(&above));
    }
}
