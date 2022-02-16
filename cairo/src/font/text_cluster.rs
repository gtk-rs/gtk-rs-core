// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt;

#[derive(Clone, Copy)]
#[repr(transparent)]
#[doc(alias = "cairo_text_cluster_t")]
pub struct TextCluster(ffi::cairo_text_cluster_t);

impl TextCluster {
    pub fn num_bytes(&self) -> i32 {
        self.0.num_bytes
    }

    pub fn num_glyphs(&self) -> i32 {
        self.0.num_glyphs
    }
}

impl fmt::Debug for TextCluster {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TextCluster")
            .field("num_glyphs", &self.num_glyphs())
            .field("num_glyphs", &self.num_glyphs())
            .finish()
    }
}
