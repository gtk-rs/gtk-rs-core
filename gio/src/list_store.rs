// Take a look at the license at the top of the repository in the LICENSE file.

use crate::auto::traits::ListModelExt;
use crate::ListStore;
use glib::translate::*;
use glib::{IsA, Object};
use std::cmp::Ordering;

impl ListStore {
    #[doc(alias = "g_list_store_insert_sorted")]
    pub fn insert_sorted<P: IsA<glib::Object>, F: FnMut(&Object, &Object) -> Ordering>(
        &self,
        item: &P,
        compare_func: F,
    ) -> u32 {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&Object, &Object) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (dyn FnMut(&Object, &Object) -> Ordering)
                as glib::ffi::gpointer;

            ffi::g_list_store_insert_sorted(
                self.to_glib_none().0,
                item.as_ref().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    #[doc(alias = "g_list_store_sort")]
    pub fn sort<F: FnMut(&Object, &Object) -> Ordering>(&self, compare_func: F) {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&Object, &Object) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (dyn FnMut(&Object, &Object) -> Ordering)
                as glib::ffi::gpointer;

            ffi::g_list_store_sort(
                self.to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    #[doc(alias = "g_list_store_splice")]
    pub fn splice(&self, position: u32, n_removals: u32, additions: &[impl IsA<glib::Object>]) {
        let n_additions = additions.len() as u32;
        unsafe {
            let additions = additions.as_ptr() as *mut *mut glib::gobject_ffi::GObject;

            ffi::g_list_store_splice(
                self.to_glib_none().0,
                position,
                n_removals,
                additions,
                n_additions,
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Appends all elements in a slice to the `ListStore`.
    pub fn extend_from_slice(&self, additions: &[impl IsA<glib::Object>]) {
        self.splice(self.n_items(), 0, additions)
    }
}

unsafe extern "C" fn compare_func_trampoline(
    a: glib::ffi::gconstpointer,
    b: glib::ffi::gconstpointer,
    func: glib::ffi::gpointer,
) -> i32 {
    let func = func as *mut &mut (dyn FnMut(&Object, &Object) -> Ordering);

    let a = from_glib_borrow(a as *mut glib::gobject_ffi::GObject);
    let b = from_glib_borrow(b as *mut glib::gobject_ffi::GObject);

    (*func)(&a, &b).into_glib()
}

impl<A: AsRef<glib::Object>> std::iter::Extend<A> for ListStore {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        let additions = iter
            .into_iter()
            .map(|o| o.as_ref().clone())
            .collect::<Vec<_>>();
        self.splice(self.n_items(), 0, &additions)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::ListStore;

    #[test]
    fn splice() {
        let item0 = ListStore::new(ListStore::static_type());
        let item1 = ListStore::new(ListStore::static_type());
        let list = ListStore::new(ListStore::static_type());
        list.splice(0, 0, &[item0.clone(), item1.clone()]);
        assert_eq!(list.item(0), Some(item0.upcast()));
        assert_eq!(list.item(1), Some(item1.upcast()));
    }

    #[test]
    fn extend() {
        let item0 = ListStore::new(ListStore::static_type());
        let item1 = ListStore::new(ListStore::static_type());
        let mut list = ListStore::new(ListStore::static_type());
        list.extend(&[&item0, &item1]);
        assert_eq!(list.item(0).as_ref(), Some(item0.upcast_ref()));
        assert_eq!(list.item(1).as_ref(), Some(item1.upcast_ref()));
        list.extend(&[item0.clone(), item1.clone()]);
        assert_eq!(list.item(2).as_ref(), Some(item0.upcast_ref()));
        assert_eq!(list.item(3).as_ref(), Some(item1.upcast_ref()));

        let list_from_slice = ListStore::new(ListStore::static_type());
        list_from_slice.extend_from_slice(&[item0, item1.clone()]);
        assert_eq!(list_from_slice.item(1).as_ref(), Some(item1.upcast_ref()));
    }
}
