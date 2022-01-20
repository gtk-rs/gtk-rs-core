use glib::prelude::*;
#[test]
fn props() {
    mod foo {
        use glib::prelude::*;
        use glib::subclass::prelude::*;
        use glib_macros::Props;
        use std::cell::RefCell;
        use std::sync::Mutex;

        mod imp {
            use super::*;

            #[derive(Props, Default)]
            pub struct Foo {
                #[prop(get, set, name = "bar")]
                bar: Mutex<String>,
                #[prop(get = Self::hello_world, name = "buzz")]
                _buzz: RefCell<String>,
                #[prop(get, set = Self::set_fizz, name = "fizz")]
                fizz: RefCell<String>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }

            impl Foo {
                fn hello_world(&self) -> glib::Value {
                    "Hello world!".to_value()
                }
                fn set_fizz(&self, value: &glib::Value) {
                    let v: String = value.get().unwrap();
                    *self.fizz.borrow_mut() = format!("custom set: {}", v);
                }
            }
        }

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<imp::Foo>);
        }
    }

    let myfoo: foo::Foo = glib::object::Object::new(&[]).unwrap();

    // Read bar
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "".to_string());

    // Set bar
    myfoo.set_property("bar", "epic".to_value());
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "epic".to_string());

    // Custom getter
    let buzz: String = myfoo.property("buzz");
    assert_eq!(buzz, "Hello world!".to_string());

    // Custom setter
    myfoo.set_property("fizz", "test");
    let fizz: String = myfoo.property("fizz");
    assert_eq!(fizz, "custom set: test".to_string());
}
