// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::Binding;
use crate::Object;

impl Binding {
    #[doc(alias = "get_source")]
    pub fn source(&self) -> Option<Object> {
        self.property("source")
    }

    #[doc(alias = "get_target")]
    pub fn target(&self) -> Option<Object> {
        self.property("target")
    }
}
