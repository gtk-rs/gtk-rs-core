use glib::object::ObjectExt;

#[test]
#[glib::clone_block]
fn closure() {
    let empty = #[closure]
    || {};
    empty.invoke::<()>(&[]);

    let no_arg = #[closure]
    || 2i32;
    assert_eq!(no_arg.invoke::<i32>(&[]), 2);

    let add_1 = #[closure]
    |x: i32| x + 1;
    assert_eq!(add_1.invoke::<i32>(&[&3i32]), 4);

    let concat_str = #[closure]
    |s: &str| s.to_owned() + " World";
    assert_eq!(concat_str.invoke::<String>(&[&"Hello"]), "Hello World");

    let ignored_arg = #[closure]
    |x: i32, _, z: i32| x + z;
    assert_eq!(ignored_arg.invoke::<i32>(&[&1i32, &2i32, &3i32]), 4);

    let weak_test = {
        let obj = glib::Object::new::<glib::Object>();

        assert_eq!(obj.ref_count(), 1);
        let weak_test = move |#[watch] obj| obj.ref_count();
        assert_eq!(obj.ref_count(), 1);
        assert_eq!(weak_test.invoke::<u32>(&[]), 2);
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
                let closure = move |#[watch(self)] obj| obj.ref_count();
                closure.invoke::<u32>(&[])
            }
        }

        let obj = glib::Object::new::<glib::Object>();
        assert_eq!(obj.ref_count_in_closure(), 2);
    }

    {
        struct A {
            obj: glib::Object,
        }

        impl A {
            fn ref_count_in_closure(&self) -> u32 {
                let closure = move |#[watch(self.obj)] obj| obj.ref_count();
                closure.invoke::<u32>(&[])
            }
        }

        let a = A {
            obj: glib::Object::new(),
        };
        assert_eq!(a.ref_count_in_closure(), 2);
    }

    let strong_test = {
        let obj = glib::Object::new::<glib::Object>();

        let strong_test = #[closure(local)]
        move |#[strong] obj| obj.ref_count();
        assert_eq!(strong_test.invoke::<u32>(&[]), 2);

        strong_test
    };
    assert_eq!(strong_test.invoke::<u32>(&[]), 1);

    let weak_none_test = {
        let obj = glib::Object::new::<glib::Object>();

        let weak_none_test = #[closure(local)]
        move |#[weak] obj| obj.map(|o| o.ref_count()).unwrap_or_default();
        assert_eq!(weak_none_test.invoke::<u32>(&[]), 2);

        weak_none_test
    };
    assert_eq!(weak_none_test.invoke::<u32>(&[]), 0);

    {
        let obj1 = glib::Object::new::<glib::Object>();
        let obj2 = glib::Object::new::<glib::Object>();

        let obj_arg_test = #[closure]
        |a: glib::Object, b: glib::Object| a.ref_count() + b.ref_count();
        let rc = obj_arg_test.invoke::<u32>(&[&obj1, &obj2]);
        assert_eq!(rc, 6);

        let alias_test = #[closure(local)]
        move |#[strong(obj1)] a, #[strong] obj2| a.ref_count() + obj2.ref_count();
        assert_eq!(alias_test.invoke::<u32>(&[]), 4);
    }

    {
        struct A {
            a: glib::Object,
        }

        let a = glib::Object::new();
        let a_struct = A { a };
        let struct_test = #[closure(local)]
        move |#[strong(a_struct.a)] a| a.ref_count();
        assert_eq!(struct_test.invoke::<u32>(&[]), 2);
    }

    {
        use glib::prelude::*;
        use glib::subclass::prelude::*;

        #[derive(Default)]
        pub struct FooPrivate {}

        #[glib::object_subclass]
        impl ObjectSubclass for FooPrivate {
            const NAME: &'static str = "MyFoo2";
            type Type = Foo;
        }

        impl ObjectImpl for FooPrivate {}

        glib::wrapper! {
            pub struct Foo(ObjectSubclass<FooPrivate>);
        }

        impl Foo {
            fn my_ref_count(&self) -> u32 {
                self.ref_count()
            }
        }

        let cast_test = {
            let f = glib::Object::new::<Foo>();

            assert_eq!(f.my_ref_count(), 1);
            let cast_test = move |#[watch] f| f.my_ref_count();
            assert_eq!(f.my_ref_count(), 1);
            assert_eq!(cast_test.invoke::<u32>(&[]), 2);
            assert_eq!(f.my_ref_count(), 1);

            let f_ref = &f;
            let _ = move |#[watch] f_ref| f_ref.my_ref_count();

            cast_test
        };
        cast_test.invoke::<()>(&[]);
    }

    let sum = #[closure]
    |x: i32, #[rest] rest: &[glib::Value]| -> i32 {
        x + rest.iter().map(|v| v.get::<i32>().unwrap()).sum::<i32>()
    };
    assert_eq!(sum.invoke::<i32>(&[&2i32]), 2i32);
    assert_eq!(sum.invoke::<i32>(&[&2i32, &3i32]), 5i32);
    assert_eq!(sum.invoke::<i32>(&[&10i32, &100i32, &1000i32]), 1110i32);
}

glib::wrapper! {
    struct SendObject(ObjectSubclass<send::SendObject>);
}

mod send {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::{ParamSpec, Value};

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::SendObject)]
    pub(super) struct SendObject {
        #[property(get, set)]
        value: std::sync::atomic::AtomicU64,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SendObject {
        const NAME: &'static str = "SendObject";
        type Type = super::SendObject;
    }

    impl ObjectImpl for SendObject {
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
}

#[test]
#[glib::clone_block]
fn async_closure() {
    use futures_util::StreamExt;

    let ctx = glib::MainContext::default();
    let (tx, mut rx) = futures_channel::mpsc::unbounded::<u64>();
    let obj = glib::Object::new::<SendObject>();
    let get_obj = || &obj;
    let closure = #[closure]
    move |s: &str,
                        #[strong] tx,
                        #[weak(obj or_panic)] _obj,
                        #[watch(*get_obj())] obj2| async move {
        glib::timeout_future_seconds(0).await;
        let v = s.parse().unwrap();
        tx.unbounded_send(v).unwrap();
        obj2.set_value(v);
    };

    tx.unbounded_send(60).unwrap();
    assert_eq!(obj.value(), 0);
    closure.invoke::<()>(&[&"70"]);
    assert_eq!(obj.value(), 0);
    assert_eq!(ctx.block_on(rx.next()), Some(60));
    assert_eq!(obj.value(), 0);
    assert_eq!(ctx.block_on(rx.next()), Some(70));
    assert_eq!(obj.value(), 70);
    assert!(rx.try_next().is_err());
}
