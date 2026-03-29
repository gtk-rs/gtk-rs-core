// Take a look at the license at the top of the repository in the LICENSE file.

pub(crate) const ATTRIBUTE_NAME: &str = "dbus";

mod interface;
pub(crate) use interface::*;
mod emits_changed_signal;
pub(crate) use emits_changed_signal::*;
mod item;
pub(crate) use item::*;
mod argument;
pub(crate) use argument::*;
mod methods;
pub(crate) use methods::*;
