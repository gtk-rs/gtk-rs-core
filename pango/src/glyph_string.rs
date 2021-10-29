// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GlyphInfo, GlyphString};
use glib::translate::*;

impl GlyphString {
    pub fn num_glyphs(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_glyphs }
    }

    pub fn glyph_info(&self) -> Vec<GlyphInfo> {
        if self.num_glyphs() < 0 {
            return Vec::new();
        }
        let num_glyphs = self.num_glyphs() as usize;
        unsafe {
            let glyphs: *mut ffi::PangoGlyphInfo = (*self.to_glib_none().0).glyphs;
            FromGlibContainer::from_glib_none_num(glyphs, num_glyphs)
        }
    }
}
