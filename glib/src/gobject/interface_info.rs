// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{gobject_ffi, translate::*};

wrapper! {
    #[derive(Debug)]
    #[doc(alias = "GInterfaceInfo")]
    pub struct InterfaceInfo(BoxedInline<gobject_ffi::GInterfaceInfo>);
}

unsafe impl Send for InterfaceInfo {}
unsafe impl Sync for InterfaceInfo {}

impl Default for InterfaceInfo {
    fn default() -> Self {
        unsafe { Self::uninitialized() }
    }
}
