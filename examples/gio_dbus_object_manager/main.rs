use std::cell::RefCell;

use gio::prelude::*;
use gio::subclass::prelude::*;

use crate::block_proxy::UDisksBlockProxy;
use crate::drive_proxy::UDisksDriveProxy;
use crate::object_manager::new_udisks_object_manager_client;

mod block_proxy;
mod drive_proxy;
mod object_manager;
mod utils;

async fn object_manager_example() -> Result<gio::DBusObjectManagerClient, glib::Error> {
    let object_manager = new_udisks_object_manager_client().await?;

    object_manager.connect_object_added(|_, object| print_dbus_object_details(object, "Added: "));

    object_manager
        .connect_object_removed(|_, object| print_dbus_object_details(object, "Removed: "));

    for object in object_manager.objects() {
        print_dbus_object_details(&object, "");
    }

    Ok(object_manager)
}

fn print_dbus_object_details(object: &gio::DBusObject, prefix: &str) {
    for interface in object.interfaces() {
        if let Some(drive) = interface.downcast_ref::<UDisksDriveProxy>() {
            println!(
                "{prefix}Drive {} with vendor '{}' at {}",
                drive.id(),
                drive.vendor(),
                object.object_path()
            )
        }
        if let Some(block) = interface.downcast_ref::<UDisksBlockProxy>() {
            println!(
                "{prefix}Block {} at {}",
                block.device().display(),
                object.object_path()
            )
        }
    }
}

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
    use super::*;

    #[derive(Default)]
    pub struct SampleApplication {
        object_manager: RefCell<Option<gio::DBusObjectManagerClient>>,
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
                    match object_manager_example().await {
                        Ok(object_manager) => {
                            // We keep a reference to the object manager alive so that
                            // our signal handlers are called.
                            *this.object_manager.borrow_mut() = Some(object_manager);
                        }
                        Err(error) => eprintln!("error: {error}"),
                    };
                }
            ));
        }

        fn activate(&self) {}
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}
