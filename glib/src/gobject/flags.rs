// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;

bitflags::bitflags! {
    pub struct ParamFlags: u32 {
        const READABLE = gobject_ffi::G_PARAM_READABLE as u32;
        const WRITABLE = gobject_ffi::G_PARAM_WRITABLE as u32;
        const READWRITE = gobject_ffi::G_PARAM_READWRITE as u32;
        const CONSTRUCT = gobject_ffi::G_PARAM_CONSTRUCT as u32;
        const CONSTRUCT_ONLY = gobject_ffi::G_PARAM_CONSTRUCT_ONLY as u32;
        const LAX_VALIDATION = gobject_ffi::G_PARAM_LAX_VALIDATION as u32;
        const USER_1 = 256;
        const USER_2 = 1024;
        const USER_3 = 2048;
        const USER_4 = 4096;
        const USER_5 = 8192;
        const USER_6 = 16384;
        const USER_7 = 32768;
        const USER_8 = 65536;
        const EXPLICIT_NOTIFY = gobject_ffi::G_PARAM_EXPLICIT_NOTIFY as u32;
        const DEPRECATED = gobject_ffi::G_PARAM_DEPRECATED as u32;
    }
}

#[doc(hidden)]
impl IntoGlib for ParamFlags {
    type GlibType = gobject_ffi::GParamFlags;

    fn into_glib(self) -> gobject_ffi::GParamFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<gobject_ffi::GParamFlags> for ParamFlags {
    unsafe fn from_glib(value: gobject_ffi::GParamFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}
