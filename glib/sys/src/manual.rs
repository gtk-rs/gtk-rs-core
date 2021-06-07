// Take a look at the license at the top of the repository in the LICENSE file.

#[allow(unused_imports)]
use libc::{c_char, c_int, c_ushort, c_void};

#[cfg(unix)]
pub use libc::passwd;

#[cfg(all(not(unix), feature = "dox"))]
#[repr(C)]
pub struct passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[cfg(windows)]
pub type GPid = *mut c_void;

#[cfg(not(windows))]
pub type GPid = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(all(windows, target_arch = "x86_64"))]
pub struct GPollFD {
    pub fd: i64,
    pub events: c_ushort,
    pub revents: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(all(windows, target_arch = "x86_64")))]
pub struct GPollFD {
    pub fd: c_int,
    pub events: c_ushort,
    pub revents: c_ushort,
}

#[cfg(target_family = "windows")]
pub use self::win32::*;

#[cfg(target_family = "windows")]
mod win32 {
    use crate::gpointer;
    use libc::c_char;

    extern "C" {
        pub fn g_win32_get_package_installation_directory_of_module(
            hmodule: gpointer,
        ) -> *mut c_char;
    }
}
