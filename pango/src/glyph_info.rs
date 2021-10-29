// Take a look at the license at the top of the repository in the LICENSE file.

use crate::GlyphGeometry;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "PangoGlyphInfo")]
    pub struct GlyphInfo(BoxedInline<ffi::PangoGlyphInfo>);
}

impl GlyphInfo {
    pub fn glyph(&self) -> u32 {
        self.0.glyph
    }

    pub fn geometry(&self) -> &GlyphGeometry {
        unsafe { &*(&((self.0).geometry) as *const _ as *const GlyphGeometry) }
    }
}

impl fmt::Debug for GlyphInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("GlyphInfo")
            .field("glyph", &self.glyph())
            .field("geometry", &self.geometry())
            .finish()
    }
}

impl FromGlibContainerAsVec<*mut ffi::PangoGlyphInfo, *mut ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for x in 0..num {
            res.push(from_glib_none(ptr.add(x)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut ffi::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

impl FromGlibContainerAsVec<*mut ffi::PangoGlyphInfo, *const ffi::PangoGlyphInfo> for GlyphInfo {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for x in 0..num {
            res.push(from_glib_none(ptr.add(x)));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *const ffi::PangoGlyphInfo,
        num: usize,
    ) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        glib::ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *const ffi::PangoGlyphInfo, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}
