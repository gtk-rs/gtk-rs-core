// Take a look at the license at the top of the repository in the LICENSE file.

use crate::SortType;
use glib::object::IsA;
use glib::translate::*;
use std::cmp::Ordering;
use std::fmt;
use std::mem;

use crate::{TreeIter, TreeModel, TreeSortable};
use ffi::{GtkTreeIter, GtkTreeModel};
use glib::ffi::gpointer;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum SortColumn {
    Default,
    Index(u32),
}

#[doc(hidden)]
impl ToGlib for SortColumn {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        match *self {
            SortColumn::Default => ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID,
            SortColumn::Index(x) => {
                assert!(x <= i32::max_value() as u32, "column index is too big");
                x as i32
            }
        }
    }
}

#[doc(hidden)]
impl FromGlib<i32> for SortColumn {
    #[inline]
    unsafe fn from_glib(val: i32) -> SortColumn {
        skip_assert_initialized!();
        match val {
            ffi::GTK_TREE_SORTABLE_DEFAULT_SORT_COLUMN_ID => SortColumn::Default,
            x => {
                assert!(x >= 0, "invalid column index");
                SortColumn::Index(x as u32)
            }
        }
    }
}

impl fmt::Display for SortColumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SortColumn::{}",
            match *self {
                SortColumn::Default => "Default",
                SortColumn::Index(_) => "Index",
            }
        )
    }
}

pub trait TreeSortableExtManual: 'static {
    fn set_default_sort_func<F>(&self, sort_func: F)
    where
        F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
    where
        F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering + 'static;
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)>;
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType);
    fn set_unsorted(&self);
}

fn into_raw<F, T>(func: F) -> gpointer
where
    F: Fn(&T, &TreeIter, &TreeIter) -> Ordering + 'static,
{
    skip_assert_initialized!();
    let func: Box<F> = Box::new(func);
    Box::into_raw(func) as gpointer
}

impl<O: IsA<TreeSortable>> TreeSortableExtManual for O {
    #[inline]
    fn get_sort_column_id(&self) -> Option<(SortColumn, SortType)> {
        unsafe {
            let mut sort_column_id = mem::MaybeUninit::uninit();
            let mut order = mem::MaybeUninit::uninit();
            ffi::gtk_tree_sortable_get_sort_column_id(
                self.as_ref().to_glib_none().0,
                sort_column_id.as_mut_ptr(),
                order.as_mut_ptr(),
            );
            let sort_column_id = sort_column_id.assume_init();
            if sort_column_id != ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID {
                Some((from_glib(sort_column_id), from_glib(order.assume_init())))
            } else {
                None
            }
        }
    }

    fn set_default_sort_func<F>(&self, sort_func: F)
    where
        F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering + 'static,
    {
        unsafe extern "C" fn trampoline<F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering>(
            this: *mut GtkTreeModel,
            iter: *mut GtkTreeIter,
            iter2: *mut GtkTreeIter,
            f: gpointer,
        ) -> i32 {
            let f: &F = &*(f as *const F);
            f(
                &TreeModel::from_glib_borrow(this),
                &from_glib_borrow(iter),
                &from_glib_borrow(iter2),
            )
            .to_glib()
        }
        unsafe extern "C" fn destroy_closure<
            F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering,
        >(
            ptr: gpointer,
        ) {
            Box::<F>::from_raw(ptr as *mut _);
        }
        unsafe {
            ffi::gtk_tree_sortable_set_default_sort_func(
                self.as_ref().to_glib_none().0,
                Some(trampoline::<F>),
                into_raw(sort_func),
                Some(destroy_closure::<F>),
            )
        }
    }

    #[inline]
    fn set_sort_column_id(&self, sort_column_id: SortColumn, order: SortType) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(
                self.as_ref().to_glib_none().0,
                sort_column_id.to_glib(),
                order.to_glib(),
            );
        }
    }

    fn set_unsorted(&self) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(
                self.as_ref().to_glib_none().0,
                ffi::GTK_TREE_SORTABLE_UNSORTED_SORT_COLUMN_ID,
                SortType::Ascending.to_glib(),
            );
        }
    }

    fn set_sort_func<F>(&self, sort_column_id: SortColumn, sort_func: F)
    where
        F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering + 'static,
    {
        unsafe extern "C" fn trampoline<F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering>(
            this: *mut GtkTreeModel,
            iter: *mut GtkTreeIter,
            iter2: *mut GtkTreeIter,
            f: gpointer,
        ) -> i32 {
            let f: &F = &*(f as *const F);
            f(
                &TreeModel::from_glib_borrow(this),
                &from_glib_borrow(iter),
                &from_glib_borrow(iter2),
            )
            .to_glib()
        }
        unsafe extern "C" fn destroy_closure<
            F: Fn(&TreeModel, &TreeIter, &TreeIter) -> Ordering,
        >(
            ptr: gpointer,
        ) {
            Box::<F>::from_raw(ptr as *mut _);
        }
        unsafe {
            ffi::gtk_tree_sortable_set_sort_func(
                self.as_ref().to_glib_none().0,
                sort_column_id.to_glib(),
                Some(trampoline::<F>),
                into_raw(sort_func),
                Some(destroy_closure::<F>),
            )
        }
    }
}
