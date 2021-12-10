use crate as glib;
use crate::subclass::prelude::*;
use crate::{object_subclass, wrapper, Object};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct AnyGObject {
        pub item: RefCell<Option<Box<dyn Any>>>,
    }
    impl Default for AnyGObject {
        fn default() -> Self {
            Self {
                item: RefCell::new(None),
            }
        }
    }
    #[object_subclass]
    impl ObjectSubclass for AnyGObject {
        const NAME: &'static str = "AnyGObject";
        type Type = super::AnyGObject;
        type ParentType = Object;
    }
    impl ObjectImpl for AnyGObject {}
}

wrapper! {
    pub struct AnyGObject(ObjectSubclass<imp::AnyGObject>);
}

impl AnyGObject {
    pub fn new(item: Box<dyn Any>) -> Self {
        let obj: AnyGObject = Object::new(&[]).expect("Failed to create AnyGObject");

        obj.replace(item);
        obj
    }
    // Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    pub fn replace(&self, t: Box<dyn Any>) -> Option<Box<dyn Any>> {
        self.impl_().item.replace(Some(t))
    }

    // Immutably borrows the wrapped value, returning none if the value is currently mutably borrowed or
    // the inner value has never been seet
    pub fn try_borrow<'a, T: 'static>(&'a self) -> Option<Ref<'a, T>> {
        // Unfortunately, there isn't a nice way to do this.
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map

        // Check things inside before borrowing
        match self.impl_().item.try_borrow().ok().map_or(false, |boxed| {
            boxed
                .as_ref()
                .and_then(|boxed_ref| boxed_ref.downcast_ref::<T>())
                .is_some()
        }) {
            true => Some(self.borrow()), // Now this won't panic
            false => None,
        }
    }

    // Mutably borrows the wrapped value, returning none if the value is currently immutably borrowed or
    // the inner value has never been seet
    pub fn try_borrow_mut<'a, T: 'static>(&'a mut self) -> Option<RefMut<'a, T>> {
        // Unfortunately, there isn't a nice way to do this.
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.RefMut.html#method.filter_map

        // Check things inside before borrowing
        match self
            .impl_()
            .item
            .try_borrow_mut()
            .ok()
            .map_or(false, |mut boxed| {
                boxed
                    .as_mut()
                    .and_then(|boxed_ref| boxed_ref.downcast_mut::<T>())
                    .is_some()
            }) {
            true => Some(self.borrow_mut()), // Now this won't panic
            false => None,
        }
    }

    // Immutably borrows the wrapped value. Multiple immutable borrows can be taken out at
    // the same time.
    //
    // # Panics
    // Panics if the value is currently mutably borrowed or the inner value has never been set
    pub fn borrow<'a, T: 'static>(&'a self) -> Ref<'a, T> {
        Ref::map(self.impl_().item.borrow(), |item| {
            item.as_ref().unwrap().downcast_ref::<T>().unwrap()
        })
    }
    // Mutably borrows the wrapped value. The value cannot be borrowed while this borrow is active.
    //
    // # Panics
    // Panics if the value is currently borrowed or the inner value has never been set
    pub fn borrow_mut<'a, T: 'static>(&'a mut self) -> RefMut<'a, T> {
        RefMut::map(self.impl_().item.borrow_mut(), |item| {
            item.as_mut().unwrap().downcast_mut::<T>().unwrap()
        })
    }
}
