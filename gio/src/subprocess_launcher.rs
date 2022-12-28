// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(unix, all(docsrs, unix)))]
use std::os::unix::io::{AsRawFd, IntoRawFd, OwnedFd};

#[cfg(unix)]
use glib::translate::*;

use crate::SubprocessLauncher;

#[cfg(all(docsrs, not(unix)))]
pub trait IntoRawFd: Sized {
    fn into_raw_fd(self) -> i32 {
        0
    }
}

impl SubprocessLauncher {
    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_fd")]
    pub unsafe fn take_fd(&self, source_fd: impl IntoRawFd, target_fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_fd(
                self.to_glib_none().0,
                source_fd.into_raw_fd(),
                target_fd.into_raw_fd(),
            );
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_fd")]
    pub fn take_owned_fd(&self, source_fd: Option<OwnedFd>, target_fd: Option<OwnedFd>) {
        let source_raw_fd = source_fd.map(|fd| fd.as_raw_fd()).unwrap_or(-1);
        let target_raw_fd = target_fd.map(|fd| fd.as_raw_fd()).unwrap_or(-1);
        unsafe { self.take_fd(source_raw_fd, target_raw_fd) }
    }

    #[cfg(any(unix, docsrs))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stderr_fd")]
    pub unsafe fn take_stderr_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stderr_fd")]
    pub fn take_stderr_owned_fd(&self, fd: OwnedFd) {
        unsafe {
            self.take_stderr_fd(fd);
        }
    }

    #[cfg(any(unix, docsrs))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdin_fd")]
    pub unsafe fn take_stdin_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdin_fd")]
    pub fn take_stdin_owned_fd(&self, fd: OwnedFd) {
        unsafe { self.take_stdin_fd(fd) }
    }

    #[cfg(any(unix, docsrs))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdout_fd")]
    pub unsafe fn take_stdout_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(any(unix, docsrs))]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdout_fd")]
    pub fn take_stdout_owned_fd(&self, fd: OwnedFd) {
        unsafe { self.take_stdout_fd(fd) }
    }
}
