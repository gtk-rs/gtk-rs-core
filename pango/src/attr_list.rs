// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AttrList, Attribute};
use glib::translate::*;
use std::mem;

impl AttrList {
    #[doc(alias = "pango_attr_list_change")]
    pub fn change(&self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_change(self.to_glib_none().0, attr.to_glib_none().0 as *mut _);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_list_equal")]
    fn equal(&self, other_list: &AttrList) -> bool {
        unsafe {
            from_glib(ffi::pango_attr_list_equal(
                self.to_glib_none().0,
                other_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_attr_list_insert")]
    pub fn insert(&self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_insert(self.to_glib_none().0, attr.to_glib_none().0 as *mut _);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[doc(alias = "pango_attr_list_insert_before")]
    pub fn insert_before(&self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_insert_before(
                self.to_glib_none().0,
                attr.to_glib_none().0 as *mut _,
            );
            mem::forget(attr); //As attr transferred fully
        }
    }
}

#[cfg(any(feature = "v1_46", feature = "dox"))]
impl PartialEq for AttrList {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

#[cfg(any(feature = "v1_46", feature = "dox"))]
impl Eq for AttrList {}

#[cfg(any(feature = "v1_50", feature = "dox"))]
impl std::str::FromStr for AttrList {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}
