// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Point3D;
use crate::Vec3;
use glib::translate::*;

impl Point3D {
    #[doc(alias = "graphene_point3d_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point3D::uninitialized();
            ffi::graphene_point3d_init(p.to_glib_none_mut().0, x, y, z);
            p
        }
    }

    #[doc(alias = "graphene_point3d_init_from_point")]
    #[doc(alias = "new_from_point")]
    pub fn from_point(src: &Point3D) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point3D::uninitialized();
            ffi::graphene_point3d_init_from_point(p.to_glib_none_mut().0, src.to_glib_none().0);
            p
        }
    }

    #[doc(alias = "graphene_point3d_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(v: &Vec3) -> Point3D {
        assert_initialized_main_thread!();
        unsafe {
            let mut p = Point3D::uninitialized();
            ffi::graphene_point3d_init_from_vec3(p.to_glib_none_mut().0, v.to_glib_none().0);
            p
        }
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn set_x(&mut self, x: f32) {
        self.0.x = x;
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }

    pub fn set_y(&mut self, y: f32) {
        self.0.y = y;
    }

    pub fn z(&self) -> f32 {
        self.0.z
    }

    pub fn set_z(&mut self, z: f32) {
        self.0.z = z;
    }
}
