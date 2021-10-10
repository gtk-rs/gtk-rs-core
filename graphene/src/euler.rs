// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Euler;
use crate::EulerOrder;
use crate::Matrix;
use crate::Quaternion;
use crate::Vec3;
use glib::translate::*;

impl Euler {
    #[doc(alias = "graphene_euler_init")]
    pub fn new(x: f32, y: f32, z: f32) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init(eul.to_glib_none_mut().0, x, y, z);
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_from_euler")]
    #[doc(alias = "new_from_euler")]
    pub fn from_euler(src: Option<&Euler>) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_from_euler(eul.to_glib_none_mut().0, src.to_glib_none().0);
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_from_matrix")]
    #[doc(alias = "new_from_matrix")]
    pub fn from_matrix(m: Option<&Matrix>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_from_matrix(
                eul.to_glib_none_mut().0,
                m.to_glib_none().0,
                order.into_glib(),
            );
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_from_quaternion")]
    #[doc(alias = "new_from_quaternion")]
    pub fn from_quaternion(q: Option<&Quaternion>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_from_quaternion(
                eul.to_glib_none_mut().0,
                q.to_glib_none().0,
                order.into_glib(),
            );
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_from_radians")]
    #[doc(alias = "new_from_radians")]
    pub fn from_radians(x: f32, y: f32, z: f32, order: EulerOrder) -> Euler {
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_from_radians(
                eul.to_glib_none_mut().0,
                x,
                y,
                z,
                order.into_glib(),
            );
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_from_vec3")]
    #[doc(alias = "new_from_vec3")]
    pub fn from_vec3(v: Option<&Vec3>, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_from_vec3(
                eul.to_glib_none_mut().0,
                v.to_glib_none().0,
                order.into_glib(),
            );
            eul
        }
    }

    #[doc(alias = "graphene_euler_init_with_order")]
    pub fn new_with_order(x: f32, y: f32, z: f32, order: EulerOrder) -> Euler {
        assert_initialized_main_thread!();
        unsafe {
            let mut eul = Euler::uninitialized();
            ffi::graphene_euler_init_with_order(
                eul.to_glib_none_mut().0,
                x,
                y,
                z,
                order.into_glib(),
            );
            eul
        }
    }
}
