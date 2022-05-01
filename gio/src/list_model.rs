// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::Cell;
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
    /// Returns an iterator over the ListModel elements.
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
    fn iter<LT: IsA<glib::Object>>(&self) -> Result<ListModelIter<LT>, &Self>
    where
        Self: Sized + Clone,
    {
        if self.item_type() != LT::static_type() {
            return Err(self);
        }
        let rc = Rc::new(Cell::new(false));

        let rcc = rc.clone();
        let signal_id = Cell::new(Some(self.connect_items_changed(move |_, _, _, _| {
            rcc.replace(true);
        })));

        Ok(ListModelIter {
            ty: Default::default(),
            i: 0,
            model: self.clone().upcast(),
            changed: rc,
            signal_id,
        })
    }
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("the list model was mutated during iteration")]
pub struct ListModelMutatedDuringIter;

pub struct ListModelIter<T: IsA<glib::Object>> {
    ty: PhantomData<T>,
    i: u32,
    model: ListModel,
    changed: Rc<Cell<bool>>,
    signal_id: Cell<Option<SignalHandlerId>>,
}
impl<T: IsA<glib::Object>> Iterator for ListModelIter<T> {
    type Item = Result<T, ListModelMutatedDuringIter>;

    fn next(&mut self) -> Option<Self::Item> {
        let res = match self.changed.get() {
            true => Some(Err(ListModelMutatedDuringIter)),
            false => self
                .model
                .item(self.i)
                .map(|x| Ok(x.downcast::<T>().unwrap())),
        };
        self.i += 1;
        res
    }
}
impl<T: IsA<glib::Object>> Drop for ListModelIter<T> {
    fn drop(&mut self) {
        self.model.disconnect(self.signal_id.take().unwrap());
    }
}

impl std::iter::IntoIterator for ListModel {
    type Item = Result<glib::Object, ListModelMutatedDuringIter>;
    type IntoIter = ListModelIter<glib::Object>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
            .expect("can't create a ListModelIter with the requested item type")
    }
}

#[test]
fn list_model_iter() {
    let list = crate::ListStore::new(crate::Menu::static_type());
    let m1 = crate::Menu::new();
    let m2 = crate::Menu::new();
    let m3 = crate::Menu::new();

    list.append(&m1);
    list.append(&m2);
    list.append(&m3);

    let mut iter = list.iter::<crate::Menu>().unwrap();
    assert_eq!(iter.next(), Some(Ok(m1)));
    assert_eq!(iter.next(), Some(Ok(m2)));
    list.remove_all();
    assert_eq!(iter.next(), Some(Err(ListModelMutatedDuringIter)));
}
