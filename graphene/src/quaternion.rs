// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Euler;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use crate::Vec4;
use glib::translate::*;

impl Quaternion {
    #[doc(alias = "graphene_quaternion_init")]
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init(quat.to_glib_none_mut().0, x, y, z, w);
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angle_vec3")]
    #[doc(alias = "new_from_angle_vec3")]
    pub fn from_angle_vec3(angle: f32, axis: &Vec3) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_angle_vec3(
                quat.to_glib_none_mut().0,
                angle,
                axis.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_angles")]
    #[doc(alias = "new_from_angles")]
    pub fn from_angles(deg_x: f32, deg_y: f32, deg_z: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_angles(
                quat.to_glib_none_mut().0,
                deg_x,
                deg_y,
                deg_z,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_euler")]
    #[doc(alias = "new_from_euler")]
    pub fn from_euler(e: &Euler) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_euler(quat.to_glib_none_mut().0, e.to_glib_none().0);
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_matrix")]
    #[doc(alias = "new_from_matrix")]
    pub fn from_matrix(m: &Matrix) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_matrix(
                quat.to_glib_none_mut().0,
                m.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_quaternion")]
    #[doc(alias = "new_from_quaternion")]
    pub fn from_quaternion(src: &Quaternion) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_quaternion(
                quat.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_radians")]
    #[doc(alias = "new_from_radians")]
    pub fn from_radians(rad_x: f32, rad_y: f32, rad_z: f32) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_radians(
                quat.to_glib_none_mut().0,
                rad_x,
                rad_y,
                rad_z,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_from_vec4")]
    #[doc(alias = "new_from_vec4")]
    pub fn from_vec4(src: &Vec4) -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_from_vec4(
                quat.to_glib_none_mut().0,
                src.to_glib_none().0,
            );
            quat
        }
    }

    #[doc(alias = "graphene_quaternion_init_identity")]
    pub fn new_identity() -> Quaternion {
        assert_initialized_main_thread!();
        unsafe {
            let mut quat = Quaternion::uninitialized();
            ffi::graphene_quaternion_init_identity(quat.to_glib_none_mut().0);
            quat
        }
    }
}
