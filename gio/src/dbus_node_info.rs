// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use glib::translate::ToGlibPtr;

use crate::{DBusInterfaceInfo, DBusNodeInfo};

impl DBusNodeInfo {
    pub fn path(&self) -> Option<&str> {
        let c_obj = self.to_glib_none().0;
        unsafe {
            let path = (*c_obj).path;
            if path.is_null() {
                return None;
            }
            let c_str = CStr::from_ptr(path);
            Some(c_str.to_str().unwrap())
        }
    }

    pub fn interfaces(&self) -> &[DBusInterfaceInfo] {
        let c_obj = self.to_glib_none().0;
        unsafe {
            let c_ii = (*c_obj).interfaces;
            if c_ii.is_null() {
                return &[];
            }

            glib::collections::PtrSlice::from_glib_borrow(c_ii)
        }
    }

    pub fn nodes(&self) -> &[DBusNodeInfo] {
        let c_obj = self.to_glib_none().0;
        unsafe {
            let c_ni = (*c_obj).nodes;
            if c_ni.is_null() {
                return &[];
            }
            glib::collections::PtrSlice::from_glib_borrow(c_ni)
        }
    }
}
