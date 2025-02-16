// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Pixbuf, PixbufAnimation, PixbufFormat};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GdkPixbufLoader")]
    pub struct PixbufLoader(Object<ffi::GdkPixbufLoader, ffi::GdkPixbufLoaderClass>);

    match fn {
        type_ => || ffi::gdk_pixbuf_loader_get_type(),
    }
}

impl PixbufLoader {
    pub const NONE: Option<&'static PixbufLoader> = None;

    #[doc(alias = "gdk_pixbuf_loader_new")]
    pub fn new() -> PixbufLoader {
        unsafe { from_glib_full(ffi::gdk_pixbuf_loader_new()) }
    }

    #[doc(alias = "gdk_pixbuf_loader_new_with_mime_type")]
    #[doc(alias = "new_with_mime_type")]
    pub fn with_mime_type(mime_type: &str) -> Result<PixbufLoader, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret =
                ffi::gdk_pixbuf_loader_new_with_mime_type(mime_type.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_new_with_type")]
    #[doc(alias = "new_with_type")]
    pub fn with_type(image_type: &str) -> Result<PixbufLoader, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::gdk_pixbuf_loader_new_with_type(image_type.to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl Default for PixbufLoader {
    fn default() -> Self {
        Self::new()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PixbufLoader>> Sealed for T {}
}

pub trait PixbufLoaderExt: IsA<PixbufLoader> + sealed::Sealed + 'static {
    #[doc(alias = "gdk_pixbuf_loader_close")]
    fn close(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gdk_pixbuf_loader_close(self.as_ref().to_glib_none().0, &mut error);
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_get_animation")]
    #[doc(alias = "get_animation")]
    fn animation(&self) -> Option<PixbufAnimation> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_animation(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_get_format")]
    #[doc(alias = "get_format")]
    fn format(&self) -> Option<PixbufFormat> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_format(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_get_pixbuf")]
    #[doc(alias = "get_pixbuf")]
    fn pixbuf(&self) -> Option<Pixbuf> {
        unsafe {
            from_glib_none(ffi::gdk_pixbuf_loader_get_pixbuf(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_set_size")]
    fn set_size(&self, width: i32, height: i32) {
        unsafe {
            ffi::gdk_pixbuf_loader_set_size(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_write")]
    fn write(&self, buf: &[u8]) -> Result<(), glib::Error> {
        let count = buf.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gdk_pixbuf_loader_write(
                self.as_ref().to_glib_none().0,
                buf.to_glib_none().0,
                count,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "gdk_pixbuf_loader_write_bytes")]
    fn write_bytes(&self, buffer: &glib::Bytes) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::gdk_pixbuf_loader_write_bytes(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "area-prepared")]
    fn connect_area_prepared<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn area_prepared_trampoline<P: IsA<PixbufLoader>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkPixbufLoader,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PixbufLoader::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"area-prepared\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    area_prepared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "area-updated")]
    fn connect_area_updated<F: Fn(&Self, i32, i32, i32, i32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn area_updated_trampoline<
            P: IsA<PixbufLoader>,
            F: Fn(&P, i32, i32, i32, i32) + 'static,
        >(
            this: *mut ffi::GdkPixbufLoader,
            x: std::ffi::c_int,
            y: std::ffi::c_int,
            width: std::ffi::c_int,
            height: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                PixbufLoader::from_glib_borrow(this).unsafe_cast_ref(),
                x,
                y,
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"area-updated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    area_updated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<PixbufLoader>, F: Fn(&P) + 'static>(
            this: *mut ffi::GdkPixbufLoader,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(PixbufLoader::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "size-prepared")]
    fn connect_size_prepared<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn size_prepared_trampoline<
            P: IsA<PixbufLoader>,
            F: Fn(&P, i32, i32) + 'static,
        >(
            this: *mut ffi::GdkPixbufLoader,
            width: std::ffi::c_int,
            height: std::ffi::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                PixbufLoader::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"size-prepared\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    size_prepared_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<PixbufLoader>> PixbufLoaderExt for O {}
