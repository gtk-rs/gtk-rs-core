// Take a look at the license at the top of the repository in the LICENSE file.

use crate as glib;
use crate::subclass::prelude::*;
use crate::{object_subclass, wrapper, Object};
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

#[derive(thiserror::Error, Debug)]
pub enum BorrowError {
    #[error("Can't convert item inside BoxedAnyObject to requested type")]
    InvalidType,
    #[error("Item already immutably borrowed")]
    AlreadyBorrowed(#[from] std::cell::BorrowError),
    #[error("Item already mutably borrowed")]
    AlreadyMutBorrowed(#[from] std::cell::BorrowMutError),
}

mod imp {
    use super::*;

    #[derive(Debug)]
    pub struct BoxedAnyObject {
        pub value: RefCell<Box<dyn Any>>,
    }
    #[object_subclass]
    impl ObjectSubclass for BoxedAnyObject {
        const NAME: &'static str = "BoxedAnyObject";
        type Type = super::BoxedAnyObject;
    }
    impl Default for BoxedAnyObject {
        fn default() -> Self {
            Self {
                value: RefCell::new(Box::new(None::<usize>)),
            }
        }
    }
    impl ObjectImpl for BoxedAnyObject {}
}

wrapper! {
    /// This is a type subclassing `glib::object::Object` capable of storing any Rust type.
    /// It let's you insert a Rust type anywhere a `glib::object::Object` is needed.
    /// The inserted value can then be borrowed as a Rust type, by using the various
    /// included methods.
    ///
    /// # Examples
    /// ```
    /// use glib::BoxedAnyObject;
    /// struct Author {
    ///     name: String,
    ///     subscribers: usize
    /// }
    /// // BoxedAnyObject can contain any custom type
    /// let boxed = BoxedAnyObject::new(Author {
    ///     name: String::from("GLibAuthor"),
    ///     subscribers: 1000
    /// });
    ///
    /// // The boxed data can be stored as a `glib::object::Object`
    /// let list = gio::ListStore::new(BoxedAnyObject::static_type());
    /// list.append(boxed.clone().upcast());
    ///
    /// // And can be retrieved with `borrow`
    /// let author: Author = boxed.borrow();
    /// ```
    pub struct BoxedAnyObject(ObjectSubclass<imp::BoxedAnyObject>);
}

impl Default for BoxedAnyObject {
    fn default() -> Self {
        Object::new(&[]).expect("Failed to create BoxedAnyObject")
    }
}
impl BoxedAnyObject {
    pub fn new<T: 'static>(value: T) -> Self {
        let obj = Self::default();
        obj.replace(value);
        obj
    }

    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    pub fn replace<T: 'static>(&self, t: T) -> Box<dyn Any> {
        self.impl_().value.replace(Box::new(t) as Box<dyn Any>)
    }

    /// Immutably borrows the wrapped value, returning an error if the value is currently mutably
    /// borrowed or if it's not of type T.
    ///
    /// The borrow lasts until the returned `Ref` exits scope. Multiple immutable borrows can be
    /// taken out at the same time.
    ///
    /// This is the non-panicking variant of [`borrow`](#method.borrow).
    pub fn try_borrow<T: 'static>(&self) -> Result<Ref<'_, T>, BorrowError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map.
        // As a workaround, I check if everything is safe, then I unwrap

        let borrowed = self.impl_().value.try_borrow()?;
        borrowed
            .as_ref()
            .downcast_ref::<T>()
            .ok_or(BorrowError::InvalidType)?;
        Ok(self.borrow()) // Now this won't panic
    }

    /// Mutably borrows the wrapped value, returning an error if the value is currently borrowed.
    /// or if it's not of type T.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
    /// from it exit scope. The value cannot be borrowed while this borrow is
    /// active.
    ///
    /// This is the non-panicking variant of [`borrow_mut`](#method.borrow_mut).
    pub fn try_borrow_mut<T: 'static>(&mut self) -> Result<RefMut<'_, T>, BorrowError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map
        // As a workaround, I check if everything is safe, then I unwrap.

        let mut borrowed_mut = self.impl_().value.try_borrow_mut()?;
        borrowed_mut
            .as_mut()
            .downcast_mut::<T>()
            .ok_or(BorrowError::InvalidType)?;
        drop(borrowed_mut);
        Ok(self.borrow_mut()) // Now this won't panic
    }

    /// Immutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `Ref` exits scope. Multiple
    /// immutable borrows can be taken out at the same time.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed or if it's not of type T.
    ///
    /// For a non-panicking variant, use
    /// [`try_borrow`](#method.try_borrow).
    pub fn borrow<T: 'static>(&self) -> Ref<'_, T> {
        Ref::map(self.impl_().value.borrow(), |value| {
            value.as_ref().downcast_ref::<T>().unwrap()
        })
    }

    /// Mutably borrows the wrapped value.
    ///
    /// The borrow lasts until the returned `RefMut` or all `RefMut`s derived
    /// from it exit scope. The value cannot be borrowed while this borrow is
    /// active.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently borrowed or if it's not of type T.
    ///
    /// For a non-panicking variant, use
    /// [`try_borrow_mut`](#method.try_borrow_mut).
    pub fn borrow_mut<T: 'static>(&self) -> RefMut<'_, T> {
        RefMut::map(self.impl_().value.borrow_mut(), |value| {
            value.as_mut().downcast_mut::<T>().unwrap()
        })
    }
}
