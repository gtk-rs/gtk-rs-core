// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::ParamFlags;

#[cfg(test)]
mod base {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib_macros::Properties;
    use std::marker::PhantomData;

    pub mod imp {
        use glib::{ParamSpec, Value};

        use super::*;

        #[derive(Properties, Default)]
        #[properties(wrapper_type = super::Base)]
        pub struct Base {
            #[property(get = Self::not_overridden)]
            overridden: PhantomData<u32>,
            #[property(get = Self::not_overridden)]
            not_overridden: PhantomData<u32>,
        }

        impl ObjectImpl for Base {
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
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        impl Base {
            fn not_overridden(&self) -> u32 {
                42
            }
        }
    }

    glib::wrapper! {
        pub struct Base(ObjectSubclass<imp::Base>);
    }

    unsafe impl<T: ObjectImpl> IsSubclassable<T> for Base {}
}

#[cfg(test)]
mod foo {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib_macros::{Properties, ValueDelegate};
    use once_cell::sync::OnceCell;
    use std::cell::Cell;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::sync::Mutex;

    use super::base::Base;

    #[derive(ValueDelegate, Default, Debug, PartialEq)]
    pub struct MyPropertyValue(pub i32);

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
            #[property(get, set)]
            string_vec: RefCell<Vec<String>>,
            #[property(get, set, builder(glib::VariantTy::DOUBLE))]
            variant: RefCell<Option<glib::Variant>>,
            #[property(get = |_| 42.0, set)]
            infer_inline_type: RefCell<f64>,
            // The following property doesn't store any data. The value of the property is calculated
            // when the value is accessed.
            #[property(get = Self::hello_world)]
            _buzz: PhantomData<String>,
            #[property(get, set)]
            my_property_value: RefCell<MyPropertyValue>,
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
            #[property(get)]
            read_only_text: String,
            #[property(get, set, explicit_notify, lax_validation)]
            custom_flags: RefCell<String>,
            #[property(get, set, default = "hello")]
            with_default: RefCell<String>,
            #[property(get, set, builder())]
            simple_builder: RefCell<u32>,
            #[property(get, set, builder().minimum(0).maximum(5))]
            numeric_builder: RefCell<u32>,
            #[property(get, set, minimum = 0, maximum = 5)]
            builder_fields_without_builder: RefCell<u32>,
            #[property(get, set, builder('c'))]
            builder_with_required_param: RefCell<char>,
            #[property(get, set)]
            boxed: RefCell<SimpleBoxedString>,
            #[property(get, set, builder(SimpleEnum::One))]
            fenum: RefCell<SimpleEnum>,
            #[property(get, set, nullable)]
            object: RefCell<Option<glib::Object>>,
            #[property(get, set, nullable)]
            optional: RefCell<Option<String>>,
            #[property(get, set)]
            smart_pointer: Rc<RefCell<String>>,
            #[property(get, set)]
            once_cell: OnceCell<u8>,
            #[property(get, set)]
            cell: Cell<u8>,
            #[property(get = Self::overridden, override_class = Base)]
            overridden: PhantomData<u32>,
            #[property(get, set)]
            weak_ref_prop: glib::WeakRef<glib::Object>,
            #[property(get, set)]
            send_weak_ref_prop: glib::SendWeakRef<glib::Object>,
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
            type ParentType = Base;
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
            fn overridden(&self) -> u32 {
                43
            }
        }
    }

    glib::wrapper! {
        pub struct Foo(ObjectSubclass<imp::Foo>) @extends Base;
    }
}

#[test]
fn props() {
    let myfoo: foo::Foo = glib::object::Object::new();

    // Read values
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "".to_string());
    let string_vec: Vec<String> = myfoo.property("string-vec");
    assert!(string_vec.is_empty());
    let my_property_value: foo::MyPropertyValue = myfoo.property("my-property-value");
    assert_eq!(my_property_value, foo::MyPropertyValue(0));
    let var: Option<glib::Variant> = myfoo.property("variant");
    assert!(var.is_none());

    // Set values
    myfoo.set_property("bar", "epic".to_value());
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "epic".to_string());
    myfoo.set_property("string-vec", ["epic", "more epic"].to_value());
    let string_vec: Vec<String> = myfoo.property("string-vec");
    assert_eq!(
        string_vec,
        vec!["epic".to_string(), "more epic".to_string()]
    );
    let myv = Some(2.0f64.to_variant());
    myfoo.set_property("variant", &myv);
    let var: Option<glib::Variant> = myfoo.property("variant");
    assert_eq!(var, myv);

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

    // read_only
    assert_eq!(
        myfoo.find_property("read_only_text").unwrap().flags(),
        ParamFlags::READABLE
    );

    // custom flags
    assert_eq!(
        myfoo.find_property("custom_flags").unwrap().flags(),
        ParamFlags::EXPLICIT_NOTIFY | ParamFlags::READWRITE | ParamFlags::LAX_VALIDATION
    );

    // default value
    assert_eq!(
        myfoo
            .find_property("with_default")
            .unwrap()
            .default_value()
            .get::<String>()
            .unwrap(),
        "hello".to_string()
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

    assert_eq!(
        {
            let spec = myfoo
                .find_property("builder_fields_without_builder")
                .unwrap()
                .downcast::<glib::ParamSpecUInt>()
                .unwrap();
            (spec.minimum(), spec.maximum())
        },
        (0, 5)
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

        myfoo.set_infer_inline_type(42.0);
        assert_eq!(myfoo.property::<f64>("infer-inline-type"), 42.0);

        // simple with various String types
        myfoo.set_bar(String::from("setter working"));
        myfoo.set_bar(glib::GString::from("setter working"));
        assert_eq!(
            myfoo.property::<String>("bar"),
            "setter working".to_string()
        );

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

    // overrides
    {
        let overridden: u32 = myfoo.property("overridden");
        assert_eq!(overridden, 43);
        let not_overridden: u32 = myfoo.property("not-overridden");
        assert_eq!(not_overridden, 42);
    }

    // optional
    myfoo.set_optional(Some("Hello world"));
    assert_eq!(myfoo.optional(), Some("Hello world".to_string()));
    myfoo.connect_optional_notify(|_| println!("notified"));

    // object subclass
    let myobj = glib::BoxedAnyObject::new("");
    myfoo.set_object(Some(myobj.upcast_ref()));
    assert_eq!(myfoo.object(), Some(myobj.upcast()))
}
