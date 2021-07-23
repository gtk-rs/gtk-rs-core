// Take a look at the license at the top of the repository in the LICENSE file.

use std::cmp::Ordering;

use glib::{IsA, Object};

use crate::prelude::*;
use crate::ListModel;

pub trait ListModelExtManual {
    /// Searches the sorted list with `compare_func` in log time.
    ///
    /// On success returns the index of the found item.
    /// On failure returns the index where a matching element can be inserted
    /// to mainatain order.
    ///
    /// ### Panics
    /// Panics if `T::static_type()` is not of the model’s item type.
    fn find_sorted<T, F>(&self, compare_func: F) -> Result<usize, usize>
    where
        T: IsA<Object>,
        F: FnMut(&T) -> Ordering;
}

impl<O: IsA<ListModel>> ListModelExtManual for O {
    fn find_sorted<T, F>(&self, mut compare_func: F) -> Result<usize, usize>
    where
        T: IsA<Object>,
        F: FnMut(&T) -> Ordering,
    {
        if !T::static_type().is_a(self.item_type()) {
            panic!(
                "Item type {} is not a subtype of model type {}.",
                T::static_type(),
                self.item_type()
            );
        }

        // Perform a binary search.
        let mut size = self.n_items() as usize;
        let mut left = 0;
        let mut right = size;
        let a = |i| self.item(i as u32).unwrap().downcast().unwrap();
        while left < right {
            let mid = left + size / 2;
            match compare_func(&a(mid)) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
                Ordering::Equal => return Ok(mid),
            }
            size = right - left;
        }
        Err(left)
    }
}
