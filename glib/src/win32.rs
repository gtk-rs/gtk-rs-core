// Take a look at the license at the top of the repository in the LICENSE file.

use std::path::PathBuf;

use crate::translate::*;

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
