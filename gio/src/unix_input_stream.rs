// Take a look at the license at the top of the repository in the LICENSE file.

use crate::InputStream;
use crate::UnixInputStream;
use glib::object::{Cast, IsA};
use glib::translate::*;

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

#[cfg(all(not(unix), feature = "dox"))]
use socket::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

impl UnixInputStream {
    // rustdoc-stripper-ignore-next
    /// Creates a new [`Self`] that takes ownership of the passed in fd.
    ///
    /// # Safety
    /// You must not close the fd unless you've previously called [`UnixInputStreamExtManual::set_close_fd`]
    /// with `true` on this stream. At which point you may only do so when all references to this
    /// stream have been dropped.
    #[doc(alias = "g_unix_input_stream_new")]
    pub unsafe fn take_fd<T: IntoRawFd>(fd: T) -> UnixInputStream {
        let fd = fd.into_raw_fd();
        let close_fd = true.into_glib();
        InputStream::from_glib_full(ffi::g_unix_input_stream_new(fd, close_fd)).unsafe_cast()
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new [`Self`] that does not take ownership of the passed in fd.
    ///
    /// # Safety
    /// You may only close the fd if all references to this stream have been dropped.
    #[doc(alias = "g_unix_input_stream_new")]
    pub unsafe fn with_fd<T: AsRawFd>(fd: T) -> UnixInputStream {
        let fd = fd.as_raw_fd();
        let close_fd = false.into_glib();
        InputStream::from_glib_full(ffi::g_unix_input_stream_new(fd, close_fd)).unsafe_cast()
    }
}

impl AsRawFd for UnixInputStream {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::g_unix_input_stream_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait UnixInputStreamExtManual: Sized {
    #[doc(alias = "g_unix_input_stream_get_fd")]
    #[doc(alias = "get_fd")]
    fn fd<T: FromRawFd>(&self) -> T;

    // rustdoc-stripper-ignore-next
    /// Sets whether the fd of this stream will be closed when the stream is closed.
    ///
    /// # Safety
    /// If you pass in `false` as the parameter, you may only close the fd if the all references
    /// to the stream have been dropped. If you pass in `true`, you must never call close.
    #[doc(alias = "g_unix_input_stream_set_close_fd")]
    unsafe fn set_close_fd(&self, close_fd: bool);
}

impl<O: IsA<UnixInputStream>> UnixInputStreamExtManual for O {
    fn fd<T: FromRawFd>(&self) -> T {
        unsafe {
            T::from_raw_fd(ffi::g_unix_input_stream_get_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    unsafe fn set_close_fd(&self, close_fd: bool) {
        ffi::g_unix_input_stream_set_close_fd(self.as_ref().to_glib_none().0, close_fd.into_glib());
    }
}
