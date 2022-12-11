// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;

use crate::{DBusConnection, DebugControllerDBus};

pub trait DebugControllerDBusExtManual: Sized {
    fn connection(&self) -> DBusConnection;
}

impl<O: IsA<DebugControllerDBus>> DebugControllerDBusExtManual for O {
    fn connection(&self) -> DBusConnection {
        glib::ObjectExt::property(self.as_ref(), "connection")
    }
}
