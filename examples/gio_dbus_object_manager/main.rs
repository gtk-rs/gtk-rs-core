use gio::prelude::*;

mod block_proxy;
mod drive_proxy;

glib::wrapper! {
    pub struct SampleApplication(ObjectSubclass<imp::SampleApplication>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for SampleApplication {
    fn default() -> Self {
        glib::Object::builder().build()
    }
}

mod imp {
    use std::cell::RefCell;

    use crate::block_proxy::UDisksBlockProxy;
    use crate::drive_proxy::UDisksDriveProxy;
    use gio::subclass::prelude::*;
    use gio::{prelude::*, DBusInterface, DBusObjectManagerClient};
    use gio::{BusType, DBusObjectManagerClientFlags, DBusObjectProxy, DBusProxy};

    const UDISKS2_BUS_NAME: &str = "org.freedesktop.UDisks2";
    const UDISKS2_OBJECT_PATH: &str = "/org/freedesktop/UDisks2";
    const UDISK2_DRIVE_INTERFACE: &str = "org.freedesktop.UDisks2.Drive";
    const UDISK2_BLOCK_INTERFACE: &str = "org.freedesktop.UDisks2.Block";

    #[derive(Default)]
    pub struct SampleApplication {
        object_manager: RefCell<Option<DBusObjectManagerClient>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SampleApplication {
        const NAME: &'static str = "SampleApplication";

        type Type = super::SampleApplication;

        type ParentType = gio::Application;
    }

    impl ObjectImpl for SampleApplication {}

    impl ApplicationImpl for SampleApplication {
        fn startup(&self) {
            self.parent_startup();

            glib::spawn_future_local(glib::clone!(
                #[weak(rename_to = this)]
                self,
                async move {
                    let object_manager = DBusObjectManagerClient::new_for_bus_future_with_fn(
                        BusType::System,
                        DBusObjectManagerClientFlags::NONE,
                        UDISKS2_BUS_NAME,
                        UDISKS2_OBJECT_PATH,
                        get_proxy_type,
                    )
                    .await
                    .unwrap();
                    print_drives(&object_manager);
                    this.object_manager.replace(Some(object_manager.clone()));

                    object_manager.connect_object_added(|_, obj| {
                        println!("\nObject added at '{}'", obj.object_path());

                        for iface in obj.interfaces() {
                            print_interface(&iface);
                        }
                    });

                    object_manager.connect_object_removed(|_, obj| {
                        println!("\nObject removed at '{}'", obj.object_path());

                        for iface in obj.interfaces() {
                            print_interface(&iface);
                        }
                    });
                }
            ));
        }

        fn activate(&self) {}
    }

    fn print_interface(iface: &DBusInterface) {
        if let Some(drive) = iface.downcast_ref::<UDisksDriveProxy>() {
            print_drive(drive);
        } else if let Some(block) = iface.downcast_ref::<UDisksBlockProxy>() {
            print_block(block);
        } else if let Some(proxy) = iface.downcast_ref::<DBusProxy>() {
            println!("  • {}", proxy.interface_name());
        }
    }

    fn print_drives(object_manager: &DBusObjectManagerClient) {
        for obj in object_manager.objects() {
            println!("\nObject at '{}'", obj.object_path());
            for iface in obj.interfaces() {
                print_interface(&iface);
            }
        }
    }

    fn print_drive(drive: &UDisksDriveProxy) {
        const ITEM_INDENT: &str = "  ";
        const PROPERTY_INDENT: &str = "      ";
        println!(
            "{ITEM_INDENT}• Drive {id}\n{PROPERTY_INDENT}vendor: '{vendor}'\n{PROPERTY_INDENT}size: {size}\n{PROPERTY_INDENT}removable? {removable}\n{PROPERTY_INDENT}media compatibility: {media_compat:?}",
            id = drive.id(),
            vendor = drive.vendor(),
            size = drive.size(),
            removable = drive.removable(),
            media_compat = drive.media_compatibility(),
        );
    }

    fn print_block(drive: &UDisksBlockProxy) {
        const ITEM_INDENT: &str = "  ";
        println!("{ITEM_INDENT}• Block {}", drive.device().display());
    }

    fn get_proxy_type(
        _manager: &DBusObjectManagerClient,
        _object_path: &str,
        interface_name: Option<&str>,
    ) -> glib::types::Type {
        match interface_name {
            Some(UDISK2_DRIVE_INTERFACE) => UDisksDriveProxy::static_type(),
            Some(UDISK2_BLOCK_INTERFACE) => UDisksBlockProxy::static_type(),
            Some(_) => DBusProxy::static_type(),
            // No interface name means we need to return a proxy for an object.
            None => DBusObjectProxy::static_type(),
        }
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}
