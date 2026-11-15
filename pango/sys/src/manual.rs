// Take a look at the license at the top of the repository in the LICENSE file.

use libc::c_uint;

// C has four 1-bit bitfields sharing one guint. The Pango gir lacks the
// `bits` attribute on them, so gir would flatten them into separate
// members and double the struct size. Remove this once fixed upstream.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PangoGlyphVisAttr {
    pub is_cluster_start: c_uint,
}

impl ::std::fmt::Debug for PangoGlyphVisAttr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("PangoGlyphVisAttr @ {self:p}"))
            .field("is_cluster_start", &self.is_cluster_start)
            .finish()
    }
}
