use glib::prelude::*;
#[test]
fn props() {
    mod foo {
        use glib::prelude::*;
        use glib::subclass::prelude::*;
        use glib_macros::Props;
        use std::cell::RefCell;

        mod imp {
            use super::*;

            #[derive(Props, Default)]
            pub struct Foo {
                #[prop("bar", "bar", "This is a bar", None, glib::ParamFlags::READWRITE)]
                bar: RefCell<String>,
                #[prop("buzz", "buzz", "This is a buzz", 1, 100, 1, glib::ParamFlags::READWRITE)]
                buzz: RefCell<u32>,
            }

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }
        }

        pub trait FooExt: 'static {
            fn test(&self);
        }

        impl<O: IsA<Foo>> FooExt for O {
            fn test(&self) {
                let _self = self.as_ref().downcast_ref::<Foo>().unwrap().imp();
                unimplemented!()
            }
        }

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<imp::Foo>);
        }
    }

    let myfoo: foo::Foo = glib::object::Object::new(&[]).unwrap();

    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "".to_string());

    myfoo.set_property("bar", "epic".to_value());
    let bar: String = myfoo.property("bar");
    assert_eq!(bar, "epic".to_string());

    myfoo.set_property("buzz", 100u32);
    let buzz: u32 = myfoo.property("buzz");
    assert_eq!(buzz, 100);

}
