use crate::hello_world::HelloWorldSkeleton;
use gio::prelude::*;
use gio::subclass::prelude::*;

glib::wrapper! {
    pub struct SampleApplication(ObjectSubclass<imp::SampleApplication>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

mod hello_world;

impl Default for SampleApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property(
                "application-id",
                "com.github.gtk-rs.examples.RegisterDBusObject",
            )
            .build()
    }
}

mod imp {
    use super::*;

    #[derive(Default)]
    pub struct SampleApplication {
        hello_world_skeleton: HelloWorldSkeleton,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SampleApplication {
        const NAME: &'static str = "SampleApplication";

        type Type = super::SampleApplication;

        type ParentType = gio::Application;
    }

    impl ObjectImpl for SampleApplication {
        fn constructed(&self) {
            self.hello_world_skeleton.set_application(&self.obj());
        }
    }

    impl ApplicationImpl for SampleApplication {
        fn dbus_register(
            &self,
            connection: &gio::DBusConnection,
            object_path: &str,
        ) -> Result<(), glib::Error> {
            self.parent_dbus_register(connection, object_path)?;
            self.hello_world_skeleton
                .export(connection, "/com/github/gtk_rs/examples/HelloWorld")?;
            println!("registered object on session bus");
            Ok(())
        }

        fn dbus_unregister(&self, connection: &gio::DBusConnection, object_path: &str) {
            self.parent_dbus_unregister(connection, object_path);
            self.hello_world_skeleton
                .unexport_from_connection(connection);
            println!("Unregistered object");
        }

        fn shutdown(&self) {
            self.parent_shutdown();
            println!("Good bye!");
        }

        fn activate(&self) {
            println!(
                "Waiting for DBus Hello method to be called. Call the following command from another terminal:"
            );
            println!(
                "dbus-send --print-reply --dest=com.github.gtk-rs.examples.RegisterDBusObject /com/github/gtk_rs/examples/HelloWorld com.github.gtk_rs.examples.HelloWorld.Hello string:YourName"
            );
            println!("Quit with the following command:");
            println!(
                "dbus-send --print-reply --dest=com.github.gtk-rs.examples.RegisterDBusObject /com/github/gtk_rs/examples/HelloWorld com.github.gtk_rs.examples.HelloWorld.GoodBye"
            );
        }
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}
