use glib::prelude::*;
#[test]
fn props() {
    mod foo {
        use glib::prelude::*;
        use glib::subclass::prelude::*;
        use glib_macros::Props;
        use std::cell::RefCell;
        use std::marker::PhantomData;
        use std::sync::Mutex;

        #[derive(Default, Clone)]
        struct Author {
            name: String,
            nick: String,
        }

        mod imp {
            use super::*;

            #[derive(Props, Default)]
            pub struct Foo {
                #[prop(get, set)]
                bar: Mutex<String>,
                #[prop(get = Self::hello_world)]
                _buzz: PhantomData<String>,
                #[prop(get, set = Self::set_fizz, name = "fizz")]
                fizz: RefCell<String>,
                #[prop(type = String, member = name, get, name = "author-name")]
                #[prop(type = String, member = nick, get, name = "author-nick")]
                author: RefCell<Author>,
                #[prop(
                    type = String,
                    get = |t: &Self| t.author.borrow().name.to_value(),
                    set = Self::set_author_name)]
                author_name: PhantomData<String>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }

            impl Foo {
                fn set_author_name(&self, value: &glib::Value) {
                    self.author.borrow_mut().name = value.get().unwrap();
                }
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

    // PhantomData with custom getter/setter to other inner value
    myfoo.set_property("author-name", "freddy".to_value());
    let author_name: String = myfoo.property("author-name");
    assert_eq!(author_name, "freddy".to_string());
}
