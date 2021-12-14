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
        pub item: RefCell<Box<dyn Any>>,
    }
    #[object_subclass]
    impl ObjectSubclass for BoxedAnyObject {
        const NAME: &'static str = "BoxedAnyObject";
        type Type = super::BoxedAnyObject;
    }
    impl Default for BoxedAnyObject {
        fn default() -> Self {
            Self {
                item: RefCell::new(Box::new(None::<usize>)),
            }
        }
    }
    impl ObjectImpl for BoxedAnyObject {}
}

wrapper! {
    /// # Examples
    /// ```rust
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
    /// // The boxed data can be stored as a GObject
    /// let list = gio::ListStore::new(BoxedAnyObject::static_type());
    /// list.append(boxed.clone() as glib::Object);
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
    pub fn new<T: 'static>(item: T) -> Self {
        let obj = Self::default();
        obj.replace(item);
        obj
    }

    /// Replaces the wrapped value with a new one, returning the old value, without deinitializing either one.
    pub fn replace<T: 'static>(&self, t: T) -> Box<dyn Any> {
        self.impl_().item.replace(Box::new(t) as Box<dyn Any>)
    }

    /// Immutably borrows the wrapped value, returning none if the value is currently mutably borrowed or
    /// the inner value has never been seet
    pub fn try_borrow<'a, T: 'static>(&'a self) -> Result<Ref<'a, T>, BorrowError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map.
        // As a workaround, I check if everything is safe, then I unwrap

        let borrowed = self.impl_().item.try_borrow()?;
        borrowed
            .as_ref()
            .downcast_ref::<T>()
            .ok_or(BorrowError::InvalidType)?;
        Ok(self.borrow()) // Now this won't panic
    }

    /// Mutably borrows the wrapped value, returning none if the value is currently immutably borrowed or
    /// the inner value has never been seet
    pub fn try_borrow_mut<'a, T: 'static>(&'a mut self) -> Result<RefMut<'a, T>, BorrowError> {
        // The required function is only available on nightly:
        // https://doc.rust-lang.org/std/cell/struct.Ref.html#method.filter_map
        // As a workaround, I check if everything is safe, then I unwrap

        let mut borrowed_mut = self.impl_().item.try_borrow_mut()?;
        borrowed_mut
            .as_mut()
            .downcast_mut::<T>()
            .ok_or(BorrowError::InvalidType)?;
        drop(borrowed_mut);
        Ok(self.borrow_mut()) // Now this won't panic
    }

    /// Immutably borrows the wrapped value. Multiple immutable borrows can be taken out at
    /// the same time.
    ///
    /// # Panics
    /// Panics if the value is currently mutably borrowed or the inner value has never been set
    /// or if the conversion to `T` fails
    pub fn borrow<'a, T: 'static>(&'a self) -> Ref<'a, T> {
        Ref::map(self.impl_().item.borrow(), |item| {
            item.as_ref().downcast_ref::<T>().unwrap()
        })
    }
    /// Mutably borrows the wrapped value. The value cannot be borrowed while this borrow is active.
    ///
    /// # Panics
    /// Panics if the value is currently borrowed or if the conversion to `T` fails
    pub fn borrow_mut<'a, T: 'static>(&'a mut self) -> RefMut<'a, T> {
        RefMut::map(self.impl_().item.borrow_mut(), |item| {
            item.as_mut().downcast_mut::<T>().unwrap()
        })
    }
}
