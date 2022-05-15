// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! GObject bindings

#[allow(unused_imports)]
mod auto;
mod binding;
#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
mod binding_group;
mod flags;

#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
pub use binding_group::BindingGroupBuilder;

pub use self::auto::*;
pub use self::flags::*;
//pub use self::auto::functions::*;
