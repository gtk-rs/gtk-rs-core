// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::Cell;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::rc::Rc;

use glib::SignalHandlerId;

use crate::prelude::*;
use crate::ListModel;

pub trait ListModelExtManual: Sized {
    // rustdoc-stripper-ignore-next
    /// Get an immutable snapshot of the container inside the `ListModel`.
    /// Any modification done to the returned container `Vec` will not be
    /// reflected on the `ListModel`.
    fn snapshot(&self) -> Vec<glib::Object>;

    // rustdoc-stripper-ignore-next
    /// If `T::static_type().is_a(self.item_type())` then it returns an iterator over the `ListModel` elements,
    /// else the types are not compatible and returns `Err(&Self)`.
    fn iter<T: IsA<glib::Object>>(&self) -> Result<ListModelIter<T>, &Self>;
}

impl<T: IsA<ListModel>> ListModelExtManual for T {
    fn snapshot(&self) -> Vec<glib::Object> {
        let mut res = Vec::with_capacity(self.n_items() as usize);
        for i in 0..self.n_items() {
            res.push(self.item(i).unwrap())
        }
        res
    }
    fn iter<LT: IsA<glib::Object>>(&self) -> Result<ListModelIter<LT>, &Self> {
        if !self.item_type().is_a(LT::static_type()) {
            return Err(self);
        }

        let len = self.n_items();
        let changed = Rc::new(Cell::new(false));

        let changed_clone = changed.clone();
        let signal_id = Cell::new(Some(self.connect_items_changed(move |_, pos, _, _| {
            let old = changed_clone.get();
            changed_clone.replace(old || pos < len);
        })));

        Ok(ListModelIter {
            ty: Default::default(),
            i: 0,
            reverse_pos: len,
            model: self.upcast_ref(),
            changed,
            signal_id,
        })
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("the list model was mutated during iteration")]
pub struct ListModelMutatedDuringIter;

// rustdoc-stripper-ignore-next
/// Iterator of `ListModel`'s items.
/// This iterator will always give `n = initial_model.n_items()` items, even if the `ListModel`
/// is mutated during iteration.
/// If the internal `ListModel` gets mutated, the iterator
/// will return `Some(Err(...))` for the remaining items.
/// Mutations to the `ListModel` in position >= `initial_model.n_items()` are allowed.
pub struct ListModelIter<'a, T: IsA<glib::Object>> {
    ty: PhantomData<T>,
    i: u32,
    // it's > i when valid
    reverse_pos: u32,
    model: &'a ListModel,
    changed: Rc<Cell<bool>>,
    signal_id: Cell<Option<SignalHandlerId>>,
}
impl<'a, T: IsA<glib::Object>> Iterator for ListModelIter<'a, T> {
    type Item = Result<T, ListModelMutatedDuringIter>;
    fn size_hint(&self) -> (usize, Option<usize>) {
        let n = (self.reverse_pos - self.i) as usize;
        (n as usize, Some(n))
    }

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.reverse_pos {
            return None;
        }
        let res = match self.changed.get() {
            true => Err(ListModelMutatedDuringIter),
            false => Ok(self.model.item(self.i).unwrap().downcast::<T>().unwrap()),
        };
        self.i += 1;
        Some(res)
    }
}
impl<'a, T: IsA<glib::Object>> FusedIterator for ListModelIter<'a, T> {}
impl<'a, T: IsA<glib::Object>> ExactSizeIterator for ListModelIter<'a, T> {}
impl<'a, T: IsA<glib::Object>> DoubleEndedIterator for ListModelIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.reverse_pos == self.i {
            return None;
        }
        self.reverse_pos -= 1;
        let res = match self.changed.get() {
            true => Err(ListModelMutatedDuringIter),
            false => Ok(self
                .model
                .item(self.reverse_pos)
                .unwrap()
                .downcast::<T>()
                .unwrap()),
        };
        Some(res)
    }
}
impl<'a, T: IsA<glib::Object>> Drop for ListModelIter<'a, T> {
    fn drop(&mut self) {
        self.model.disconnect(self.signal_id.take().unwrap());
    }
}

impl<'a> std::iter::IntoIterator for &'a ListModel {
    type Item = Result<glib::Object, ListModelMutatedDuringIter>;
    type IntoIter = ListModelIter<'a, glib::Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .expect("can't create a ListModelIter with the requested item type")
    }
}

#[test]
fn list_model_iter_ok() {
    let list = crate::ListStore::new(crate::Menu::static_type());
    let m1 = crate::Menu::new();
    let m2 = crate::Menu::new();
    let m3 = crate::Menu::new();
    let m4 = crate::Menu::new();

    list.append(&m1);
    list.append(&m2);
    list.append(&m3);

    let mut iter = list.iter::<crate::Menu>().unwrap();

    assert_eq!(iter.len(), 3);
    assert_eq!(iter.next(), Some(Ok(m1)));
    // Appending items at the end of the `ListModel` can't affect the items
    // we are iterating over.
    list.append(&m4);
    assert_eq!(iter.next_back(), Some(Ok(m3)));
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.next_back(), Some(Ok(m2)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}
#[test]
fn list_model_iter_err() {
    let list = crate::ListStore::new(crate::Menu::static_type());
    let m1 = crate::Menu::new();
    let m2 = crate::Menu::new();
    let m3 = crate::Menu::new();
    let m4 = crate::Menu::new();

    list.append(&m1);
    list.append(&m2);
    list.append(&m3);
    list.append(&m4);

    let mut iter = list.iter::<crate::Menu>().unwrap();
    assert_eq!(iter.next_back(), Some(Ok(m4)));

    // These two don't affect the iter
    list.append(&m2);
    list.append(&m2);

    assert_eq!(iter.next(), Some(Ok(m1)));

    // Does affect the iter
    list.remove(2);
    // Doesn't affect the iter, but the iter should stay tainted.
    list.remove(4);
    assert_eq!(iter.next(), Some(Err(ListModelMutatedDuringIter)));
    assert_eq!(iter.next(), Some(Err(ListModelMutatedDuringIter)));
    // Returned n items
    assert_eq!(iter.next(), None);
}
