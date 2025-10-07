use gio::prelude::*;

glib::wrapper! {
    pub struct SampleApplication(ObjectSubclass<imp::SampleApplication>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for SampleApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property(
                "application-id",
                "com.github.gtk-rs.examples.ReceiveDBusSignals",
            )
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use futures::{future, StreamExt};
    use gio::prelude::*;
    use gio::subclass::prelude::*;
    use gio::{bus_get_future, BusType, DBusSignalFlags, WeakSignalSubscription};

    const DESKTOP_PORTAL_BUSNAME: &str = "org.freedesktop.portal.Desktop";
    const DESKTOP_PORTAL_OBJPATH: &str = "/org/freedesktop/portal/desktop";
    const SETTINGS_PORTAL_IFACE: &str = "org.freedesktop.portal.Settings";

    #[derive(Default)]
    pub struct SampleApplication {
        signal_subscription: RefCell<Option<WeakSignalSubscription>>,
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

            self.signal_subscription.replace(Some(
                self.obj()
                    .dbus_connection()
                    .unwrap()
                    .subscribe_to_signal(
                        Some(DESKTOP_PORTAL_BUSNAME),
                        Some(SETTINGS_PORTAL_IFACE),
                        Some("SettingChanged"),
                        Some(DESKTOP_PORTAL_OBJPATH),
                        None,
                        DBusSignalFlags::NONE,
                        |signal| {
                            println!(
                                "Callback received signal {}.{} from {} at {} with parameters: {}",
                                signal.interface_name,
                                signal.signal_name,
                                signal.object_path,
                                signal.sender_name,
                                signal.parameters
                            )
                        },
                    )
                    .downgrade(),
            ));

            glib::spawn_future_local(async move {
                let session_bus = bus_get_future(BusType::Session).await.unwrap();
                session_bus
                    .receive_signal_parameters::<(String, String, glib::Variant)>(
                        Some(DESKTOP_PORTAL_BUSNAME),
                        Some(SETTINGS_PORTAL_IFACE),
                        Some("SettingChanged"),
                        Some(DESKTOP_PORTAL_OBJPATH),
                        None,
                        DBusSignalFlags::NONE,
                    )
                    .for_each(|result| {
                        let (iface, setting, value) = result.unwrap();
                        println!("Setting {iface}.{setting} changed to {value}");
                        future::ready(())
                    })
                    .await
            });
        }

        fn activate(&self) {}
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}
