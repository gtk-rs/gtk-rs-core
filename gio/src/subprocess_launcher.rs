// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(any(unix, all(docsrs, unix)))]
use std::os::unix::io::IntoRawFd;

use glib::translate::*;

use crate::ffi;
use crate::SubprocessLauncher;

#[cfg(all(docsrs, not(unix)))]
pub trait IntoRawFd: Sized {
    fn into_raw_fd(self) -> i32 {
        0
    }
}

impl SubprocessLauncher {
    #[doc(alias = "g_subprocess_launcher_set_environ")]
    pub fn set_environ(&self, env: &[std::ffi::OsString]) {
        unsafe {
            ffi::g_subprocess_launcher_set_environ(self.to_glib_none().0, env.to_glib_none().0);
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_fd")]
    pub fn take_fd(&self, source_fd: impl IntoRawFd, target_fd: impl IntoRawFd) {
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
    #[doc(alias = "g_subprocess_launcher_take_stderr_fd")]
    pub fn take_stderr_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stderr_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdin_fd")]
    pub fn take_stdin_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdin_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_take_stdout_fd")]
    pub fn take_stdout_fd(&self, fd: impl IntoRawFd) {
        unsafe {
            ffi::g_subprocess_launcher_take_stdout_fd(self.to_glib_none().0, fd.into_raw_fd());
        }
    }
}
