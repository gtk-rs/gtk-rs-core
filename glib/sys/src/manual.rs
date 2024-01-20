// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
pub use libc::passwd;
#[allow(unused_imports)]
use libc::{c_char, c_int, c_ushort, c_void};

pub type GType = libc::size_t;

#[cfg(all(not(unix), docsrs))]
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

// These are all non-NUL terminated strings in C
pub const G_VARIANT_TYPE_BOOLEAN: &str = "b";
pub const G_VARIANT_TYPE_BYTE: &str = "y";
pub const G_VARIANT_TYPE_INT16: &str = "n";
pub const G_VARIANT_TYPE_UINT16: &str = "q";
pub const G_VARIANT_TYPE_INT32: &str = "i";
pub const G_VARIANT_TYPE_UINT32: &str = "u";
pub const G_VARIANT_TYPE_INT64: &str = "x";
pub const G_VARIANT_TYPE_UINT64: &str = "t";
pub const G_VARIANT_TYPE_DOUBLE: &str = "d";
pub const G_VARIANT_TYPE_STRING: &str = "s";
pub const G_VARIANT_TYPE_OBJECT_PATH: &str = "o";
pub const G_VARIANT_TYPE_SIGNATURE: &str = "g";
pub const G_VARIANT_TYPE_VARIANT: &str = "v";
pub const G_VARIANT_TYPE_HANDLE: &str = "h";
pub const G_VARIANT_TYPE_UNIT: &str = "()";
pub const G_VARIANT_TYPE_ANY: &str = "*";
pub const G_VARIANT_TYPE_BASIC: &str = "?";
pub const G_VARIANT_TYPE_MAYBE: &str = "m*";
pub const G_VARIANT_TYPE_ARRAY: &str = "a*";
pub const G_VARIANT_TYPE_TUPLE: &str = "r";
pub const G_VARIANT_TYPE_DICT_ENTRY: &str = "{?*}";
pub const G_VARIANT_TYPE_DICTIONARY: &str = "a{?*}";
pub const G_VARIANT_TYPE_STRING_ARRAY: &str = "as";
pub const G_VARIANT_TYPE_OBJECT_PATH_ARRAY: &str = "ao";
pub const G_VARIANT_TYPE_BYTE_STRING: &str = "ay";
pub const G_VARIANT_TYPE_BYTE_STRING_ARRAY: &str = "ayy";
pub const G_VARIANT_TYPE_VARDICT: &str = "a{sv}";
