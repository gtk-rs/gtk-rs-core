// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GlyphItem, GlyphString, Item};
use glib::translate::*;

impl GlyphItem {
    pub fn item(&self) -> Item {
        unsafe { from_glib_none((*self.to_glib_none().0).item) }
    }

    pub fn glyph_string(&self) -> GlyphString {
        unsafe { from_glib_none((*self.to_glib_none().0).glyphs) }
    }
}
