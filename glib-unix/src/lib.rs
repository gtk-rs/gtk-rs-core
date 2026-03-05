// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "strict-provenance", feature(strict_provenance_lints))]
#![cfg_attr(feature = "strict-provenance", deny(fuzzy_provenance_casts))]
#![cfg_attr(feature = "strict-provenance", deny(lossy_provenance_casts))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
pub use glib_unix_sys as ffi;

mod auto;
mod functions;

pub use auto::functions::*;
pub use functions::*;

pub mod prelude {
    pub use glib::prelude::*;
}
