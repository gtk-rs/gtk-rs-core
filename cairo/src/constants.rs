// Take a look at the license at the top of the repository in the LICENSE file.

pub const MIME_TYPE_JPEG: &str = "image/jpeg";
pub const MIME_TYPE_PNG: &str = "image/png";
pub const MIME_TYPE_JP2: &str = "image/jp2";
pub const MIME_TYPE_URI: &str = "text/x-uri";
pub const MIME_TYPE_UNIQUE_ID: &str = "application/x-cairo.uuid";
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub const MIME_TYPE_JBIG2: &str = "application/x-cairo.jbig2";
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub const MIME_TYPE_JBIG2_GLOBAL: &str = "application/x-cairo.jbig2-global";
#[cfg(any(feature = "v1_14", feature = "dox"))]
pub const MIME_TYPE_JBIG2_GLOBAL_ID: &str = "application/x-cairo.jbig2-global-id";
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const MIME_TYPE_CCITT_FAX: &str = "image/g3fax";
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const MIME_TYPE_CCITT_FAX_PARAMS: &str = "application/x-cairo.ccitt.params";
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const MIME_TYPE_EPS: &str = "application/postscript";
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const MIME_TYPE_EPS_PARAMS: &str = "application/x-cairo.eps.params";

#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const PDF_OUTLINE_ROOT: i32 = 0;

#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const CAIRO_TAG_DEST: &str = "cairo.dest";
#[cfg(any(feature = "v1_16", feature = "dox"))]
pub const CAIRO_TAG_LINK: &str = "Link";
