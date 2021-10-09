// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Vec3;
use glib::translate::*;

impl Vec3 {
    #[doc(alias = "graphene_vec3_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec3::uninitialized();
            ffi::graphene_vec3_init(vec.to_glib_none_mut().0, x, y, z);
            vec
        }
    }

    #[doc(alias = "graphene_vec3_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(src: &Vec3) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec3::uninitialized();
            ffi::graphene_vec3_init_from_vec3(vec.to_glib_none_mut().0, src.to_glib_none().0);
            vec
        }
    }

    #[doc(alias = "graphene_vec3_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(src: &[f32; 3]) -> Vec3 {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Vec3::uninitialized();
            ffi::graphene_vec3_init_from_float(vec.to_glib_none_mut().0, src.as_ptr() as *const _);
            vec
        }
    }

    #[doc(alias = "graphene_vec3_to_float")]
    pub fn to_float(&self) -> [f32; 3] {
        unsafe {
            let mut out = std::mem::MaybeUninit::uninit();
            ffi::graphene_vec3_to_float(self.to_glib_none().0, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
