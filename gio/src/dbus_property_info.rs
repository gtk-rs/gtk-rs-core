use std::ffi::CString;
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{DBusAnnotationInfo, DBusPropertyInfo, DBusPropertyInfoFlags, ffi};

impl DBusPropertyInfo {
    pub fn builder<'a>() -> DBusPropertyInfoBuilder<'a> {
        DBusPropertyInfoBuilder::default()
    }
}

#[derive(Default)]
pub struct DBusPropertyInfoBuilder<'a> {
    name: Option<&'a str>,
    signature: Option<&'a str>,
    flags: Option<DBusPropertyInfoFlags>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusPropertyInfoBuilder<'a> {
    /// Required
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = Some(name);
        self
    }

    /// Required
    pub fn signature(mut self, signature: &'a str) -> Self {
        self.signature = Some(signature);
        self
    }

    pub fn flags(mut self, flags: DBusPropertyInfoFlags) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusPropertyInfo {
        let name = self.name.expect("`name` should be set");
        let signature = self.signature.expect("`name` should be set");
        let flags = self.flags.unwrap_or(DBusPropertyInfoFlags::NONE);
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusPropertyInfo>()) }
                as *mut ffi::GDBusPropertyInfo;
            (*ptr).ref_count = 1;
            (*ptr).name = CString::new(name).unwrap().to_glib_full();
            (*ptr).signature = CString::new(signature).unwrap().to_glib_full();
            (*ptr).flags = flags.into_glib();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
