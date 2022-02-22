// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::ParamFlags;
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

        pub mod imp {
            use super::*;

            #[derive(Props, Default)]
            pub struct Foo {
                #[prop(get, set)]
                bar: Mutex<String>,
                #[prop(get = Self::hello_world)]
                _buzz: PhantomData<String>,
                #[prop(get, set = Self::set_fizz, name = "fizz")]
                fizz: RefCell<String>,
                #[prop(name = "author-name", get, set, type = String, member = name)]
                #[prop(name = "author-nick", get, set, type = String, member = nick)]
                author: RefCell<Author>,
                #[prop(
                    type = String,
                    get = |t: &Self| t.author.borrow().name.to_owned(),
                    set = Self::set_author_name)]
                fake_field: PhantomData<String>,
                #[prop(get, set, user_1, user_2, lax_validation)]
                custom_flags: RefCell<String>,
                #[prop(get, set, builder())]
                simple_builder: RefCell<u32>,
                #[prop(get, set, builder().minimum(0).maximum(5))]
                numeric_builder: RefCell<u32>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }

            impl Foo {
                fn set_author_name(&self, value: String) {
                    self.author.borrow_mut().name = value;
                }
                fn hello_world(&self) -> String {
                    String::from("Hello world!")
                }
                fn set_fizz(&self, value: String) {
                    *self.fizz.borrow_mut() = format!("custom set: {}", value);
                }
            }
        }

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<imp::Foo>);
        }
    }

    use foo::imp::FooExt;

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

    // Multiple props on the same field
    myfoo.set_property("author-name", "freddy".to_value());
    let author_name: String = myfoo.property("author-name");
    assert_eq!(author_name, "freddy".to_string());

    myfoo.set_property("author-nick", "freddy-nick".to_value());
    let author_name: String = myfoo.property("author-nick");
    assert_eq!(author_name, "freddy-nick".to_string());

    // custom flags
    assert_eq!(
        myfoo.find_property("custom_flags").unwrap().flags(),
        ParamFlags::USER_1
            | ParamFlags::USER_2
            | ParamFlags::READWRITE
            | ParamFlags::LAX_VALIDATION
    );

    // Test `FooExt`
    // getters
    {
        // simple
        let bar = myfoo.bar();
        assert_eq!(bar, myfoo.property::<String>("bar"));

        // custom
        let buzz = myfoo.buzz();
        assert_eq!(buzz, myfoo.property::<String>("buzz"));

        // member of struct field
        let author_nick = myfoo.author_nick();
        assert_eq!(author_nick, myfoo.property::<String>("author-nick"));
    }

    // setters
    {
        // simple
        myfoo.set_bar("setter working".to_string());
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

        // custom
        myfoo.set_fake_field("fake setter".to_string());
        assert_eq!(
            myfoo.property::<String>("author-name"),
            "fake setter".to_string()
        );

        // member of struct field
        myfoo.set_author_nick("setter nick".to_string());
        assert_eq!(
            myfoo.property::<String>("author-nick"),
            "setter nick".to_string()
        );
    }
}
