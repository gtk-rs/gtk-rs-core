// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{gobject_ffi, translate::*};

wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GTypeValueTable")]
    pub struct TypeValueTable(BoxedInline<gobject_ffi::GTypeValueTable>);
}

unsafe impl Send for TypeValueTable {}
unsafe impl Sync for TypeValueTable {}

impl Default for TypeValueTable {
    fn default() -> Self {
        unsafe { Self::uninitialized() }
    }
}
