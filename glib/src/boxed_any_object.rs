use crate as glib;
use crate::subclass::prelude::*;
use crate::{object_subclass, wrapper, Object};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

#[derive(thiserror::Error, Debug)]
pub enum BorrowError {
    #[error("Can't convert item inside BoxedAnyObject to requested type")]
    InvalidType,
    #[error("BoxedAnyObject item is None")]
    IsNone,
    #[error("BoxedAnyObject item already immutably borrowed")]
    AlreadyBorrowed(#[from] std::cell::BorrowError),
    #[error("BoxedAnyObject item already mutably borrowed")]
    AlreadyMutBorrowed(#[from] std::cell::BorrowMutError),
}

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct BoxedAnyObject {
        pub item: RefCell<Option<Box<dyn Any>>>,
    }
    #[object_subclass]
    impl ObjectSubclass for BoxedAnyObject {
        const NAME: &'static str = "BoxedAnyObject";
        type Type = super::BoxedAnyObject;
        type ParentType = Object;
    }
    impl ObjectImpl for BoxedAnyObject {}
}

wrapper! {
    pub struct BoxedAnyObject(ObjectSubclass<imp::BoxedAnyObject>);
}

impl Default for BoxedAnyObject {
    fn default() -> Self {
        BoxedAnyObject::new_empty()
    }
}
impl BoxedAnyObject {
    pub fn new<T: 'static>(item: T) -> Self {
        let obj = Self::new_empty();
        obj.replace::<T, Option<Box<dyn Any>>>(item);
        obj
    }
    pub fn new_empty() -> Self {
        Object::new(&[]).expect("Failed to create BoxedAnyObject")
    }
    // Unset the internal item. Returns true if the internal item was set
    pub fn clear(&self) -> bool {
        if self.impl_().item.borrow_mut().is_some() {
            true
        } else {
            false
        }
    }
    // Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    pub fn replace<T: 'static, R: 'static>(&self, t: T) -> Option<R> {
        self.impl_()
            .item
            .replace(Some(Box::new(t) as Box<dyn Any>))
            .and_then(|res| res.downcast::<R>().ok())
            .map(|res| *res)
    }

    // Immutably borrows the wrapped value, returning none if the value is currently mutably borrowed or
    // the inner value has never been seet
    pub fn try_borrow<'a, T: 'static>(&'a self) -> Result<Ref<'a, T>, BorrowError> {
        // Unfortunately, there isn't a nice way to do this.
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map
        //
        // Manually check if every step is safe.
        let borrowed = self.impl_().item.try_borrow()?;
        borrowed
            .as_ref()
            .ok_or(BorrowError::IsNone)?
            .downcast_ref::<T>()
            .ok_or(BorrowError::InvalidType)?;
        Ok(self.borrow()) // Now this won't panic
    }

    // Mutably borrows the wrapped value, returning none if the value is currently immutably borrowed or
    // the inner value has never been seet
    pub fn try_borrow_mut<'a, T: 'static>(&'a mut self) -> Result<RefMut<'a, T>, BorrowError> {
        // Unfortunately, there isn't a nice way to do this.
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map
        //
        // Manually check if every step is safe.
        let mut borrowed_mut = self.impl_().item.try_borrow_mut()?;
        borrowed_mut
            .as_mut()
            .ok_or(BorrowError::IsNone)?
            .downcast_mut::<T>()
            .ok_or(BorrowError::InvalidType)?;
        drop(borrowed_mut);
        Ok(self.borrow_mut()) // Now this won't panic
    }

    // Immutably borrows the wrapped value. Multiple immutable borrows can be taken out at
    // the same time.
    //
    // # Panics
    // Panics if the value is currently mutably borrowed or the inner value has never been set
    // or if the conversion to `T` fails
    pub fn borrow<'a, T: 'static>(&'a self) -> Ref<'a, T> {
        Ref::map(self.impl_().item.borrow(), |item| {
            item.as_ref().unwrap().downcast_ref::<T>().unwrap()
        })
    }
    // Mutably borrows the wrapped value. The value cannot be borrowed while this borrow is active.
    //
    // # Panics
    // Panics if the value is currently borrowed or the inner value has never been set
    // or if the conversion to `T` fails
    pub fn borrow_mut<'a, T: 'static>(&'a mut self) -> RefMut<'a, T> {
        RefMut::map(self.impl_().item.borrow_mut(), |item| {
            item.as_mut().unwrap().downcast_mut::<T>().unwrap()
        })
    }
}
