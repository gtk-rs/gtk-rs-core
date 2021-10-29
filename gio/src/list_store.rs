// Take a look at the license at the top of the repository in the LICENSE file.

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
            let additions = additions
                .iter()
                .map(|o| o.as_ptr() as *mut glib::gobject_ffi::GObject)
                .collect::<Vec<_>>();

            ffi::g_list_store_splice(
                self.to_glib_none().0,
                position,
                n_removals,
                mut_override(additions.as_ptr()),
                n_additions,
            );
        }
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
