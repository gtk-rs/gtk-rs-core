// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Module that contains all types needed for creating a direct subclass of `GObject`
//! or implementing virtual methods of it.

use super::prelude::*;
use super::Signal;
use crate::translate::*;
use crate::{Cast, Object, ObjectType, ParamSpec, Value};
use std::mem;
use std::ptr;

// rustdoc-stripper-ignore-next
/// Trait for implementors of `glib::Object` subclasses.
///
/// This allows overriding the virtual methods of `glib::Object`.
pub trait ObjectImpl: ObjectSubclass + ObjectImplExt {
    // rustdoc-stripper-ignore-next
    /// Properties installed for this type.
    fn properties() -> &'static [ParamSpec] {
        &[]
    }

    // rustdoc-stripper-ignore-next
    /// Signals installed for this type.
    fn signals() -> &'static [Signal] {
        &[]
    }

    // rustdoc-stripper-ignore-next
    /// Property setter.
    ///
    /// This is called whenever the property of this specific subclass with the
    /// given index is set. The new value is passed as `glib::Value`.
    fn set_property(&self, _obj: &Self::Type, _id: usize, _value: &Value, _pspec: &ParamSpec) {
        unimplemented!()
    }

    // rustdoc-stripper-ignore-next
    /// Property getter.
    ///
    /// This is called whenever the property value of the specific subclass with the
    /// given index should be returned.
    #[doc(alias = "get_property")]
    fn property(&self, _obj: &Self::Type, _id: usize, _pspec: &ParamSpec) -> Value {
        unimplemented!()
    }

    // rustdoc-stripper-ignore-next
    /// Constructed.
    ///
    /// This is called once construction of the instance is finished.
    ///
    /// Should chain up to the parent class' implementation.
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }

    // rustdoc-stripper-ignore-next
    /// Disposes of the object.
    ///
    /// When `dispose()` ends, the object should not hold any reference to any other member object.
    /// The object is also expected to be able to answer client method invocations (with possibly an
    /// error code but no memory violation) until it is dropped. `dispose()` can be executed more
    /// than once.
    fn dispose(&self, _obj: &Self::Type) {}
}

#[doc(alias = "get_property")]
unsafe extern "C" fn property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    let v = imp.property(
        from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        id as usize,
        &from_glib_borrow(pspec),
    );

    // We first unset the value we get passed in, in case it contained
    // any previous data. Then we directly overwrite it with our new
    // value, and pass ownership of the contained data to the C GValue
    // by forgetting it on the Rust side.
    //
    // Without this, by using the GValue API, we would have to create
    // a copy of the value when setting it on the destination just to
    // immediately free the original value afterwards.
    gobject_ffi::g_value_unset(value);
    let v = mem::ManuallyDrop::new(v);
    ptr::write(value, ptr::read(v.to_glib_none().0));
}

unsafe extern "C" fn set_property<T: ObjectImpl>(
    obj: *mut gobject_ffi::GObject,
    id: u32,
    value: *mut gobject_ffi::GValue,
    pspec: *mut gobject_ffi::GParamSpec,
) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();
    imp.set_property(
        from_glib_borrow::<_, Object>(obj).unsafe_cast_ref(),
        id as usize,
        &*(value as *mut Value),
        &from_glib_borrow(pspec),
    );
}

unsafe extern "C" fn constructed<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    imp.constructed(from_glib_borrow::<_, Object>(obj).unsafe_cast_ref());
}

unsafe extern "C" fn dispose<T: ObjectImpl>(obj: *mut gobject_ffi::GObject) {
    let instance = &*(obj as *mut T::Instance);
    let imp = instance.imp();

    imp.dispose(from_glib_borrow::<_, Object>(obj).unsafe_cast_ref());

    // Chain up to the parent's dispose.
    let data = T::type_data();
    let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;
    if let Some(ref func) = (*parent_class).dispose {
        func(obj);
    }
}

// rustdoc-stripper-ignore-next
/// Extension trait for `glib::Object`'s class struct.
///
/// This contains various class methods and allows subclasses to override signal class handlers.
pub unsafe trait ObjectClassSubclassExt: Sized + 'static {
    fn override_signal_class_handler<F>(&mut self, name: &str, class_handler: F)
    where
        F: Fn(&super::SignalClassHandlerToken, &[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe {
            super::types::signal_override_class_handler(
                name,
                *(self as *mut _ as *mut ffi::GType),
                class_handler,
            );
        }
    }
}

unsafe impl ObjectClassSubclassExt for crate::Class<Object> {}

unsafe impl<T: ObjectImpl> IsSubclassable<T> for Object {
    fn class_init(class: &mut crate::Class<Self>) {
        let klass = class.as_mut();
        klass.set_property = Some(set_property::<T>);
        klass.get_property = Some(property::<T>);
        klass.constructed = Some(constructed::<T>);
        klass.dispose = Some(dispose::<T>);

        let pspecs = <T as ObjectImpl>::properties();
        if !pspecs.is_empty() {
            unsafe {
                let mut pspecs_ptrs = Vec::with_capacity(pspecs.len() + 1);

                pspecs_ptrs.push(ptr::null_mut());

                for pspec in pspecs {
                    pspecs_ptrs.push(pspec.to_glib_none().0);
                }

                gobject_ffi::g_object_class_install_properties(
                    klass,
                    pspecs_ptrs.len() as u32,
                    pspecs_ptrs.as_mut_ptr(),
                );
            }
        }

        let type_ = T::type_();
        let signals = <T as ObjectImpl>::signals();
        for signal in signals {
            signal.register(type_);
        }
    }

    fn instance_init(_instance: &mut super::InitializingObject<T>) {}
}

pub trait ObjectImplExt: ObjectSubclass {
    // rustdoc-stripper-ignore-next
    /// Chain up to the parent class' implementation of `glib::Object::constructed()`.
    fn parent_constructed(&self, obj: &Self::Type);

    // rustdoc-stripper-ignore-next
    /// Chain up to parent class signal handler.
    fn signal_chain_from_overridden(
        &self,
        token: &super::SignalClassHandlerToken,
        values: &[Value],
    ) -> Option<Value>;
}

impl<T: ObjectImpl> ObjectImplExt for T {
    fn parent_constructed(&self, obj: &Self::Type) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut gobject_ffi::GObjectClass;

            if let Some(ref func) = (*parent_class).constructed {
                func(obj.unsafe_cast_ref::<Object>().to_glib_none().0);
            }
        }
    }

    fn signal_chain_from_overridden(
        &self,
        token: &super::SignalClassHandlerToken,
        values: &[Value],
    ) -> Option<Value> {
        unsafe {
            super::types::signal_chain_from_overridden(
                self.instance().as_ptr() as *mut _,
                token,
                values,
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::super::object::ObjectExt;
    use super::super::super::value::{ToValue, Value};
    use super::*;
    // We rename the current crate as glib, since the macros in glib-macros
    // generate the glib namespace through the crate_ident_new utility,
    // and that returns `glib` (and not `crate`) when called inside the glib crate
    use crate as glib;
    use crate::StaticType;

    use std::cell::RefCell;

    mod imp {
        use super::*;

        // A dummy `Object` to test setting an `Object` property and returning an `Object` in signals
        #[derive(Default)]
        pub struct ChildObject;

        #[glib::object_subclass]
        impl ObjectSubclass for ChildObject {
            const NAME: &'static str = "ChildObject";
            type Type = super::ChildObject;
        }

        impl ObjectImpl for ChildObject {}

        #[derive(Default)]
        pub struct SimpleObject {
            name: RefCell<Option<String>>,
            construct_name: RefCell<Option<String>>,
            constructed: RefCell<bool>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for SimpleObject {
            const NAME: &'static str = "SimpleObject";
            type Type = super::SimpleObject;
            type Interfaces = (super::Dummy,);
        }

        impl ObjectImpl for SimpleObject {
            fn properties() -> &'static [ParamSpec] {
                use once_cell::sync::Lazy;
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        crate::ParamSpecString::new(
                            "name",
                            "Name",
                            "Name of this object",
                            None,
                            crate::ParamFlags::READWRITE,
                        ),
                        crate::ParamSpecString::new(
                            "construct-name",
                            "Construct Name",
                            "Construct Name of this object",
                            None,
                            crate::ParamFlags::READWRITE | crate::ParamFlags::CONSTRUCT_ONLY,
                        ),
                        crate::ParamSpecBoolean::new(
                            "constructed",
                            "Constructed",
                            "True if the constructed() virtual method was called",
                            false,
                            crate::ParamFlags::READABLE,
                        ),
                        crate::ParamSpecObject::new(
                            "child",
                            "Child",
                            "Child object",
                            super::ChildObject::static_type(),
                            crate::ParamFlags::READWRITE,
                        ),
                    ]
                });

                PROPERTIES.as_ref()
            }

            fn signals() -> &'static [super::Signal] {
                use once_cell::sync::Lazy;
                static SIGNALS: Lazy<Vec<super::Signal>> = Lazy::new(|| {
                    vec![
                        super::Signal::builder(
                            "name-changed",
                            &[String::static_type().into()],
                            crate::Type::UNIT.into(),
                        )
                        .build(),
                        super::Signal::builder(
                            "change-name",
                            &[String::static_type().into()],
                            String::static_type().into(),
                        )
                        .action()
                        .class_handler(|_, args| {
                            let obj = args[0]
                                .get::<super::SimpleObject>()
                                .expect("Failed to get Object from args[0]");
                            let new_name = args[1]
                                .get::<String>()
                                .expect("Failed to get Object from args[1]");
                            let imp = obj.imp();

                            let old_name = imp.name.replace(Some(new_name));

                            obj.emit_by_name::<()>("name-changed", &[&*imp.name.borrow()]);

                            Some(old_name.to_value())
                        })
                        .build(),
                        super::Signal::builder("create-string", &[], String::static_type().into())
                            .build(),
                        super::Signal::builder(
                            "create-child-object",
                            &[],
                            ChildObject::type_().into(),
                        )
                        .build(),
                    ]
                });

                SIGNALS.as_ref()
            }

            fn set_property(
                &self,
                obj: &Self::Type,
                _id: usize,
                value: &Value,
                pspec: &crate::ParamSpec,
            ) {
                match pspec.name() {
                    "name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.name.replace(name);
                        obj.emit_by_name::<()>("name-changed", &[&*self.name.borrow()]);
                    }
                    "construct-name" => {
                        let name = value
                            .get()
                            .expect("type conformity checked by 'Object::set_property'");
                        self.construct_name.replace(name);
                    }
                    "child" => {
                        // not stored, only used to test `set_property` with `Objects`
                    }
                    _ => unimplemented!(),
                }
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &crate::ParamSpec) -> Value {
                match pspec.name() {
                    "name" => self.name.borrow().to_value(),
                    "construct-name" => self.construct_name.borrow().to_value(),
                    "constructed" => self.constructed.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }

            fn constructed(&self, obj: &Self::Type) {
                self.parent_constructed(obj);

                assert_eq!(obj, &self.instance());
                assert_eq!(self as *const _, obj.imp() as *const _);

                *self.constructed.borrow_mut() = true;
            }
        }

        #[derive(Clone, Copy)]
        #[repr(C)]
        pub struct DummyInterface {
            parent: gobject_ffi::GTypeInterface,
        }

        #[glib::object_interface]
        unsafe impl ObjectInterface for DummyInterface {
            const NAME: &'static str = "Dummy";
        }
    }

    wrapper! {
        pub struct ChildObject(ObjectSubclass<imp::ChildObject>);
    }

    wrapper! {
        pub struct SimpleObject(ObjectSubclass<imp::SimpleObject>);
    }

    wrapper! {
        pub struct Dummy(ObjectInterface<imp::DummyInterface>);
    }

    unsafe impl<T: ObjectSubclass> IsImplementable<T> for Dummy {}

    #[test]
    fn test_create() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[]).expect("Object::new failed");

        assert!(obj.type_().is_a(Dummy::static_type()));

        // Assert that the object representation is equivalent to the underlying C GObject pointer
        assert_eq!(
            mem::size_of::<SimpleObject>(),
            mem::size_of::<ffi::gpointer>()
        );
        assert_eq!(obj.as_ptr() as ffi::gpointer, unsafe {
            *(&obj as *const _ as *const ffi::gpointer)
        });

        assert!(obj.property::<bool>("constructed"));

        let weak = obj.downgrade();
        drop(obj);
        assert!(weak.upgrade().is_none());
    }

    #[test]
    fn test_properties() {
        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[]).expect("Object::new failed");

        assert!(obj.type_().is_a(Dummy::static_type()));

        let properties = obj.list_properties();
        assert_eq!(properties.len(), 4);
        assert_eq!(properties[0].name(), "name");
        assert_eq!(properties[1].name(), "construct-name");
        assert_eq!(properties[2].name(), "constructed");
        assert_eq!(properties[3].name(), "child");
    }

    #[test]
    fn test_create_child_object() {
        let obj: ChildObject = Object::new(&[]).expect("Object::new failed");

        assert_eq!(obj, obj.imp().instance());
    }

    #[test]
    fn test_builder() {
        let obj = Object::builder::<SimpleObject>()
            .property("construct-name", "meh")
            .property("name", "initial")
            .build()
            .expect("Object::new failed");

        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );

        assert_eq!(obj.property::<String>("name"), String::from("initial"));
    }

    #[test]
    fn test_set_properties() {
        let obj = Object::with_type(
            SimpleObject::static_type(),
            &[("construct-name", &"meh"), ("name", &"initial")],
        )
        .expect("Object::new failed");

        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );
        assert_eq!(
            obj.try_set_property("construct-name", &"test")
                .expect_err("Failed to set 'construct-name' property")
                .to_string(),
            "property 'construct-name' of type 'SimpleObject' is not writable",
        );
        assert_eq!(
            obj.property::<String>("construct-name"),
            String::from("meh")
        );
        assert_eq!(obj.property::<String>("name"), String::from("initial"));
        assert!(obj.try_set_property("name", &"test").is_ok());
        assert_eq!(obj.property::<String>("name"), String::from("test"));

        assert_eq!(
            obj.try_set_property("test", &true)
                .expect_err("set_property failed")
                .to_string(),
            "property 'test' of type 'SimpleObject' not found",
        );

        assert_eq!(
            obj.try_set_property("constructed", &false)
                .expect_err("Failed to set 'constructed' property")
                .to_string(),
            "property 'constructed' of type 'SimpleObject' is not writable",
        );

        assert_eq!(
            obj.try_set_property("name", &false)
                .expect_err("Failed to set 'name' property")
                .to_string(),
            "property 'name' of type 'SimpleObject' can't be set from the given type (expected: 'gchararray', got: 'gboolean')",
        );

        let other_obj =
            Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");
        assert_eq!(
            obj.try_set_property("child", &other_obj)
                .expect_err("Failed to set 'child' property")
                .to_string(),
            "property 'child' of type 'SimpleObject' can't be set from the given object type (expected: 'ChildObject', got: 'SimpleObject')",
        );

        let child = Object::with_type(ChildObject::static_type(), &[]).expect("Object::new failed");
        assert!(obj.try_set_property("child", &child).is_ok());
    }

    #[test]
    fn test_signals() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[("name", &"old-name")]).expect("Object::new failed");

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();
        obj.connect("name-changed", false, move |args| {
            let _obj = args[0].get::<Object>().expect("Failed to get args[0]");
            let name = args[1].get::<&str>().expect("Failed to get args[1]");

            assert_eq!(name, "new-name");
            name_changed_clone.store(true, Ordering::Relaxed);

            None
        });

        assert_eq!(obj.property::<String>("name"), String::from("old-name"));
        assert!(!name_changed_triggered.load(Ordering::Relaxed));

        assert_eq!(
            obj.emit_by_name::<String>("change-name", &[&"new-name"]),
            "old-name"
        );
        assert!(name_changed_triggered.load(Ordering::Relaxed));
    }

    #[test]
    fn test_signal_return_expected_type() {
        let obj = Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");

        obj.connect("create-string", false, move |_args| {
            Some("return value".to_value())
        });

        let signal_id = imp::SimpleObject::signals()[2].signal_id();

        let value = obj.emit::<String>(signal_id, &[]);
        assert_eq!(value, "return value");
    }

    #[test]
    fn test_callback_validity() {
        use std::sync::atomic::{AtomicBool, Ordering};
        use std::sync::Arc;

        let type_ = SimpleObject::static_type();
        let obj = Object::with_type(type_, &[("name", &"old-name")]).expect("Object::new failed");

        let name_changed_triggered = Arc::new(AtomicBool::new(false));
        let name_changed_clone = name_changed_triggered.clone();

        obj.connect_notify(Some("name"), move |_, _| {
            name_changed_clone.store(true, Ordering::Relaxed);
        });
        obj.notify("name");
        assert!(name_changed_triggered.load(Ordering::Relaxed));
    }

    // Note: can't test type mismatch in signals since panics accross FFI boundaries
    // are UB. See https://github.com/gtk-rs/glib/issues/518

    #[test]
    fn test_signal_return_expected_object_type() {
        let obj = Object::with_type(SimpleObject::static_type(), &[]).expect("Object::new failed");

        obj.connect("create-child-object", false, move |_args| {
            Some(
                Object::with_type(ChildObject::static_type(), &[])
                    .expect("Object::new failed")
                    .to_value(),
            )
        });
        let value: glib::Object = obj.emit_by_name("create-child-object", &[]);
        assert!(value.type_().is_a(ChildObject::static_type()));
    }
}
