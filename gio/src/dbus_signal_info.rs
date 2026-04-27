use std::ffi::CString;
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{DBusAnnotationInfo, DBusArgInfo, DBusSignalInfo, ffi};

impl DBusSignalInfo {
    pub fn builder<'a>() -> DBusSignalInfoBuilder<'a> {
        DBusSignalInfoBuilder::default()
    }
}

#[derive(Default)]
pub struct DBusSignalInfoBuilder<'a> {
    name: Option<&'a str>,
    args: SmallVec<[DBusArgInfo; 4]>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusSignalInfoBuilder<'a> {
    /// Required
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    pub fn args<I: IntoIterator<Item = DBusArgInfo>>(mut self, args: I) -> Self {
        self.args = args.into_iter().collect();
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusSignalInfo {
        let name = self.name.expect("`name` should be set");
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusSignalInfo>()) }
                as *mut ffi::GDBusSignalInfo;
            (*ptr).ref_count = 1;
            (*ptr).name = CString::new(name).unwrap().to_glib_full();
            (*ptr).args = self.args.to_glib_full();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
