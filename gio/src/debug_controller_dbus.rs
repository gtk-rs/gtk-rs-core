// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DBusConnection;
use crate::DebugControllerDBus;
use glib::object::IsA;

pub trait DebugControllerDBusExtManual: Sized {
    fn connection(&self) -> DBusConnection;
}

impl<O: IsA<DebugControllerDBus>> DebugControllerDBusExtManual for O {
    fn connection(&self) -> DBusConnection {
        glib::ObjectExt::property(self.as_ref(), "connection")
    }
}
