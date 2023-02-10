// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::*;

use crate::Vec3;

impl Vec3 {
    #[doc(alias = "graphene_vec3_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Self::uninitialized();
            ffi::graphene_vec3_init(vec.to_glib_none_mut().0, x, y, z);
            vec
        }
    }

    #[doc(alias = "graphene_vec3_init_from_float")]
    #[doc(alias = "init_from_float")]
    pub fn from_float(src: [f32; 3]) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut vec = Self::uninitialized();
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

impl fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vec3")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Self::zero()
    }
}
