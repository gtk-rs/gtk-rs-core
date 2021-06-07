// Take a look at the license at the top of the repository in the LICENSE file.

//! # Rust GDK-PixBuf bindings
//!
//! This library contains safe Rust bindings for [GDK-PixBuf](https://docs.gtk.org/gdk-pixbuf).
//! It is a part of [gtk-rs](https://gtk-rs.org/).
//!
//! GDK-PixBuf 2.32 is the lowest supported version for the underlying library.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use gio;
pub use glib;

#[allow(clippy::too_many_arguments)]
#[allow(unused_imports)]
mod auto;

mod pixbuf;
mod pixbuf_animation;
mod pixbuf_animation_iter;
pub mod prelude;

pub use crate::auto::*;

pub use self::pixbuf_animation_iter::PixbufAnimationIter;
