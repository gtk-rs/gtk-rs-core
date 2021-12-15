// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{GlyphInfo, GlyphString};
use glib::translate::*;
use std::slice;

impl GlyphString {
    pub fn num_glyphs(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).num_glyphs }
    }

    pub fn glyph_info(&self) -> &[GlyphInfo] {
        unsafe {
            let ptr = (*self.to_glib_none().0).glyphs as *const GlyphInfo;

            if ptr.is_null() || self.num_glyphs() <= 0 {
                &[]
            } else {
                slice::from_raw_parts(ptr, self.num_glyphs() as usize)
            }
        }
    }

    pub fn glyph_info_mut(&mut self) -> &mut [GlyphInfo] {
        unsafe {
            let ptr = (*self.to_glib_none().0).glyphs as *mut GlyphInfo;

            if ptr.is_null() || self.num_glyphs() <= 0 {
                &mut []
            } else {
                slice::from_raw_parts_mut(ptr, self.num_glyphs() as usize)
            }
        }
    }
}
