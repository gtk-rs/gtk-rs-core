// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

use glib::translate::*;

use crate::{Point3D, Ray, Vec3};

impl Ray {
    #[doc(alias = "graphene_ray_init")]
    pub fn new(origin: Option<&Point3D>, direction: Option<&Vec3>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut ray = Self::uninitialized();
            ffi::graphene_ray_init(
                ray.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
            ray
        }
    }

    #[doc(alias = "graphene_ray_init_from_vec3")]
    #[doc(alias = "init_from_vec3")]
    pub fn from_vec3(origin: Option<&Vec3>, direction: Option<&Vec3>) -> Self {
        assert_initialized_main_thread!();
        unsafe {
            let mut ray = Self::uninitialized();
            ffi::graphene_ray_init_from_vec3(
                ray.to_glib_none_mut().0,
                origin.to_glib_none().0,
                direction.to_glib_none().0,
            );
            ray
        }
    }
}

impl fmt::Debug for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Ray")
            .field("origin", &self.origin())
            .field("direction", &self.direction())
            .finish()
    }
}
