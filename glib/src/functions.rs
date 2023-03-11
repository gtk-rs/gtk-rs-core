// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(not(windows))]
use std::boxed::Box as Box_;
#[cfg(not(windows))]
use std::mem;
#[cfg(not(windows))]
#[cfg(any(feature = "v2_58", feature = "dox"))]
use std::os::unix::io::AsRawFd;
#[cfg(not(windows))]
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::ptr;

// #[cfg(windows)]
// #[cfg(any(feature = "v2_58", feature = "dox"))]
// use std::os::windows::io::AsRawHandle;
use crate::{translate::*, GStr};
#[cfg(not(windows))]
use crate::{Error, Pid, SpawnFlags};

#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg(not(windows))]
#[cfg_attr(feature = "dox", doc(cfg(all(feature = "v2_58", not(windows)))))]
#[allow(clippy::too_many_arguments)]
#[doc(alias = "g_spawn_async_with_fds")]
pub fn spawn_async_with_fds<P: AsRef<std::path::Path>, T: AsRawFd, U: AsRawFd, V: AsRawFd>(
    working_directory: P,
    argv: &[&str],
    envp: &[&str],
    flags: SpawnFlags,
    child_setup: Option<Box_<dyn FnOnce() + 'static>>,
    stdin_fd: T,
    stdout_fd: U,
    stderr_fd: V,
) -> Result<Pid, Error> {
    let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
    unsafe extern "C" fn child_setup_func(user_data: ffi::gpointer) {
        let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
            Box_::from_raw(user_data as *mut _);
        let callback = (*callback).expect("cannot get closure...");
        callback()
    }
    let child_setup = if child_setup_data.is_some() {
        Some(child_setup_func as _)
    } else {
        None
    };
    let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
    unsafe {
        let mut child_pid = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::g_spawn_async_with_fds(
            working_directory.as_ref().to_glib_none().0,
            argv.to_glib_none().0,
            envp.to_glib_none().0,
            flags.into_glib(),
            child_setup,
            Box_::into_raw(super_callback0) as *mut _,
            child_pid.as_mut_ptr(),
            stdin_fd.as_raw_fd(),
            stdout_fd.as_raw_fd(),
            stderr_fd.as_raw_fd(),
            &mut error,
        );
        let child_pid = from_glib(child_pid.assume_init());
        if error.is_null() {
            Ok(child_pid)
        } else {
            Err(from_glib_full(error))
        }
    }
}

// #[cfg(any(feature = "v2_58", feature = "dox"))]
// #[cfg(windows)]
// pub fn spawn_async_with_fds<
//     P: AsRef<std::path::Path>,
//     T: AsRawHandle,
//     U: AsRawHandle,
//     V: AsRawHandle,
// >(
//     working_directory: P,
//     argv: &[&str],
//     envp: &[&str],
//     flags: SpawnFlags,
//     child_setup: Option<Box_<dyn FnOnce() + 'static>>,
//     stdin_fd: T,
//     stdout_fd: U,
//     stderr_fd: V,
// ) -> Result<Pid, Error> {
//     let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
//     unsafe extern "C" fn child_setup_func<P: AsRef<std::path::Path>>(
//         user_data: ffi::gpointer,
//     ) {
//         let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
//             Box_::from_raw(user_data as *mut _);
//         let callback = (*callback).expect("cannot get closure...");
//         callback()
//     }
//     let child_setup = if child_setup_data.is_some() {
//         Some(child_setup_func::<P> as _)
//     } else {
//         None
//     };
//     let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
//     unsafe {
//         let mut child_pid = mem::MaybeUninit::uninit();
//         let mut error = ptr::null_mut();
//         let _ = ffi::g_spawn_async_with_fds(
//             working_directory.as_ref().to_glib_none().0,
//             argv.to_glib_none().0,
//             envp.to_glib_none().0,
//             flags.into_glib(),
//             child_setup,
//             Box_::into_raw(super_callback0) as *mut _,
//             child_pid.as_mut_ptr(),
//             stdin_fd.as_raw_handle() as usize as _,
//             stdout_fd.as_raw_handle() as usize as _,
//             stderr_fd.as_raw_handle() as usize as _,
//             &mut error,
//         );
//         let child_pid = from_glib(child_pid.assume_init());
//         if error.is_null() {
//             Ok(child_pid)
//         } else {
//             Err(from_glib_full(error))
//         }
//     }
// }

#[cfg(not(windows))]
#[cfg_attr(feature = "dox", doc(cfg(not(windows))))]
#[doc(alias = "g_spawn_async_with_pipes")]
pub fn spawn_async_with_pipes<
    P: AsRef<std::path::Path>,
    T: FromRawFd,
    U: FromRawFd,
    V: FromRawFd,
>(
    working_directory: P,
    argv: &[&std::path::Path],
    envp: &[&std::path::Path],
    flags: SpawnFlags,
    child_setup: Option<Box_<dyn FnOnce() + 'static>>,
) -> Result<(Pid, T, U, V), Error> {
    let child_setup_data: Box_<Option<Box_<dyn FnOnce() + 'static>>> = Box_::new(child_setup);
    unsafe extern "C" fn child_setup_func(user_data: ffi::gpointer) {
        let callback: Box_<Option<Box_<dyn FnOnce() + 'static>>> =
            Box_::from_raw(user_data as *mut _);
        let callback = (*callback).expect("cannot get closure...");
        callback()
    }
    let child_setup = if child_setup_data.is_some() {
        Some(child_setup_func as _)
    } else {
        None
    };
    let super_callback0: Box_<Option<Box_<dyn FnOnce() + 'static>>> = child_setup_data;
    unsafe {
        let mut child_pid = mem::MaybeUninit::uninit();
        let mut standard_input = mem::MaybeUninit::uninit();
        let mut standard_output = mem::MaybeUninit::uninit();
        let mut standard_error = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::g_spawn_async_with_pipes(
            working_directory.as_ref().to_glib_none().0,
            argv.to_glib_none().0,
            envp.to_glib_none().0,
            flags.into_glib(),
            child_setup,
            Box_::into_raw(super_callback0) as *mut _,
            child_pid.as_mut_ptr(),
            standard_input.as_mut_ptr(),
            standard_output.as_mut_ptr(),
            standard_error.as_mut_ptr(),
            &mut error,
        );
        let child_pid = from_glib(child_pid.assume_init());
        let standard_input = standard_input.assume_init();
        let standard_output = standard_output.assume_init();
        let standard_error = standard_error.assume_init();
        if error.is_null() {
            #[cfg(not(windows))]
            {
                Ok((
                    child_pid,
                    FromRawFd::from_raw_fd(standard_input),
                    FromRawFd::from_raw_fd(standard_output),
                    FromRawFd::from_raw_fd(standard_error),
                ))
            }
        // #[cfg(windows)]
        // {
        //     use std::os::windows::io::{FromRawHandle, RawHandle};
        //     Ok((
        //         child_pid,
        //         File::from_raw_handle(standard_input as usize as RawHandle),
        //         File::from_raw_handle(standard_output as usize as RawHandle),
        //         File::from_raw_handle(standard_error as usize as RawHandle),
        //     ))
        // }
        } else {
            Err(from_glib_full(error))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Obtain the character set for the current locale.
///
/// This returns whether the locale's encoding is UTF-8, and the current
/// charset if available.
#[doc(alias = "g_get_charset")]
#[doc(alias = "get_charset")]
pub fn charset() -> (bool, Option<&'static GStr>) {
    unsafe {
        let mut out_charset = ptr::null();
        let is_utf8 = from_glib(ffi::g_get_charset(&mut out_charset));
        let charset = from_glib_none(out_charset);
        (is_utf8, charset)
    }
}

#[cfg(unix)]
#[doc(alias = "g_unix_open_pipe")]
pub fn unix_open_pipe(flags: i32) -> Result<(RawFd, RawFd), Error> {
    unsafe {
        let mut fds = [0, 2];
        let mut error = ptr::null_mut();
        let _ = ffi::g_unix_open_pipe(&mut fds, flags, &mut error);
        if error.is_null() {
            Ok((
                FromRawFd::from_raw_fd(fds[0]),
                FromRawFd::from_raw_fd(fds[1]),
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(unix)]
#[doc(alias = "g_file_open_tmp")]
pub fn file_open_tmp(
    tmpl: Option<impl AsRef<std::path::Path>>,
) -> Result<(RawFd, std::path::PathBuf), crate::Error> {
    unsafe {
        let mut name_used = ptr::null_mut();
        let mut error = ptr::null_mut();
        let ret = ffi::g_file_open_tmp(
            tmpl.as_ref().map(|p| p.as_ref()).to_glib_none().0,
            &mut name_used,
            &mut error,
        );
        if error.is_null() {
            Ok((ret.into_raw_fd(), from_glib_full(name_used)))
        } else {
            Err(from_glib_full(error))
        }
    }
}
