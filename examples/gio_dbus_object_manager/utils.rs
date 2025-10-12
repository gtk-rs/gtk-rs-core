use gio::prelude::*;
use glib::object::IsA;
use glib::variant::FromVariant;

pub(crate) fn cached_dbus_property<P: IsA<gio::DBusProxy>, T: FromVariant + Default>(
    proxy: &P,
    name: &str,
) -> T {
    proxy
        .cached_property(name)
        .map(|v| v.get().expect("DBus Property to have correct type"))
        .unwrap_or_default()
}
