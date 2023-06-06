// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(ambiguous_glob_reexports)]

#[doc(hidden)]
pub use glib::prelude::*;
#[doc(hidden)]
pub use pango::prelude::*;

pub use crate::{auto::traits::*, font_map::FontMapExtManual};
