// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::{CStr, CString};
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{
    DBusAnnotationInfo, DBusInterfaceInfo, DBusMethodInfo, DBusPropertyInfo, DBusSignalInfo, ffi,
};

impl DBusInterfaceInfo {
    pub fn builder<'a>() -> DBusInterfaceInfoBuilder<'a> {
        DBusInterfaceInfoBuilder::default()
    }

    pub fn name(&self) -> &str {
        unsafe {
            let c_obj = self.as_ptr();
            let name = (*c_obj).name;
            assert!(!name.is_null());
            let c_str = CStr::from_ptr(name);
            c_str.to_str().unwrap()
        }
    }
}

#[derive(Default)]
pub struct DBusInterfaceInfoBuilder<'a> {
    name: Option<&'a str>,
    methods: SmallVec<[DBusMethodInfo; 4]>,
    signals: SmallVec<[DBusSignalInfo; 4]>,
    properties: SmallVec<[DBusPropertyInfo; 4]>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusInterfaceInfoBuilder<'a> {
    /// Required
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn methods<I: IntoIterator<Item = DBusMethodInfo>>(mut self, methods: I) -> Self {
        self.methods = methods.into_iter().collect();
        self
    }

    pub fn signals<I: IntoIterator<Item = DBusSignalInfo>>(mut self, args: I) -> Self {
        self.signals = args.into_iter().collect();
        self
    }

    pub fn properties<I: IntoIterator<Item = DBusPropertyInfo>>(mut self, args: I) -> Self {
        self.properties = args.into_iter().collect();
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusInterfaceInfo {
        let name = self.name.expect("`name` should be set");
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusInterfaceInfo>()) }
                as *mut ffi::GDBusInterfaceInfo;
            (*ptr).ref_count = 1;
            (*ptr).name = CString::new(name).unwrap().to_glib_full();
            (*ptr).methods = self.methods.to_glib_full();
            (*ptr).signals = self.signals.to_glib_full();
            (*ptr).properties = self.properties.to_glib_full();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
