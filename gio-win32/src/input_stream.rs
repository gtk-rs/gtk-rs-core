// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(windows)]
use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle, RawHandle};

use glib::{prelude::*, translate::*};

use crate::{InputStream, ffi};

impl InputStream {
    // rustdoc-stripper-ignore-next
    /// Creates a new [`Self`] that takes ownership of the passed in handle.
    ///
    /// # Safety
    /// You must not close the handle unless you've previously called [`InputStreamExtManual::set_close_handle`]
    /// with `true` on this stream. At which point you may only do so when all references to this
    /// stream have been dropped.
    #[doc(alias = "g_win32_input_stream_new")]
    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    pub unsafe fn take_handle(handle: impl IntoRawHandle) -> InputStream {
        let handle = handle.into_raw_handle();
        let close_handle = true.into_glib();
        gio::InputStream::from_glib_full(ffi::g_win32_input_stream_new(handle, close_handle))
            .unsafe_cast()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Self`] that does not take ownership of the passed in handle.
    ///
    /// # Safety
    /// You may only close the handle if all references to this stream have been dropped.
    #[doc(alias = "g_win32_input_stream_new")]
    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    pub unsafe fn with_handle<T: AsRawHandle>(handle: T) -> InputStream {
        let handle = handle.as_raw_handle();
        let close_handle = false.into_glib();
        gio::InputStream::from_glib_full(ffi::g_win32_input_stream_new(handle, close_handle))
            .unsafe_cast()
    }
}

#[cfg(windows)]
#[cfg_attr(docsrs, doc(cfg(windows)))]
impl AsRawHandle for InputStream {
    fn as_raw_handle(&self) -> RawHandle {
        unsafe { ffi::g_win32_input_stream_get_handle(self.to_glib_none().0) as _ }
    }
}

pub trait Win32InputStreamExtManual: IsA<InputStream> + Sized {
    #[doc(alias = "g_win32_input_stream_get_handle")]
    #[doc(alias = "get_handle")]
    #[cfg(windows)]
    #[cfg_attr(docsrs, doc(cfg(windows)))]
    fn handle<T: FromRawHandle>(&self) -> T {
        unsafe {
            T::from_raw_handle(ffi::g_win32_input_stream_get_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<InputStream>> Win32InputStreamExtManual for O {}
