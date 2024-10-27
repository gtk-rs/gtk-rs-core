use gio::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};

const EXAMPLE_XML: &str = r#"
  <node>
    <interface name='com.github.gtk_rs.examples.HelloWorld'>
      <method name='Hello'>
        <arg type='s' name='name' direction='in'/>
        <arg type='s' name='greet' direction='out'/>
      </method>
    </interface>
  </node>
"#;

fn on_startup(app: &gio::Application, tx: &Sender<gio::RegistrationId>) {
    let connection = app.dbus_connection().expect("connection");

    let example = gio::DBusNodeInfo::for_xml(EXAMPLE_XML)
        .ok()
        .and_then(|e| e.lookup_interface("com.github.gtk_rs.examples.HelloWorld"))
        .expect("Example interface");

    if let Ok(id) = connection
        .register_object("/com/github/gtk_rs/examples/HelloWorld", &example)
        .method_call(glib::clone!(
            #[strong]
            app,
            move |_, _, _, _, method, params, invocation| {
                match method {
                    "Hello" => {
                        let result = <(String,)>::from_variant(&params)
                            .ok_or_else(|| {
                                glib::Error::new(gio::IOErrorEnum::Failed, "Invalid parameters")
                            })
                            .map(|(name,)| {
                                let greet = format!("Hello {name}!");
                                println!("{greet}");
                                Some(greet.to_variant())
                            });
                        invocation.return_result(result);
                    }
                    _ => unreachable!(),
                }
                app.quit();
            }
        ))
        .build()
    {
        println!("Registered object");
        tx.send(id).unwrap();
    } else {
        eprintln!("Could not register object");
    }
}

fn on_shutdown(app: &gio::Application, rx: &Receiver<gio::RegistrationId>) {
    let connection = app.dbus_connection().expect("connection");
    if let Ok(registration_id) = rx.try_recv() {
        if connection.unregister_object(registration_id).is_ok() {
            println!("Unregistered object");
        } else {
            eprintln!("Could not unregister object");
        }
    }
}

fn main() -> glib::ExitCode {
    let app = gio::Application::builder()
        .application_id("com.github.gtk-rs.examples.RegisterDBusObject")
        .build();
    let _guard = app.hold();
    let (tx, rx) = channel::<gio::RegistrationId>();

    app.connect_startup(move |app| {
        on_startup(app, &tx);
    });

    app.connect_activate(move |_| {
        println!("Waiting for DBus Hello method to be called. Call the following command from another terminal:");
        println!("dbus-send --print-reply --dest=com.github.gtk-rs.examples.RegisterDBusObject /com/github/gtk_rs/examples/HelloWorld com.github.gtk_rs.examples.HelloWorld.Hello string:YourName");
    });

    app.connect_shutdown(move |app| {
        on_shutdown(app, &rx);
    });

    app.run()
}
