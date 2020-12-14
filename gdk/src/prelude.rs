// Take a look at the license at the top of the repository in the LICENSE file.

//! Traits intended for blanket imports.

pub use crate::auto::traits::*;
pub use crate::cairo_interaction::{GdkContextExt, GdkPixbufExt, GdkSurfaceExt};
pub use crate::window::WindowExtManual;
#[doc(hidden)]
pub use glib::prelude::*;
