use std::ffi::CString;
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{DBusAnnotationInfo, DBusArgInfo, DBusMethodInfo, ffi};

impl DBusMethodInfo {
    pub fn builder<'a>() -> DBusMethodInfoBuilder<'a> {
        DBusMethodInfoBuilder::default()
    }
}

#[derive(Default)]
pub struct DBusMethodInfoBuilder<'a> {
    name: Option<&'a str>,
    in_args: SmallVec<[DBusArgInfo; 4]>,
    out_args: SmallVec<[DBusArgInfo; 2]>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusMethodInfoBuilder<'a> {
    /// Required
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn in_args<I: IntoIterator<Item = DBusArgInfo>>(mut self, args: I) -> Self {
        self.in_args = args.into_iter().collect();
        self
    }

    pub fn out_args<I: IntoIterator<Item = DBusArgInfo>>(mut self, args: I) -> Self {
        self.out_args = args.into_iter().collect();
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusMethodInfo {
        let name = self.name.expect("`name` should be set");
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusMethodInfo>()) }
                as *mut ffi::GDBusMethodInfo;
            (*ptr).ref_count = 1;
            (*ptr).name = CString::new(name).unwrap().to_glib_full();
            (*ptr).in_args = self.in_args.to_glib_full();
            (*ptr).out_args = self.out_args.to_glib_full();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
