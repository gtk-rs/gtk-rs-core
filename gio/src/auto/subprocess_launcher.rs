// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Subprocess, SubprocessFlags};
use glib::translate::*;
#[cfg(unix)]
#[cfg_attr(docsrs, doc(cfg(unix)))]
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GSubprocessLauncher")]
    pub struct SubprocessLauncher(Object<ffi::GSubprocessLauncher>);

    match fn {
        type_ => || ffi::g_subprocess_launcher_get_type(),
    }
}

impl SubprocessLauncher {
    #[doc(alias = "g_subprocess_launcher_new")]
    pub fn new(flags: SubprocessFlags) -> SubprocessLauncher {
        unsafe { from_glib_full(ffi::g_subprocess_launcher_new(flags.into_glib())) }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[cfg(feature = "v2_68")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
    #[doc(alias = "g_subprocess_launcher_close")]
    pub fn close(&self) {
        unsafe {
            ffi::g_subprocess_launcher_close(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_subprocess_launcher_getenv")]
    pub fn getenv(&self, variable: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
        unsafe {
            from_glib_none(ffi::g_subprocess_launcher_getenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_child_setup")]
    pub fn set_child_setup<P: Fn() + 'static>(&self, child_setup: P) {
        let child_setup_data: Box_<P> = Box_::new(child_setup);
        unsafe extern "C" fn child_setup_func<P: Fn() + 'static>(data: glib::ffi::gpointer) {
            let callback = &*(data as *mut P);
            (*callback)()
        }
        let child_setup = Some(child_setup_func::<P> as _);
        unsafe extern "C" fn destroy_notify_func<P: Fn() + 'static>(data: glib::ffi::gpointer) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(destroy_notify_func::<P> as _);
        let super_callback0: Box_<P> = child_setup_data;
        unsafe {
            ffi::g_subprocess_launcher_set_child_setup(
                self.to_glib_none().0,
                child_setup,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_set_cwd")]
    pub fn set_cwd(&self, cwd: impl AsRef<std::path::Path>) {
        unsafe {
            ffi::g_subprocess_launcher_set_cwd(
                self.to_glib_none().0,
                cwd.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_set_flags")]
    pub fn set_flags(&self, flags: SubprocessFlags) {
        unsafe {
            ffi::g_subprocess_launcher_set_flags(self.to_glib_none().0, flags.into_glib());
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stderr_file_path")]
    pub fn set_stderr_file_path(&self, path: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::g_subprocess_launcher_set_stderr_file_path(
                self.to_glib_none().0,
                path.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stdin_file_path")]
    pub fn set_stdin_file_path(&self, path: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdin_file_path(
                self.to_glib_none().0,
                path.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[cfg(unix)]
    #[cfg_attr(docsrs, doc(cfg(unix)))]
    #[doc(alias = "g_subprocess_launcher_set_stdout_file_path")]
    pub fn set_stdout_file_path(&self, path: Option<impl AsRef<std::path::Path>>) {
        unsafe {
            ffi::g_subprocess_launcher_set_stdout_file_path(
                self.to_glib_none().0,
                path.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_subprocess_launcher_setenv")]
    pub fn setenv(
        &self,
        variable: impl AsRef<std::ffi::OsStr>,
        value: impl AsRef<std::ffi::OsStr>,
        overwrite: bool,
    ) {
        unsafe {
            ffi::g_subprocess_launcher_setenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
                value.as_ref().to_glib_none().0,
                overwrite.into_glib(),
            );
        }
    }

    //#[doc(alias = "g_subprocess_launcher_spawn")]
    //pub fn spawn(&self, error: &mut glib::Error, argv0: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Subprocess {
    //    unsafe { TODO: call ffi:g_subprocess_launcher_spawn() }
    //}

    #[doc(alias = "g_subprocess_launcher_spawnv")]
    #[doc(alias = "spawnv")]
    pub fn spawn(&self, argv: &[&std::ffi::OsStr]) -> Result<Subprocess, glib::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_subprocess_launcher_spawnv(
                self.to_glib_none().0,
                argv.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_subprocess_launcher_unsetenv")]
    pub fn unsetenv(&self, variable: impl AsRef<std::ffi::OsStr>) {
        unsafe {
            ffi::g_subprocess_launcher_unsetenv(
                self.to_glib_none().0,
                variable.as_ref().to_glib_none().0,
            );
        }
    }
}
