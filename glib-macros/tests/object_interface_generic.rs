// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

pub mod imp {
    use std::marker::PhantomData;

    use super::*;

    #[repr(C)]
    pub struct MyGenericInterface<T> {
        parent: glib::gobject_ffi::GTypeInterface,
        marker: PhantomData<T>,
    }

    impl<T> Clone for MyGenericInterface<T> {
        fn clone(&self) -> Self {
            Self {
                parent: self.parent,
                marker: self.marker,
            }
        }
    }

    impl<T> Copy for MyGenericInterface<T> {}

    #[glib::object_interface]
    unsafe impl<T: 'static> ObjectInterface for MyGenericInterface<T> {
        const NAME: &'static str = "MyGenericInterface";
    }

    pub trait MyGenericInterfaceImpl<T>: ObjectImpl + ObjectSubclass {}

    pub struct MyGenericType<T> {
        marker: PhantomData<T>,
    }

    #[glib::object_subclass]
    impl<T: 'static> ObjectSubclass for MyGenericType<T> {
        const NAME: &'static str = "MyGenericType";
        type Type = super::MyGenericType<T>;
        type Interfaces = (super::MyGenericInterface<T>,);

        fn new() -> Self {
            Self {
                marker: PhantomData,
            }
        }
    }

    impl<T: 'static> ObjectImpl for MyGenericType<T> {}

    impl<T: 'static> MyGenericInterfaceImpl<T> for MyGenericType<T> {}

    pub trait MyGenericTypeImpl<T>: ObjectImpl + ObjectSubclass {}
}

glib::wrapper! {
    pub struct MyGenericInterface<T: 'static>(ObjectInterface<imp::MyGenericInterface<T>>);
}

unsafe impl<I: imp::MyGenericInterfaceImpl<T>, T> IsImplementable<I> for MyGenericInterface<T> {}

glib::wrapper! {
    pub struct MyGenericType<T: 'static>(ObjectSubclass<imp::MyGenericType<T>>) @implements MyGenericInterface<T>;
}

unsafe impl<I: imp::MyGenericTypeImpl<T>, T> IsSubclassable<I> for MyGenericType<T> {}
