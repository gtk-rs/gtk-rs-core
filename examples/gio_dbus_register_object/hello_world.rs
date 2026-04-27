use gio::DBusCallFlags;
use std::time::Duration;

#[gio::dbus_interface(
    name = "com.github.gtk_rs.examples.HelloWorld",
    type_name = "GtkRustHelloWorld",
    emits_changed_signal = "true",
    // crate = gio,
)]
pub(crate) trait HelloWorld {
    #[deprecated]
    fn hello(&self, name: String) -> String;

    #[dbus(name = "Hello2", out_args("name", "effective_delay"))]
    fn hello_with_delay(&self, name: String, delay: u32) -> (String, u32);

    #[dbus(no_reply)]
    fn no_greeting(&self, name: String);

    #[dbus(property, get, name = "Name")]
    fn last_greeted_name(&self) -> String;

    #[dbus(property, get)]
    #[deprecated]
    fn count(&self) -> i64;

    #[dbus(signal)]
    fn greeted(&self, name: String);
}

async fn hello_tau(iface: &HelloWorld) -> Result<String, glib::Error> {
    iface
        .hello("Tau".to_owned())
        .timeout(Duration::from_secs(5))
        .flags(DBusCallFlags::ALLOW_INTERACTIVE_AUTHORIZATION)
        .await
}
