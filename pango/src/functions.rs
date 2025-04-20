// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use std::{ffi::c_char, ptr};

pub use crate::auto::functions::*;
#[cfg(feature = "v1_44")]
use crate::ShapeFlags;
use crate::{ffi, Analysis, GlyphString, Item};

#[doc(alias = "pango_reorder_items")]
pub fn reorder_items(logical_items: &glib::List<Item>) -> glib::List<Item> {
    unsafe {
        FromGlibPtrContainer::from_glib_full(ffi::pango_reorder_items(
            logical_items.as_ptr() as *mut _
        ))
    }
}

#[doc(alias = "pango_shape_full")]
pub fn shape_full(
    item_text: &str,
    paragraph_text: Option<&str>,
    analysis: &Analysis,
    glyphs: &mut GlyphString,
) {
    let item_length = item_text.len() as i32;
    let paragraph_length = paragraph_text.map(|t| t.len() as i32).unwrap_or_default();
    let paragraph_ptr = paragraph_text.map_or(ptr::null(), |t| t.as_ptr() as *const c_char);
    unsafe {
        // The function does not take null-terminated strings when a length is provided.
        // It also requires item_text to point to a subsequence of paragraph_text.
        // Using to_glib_none() on &str will copy the string and cause problems.
        ffi::pango_shape_full(
            item_text.as_ptr() as *const c_char,
            item_length,
            paragraph_ptr,
            paragraph_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

#[doc(alias = "pango_shape")]
pub fn shape(item_text: &str, analysis: &Analysis, glyphs: &mut GlyphString) {
    let item_length = item_text.len() as i32;
    unsafe {
        // The function does not take null-terminated strings when a length is provided.
        // Using to_glib_none() on &str will copy the string unnecessarily.
        ffi::pango_shape(
            item_text.as_ptr() as *const c_char,
            item_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
        );
    }
}

#[cfg(feature = "v1_44")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_44")))]
#[doc(alias = "pango_shape_with_flags")]
pub fn shape_with_flags(
    item_text: &str,
    paragraph_text: Option<&str>,
    analysis: &Analysis,
    glyphs: &mut GlyphString,
    flags: ShapeFlags,
) {
    let item_length = item_text.len() as i32;
    let paragraph_length = paragraph_text.map(|t| t.len() as i32).unwrap_or_default();
    let paragraph_ptr = paragraph_text.map_or(ptr::null(), |t| t.as_ptr() as *const c_char);
    unsafe {
        // See: shape_full
        ffi::pango_shape_with_flags(
            item_text.as_ptr() as *const c_char,
            item_length,
            paragraph_ptr,
            paragraph_length,
            analysis.to_glib_none().0,
            glyphs.to_glib_none_mut().0,
            flags.into_glib(),
        );
    }
}

#[doc(alias = "pango_extents_to_pixels")]
pub fn extents_to_pixels(
    mut inclusive: Option<&mut crate::Rectangle>,
    mut nearest: Option<&mut crate::Rectangle>,
) {
    unsafe {
        ffi::pango_extents_to_pixels(inclusive.to_glib_none_mut().0, nearest.to_glib_none_mut().0);
    }
}
