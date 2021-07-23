// Take a look at the license at the top of the repository in the LICENSE file.

use crate::auto::traits::ListModelExt;
use crate::prelude::*;
use crate::ListStore;
use glib::translate::*;
use glib::{Cast, IsA, Object};
use std::cmp::Ordering;

pub trait ListStoreExtManual {
    #[doc(alias = "g_list_store_insert_sorted")]
    fn insert_sorted<P: IsA<glib::Object>, F: FnMut(&Object, &Object) -> Ordering>(
        &self,
        item: &P,
        compare_func: F,
    ) -> u32;

    #[doc(alias = "g_list_store_sort")]
    fn sort<F: FnMut(&Object, &Object) -> Ordering>(&self, compare_func: F);

    /// Returns the item found by `compare_func` or else the inserted `default`.
    ///
    /// Call this method only if the list is already sorted in accordance to
    /// `compare_func`. The search is performed in log time.
    ///
    /// ### Panics
    /// Panics if `T::static_type()` is not of the model’s item type.
    fn find_or_insert_sorted<T, F, V>(&self, compare_func: F, default: V) -> T
    where
        T: IsA<Object>,
        F: FnMut(&T) -> Ordering,
        V: FnOnce() -> T;
}

impl<O: IsA<ListStore> + ListModelExt + ListModelExtManual> ListStoreExtManual for O {
    fn insert_sorted<P: IsA<glib::Object>, F: FnMut(&Object, &Object) -> Ordering>(
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
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    fn sort<F: FnMut(&Object, &Object) -> Ordering>(&self, compare_func: F) {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&Object, &Object) -> Ordering) = &mut func;
            let func_ptr = &func_obj as *const &mut (dyn FnMut(&Object, &Object) -> Ordering)
                as glib::ffi::gpointer;

            ffi::g_list_store_sort(
                self.as_ref().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    fn find_or_insert_sorted<T, F, V>(&self, compare_func: F, default: V) -> T
    where
        T: IsA<Object>,
        F: FnMut(&T) -> Ordering,
        V: FnOnce() -> T,
    {
        let this = self.upcast_ref();
        let i = match this.find_sorted(compare_func) {
            Ok(i) => i,
            Err(i) => {
                this.insert(i as u32, &default().upcast());
                i
            }
        };
        this.item(i as u32).unwrap().downcast().unwrap()
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

    match (*func)(&a, &b) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
