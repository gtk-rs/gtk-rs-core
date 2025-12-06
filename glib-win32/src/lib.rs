// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
pub use ffi;

mod auto;
pub use auto::*;
mod functions;

pub use auto::functions::*;
pub use functions::*;

pub mod prelude {
    pub use glib::prelude::*;
}
