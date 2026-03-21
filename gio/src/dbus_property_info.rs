// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DBusPropertyInfo, DBusPropertyInfoFlags};
use glib::translate::*;
use std::ffi::CStr;

// SAFETY:
// though not explicitly documented, this struct is assumed to be immutable after creation
// (with the exception of ref_count of course). See usage in gdbusconnection.c

impl DBusPropertyInfo {
    pub fn name(&self) -> &str {
        // SAFETY: See top-level comment.
        unsafe {
            let c_obj = self.as_ptr();
            let name = (*c_obj).name;
            assert!(!name.is_null());
            let c_str = CStr::from_ptr(name);
            c_str.to_str().unwrap()
        }
    }

    pub fn signature(&self) -> &str {
        // SAFETY: See top-level comment.
        unsafe {
            let c_obj = self.as_ptr();
            let signature = (*c_obj).signature;
            assert!(!signature.is_null());
            let c_str = CStr::from_ptr(signature);
            c_str.to_str().unwrap()
        }
    }

    pub fn flags(&self) -> DBusPropertyInfoFlags {
        unsafe {
            let c_obj = self.as_ptr();
            let flags = (*c_obj).flags;
            from_glib(flags)
        }
    }
}
