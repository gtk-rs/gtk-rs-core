// Take a look at the license at the top of the repository in the LICENSE file.

use std::mem;

use glib::translate::*;

use crate::{AttrIterator, AttrList, Attribute, ffi};

impl AttrList {
    #[doc(alias = "pango_attr_list_change")]
    pub fn change(&mut self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_change(self.to_glib_none().0, attr.to_glib_none().0);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[cfg(feature = "v1_46")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_list_equal")]
    fn equal(&self, other_list: &AttrList) -> bool {
        unsafe {
            from_glib(ffi::pango_attr_list_equal(
                self.to_glib_none().0,
                other_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_attr_list_filter")]
    #[must_use]
    pub fn filter<P: FnMut(&Attribute) -> bool>(&mut self, func: P) -> Option<AttrList> {
        let mut func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Attribute) -> bool>(
            attribute: *mut ffi::PangoAttribute,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            unsafe {
                let attribute = from_glib_borrow(attribute);
                let callback = user_data as *mut P;
                (*callback)(&attribute).into_glib()
            }
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &mut P = &mut func_data;
        unsafe {
            from_glib_full(ffi::pango_attr_list_filter(
                self.to_glib_none().0,
                func,
                super_callback0 as *mut _ as *mut _,
            ))
        }
    }

    #[doc(alias = "pango_attr_list_insert")]
    pub fn insert(&mut self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_insert(self.to_glib_none().0, attr.to_glib_none().0);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[doc(alias = "pango_attr_list_insert_before")]
    pub fn insert_before(&mut self, attr: impl Into<Attribute>) {
        unsafe {
            let attr = attr.into();
            ffi::pango_attr_list_insert_before(self.to_glib_none().0, attr.to_glib_none().0);
            mem::forget(attr); //As attr transferred fully
        }
    }

    #[doc(alias = "pango_attr_list_get_iterator")]
    #[doc(alias = "get_iterator")]
    pub fn iterator(&self) -> AttrIterator<'_> {
        unsafe { from_glib_full(ffi::pango_attr_list_get_iterator(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_attr_list_splice")]
    pub fn splice(&mut self, other: &AttrList, pos: i32, len: i32) {
        unsafe {
            ffi::pango_attr_list_splice(self.to_glib_none().0, other.to_glib_none().0, pos, len);
        }
    }

    #[cfg(feature = "v1_44")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_list_update")]
    pub fn update(&mut self, pos: i32, remove: i32, add: i32) {
        unsafe {
            ffi::pango_attr_list_update(self.to_glib_none().0, pos, remove, add);
        }
    }
}

#[cfg(feature = "v1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
impl PartialEq for AttrList {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.equal(other)
    }
}

#[cfg(feature = "v1_46")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_46")))]
impl Eq for AttrList {}

#[cfg(feature = "v1_50")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_50")))]
impl std::str::FromStr for AttrList {
    type Err = glib::BoolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_string(s)
    }
}
