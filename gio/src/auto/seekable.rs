// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Cancellable};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GSeekable")]
    pub struct Seekable(Interface<ffi::GSeekable, ffi::GSeekableIface>);

    match fn {
        type_ => || ffi::g_seekable_get_type(),
    }
}

impl Seekable {
    pub const NONE: Option<&'static Seekable> = None;
}

pub trait SeekableExt: IsA<Seekable> + 'static {
    #[doc(alias = "g_seekable_can_seek")]
    fn can_seek(&self) -> bool {
        unsafe { from_glib(ffi::g_seekable_can_seek(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_seekable_can_truncate")]
    fn can_truncate(&self) -> bool {
        unsafe { from_glib(ffi::g_seekable_can_truncate(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_seekable_seek")]
    fn seek<'a, P: IsA<Cancellable>>(
        &self,
        offset: i64,
        type_: glib::SeekType,
        cancellable: impl Into<Option<&'a P>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_seekable_seek(
                self.as_ref().to_glib_none().0,
                offset,
                type_.into_glib(),
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
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

    #[doc(alias = "g_seekable_tell")]
    fn tell(&self) -> i64 {
        unsafe { ffi::g_seekable_tell(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_seekable_truncate")]
    fn truncate<'a, P: IsA<Cancellable>>(
        &self,
        offset: i64,
        cancellable: impl Into<Option<&'a P>>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_seekable_truncate(
                self.as_ref().to_glib_none().0,
                offset,
                cancellable
                    .into()
                    .as_ref()
                    .map(|p| p.as_ref())
                    .to_glib_none()
                    .0,
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
}

impl<O: IsA<Seekable>> SeekableExt for O {}
