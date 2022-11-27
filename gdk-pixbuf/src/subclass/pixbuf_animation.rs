// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`PixbufAnimation`](crate::PixbufAnimation).

use std::mem::MaybeUninit;
use std::time::Duration;

use crate::{Pixbuf, PixbufAnimation, PixbufAnimationIter};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait PixbufAnimationImpl: ObjectImpl {
    fn is_static_image(&self) -> bool {
        self.parent_is_static_image()
    }

    fn static_image(&self) -> Option<Pixbuf> {
        self.parent_static_image()
    }

    fn size(&self) -> (i32, i32) {
        self.parent_size()
    }

    fn iter(&self, start_time: Duration) -> PixbufAnimationIter {
        self.parent_iter(start_time)
    }
}

pub trait PixbufAnimationImplExt: ObjectSubclass {
    fn parent_is_static_image(&self) -> bool;
    fn parent_static_image(&self) -> Option<Pixbuf>;
    fn parent_size(&self) -> (i32, i32);
    fn parent_iter(&self, start_time: Duration) -> PixbufAnimationIter;
}

impl<T: PixbufAnimationImpl> PixbufAnimationImplExt for T {
    fn parent_is_static_image(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationClass;
            let f = (*parent_class)
                .is_static_image
                .expect("No parent class implementation for \"is_static_image\"");

            from_glib(f(self
                .obj()
                .unsafe_cast_ref::<PixbufAnimation>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_static_image(&self) -> Option<Pixbuf> {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationClass;
            let f = (*parent_class)
                .get_static_image
                .expect("No parent class implementation for \"get_static_image\"");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<PixbufAnimation>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_size(&self) -> (i32, i32) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationClass;
            let f = (*parent_class)
                .get_size
                .expect("No parent class implementation for \"get_size\"");
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();
            f(
                self.obj()
                    .unsafe_cast_ref::<PixbufAnimation>()
                    .to_glib_none()
                    .0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    fn parent_iter(&self, start_time: Duration) -> PixbufAnimationIter {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationClass;
            let f = (*parent_class)
                .get_iter
                .expect("No parent class implementation for \"get_iter\"");

            let time = glib::ffi::GTimeVal {
                tv_sec: start_time.as_secs() as _,
                tv_usec: start_time.subsec_micros() as _,
            };
            from_glib_full(f(
                self.obj()
                    .unsafe_cast_ref::<PixbufAnimation>()
                    .to_glib_none()
                    .0,
                &time as *const _,
            ))
        }
    }
}

unsafe impl<T: PixbufAnimationImpl> IsSubclassable<T> for PixbufAnimation {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.get_static_image = Some(animation_get_static_image::<T>);
        klass.get_size = Some(animation_get_size::<T>);
        klass.get_iter = Some(animation_get_iter::<T>);
        klass.is_static_image = Some(animation_is_static_image::<T>);
    }
}

unsafe extern "C" fn animation_is_static_image<T: PixbufAnimationImpl>(
    ptr: *mut ffi::GdkPixbufAnimation,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.is_static_image().into_glib()
}

unsafe extern "C" fn animation_get_size<T: PixbufAnimationImpl>(
    ptr: *mut ffi::GdkPixbufAnimation,
    width_ptr: *mut libc::c_int,
    height_ptr: *mut libc::c_int,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let (width, height) = imp.size();
    *width_ptr = width;
    *height_ptr = height;
}

unsafe extern "C" fn animation_get_static_image<T: PixbufAnimationImpl>(
    ptr: *mut ffi::GdkPixbufAnimation,
) -> *mut ffi::GdkPixbuf {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.static_image().to_glib_none().0
}

unsafe extern "C" fn animation_get_iter<T: PixbufAnimationImpl>(
    ptr: *mut ffi::GdkPixbufAnimation,
    start_time_ptr: *const glib::ffi::GTimeVal,
) -> *mut ffi::GdkPixbufAnimationIter {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let total = Duration::from_secs((*start_time_ptr).tv_sec.try_into().unwrap())
        + Duration::from_micros((*start_time_ptr).tv_usec.try_into().unwrap());

    imp.iter(total).to_glib_full()
}
