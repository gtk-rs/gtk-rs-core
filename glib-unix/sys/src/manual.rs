use glib_sys::{GError, gboolean};
use libc::c_int;
pub use libc::passwd;

unsafe extern "C" {
    pub fn g_unix_open_pipe(
        fds: *mut [c_int; 2],
        flags: c_int,
        error: *mut *mut GError,
    ) -> gboolean;
}
