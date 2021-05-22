// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::path::PathBuf;

#[doc(alias = "g_win32_get_package_installation_directory_of_module")]
pub fn win32_get_package_installation_directory_of_module(
    hmodule: ffi::gpointer,
) -> Option<PathBuf> {
    unsafe {
        from_glib_full(ffi::g_win32_get_package_installation_directory_of_module(
            hmodule,
        ))
    }
}
