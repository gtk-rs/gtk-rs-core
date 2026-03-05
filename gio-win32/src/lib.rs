// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "strict-provenance", feature(strict_provenance_lints))]
#![cfg_attr(feature = "strict-provenance", deny(fuzzy_provenance_casts))]
#![cfg_attr(feature = "strict-provenance", deny(lossy_provenance_casts))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]
pub use gio_win32_sys as ffi;

mod auto;
pub use auto::*;

mod input_stream;
mod output_stream;

pub mod functions {
    pub use super::auto::functions::*;
}

pub mod prelude {
    pub use gio::prelude::*;

    pub use super::auto::traits::*;
    pub use super::input_stream::Win32InputStreamExtManual;
    pub use super::output_stream::Win32OutputStreamExtManual;
}
