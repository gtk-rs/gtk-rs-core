// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::ListModel;

pub trait ListModelExtManual: Sized {
    // rustdoc-stripper-ignore-next
    /// Get an immutable snapshot of the container inside the `ListModel`.
    /// Any modification done to the returned container `Vec` will not be
    /// reflected on the `ListModel`.
    fn snapshot(&self) -> Vec<glib::Object>;
}

impl<T: IsA<ListModel>> ListModelExtManual for T {
    fn snapshot(&self) -> Vec<glib::Object> {
        let mut res = Vec::with_capacity(self.n_items() as usize);
        for i in 0..self.n_items() {
            res.push(self.item(i).unwrap())
        }
        res
    }
}

impl std::iter::IntoIterator for ListModel {
    type Item = glib::Object;
    type IntoIter = std::vec::IntoIter<glib::Object>;

    // rustdoc-stripper-ignore-next
    /// Returns an iterator with the elements returned by `ListModel::snapshot`
    fn into_iter(self) -> Self::IntoIter {
        self.snapshot().into_iter()
    }
}
