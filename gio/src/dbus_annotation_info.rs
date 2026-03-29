use std::ffi::CString;
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{DBusAnnotationInfo, ffi};

impl DBusAnnotationInfo {
    pub fn builder<'a>() -> DBusAnnotationInfoBuilder<'a> {
        DBusAnnotationInfoBuilder::default()
    }
}

#[derive(Default)]
pub struct DBusAnnotationInfoBuilder<'a> {
    key: Option<&'a str>,
    value: Option<&'a str>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusAnnotationInfoBuilder<'a> {
    /// Required
    pub fn key(mut self, key: &'a str) -> Self {
        self.key = Some(key);
        self
    }

    /// Required
    pub fn value(mut self, value: &'a str) -> Self {
        self.value = Some(value);
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusAnnotationInfo {
        let key = self.value.expect("`key` should be set");
        let value = self.value.expect("`value` should be set");
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusAnnotationInfo>()) }
                as *mut ffi::GDBusAnnotationInfo;
            (*ptr).ref_count = 1;
            (*ptr).key = CString::new(key).unwrap().to_glib_full();
            (*ptr).value = CString::new(value).unwrap().to_glib_full();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
