use glib::types::StaticType as _;

const UDISKS2_BUS_NAME: &str = "org.freedesktop.UDisks2";
const UDISKS2_OBJECT_PATH: &str = "/org/freedesktop/UDisks2";
const UDISK2_DRIVE_INTERFACE: &str = "org.freedesktop.UDisks2.Drive";
const UDISK2_BLOCK_INTERFACE: &str = "org.freedesktop.UDisks2.Block";

pub(crate) async fn new_udisks_object_manager_client()
-> Result<gio::DBusObjectManagerClient, glib::Error> {
    gio::DBusObjectManagerClient::new_for_bus_future_with_fn(
        gio::BusType::System,
        gio::DBusObjectManagerClientFlags::NONE,
        UDISKS2_BUS_NAME,
        UDISKS2_OBJECT_PATH,
        get_proxy_type,
    )
    .await
}

/// This function is called for creating both object and interface proxies.
///
/// When an interface name is given, we must return a type that implements [`gio::DBusInterface`]. \
/// When no interface name is given, we must return a type that implements [`gio::DBusObject`].
fn get_proxy_type(
    _manager: &gio::DBusObjectManagerClient,
    _object_path: &str,
    interface_name: Option<&str>,
) -> glib::types::Type {
    match interface_name {
        Some(UDISK2_DRIVE_INTERFACE) => crate::drive_proxy::UDisksDriveProxy::static_type(),
        Some(UDISK2_BLOCK_INTERFACE) => crate::block_proxy::UDisksBlockProxy::static_type(),

        // We use the default implementations for unknown interfaces and objects.
        Some(_) => gio::DBusProxy::static_type(),
        None => gio::DBusObjectProxy::static_type(),
    }
}
