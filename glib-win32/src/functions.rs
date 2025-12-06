// Take a look at the license at the top of the repository in the LICENSE file.
use glib::translate::*;
use std::path::PathBuf;

#[doc(alias = "g_win32_get_package_installation_directory_of_module")]
#[doc(alias = "get_package_installation_directory_of_module")]
#[cfg(all(docsrs, windows))]
pub fn package_installation_directory_of_module(
    hmodule: std::os::windows::raw::HANDLE,
) -> Result<PathBuf, std::io::Error> {
    // # Safety
    // The underlying `GetModuleFilenameW` function has three possible
    // outcomes when a raw pointer get passed to it:
    // - When the pointer is a valid HINSTANCE of a DLL (e.g. acquired
    // through the `GetModuleHandleW`), it sets a file path to the
    // assigned "out" buffer and sets the return value to be the length
    // of said path string
    // - When the pointer is null, it sets the full path of the process'
    // executable binary to the assigned buffer and sets the return value
    // to be the length of said string
    // - Whenever the provided buffer size is too small, it will set a
    // truncated version of the path and return the length of said string
    // while also setting the thread-local last-error code to
    // `ERROR_INSUFFICIENT_BUFFER` (evaluates to 0x7A)
    // - When the pointer is not a valid HINSTANCE that isn't NULL (e.g.
    // a pointer to some GKeyFile), it will return 0 and set the last-error
    // code to `ERROR_MOD_NOT_FOUND` (evaluates to 0x7E)
    //
    // The `g_win32_get_package_installation_directory_of_module` already
    // handles all of the outcomes gracefully by:
    // - Preallocating a MAX_PATH-long array of wchar_t for the out buffer,
    // so that outcome #3 can be safely assumed to never happen
    // - Returning NULL when outcome #4 happens
    match unsafe {
        from_glib_full::<_, Option<PathBuf>>(
            ffi::g_win32_get_package_installation_directory_of_module(hmodule),
        )
    } {
        Some(pb) => Ok(pb),
        None => Err(std::io::Error::last_os_error()),
    }
}
