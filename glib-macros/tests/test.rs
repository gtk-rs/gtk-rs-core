// Take a look at the license at the top of the repository in the LICENSE file.

use glib::prelude::*;
use glib::translate::{FromGlib, IntoGlib};

#[test]
fn derive_error_domain() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::ErrorDomain)]
    #[error_domain(name = "TestError")]
    enum TestError {
        Invalid,
        Bad,
        Wrong,
    }

    let err = glib::Error::new(TestError::Bad, "oh no!");
    assert!(err.is::<TestError>());
    assert!(matches!(err.kind::<TestError>(), Some(TestError::Bad)));
}

#[test]
fn derive_shared_arc() {
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct MyInnerShared {
        foo: String,
    }
    #[derive(Debug, Eq, PartialEq, Clone, glib::SharedBoxed)]
    #[shared_boxed_type(name = "MyShared")]
    struct MyShared(std::sync::Arc<MyInnerShared>);

    let t = MyShared::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyShared");

    let p = MyShared(std::sync::Arc::new(MyInnerShared {
        foo: String::from("bar"),
    }));

    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
    let v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);
    let p_clone = v.get::<MyShared>().unwrap();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 3);
    drop(p_clone);
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);
    drop(v);
    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
}

#[test]
fn derive_shared_arc_nullable() {
    #[derive(Debug, Eq, PartialEq, Clone)]
    struct MyInnerNullableShared {
        foo: String,
    }
    #[derive(Clone, Debug, PartialEq, Eq, glib::SharedBoxed)]
    #[shared_boxed_type(name = "MyNullableShared", nullable)]
    struct MyNullableShared(std::sync::Arc<MyInnerNullableShared>);

    let t = MyNullableShared::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyNullableShared");

    let p = MyNullableShared(std::sync::Arc::new(MyInnerNullableShared {
        foo: String::from("bar"),
    }));

    assert_eq!(std::sync::Arc::strong_count(&p.0), 1);
    let _v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.0), 2);

    let p = Some(MyNullableShared(std::sync::Arc::new(
        MyInnerNullableShared {
            foo: String::from("foo"),
        },
    )));

    assert_eq!(std::sync::Arc::strong_count(&p.as_ref().unwrap().0), 1);
    let v = p.to_value();
    assert_eq!(std::sync::Arc::strong_count(&p.as_ref().unwrap().0), 2);
    assert_eq!(
        p.as_ref().unwrap().0.foo,
        v.get::<MyNullableShared>().unwrap().0.foo
    );

    let b: Option<&MyNullableShared> = None;
    let v = b.to_value();
    assert_eq!(None, v.get::<Option<MyNullableShared>>().unwrap());
}

#[test]
fn derive_enum() {
    #[derive(Debug, Eq, PartialEq, Clone, Copy, glib::Enum)]
    #[repr(u32)]
    #[enum_type(name = "TestAnimalType")]
    enum Animal {
        Goat,
        #[enum_value(name = "The Dog")]
        Dog,
        #[enum_value(name = "The Cat", nick = "chat")]
        Cat = 5,
        Badger,
    }

    assert_eq!(Animal::Goat.into_glib(), 0);
    assert_eq!(Animal::Dog.into_glib(), 1);
    assert_eq!(Animal::Cat.into_glib(), 5);

    assert_eq!(unsafe { Animal::from_glib(0) }, Animal::Goat);
    assert_eq!(unsafe { Animal::from_glib(1) }, Animal::Dog);
    assert_eq!(unsafe { Animal::from_glib(5) }, Animal::Cat);

    assert_eq!(Animal::Goat.to_value().get::<Animal>(), Ok(Animal::Goat));
    assert_eq!(Animal::Dog.to_value().get::<Animal>(), Ok(Animal::Dog));
    assert_eq!(Animal::Cat.to_value().get::<Animal>(), Ok(Animal::Cat));

    let t = Animal::static_type();
    assert!(t.is_a(glib::Type::ENUM));
    assert_eq!(t.name(), "TestAnimalType");

    let e = glib::EnumClass::new(t).expect("EnumClass::new failed");
    let v = e.value(0).expect("EnumClass::get_value(0) failed");
    assert_eq!(v.name(), "Goat");
    assert_eq!(v.nick(), "goat");
    let v = e.value(1).expect("EnumClass::get_value(1) failed");
    assert_eq!(v.name(), "The Dog");
    assert_eq!(v.nick(), "dog");
    let v = e.value(5).expect("EnumClass::get_value(5) failed");
    assert_eq!(v.name(), "The Cat");
    assert_eq!(v.nick(), "chat");
    assert_eq!(e.value(2), None);
}

#[test]
fn derive_boxed() {
    #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "MyBoxed")]
    struct MyBoxed(String);

    let t = MyBoxed::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyBoxed");

    let b = MyBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<&MyBoxed>().unwrap());
    assert_eq!(b, v.get::<MyBoxed>().unwrap());
}

#[test]
fn derive_boxed_nullable() {
    #[derive(Clone, Debug, PartialEq, Eq, glib::Boxed)]
    #[boxed_type(name = "MyNullableBoxed", nullable)]
    struct MyNullableBoxed(String);

    let t = MyNullableBoxed::static_type();
    assert!(t.is_a(glib::Type::BOXED));
    assert_eq!(t.name(), "MyNullableBoxed");

    let b = MyNullableBoxed(String::from("abc"));
    let v = b.to_value();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b = Some(MyNullableBoxed(String::from("def")));
    let v = b.to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b = Some(MyNullableBoxed(String::from("def")));
    let v = (&b).to_value();
    let b = b.unwrap();
    assert_eq!(&b, v.get::<Option<&MyNullableBoxed>>().unwrap().unwrap());
    assert_eq!(b, v.get::<Option<MyNullableBoxed>>().unwrap().unwrap());

    let b: Option<MyNullableBoxed> = None;
    let v = b.to_value();
    assert_eq!(None, v.get::<Option<&MyNullableBoxed>>().unwrap());
    assert_eq!(None, v.get::<Option<MyNullableBoxed>>().unwrap());
}

#[test]
fn attr_flags() {
    #[glib::flags(name = "MyFlags")]
    enum MyFlags {
        #[flags_value(name = "Flag A", nick = "nick-a")]
        A = 0b00000001,
        #[flags_value(name = "Flag B")]
        B = 0b00000010,
        #[flags_value(skip)]
        AB = Self::A.bits() | Self::B.bits(),
        C = 0b00000100,
    }

    assert_eq!(MyFlags::A.bits(), 1);
    assert_eq!(MyFlags::B.bits(), 2);
    assert_eq!(MyFlags::AB.bits(), 3);

    assert_eq!(MyFlags::empty().into_glib(), 0);
    assert_eq!(MyFlags::A.into_glib(), 1);
    assert_eq!(MyFlags::B.into_glib(), 2);
    assert_eq!(MyFlags::AB.into_glib(), 3);

    assert_eq!(unsafe { MyFlags::from_glib(0) }, MyFlags::empty());
    assert_eq!(unsafe { MyFlags::from_glib(1) }, MyFlags::A);
    assert_eq!(unsafe { MyFlags::from_glib(2) }, MyFlags::B);
    assert_eq!(unsafe { MyFlags::from_glib(3) }, MyFlags::AB);

    assert_eq!(
        MyFlags::empty().to_value().get::<MyFlags>(),
        Ok(MyFlags::empty())
    );
    assert_eq!(MyFlags::A.to_value().get::<MyFlags>(), Ok(MyFlags::A));
    assert_eq!(MyFlags::B.to_value().get::<MyFlags>(), Ok(MyFlags::B));
    assert_eq!(MyFlags::AB.to_value().get::<MyFlags>(), Ok(MyFlags::AB));

    let t = MyFlags::static_type();
    assert!(t.is_a(glib::Type::FLAGS));
    assert_eq!(t.name(), "MyFlags");

    let e = glib::FlagsClass::new(t).expect("FlagsClass::new failed");
    let v = e.value(1).expect("FlagsClass::get_value(1) failed");
    assert_eq!(v.name(), "Flag A");
    assert_eq!(v.nick(), "nick-a");
    let v = e.value(2).expect("FlagsClass::get_value(2) failed");
    assert_eq!(v.name(), "Flag B");
    assert_eq!(v.nick(), "b");
    let v = e.value(4).expect("FlagsClass::get_value(4) failed");
    assert_eq!(v.name(), "C");
    assert_eq!(v.nick(), "c");

    assert!(e.value_by_name("Flag A").is_some());
    assert!(e.value_by_name("Flag B").is_some());
    assert!(e.value_by_name("AB").is_none());
    assert!(e.value_by_name("C").is_some());

    assert!(e.value_by_nick("nick-a").is_some());
    assert!(e.value_by_nick("b").is_some());
    assert!(e.value_by_nick("ab").is_none());
    assert!(e.value_by_nick("c").is_some());
}

#[test]
fn subclassable() {
    mod foo {
        use super::*;
        use glib::subclass::prelude::*;

        mod imp {
            use super::*;

            #[derive(Default)]
            pub struct Foo {}

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                const NAME: &'static str = "MyFoo";
                type Type = super::Foo;
            }

            impl ObjectImpl for Foo {}
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
}

#[test]
fn subclass_derives_name() {
    mod foo {
        use glib::subclass::prelude::*;

        mod imp {
            use super::*;

            #[derive(Default)]
            pub struct Foo {}

            #[glib::object_subclass]
            impl ObjectSubclass for Foo {
                type Type = super::Foo;
            }

            impl ObjectImpl for Foo {}
        }

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<imp::Foo>);
        }
    }

    mod bar {
        use glib::subclass::prelude::*;

        mod imp {
            use super::*;

            #[derive(Default)]
            pub struct Bar {}

            #[glib::object_subclass]
            impl ObjectSubclass for super::imp::Bar {
                type Type = super::Bar;
            }

            impl ObjectImpl for Bar {}
        }

        glib::wrapper! {
            pub struct Bar(ObjectSubclass<imp::Bar>);
        }
    }

    let foo_obj: foo::Foo = glib::Object::new(&[]).unwrap();
    assert_eq!(foo_obj.type_().name(), "test__foo__imp_Foo");

    let bar_obj: bar::Bar = glib::Object::new(&[]).unwrap();
    assert_eq!(bar_obj.type_().name(), "test__bar__imp_Bar");
}

#[test]
fn derive_variant() {
    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant1 {
        some_string: String,
        some_int: i32,
    }

    assert_eq!(Variant1::static_variant_type().as_str(), "(si)");
    let v = Variant1 {
        some_string: String::from("bar"),
        some_int: 2,
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(si)");
    assert_eq!(var.get::<Variant1>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant2 {
        some_string: Option<String>,
        some_int: i32,
    }

    assert_eq!(Variant2::static_variant_type().as_str(), "(msi)");
    let v = Variant2 {
        some_string: Some(String::from("bar")),
        some_int: 2,
    };
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(msi)");
    assert_eq!(var.get::<Variant2>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant3(u32, String);

    assert_eq!(Variant3::static_variant_type().as_str(), "(us)");
    let v = Variant3(1, String::from("foo"));
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "(us)");
    assert_eq!(var.get::<Variant3>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant4;

    assert_eq!(Variant4::static_variant_type().as_str(), "()");
    let v = Variant4;
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "()");
    assert_eq!(var.get::<Variant4>(), Some(v));

    #[derive(Debug, PartialEq, Eq, glib::Variant)]
    struct Variant5();

    assert_eq!(Variant5::static_variant_type().as_str(), "()");
    let v = Variant5();
    let var = v.to_variant();
    assert_eq!(var.type_().as_str(), "()");
    assert_eq!(var.get::<Variant5>(), Some(v));
}

#[test]
fn closure() {
    let empty = glib::closure!(|| {});
    empty.invoke::<()>(&[]);

    let no_arg = glib::closure!(|| 2i32);
    assert_eq!(no_arg.invoke::<i32>(&[]), 2);

    let add_1 = glib::closure!(|x: i32| x + 1);
    assert_eq!(add_1.invoke::<i32>(&[&3i32]), 4);

    let concat_str = glib::closure!(|s: &str| s.to_owned() + " World");
    assert_eq!(concat_str.invoke::<String>(&[&"Hello"]), "Hello World");

    let weak_test = {
        let obj = glib::Object::new::<glib::Object>(&[]).unwrap();

        assert_eq!(obj.ref_count(), 1);
        let weak_test = glib::closure_local!(@watch obj => move || obj.ref_count());
        assert_eq!(obj.ref_count(), 1);
        assert_eq!(weak_test.invoke::<u32>(&[]), 3);
        assert_eq!(obj.ref_count(), 1);

        weak_test
    };
    weak_test.invoke::<()>(&[]);

    {
        trait TestExt {
            fn ref_count_in_closure(&self) -> u32;
        }

        impl TestExt for glib::Object {
            fn ref_count_in_closure(&self) -> u32 {
                let closure = glib::closure_local!(@watch self as obj => move || obj.ref_count());
                closure.invoke::<u32>(&[])
            }
        }

        let obj = glib::Object::new::<glib::Object>(&[]).unwrap();
        assert_eq!(obj.ref_count_in_closure(), 3);
    }

    {
        struct A {
            obj: glib::Object,
        }

        impl A {
            fn ref_count_in_closure(&self) -> u32 {
                let closure =
                    glib::closure_local!(@watch self.obj as obj => move || obj.ref_count());
                closure.invoke::<u32>(&[])
            }
        }

        let a = A {
            obj: glib::Object::new::<glib::Object>(&[]).unwrap(),
        };
        assert_eq!(a.ref_count_in_closure(), 3);
    }

    let strong_test = {
        let obj = glib::Object::new::<glib::Object>(&[]).unwrap();

        let strong_test = glib::closure_local!(@strong obj => move || obj.ref_count());
        assert_eq!(strong_test.invoke::<u32>(&[]), 2);

        strong_test
    };
    assert_eq!(strong_test.invoke::<u32>(&[]), 1);

    let weak_none_test = {
        let obj = glib::Object::new::<glib::Object>(&[]).unwrap();

        let weak_none_test = glib::closure_local!(@weak-allow-none obj => move || {
            obj.map(|o| o.ref_count()).unwrap_or_default()
        });
        assert_eq!(weak_none_test.invoke::<u32>(&[]), 2);

        weak_none_test
    };
    assert_eq!(weak_none_test.invoke::<u32>(&[]), 0);

    {
        let obj1 = glib::Object::new::<glib::Object>(&[]).unwrap();
        let obj2 = glib::Object::new::<glib::Object>(&[]).unwrap();

        let obj_arg_test =
            glib::closure!(|a: glib::Object, b: glib::Object| { a.ref_count() + b.ref_count() });
        let rc = obj_arg_test.invoke::<u32>(&[&obj1, &obj2]);
        assert_eq!(rc, 6);

        let alias_test = glib::closure_local!(@strong obj1 as a, @strong obj2 => move || {
            a.ref_count() + obj2.ref_count()
        });
        assert_eq!(alias_test.invoke::<u32>(&[]), 4);
    }

    {
        struct A {
            a: glib::Object,
        }

        let a = glib::Object::new::<glib::Object>(&[]).unwrap();
        let a_struct = A { a };
        let struct_test = glib::closure_local!(@strong a_struct.a as a => move || {
            a.ref_count()
        });
        assert_eq!(struct_test.invoke::<u32>(&[]), 2);
    }
}
