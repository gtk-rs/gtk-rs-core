// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::ffi::{OsStr, OsString};
use std::mem;
use std::ptr;

// rustdoc-stripper-ignore-next
/// Same as [`get_prgname()`].
///
/// [`get_prgname()`]: fn.get_prgname.html
#[doc(alias = "get_program_name")]
pub fn program_name() -> Option<String> {
    prgname()
}

#[doc(alias = "g_get_prgname")]
#[doc(alias = "get_prgname")]
pub fn prgname() -> Option<String> {
    unsafe { from_glib_none(ffi::g_get_prgname()) }
}

// rustdoc-stripper-ignore-next
/// Same as [`set_prgname()`].
///
/// [`set_prgname()`]: fn.set_prgname.html
pub fn set_program_name(name: Option<&str>) {
    set_prgname(name)
}

#[doc(alias = "g_set_prgname")]
pub fn set_prgname(name: Option<&str>) {
    unsafe { ffi::g_set_prgname(name.to_glib_none().0) }
}

#[doc(alias = "g_environ_getenv")]
pub fn environ_getenv<K: AsRef<OsStr>>(envp: &[OsString], variable: K) -> Option<OsString> {
    unsafe {
        from_glib_none(ffi::g_environ_getenv(
            envp.to_glib_none().0,
            variable.as_ref().to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_mkstemp")]
pub fn mkstemp<P: AsRef<std::path::Path>>(tmpl: P) -> i32 {
    unsafe {
        // NOTE: This modifies the string in place, which is fine here because
        // to_glib_none() will create a temporary, NUL-terminated copy of the string.
        ffi::g_mkstemp(tmpl.as_ref().to_glib_none().0)
    }
}

#[doc(alias = "g_mkstemp_full")]
pub fn mkstemp_full(tmpl: impl AsRef<std::path::Path>, flags: i32, mode: i32) -> i32 {
    unsafe {
        // NOTE: This modifies the string in place, which is fine here because
        // to_glib_none() will create a temporary, NUL-terminated copy of the string.
        ffi::g_mkstemp_full(tmpl.as_ref().to_glib_none().0, flags, mode)
    }
}

#[doc(alias = "g_mkdtemp")]
pub fn mkdtemp(tmpl: impl AsRef<std::path::Path>) -> Option<std::path::PathBuf> {
    unsafe {
        // NOTE: This modifies the string in place and returns it but does not free it
        // if it returns NULL.
        let tmpl = tmpl.as_ref().to_glib_full();
        let res = ffi::g_mkdtemp(tmpl);
        if res.is_null() {
            ffi::g_free(tmpl as ffi::gpointer);
            None
        } else {
            from_glib_full(res)
        }
    }
}

#[doc(alias = "g_mkdtemp_full")]
pub fn mkdtemp_full(tmpl: impl AsRef<std::path::Path>, mode: i32) -> Option<std::path::PathBuf> {
    unsafe {
        // NOTE: This modifies the string in place and returns it but does not free it
        // if it returns NULL.
        let tmpl = tmpl.as_ref().to_glib_full();
        let res = ffi::g_mkdtemp_full(tmpl, mode);
        if res.is_null() {
            ffi::g_free(tmpl as ffi::gpointer);
            None
        } else {
            from_glib_full(res)
        }
    }
}

#[doc(alias = "g_file_get_contents")]
pub fn file_get_contents(
    filename: impl AsRef<std::path::Path>,
) -> Result<crate::Slice<u8>, crate::Error> {
    unsafe {
        let mut contents = ptr::null_mut();
        let mut length = mem::MaybeUninit::uninit();
        let mut error = ptr::null_mut();
        let _ = ffi::g_file_get_contents(
            filename.as_ref().to_glib_none().0,
            &mut contents,
            length.as_mut_ptr(),
            &mut error,
        );
        if error.is_null() {
            Ok(crate::Slice::from_glib_full_num_copy(
                contents,
                length.assume_init() as _,
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

pub fn is_canonical_pspec_name(name: &str) -> bool {
    name.as_bytes().iter().enumerate().all(|(i, c)| {
        i != 0 && (*c >= b'0' && *c <= b'9' || *c == b'-')
            || (*c >= b'A' && *c <= b'Z')
            || (*c >= b'a' && *c <= b'z')
    })
}

#[doc(alias = "g_uri_escape_string")]
pub fn uri_escape_string(
    unescaped: &str,
    reserved_chars_allowed: Option<&str>,
    allow_utf8: bool,
) -> crate::GString {
    unsafe {
        from_glib_full(ffi::g_uri_escape_string(
            unescaped.to_glib_none().0,
            reserved_chars_allowed.to_glib_none().0,
            allow_utf8.into_glib(),
        ))
    }
}

#[doc(alias = "g_uri_unescape_string")]
pub fn uri_unescape_string(
    escaped_string: &str,
    illegal_characters: Option<&str>,
) -> Option<crate::GString> {
    unsafe {
        from_glib_full(ffi::g_uri_unescape_string(
            escaped_string.to_glib_none().0,
            illegal_characters.to_glib_none().0,
        ))
    }
}

#[doc(alias = "g_uri_parse_scheme")]
pub fn uri_parse_scheme(uri: &str) -> Option<crate::GString> {
    unsafe { from_glib_full(ffi::g_uri_parse_scheme(uri.to_glib_none().0)) }
}

#[doc(alias = "g_uri_unescape_segment")]
pub fn uri_unescape_segment(
    escaped_string: Option<&str>,
    escaped_string_end: Option<&str>,
    illegal_characters: Option<&str>,
) -> Option<crate::GString> {
    unsafe {
        from_glib_full(ffi::g_uri_unescape_segment(
            escaped_string.to_glib_none().0,
            escaped_string_end.to_glib_none().0,
            illegal_characters.to_glib_none().0,
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Mutex;

    //Mutex to prevent run environment tests parallel
    static LOCK: once_cell::sync::Lazy<Mutex<()>> = once_cell::sync::Lazy::new(|| Mutex::new(()));

    const VAR_NAME: &str = "function_environment_test";

    fn check_getenv(val: &str) {
        let _data = LOCK.lock().unwrap();

        env::set_var(VAR_NAME, val);
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
        assert_eq!(crate::getenv(VAR_NAME), Some(val.into()));

        let environ = crate::environ();
        assert_eq!(crate::environ_getenv(&environ, VAR_NAME), Some(val.into()));
    }

    fn check_setenv(val: &str) {
        let _data = LOCK.lock().unwrap();

        crate::setenv(VAR_NAME, val, true).unwrap();
        assert_eq!(env::var_os(VAR_NAME), Some(val.into()));
    }

    #[test]
    fn getenv() {
        check_getenv("Test");
        check_getenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn setenv() {
        check_setenv("Test");
        check_setenv("Тест"); // "Test" in Russian
    }

    #[test]
    fn test_filename_from_uri() {
        use crate::GString;
        use std::path::PathBuf;
        let uri: GString = "file:///foo/bar.txt".into();
        if let Ok((filename, hostname)) = crate::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, None);
        } else {
            unreachable!();
        }

        let uri: GString = "file://host/foo/bar.txt".into();
        if let Ok((filename, hostname)) = crate::filename_from_uri(&uri) {
            assert_eq!(filename, PathBuf::from(r"/foo/bar.txt"));
            assert_eq!(hostname, Some(GString::from("host")));
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_uri_parsing() {
        use crate::GString;
        assert_eq!(
            crate::uri_parse_scheme("foo://bar"),
            Some(GString::from("foo"))
        );
        assert_eq!(crate::uri_parse_scheme("foo"), None);

        let escaped = crate::uri_escape_string("&foo", None, true);
        assert_eq!(escaped, GString::from("%26foo"));

        let unescaped = crate::uri_unescape_string(escaped.as_str(), None);
        assert_eq!(unescaped, Some(GString::from("&foo")));

        assert_eq!(
            crate::uri_unescape_segment(Some("/foo"), None, None),
            Some(GString::from("/foo"))
        );
        assert_eq!(crate::uri_unescape_segment(Some("/foo%"), None, None), None);
    }
}
