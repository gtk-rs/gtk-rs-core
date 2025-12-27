// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{gobject_ffi, translate::*};

wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GTypeInfo")]
    pub struct TypeInfo(BoxedInline<gobject_ffi::GTypeInfo>);
}

unsafe impl Send for TypeInfo {}
unsafe impl Sync for TypeInfo {}

impl Default for TypeInfo {
    fn default() -> Self {
        unsafe { Self::uninitialized() }
    }
}
