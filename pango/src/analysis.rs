// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{Attribute, Font, Gravity, Language, Script};
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoAnalysis")]
    pub struct Analysis(BoxedInline<ffi::PangoAnalysis>);
}

impl Analysis {
    pub fn font(&self) -> Font {
        unsafe { from_glib_none(self.inner.font) }
    }

    pub fn level(&self) -> u8 {
        self.inner.level
    }

    pub fn gravity(&self) -> Gravity {
        unsafe { from_glib(self.inner.gravity as i32) }
    }

    pub fn flags(&self) -> u8 {
        self.inner.flags
    }

    pub fn script(&self) -> Script {
        unsafe { from_glib(self.inner.script as i32) }
    }

    pub fn language(&self) -> Language {
        unsafe { from_glib_none(self.inner.language) }
    }

    pub fn extra_attrs(&self) -> Vec<Attribute> {
        unsafe { FromGlibPtrContainer::from_glib_none(self.inner.extra_attrs) }
    }
}

impl fmt::Debug for Analysis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Analysis")
            .field("font", &self.font())
            .field("level", &self.level())
            .field("gravity", &self.gravity())
            .field("flags", &self.flags())
            .field("script", &self.script())
            .field("extra_attrs", &self.extra_attrs())
            .finish()
    }
}
