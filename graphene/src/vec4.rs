// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec2;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Vec4 {
    #[doc(alias = "graphene_vec4_init")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec4::uninitialized();
            ffi::graphene_vec4_init(vec.to_glib_none_mut().0, x, y, z, w);
            vec
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec2")]
    #[doc(alias = "new_from_vec2")]
    pub fn from_vec2(src: &Vec2, z: f32, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec4::uninitialized();
            ffi::graphene_vec4_init_from_vec2(vec.to_glib_none_mut().0, src.to_glib_none().0, z, w);
            vec
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(src: &Vec3, w: f32) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec4::uninitialized();
            ffi::graphene_vec4_init_from_vec3(vec.to_glib_none_mut().0, src.to_glib_none().0, w);
            vec
        }
    }

    #[doc(alias = "graphene_vec4_init_from_vec4")]
    #[doc(alias = "new_from_vec4")]
    pub fn from_vec4(src: &Vec4) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec4::uninitialized();
            ffi::graphene_vec4_init_from_vec4(vec.to_glib_none_mut().0, src.to_glib_none().0);
            vec
        }
    }

    #[doc(alias = "graphene_vec4_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(src: &[f32; 4]) -> Vec4 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec4::uninitialized();
            ffi::graphene_vec4_init_from_float(vec.to_glib_none_mut().0, src.as_ptr() as *const _);
            vec
        }
    }

    #[doc(alias = "graphene_vec4_to_float")]
    pub fn to_float(&self) -> [f32; 4] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_vec4_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
