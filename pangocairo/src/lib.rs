// Take a look at the license at the top of the repository in the LICENSE file.

// Take a look at the license at the top of the repository in the LICENSE file.

//! # Rust PangoCairo bindings
//!
//! This library contains safe Rust bindings for [PangoCairo](https://docs.gtk.org/PangoCairo).
//! It is a part of [Gtk-rs](https://gtk-rs.org/).
//!
//! PangoCairo 1.38 is the lowest supported version for the underlying library.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use cairo;
pub use ffi;
pub use glib;
pub use pango;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

pub use crate::auto::functions::*;
pub use crate::auto::*;
pub mod prelude;

mod font_map;
