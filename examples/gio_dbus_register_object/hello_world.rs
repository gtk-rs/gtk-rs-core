use crate::SampleApplication;
use gio::prelude::*;
use gio::subclass::prelude::*;

glib::wrapper! {
    pub struct HelloWorldSkeleton(ObjectSubclass<imp::HelloWorldSkeleton>)
        @extends gio::DBusInterfaceSkeleton;
}

impl Default for HelloWorldSkeleton {
    fn default() -> Self {
        glib::Object::new()
    }
}

impl HelloWorldSkeleton {
    pub(crate) fn set_application(&self, app: &SampleApplication) {
        self.imp().set_application(app);
    }
}

mod imp {
    use super::*;
    use glib::WeakRef;
    use std::cell::RefCell;
    use std::fs::File;
    use std::time::{Duration, Instant};

    #[derive(Default, gio::DBusInterfaceSkeleton)]
    #[dbus_interface(name = "com.github.gtk_rs.examples.HelloWorld")]
    pub struct HelloWorldSkeleton {
        application: RefCell<WeakRef<SampleApplication>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HelloWorldSkeleton {
        const NAME: &'static str = "HelloWorldSkeleton";
        type Type = super::HelloWorldSkeleton;
        type ParentType = gio::DBusInterfaceSkeleton;
    }

    impl ObjectImpl for HelloWorldSkeleton {}

    impl HelloWorldSkeleton {
        pub(super) fn set_application(&self, app: &SampleApplication) {
            self.application.replace(app.downgrade());
        }
    }

    #[gio::dbus_methods]
    impl HelloWorldSkeleton {
        #[deprecated]
        fn hello(&self, name: String) -> String {
            let greet = format!("Hello {name}!");
            println!("{greet}");
            _ = self.emit_greeted(&name);
            greet
        }

        #[dbus(out_args("name", "effective_delay"))]
        async fn slow_hello(
            &self,
            #[dbus(connection)] connection: gio::DBusConnection,
            #[dbus(invocation)] invocation: gio::DBusMethodInvocation,
            name: String,
            delay: u32,
        ) -> Result<(String, f64), glib::Error> {
            if delay > 4 {
                return Err(glib::Error::new(
                    gio::DBusError::InvalidArgs,
                    "delay must not be greater than 4 seconds",
                ));
            }

            let instant = Instant::now();
            glib::timeout_future(Duration::from_secs(delay as u64)).await;
            let greet = format!("Hello {name} after {delay} seconds!");
            println!("{greet}");
            _ = self.emit_greeted(&name);
            Ok((greet, instant.elapsed().as_secs_f64()))
        }

        #[dbus(manual_return(glib::variant::Handle))]
        fn fd(&self, path: String, #[dbus(invocation)] invocation: gio::DBusMethodInvocation) {
            let fd_list = gio::UnixFDList::new();
            match File::open(path) {
                Ok(file) => {
                    fd_list.append(file).unwrap();
                    let handle = glib::variant::Handle(0);
                    let result = (handle,).to_variant();
                    invocation.return_value_with_unix_fd_list(Some(&result), Some(&fd_list));
                }
                Err(error) => {
                    invocation.return_gerror(glib::Error::new(
                        gio::DBusError::IoError,
                        &format!("unable to open file: {error}"),
                    ));
                }
            }
        }

        #[allow(clippy::manual_async_fn)]
        fn goodbye(&self) -> impl Future<Output = ()> {
            async {
                if let Some(app) = self.application.borrow().upgrade() {
                    app.quit();
                }
            }
        }

        fn r#raw_ident(&self) {}

        #[dbus(signal)]
        fn greeted(&self, name: &str) {}
    }
}
