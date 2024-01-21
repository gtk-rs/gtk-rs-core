// Take a look at the license at the top of the repository in the LICENSE file.

use glib::subclass::prelude::*;

pub mod imp {
    use super::*;
    use std::marker::PhantomData;

    pub struct MyGenericType<T>(PhantomData<T>);

    #[glib::object_subclass]
    impl<T: 'static> ObjectSubclass for MyGenericType<T> {
        const NAME: &'static str = "MyGenericType";
        type Type = super::MyGenericType<T>;

        fn new() -> Self {
            MyGenericType(PhantomData::<T>)
        }
    }

    impl<T: 'static> ObjectImpl for MyGenericType<T> {}
}

glib::wrapper! {
    pub struct MyGenericType<T: 'static>(ObjectSubclass<imp::MyGenericType::<T>>);
}
