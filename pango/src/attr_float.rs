// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AttrType;
use glib::translate::*;

define_attribute_struct!(
    AttrFloat,
    ffi::PangoAttrFloat,
    &[
        AttrType::Scale,
        #[cfg(any(feature = "v1_50", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
        AttrType::LineHeight
    ]
);

impl AttrFloat {
    #[doc(alias = "pango_attr_scale_new")]
    pub fn new_scale(scale_factor: f64) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_scale_new(scale_factor)) }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_attr_line_height_new")]
    pub fn new_line_height(factor: f64) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_line_height_new(factor)) }
    }

    pub fn value(&self) -> f64 {
        self.inner.value
    }
}
