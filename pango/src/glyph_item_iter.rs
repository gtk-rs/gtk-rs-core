// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GlyphItem;
use crate::GlyphItemIter;
use glib::translate::*;
use std::mem;

impl GlyphItemIter {
    #[doc(alias = "pango_glyph_item_iter_init_end")]
    pub fn new_end(glyph_item: &GlyphItem, text: &str) -> Result<Self, glib::BoolError> {
        unsafe {
            let mut iter = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::pango_glyph_item_iter_init_end(
                iter.as_mut_ptr(),
                mut_override(glyph_item.to_glib_none().0),
                text.to_glib_none().0,
            ));

            if res {
                Ok(from_glib_none(&iter.assume_init() as *const _))
            } else {
                Err(glib::bool_error!("Failed to create glyph item iter"))
            }
        }
    }

    #[doc(alias = "pango_glyph_item_iter_init_start")]
    pub fn new_start(glyph_item: &GlyphItem, text: &str) -> Result<Self, glib::BoolError> {
        unsafe {
            let mut iter = mem::MaybeUninit::zeroed();
            let res: bool = from_glib(ffi::pango_glyph_item_iter_init_start(
                iter.as_mut_ptr(),
                mut_override(glyph_item.to_glib_none().0),
                text.to_glib_none().0,
            ));

            if res {
                Ok(from_glib_none(&iter.assume_init() as *const _))
            } else {
                Err(glib::bool_error!("Failed to create glyph item iter"))
            }
        }
    }

    pub fn start_char(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).start_char }
    }
    pub fn end_char(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).end_char }
    }
    pub fn start_glyph(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).start_glyph }
    }
    pub fn end_glyph(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).end_glyph }
    }
    pub fn start_index(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).start_index }
    }
    pub fn end_index(&self) -> i32 {
        unsafe { (*self.to_glib_none().0).start_index }
    }
}
