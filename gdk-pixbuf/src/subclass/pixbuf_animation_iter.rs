// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits intended for subclassing [`PixbufAnimationIter`](crate::PixbufAnimationIter).

use std::time::Duration;

use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

use crate::{Pixbuf, PixbufAnimationIter};

pub trait PixbufAnimationIterImpl: ObjectImpl {
    // rustdoc-stripper-ignore-next
    /// Time in milliseconds, returning `None` implies showing the same pixbuf forever.
    fn delay_time(&self) -> Option<Duration> {
        self.parent_delay_time()
    }

    fn pixbuf(&self) -> Pixbuf {
        self.parent_pixbuf()
    }

    fn on_currently_loading_frame(&self) -> bool {
        self.parent_on_currently_loading_frame()
    }

    fn advance(&self, time: Duration) -> bool {
        self.parent_advance(time)
    }
}

pub trait PixbufAnimationIterImplExt: ObjectSubclass {
    fn parent_delay_time(&self) -> Option<Duration>;
    fn parent_pixbuf(&self) -> Pixbuf;
    fn parent_on_currently_loading_frame(&self) -> bool;
    fn parent_advance(&self, time: Duration) -> bool;
}

impl<T: PixbufAnimationIterImpl> PixbufAnimationIterImplExt for T {
    fn parent_delay_time(&self) -> Option<Duration> {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationIterClass;
            let f = (*parent_class)
                .get_delay_time
                .expect("No parent class implementation for \"get_delay_time\"");

            let time = f(self
                .obj()
                .unsafe_cast_ref::<PixbufAnimationIter>()
                .to_glib_none()
                .0);
            if time == -1 {
                None
            } else {
                Some(Duration::from_millis(time.try_into().unwrap()))
            }
        }
    }

    fn parent_pixbuf(&self) -> Pixbuf {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationIterClass;
            let f = (*parent_class)
                .get_pixbuf
                .expect("No parent class implementation for \"get_pixbuf\"");

            from_glib_none(f(self
                .obj()
                .unsafe_cast_ref::<PixbufAnimationIter>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_on_currently_loading_frame(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationIterClass;
            let f = (*parent_class)
                .on_currently_loading_frame
                .expect("No parent class implementation for \"on_currently_loading_frame\"");

            from_glib(f(self
                .obj()
                .unsafe_cast_ref::<PixbufAnimationIter>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_advance(&self, time: Duration) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class =
                data.as_ref().parent_class() as *mut ffi::GdkPixbufAnimationIterClass;
            let f = (*parent_class)
                .advance
                .expect("No parent class implementation for \"advance\"");

            let time = glib::ffi::GTimeVal {
                tv_sec: time.as_secs() as _,
                tv_usec: time.subsec_micros() as _,
            };
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<PixbufAnimationIter>()
                    .to_glib_none()
                    .0,
                &time as *const _,
            ))
        }
    }
}

unsafe impl<T: PixbufAnimationIterImpl> IsSubclassable<T> for PixbufAnimationIter {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.get_delay_time = Some(animation_iter_get_delay_time::<T>);
        klass.get_pixbuf = Some(animation_iter_get_pixbuf::<T>);
        klass.on_currently_loading_frame = Some(animation_iter_on_currently_loading_frame::<T>);
        klass.advance = Some(animation_iter_advance::<T>);
    }
}

unsafe extern "C" fn animation_iter_get_delay_time<T: PixbufAnimationIterImpl>(
    ptr: *mut ffi::GdkPixbufAnimationIter,
) -> i32 {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.delay_time().map(|t| t.as_millis() as i32).unwrap_or(-1)
}

unsafe extern "C" fn animation_iter_get_pixbuf<T: PixbufAnimationIterImpl>(
    ptr: *mut ffi::GdkPixbufAnimationIter,
) -> *mut ffi::GdkPixbuf {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.pixbuf().to_glib_none().0
}

unsafe extern "C" fn animation_iter_on_currently_loading_frame<T: PixbufAnimationIterImpl>(
    ptr: *mut ffi::GdkPixbufAnimationIter,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.on_currently_loading_frame().into_glib()
}

unsafe extern "C" fn animation_iter_advance<T: PixbufAnimationIterImpl>(
    ptr: *mut ffi::GdkPixbufAnimationIter,
    time_ptr: *const glib::ffi::GTimeVal,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    let total = Duration::from_secs((*time_ptr).tv_sec.try_into().unwrap())
        + Duration::from_micros((*time_ptr).tv_usec.try_into().unwrap());

    imp.advance(total).into_glib()
}
