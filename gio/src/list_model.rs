// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::Cell;
use std::marker::PhantomData;
use std::rc::Rc;

use glib::{clone, IsA, Object};

use crate::prelude::*;
use crate::ListModel;

pub trait ListModelExtManual: ListModelExt {
    /// Returns an iterator over the ListModel elements.
    fn iter<T: IsA<Object>>(&self) -> ListModelIter<Self, T>
    where
        Self: Sized + Clone;
}

impl<O: IsA<ListModel>> ListModelExtManual for O {
    fn iter<T: IsA<Object>>(&self) -> ListModelIter<Self, T>
    where
        Self: Sized + Clone,
    {
        ListModelIter::new(self.clone())
    }
}

pub struct ListModelIter<M: ListModelExt, T: ObjectExt> {
    data_type: PhantomData<T>,
    model: M,
    i: Rc<Cell<u32>>,
}

impl<M: ListModelExt + Clone, T: IsA<Object>> ListModelIter<M, T> {
    /// Creates a new ListModel iterator.
    /// 
    /// ### Panics
    /// Panics if `T::static_type()` is not of the model’s item type.
    fn new(model: M) -> Self {
        if !T::static_type().is_a(model.item_type()) {
            panic!(
                "Item type {} is not a subtype of model type {}.",
                T::static_type(),
                model.item_type()
            );
        }

        let iter = ListModelIter::<M, T> {
            data_type: PhantomData,
            model: model.clone(),
            i: Rc::new(Cell::new(0)),
        };

        // Adjust index when the underlying model changes.
        model.connect_items_changed(
            clone!(@weak iter.i as i_rc => move |_model, pos, n_removed, n_added| {
                i_rc.set(adjust_index(i_rc.get(), pos, n_removed, n_added));
            }),
        );

        iter
    }
}

impl<M: ListModelExt, T: IsA<Object>> Iterator for ListModelIter<M, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.model.item(self.i.get());
        self.i.set(self.i.get() + 1);
        item.map(|x| x.downcast::<T>().unwrap())
    }
}

/// Calculates the new index from the given change.
fn adjust_index(index: u32, pos: u32, n_removed: u32, n_added: u32) -> u32 {
    if index <= pos {
        // Before the changed area; do nothing.
        index
    } else if index < pos + n_removed {
        // In the changed area; skip the new elements.
        pos + n_added
    } else {
        // index >= pos + n_removed
        // After the changed area; move by the difference.
        index + n_added - n_removed
    }
}
