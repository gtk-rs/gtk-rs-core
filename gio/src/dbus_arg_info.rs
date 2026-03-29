use std::ffi::CString;
use std::mem;

use glib::translate::*;
use smallvec::SmallVec;

use crate::{DBusAnnotationInfo, DBusArgInfo, ffi};

impl DBusArgInfo {
    pub fn builder<'a>() -> DBusArgInfoBuilder<'a> {
        DBusArgInfoBuilder::default()
    }
}

#[derive(Default)]
pub struct DBusArgInfoBuilder<'a> {
    name: Option<&'a str>,
    signature: Option<&'a str>,
    annotations: SmallVec<[DBusAnnotationInfo; 2]>,
}

impl<'a> DBusArgInfoBuilder<'a> {
    pub fn name<N: Into<Option<&'a str>>>(mut self, name: N) -> Self {
        self.name = name.into();
        self
    }

    /// Required
    pub fn signature(mut self, signature: &'a str) -> Self {
        self.signature = Some(signature);
        self
    }

    pub fn annotations<I: IntoIterator<Item = DBusAnnotationInfo>>(
        mut self,
        annotations: I,
    ) -> Self {
        self.annotations = annotations.into_iter().collect();
        self
    }

    pub fn build(self) -> DBusArgInfo {
        let signature = self.signature.expect("`signature` should be set");
        let name = self.name.map(|name| CString::new(name).unwrap());
        unsafe {
            let ptr = { glib::ffi::g_malloc0(mem::size_of::<ffi::GDBusArgInfo>()) }
                as *mut ffi::GDBusArgInfo;
            (*ptr).ref_count = 1;
            (*ptr).name = name.to_glib_full();
            (*ptr).signature = CString::new(signature).unwrap().to_glib_full();
            (*ptr).annotations = self.annotations.to_glib_full();
            from_glib_full(ptr)
        }
    }
}
