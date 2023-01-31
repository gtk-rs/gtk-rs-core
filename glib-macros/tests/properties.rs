// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::ParamFlags;
#[test]
fn props() {
    mod foo {
        use glib::prelude::*;
        use glib::subclass::prelude::*;
        use glib_macros::Properties;
        use std::cell::Cell;
        use std::cell::RefCell;
        use std::marker::PhantomData;
        use std::sync::Mutex;

        use once_cell::sync::OnceCell;

        #[derive(Clone, Default, Debug, PartialEq, Eq, glib::Boxed)]
        #[boxed_type(name = "SimpleBoxedString")]
        pub struct SimpleBoxedString(pub String);

        #[derive(Copy, Default, Clone, Debug, PartialEq, Eq, glib::Enum)]
        #[enum_type(name = "SimpleEnum")]
        pub enum SimpleEnum {
            #[default]
            One,
        }

        #[derive(Default, Clone)]
        struct Author {
            name: String,
            nick: String,
        }

        pub mod imp {
            use glib::{ParamSpec, Value};
            use std::rc::Rc;

            use super::*;

            #[derive(Properties, Default)]
            #[properties(wrapper_type = super::Foo)]
            pub struct Foo {
                #[property(get, set)]
                bar: Mutex<String>,
                #[property(get, set)]
                double: RefCell<f64>,
                // The following property doesn't store any data. The value of the property is calculated
                // when the value is accessed.
                #[property(get = Self::hello_world)]
                _buzz: PhantomData<String>,
                #[property(get, set = Self::set_fizz, name = "fizz", nick = "fizz-nick",
                    blurb = "short description stored in the GLib type system"
                )]
                fizz: RefCell<String>,
                #[property(name = "author-name", get, set, type = String, member = name)]
                #[property(name = "author-nick", get, set, type = String, member = nick)]
                author: RefCell<Author>,
                #[property(
                    type = String,
                    get = |t: &Self| t.author.borrow().name.to_owned(),
                    set = Self::set_author_name)]
                fake_field: PhantomData<String>,
                #[property(get, set, user_1, user_2, lax_validation)]
                custom_flags: RefCell<String>,
                #[property(get, set, user_1, glib::ParamFlags::USER_2)]
                custom_flags_by_path: RefCell<String>,
                #[property(get, set, builder())]
                simple_builder: RefCell<u32>,
                #[property(get, set, builder().minimum(0).maximum(5))]
                numeric_builder: RefCell<u32>,
                #[property(get, set, builder('c'))]
                builder_with_required_param: RefCell<char>,
                #[property(get, set)]
                boxed: RefCell<SimpleBoxedString>,
                #[property(get, set, builder(SimpleEnum::One))]
                fenum: RefCell<SimpleEnum>,
                #[property(get, set)]
                object: RefCell<Option<glib::Object>>,
                #[property(get, set)]
                optional: RefCell<Option<String>>,
                #[property(get, set)]
                smart_pointer: Rc<RefCell<String>>,
                #[property(get, set)]
                once_cell: OnceCell<u8>,
                #[property(get, set)]
                cell: Cell<u8>,
            }

            impl ObjectImpl for Foo {
                fn properties() -> &'static [ParamSpec] {
                    Self::derived_properties()
                }
                fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
                    Self::derived_set_property(self, _id, _value, _pspec)
                }
                fn property(&self, id: usize, _pspec: &ParamSpec) -> Value {
                    Self::derived_property(self, id, _pspec)
                }
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

    let myfoo: foo::Foo = glib::object::Object::new();

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
    // custom flags
    assert_eq!(
        myfoo.find_property("custom_flags_by_path").unwrap().flags(),
        ParamFlags::READWRITE | ParamFlags::USER_1 | ParamFlags::USER_2
    );

    // numeric builder
    assert_eq!(
        myfoo
            .find_property("numeric_builder")
            .unwrap()
            .downcast::<glib::ParamSpecUInt>()
            .unwrap()
            .maximum(),
        5
    );

    // builder with required param
    assert_eq!(
        myfoo
            .find_property("builder_with_required_param")
            .unwrap()
            .default_value()
            .get::<char>()
            .unwrap(),
        'c'
    );

    // boxed type
    assert_eq!(
        myfoo.property::<foo::SimpleBoxedString>("boxed"),
        foo::SimpleBoxedString("".into())
    );

    // optional
    assert_eq!(myfoo.property::<Option<String>>("optional"), None,);

    myfoo.connect_optional_notify(|_| println!("notified"));

    // Test `FooPropertiesExt`
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
        myfoo.set_bar("setter working");
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

        myfoo.set_double(0.1);
        assert_eq!(myfoo.property::<f64>("double"), 0.1);

        // simple with various String types
        myfoo.set_bar(String::from("setter working"));
        myfoo.set_bar(glib::GString::from("setter working"));
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

        // object subclass
        myfoo.set_object(glib::BoxedAnyObject::new(""));

        // custom
        myfoo.set_fake_field("fake setter");
        assert_eq!(
            myfoo.property::<String>("author-name"),
            "fake setter".to_string()
        );

        // member of struct field
        myfoo.set_author_nick("setter nick");
        assert_eq!(
            myfoo.property::<String>("author-nick"),
            "setter nick".to_string()
        );
    }
}
