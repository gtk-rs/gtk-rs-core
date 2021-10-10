// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Triangle;
use crate::Vec3;
use glib::translate::*;

impl Triangle {
    #[doc(alias = "graphene_triangle_init_from_float")]
    #[doc(alias = "new_from_float")]
    pub fn from_float(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let mut tri = Triangle::uninitialized();
            ffi::graphene_triangle_init_from_float(
                tri.to_glib_none_mut().0,
                a.as_ptr() as *const _,
                b.as_ptr() as *const _,
                c.as_ptr() as *const _,
            );
            tri
        }
    }

    #[doc(alias = "graphene_triangle_init_from_point3d")]
    #[doc(alias = "new_from_point3d")]
    pub fn from_point3d(a: Option<&Point3D>, b: Option<&Point3D>, c: Option<&Point3D>) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let mut tri = Triangle::uninitialized();
            ffi::graphene_triangle_init_from_point3d(
                tri.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            tri
        }
    }

    #[doc(alias = "graphene_triangle_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(a: Option<&Vec3>, b: Option<&Vec3>, c: Option<&Vec3>) -> Triangle {
        assert_initialized_main_thread!();
        unsafe {
            let mut tri = Triangle::uninitialized();
            ffi::graphene_triangle_init_from_vec3(
                tri.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                c.to_glib_none().0,
            );
            tri
        }
    }
}
