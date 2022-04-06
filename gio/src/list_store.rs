// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::ListModelExt;
use crate::ListModel;
use glib::translate::*;
use glib::{Cast, IsA, Object};
use std::cmp::Ordering;
#[cfg(any(feature = "v2_64", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
use std::mem;

glib::wrapper! {
    pub struct ListStore<T: (IsA<Object>)>(Object<ffi::GListStore, ffi::GListStoreClass>),
        @implements_generic
            <Super: (IsA<Super>) + (IsA<Object>), Sub: (IsA<Super>) + (IsA<Object>)> ListModel<Super> for ListStore<Sub>,
        @default_casts true,
        @checkers ListStoreCastChecker<T>, ListStoreValueChecker<T>;

    match fn {
        type_ => || ffi::g_list_store_get_type(),
    }
}

#[doc(hidden)]
pub struct ListStoreCastChecker<T>(std::marker::PhantomData<T>);

impl<T: IsA<Object>> glib::object::ObjectCastChecker<ListStore<T>> for ListStoreCastChecker<T> {
    fn check<U: glib::ObjectType>(obj: &U) -> bool {
        if glib::object::GenericObjectCastChecker::<ListStore<T>>::check(obj) {
            let item_type: glib::Type =
                unsafe { from_glib(ffi::g_list_model_get_item_type(obj.as_ptr() as *mut _)) };
            if item_type == T::static_type() {
                return true;
            }
        }
        false
    }
}

#[doc(hidden)]
pub struct ListStoreValueChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: IsA<Object>> glib::value::ValueTypeChecker for ListStoreValueChecker<T> {
    type Error = glib::value::ValueTypeMismatchOrNoneError<glib::value::ValueTypeMismatchError>;

    fn check(value: &glib::Value) -> Result<(), Self::Error> {
        glib::object::ObjectValueTypeChecker::<ListStore<Object>>::check(value)?;
        let store: &ListStore<Object> = unsafe { glib::value::FromValue::from_value(value) };
        let store_type = store.item_type();
        let expected = T::static_type();
        if store_type != expected {
            return Err(glib::value::ValueTypeMismatchError::new(store_type, expected).into());
        }

        Ok(())
    }
}

impl<T: IsA<Object>> ListStore<T> {
    #[doc(alias = "g_list_store_new")]
    pub fn new() -> Self {
        unsafe { from_glib_full(ffi::g_list_store_new(T::static_type().into_glib())) }
    }

    #[doc(alias = "g_list_store_append")]
    pub fn append(&self, item: &(impl IsA<T> + IsA<Object>)) {
        unsafe {
            ffi::g_list_store_append(
                self.to_glib_none().0,
                item.upcast_ref::<Object>().to_glib_none().0,
            );
        }
    }

    #[cfg(any(feature = "v2_64", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_64")))]
    #[doc(alias = "g_list_store_find")]
    pub fn find(&self, item: &(impl IsA<T> + IsA<Object>)) -> Option<u32> {
        unsafe {
            let mut position = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_list_store_find(
                self.to_glib_none().0,
                item.upcast_ref::<Object>().to_glib_none().0,
                position.as_mut_ptr(),
            ));
            let position = position.assume_init();
            if ret {
                Some(position)
            } else {
                None
            }
        }
    }

    #[doc(alias = "g_list_store_insert")]
    pub fn insert(&self, position: u32, item: &(impl IsA<T> + IsA<Object>)) {
        unsafe {
            ffi::g_list_store_insert(
                self.to_glib_none().0,
                position,
                item.upcast_ref::<Object>().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_list_store_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::g_list_store_remove(self.to_glib_none().0, position);
        }
    }

    #[doc(alias = "g_list_store_remove_all")]
    pub fn remove_all(&self) {
        unsafe {
            ffi::g_list_store_remove_all(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_list_store_insert_sorted")]
    pub fn insert_sorted<P: IsA<T> + IsA<Object>, F: FnMut(&T, &T) -> Ordering>(
        &self,
        item: &P,
        compare_func: F,
    ) -> u32 {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&T, &T) -> Ordering) = &mut func;
            let func_ptr =
                &func_obj as *const &mut (dyn FnMut(&T, &T) -> Ordering) as glib::ffi::gpointer;

            ffi::g_list_store_insert_sorted(
                self.to_glib_none().0,
                item.upcast_ref::<Object>().to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    #[doc(alias = "g_list_store_sort")]
    pub fn sort<F: FnMut(&T, &T) -> Ordering>(&self, compare_func: F) {
        unsafe {
            let mut func = compare_func;
            let func_obj: &mut (dyn FnMut(&T, &T) -> Ordering) = &mut func;
            let func_ptr =
                &func_obj as *const &mut (dyn FnMut(&T, &T) -> Ordering) as glib::ffi::gpointer;

            ffi::g_list_store_sort(
                self.to_glib_none().0,
                Some(compare_func_trampoline),
                func_ptr,
            )
        }
    }

    #[doc(alias = "g_list_store_splice")]
    pub fn splice(&self, position: u32, n_removals: u32, additions: &[impl IsA<T> + IsA<Object>]) {
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
    pub fn extend_from_slice(&self, additions: &[impl IsA<T> + IsA<Object>]) {
        self.splice(self.n_items() - 1, 0, additions)
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

impl<T: IsA<Object>> Default for ListStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: IsA<Object>> std::fmt::Display for ListStore<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ListStore")
    }
}

impl<T: IsA<Object> + IsA<T>, A: AsRef<T>> std::iter::Extend<A> for ListStore<T> {
    fn extend<I: IntoIterator<Item = A>>(&mut self, iter: I) {
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
    use crate::Application;
    use crate::File;
    use crate::ListModel;
    use crate::ListStore;
    use glib::Object;
    use glib::StaticType;
    use glib::ToValue;
    use std::path::PathBuf;

    #[test]
    fn type_checking() {
        let store = ListStore::<File>::new();
        let f = File::for_path("/");
        store.append(&f);
        let _: Object = store.item(0).unwrap();
        let file = store.item(0);
        assert_eq!(file.as_ref(), Some(&f));
        assert_eq!(file.unwrap().path().unwrap(), PathBuf::from("/"));

        assert!(store.dynamic_cast_ref::<ListStore<Object>>().is_none());

        let object = store.upcast_ref::<Object>();
        object.downcast_ref::<ListModel<File>>().unwrap();
        object.downcast_ref::<ListModel<Object>>().unwrap();
        assert!(object.downcast_ref::<ListStore<Object>>().is_none());
        assert!(object.downcast_ref::<ListStore<Application>>().is_none());
        assert!(object.downcast_ref::<ListModel<Application>>().is_none());

        let typed_model = store.upcast_ref::<ListModel<File>>();
        assert_eq!(typed_model.item_type(), File::static_type());
        typed_model.downcast_ref::<ListStore<File>>().unwrap();

        let object_model = store.upcast_ref::<ListModel<Object>>();
        object_model.downcast_ref::<ListModel<File>>().unwrap();
        object_model.downcast_ref::<ListStore<File>>().unwrap();
        assert!(object_model.downcast_ref::<ListStore<Object>>().is_none());

        let value = store.to_value();
        value.get::<ListStore<File>>().unwrap();
        value.get::<Option<ListStore<File>>>().unwrap().unwrap();
        value.get::<ListModel<File>>().unwrap();
        value.get::<ListModel<Object>>().unwrap();
        value.get::<Option<ListModel<Object>>>().unwrap().unwrap();
        value.get::<Option<ListModel<File>>>().unwrap().unwrap();
        assert!(value.get::<ListStore<Object>>().is_err());
        assert!(value.get::<ListStore<Application>>().is_err());
        assert!(value.get::<ListModel<Application>>().is_err());

        let none = None::<ListStore<File>>;
        let none_value = none.to_value();
        assert!(none_value.get::<ListStore<File>>().is_err());
        assert!(none_value.get::<ListStore<Object>>().is_err());
        assert!(none_value.get::<ListModel<File>>().is_err());
        assert!(none_value.get::<ListModel<Object>>().is_err());
        assert!(none_value
            .get::<Option<ListStore<Object>>>()
            .unwrap()
            .is_none());
        assert!(none_value
            .get::<Option<ListStore<File>>>()
            .unwrap()
            .is_none());
        assert!(none_value
            .get::<Option<ListStore<Application>>>()
            .unwrap()
            .is_none());
        assert!(none_value
            .get::<Option<ListModel<Object>>>()
            .unwrap()
            .is_none());
        assert!(none_value
            .get::<Option<ListModel<File>>>()
            .unwrap()
            .is_none());
        assert!(none_value
            .get::<Option<ListModel<Application>>>()
            .unwrap()
            .is_none());
    }

    #[test]
    fn splice() {
        let item0 = ListStore::<Object>::new();
        let item1 = ListStore::<Object>::new();
        let list = ListStore::<ListStore<Object>>::new();
        list.splice(0, 0, &[item0.clone(), item1.clone()]);
        assert_eq!(list.item(0), Some(item0));
        assert_eq!(list.item(1), Some(item1));
    }

    #[test]
    fn extend() {
        let item0 = ListStore::<Object>::new();
        let item1 = ListStore::<Object>::new();
        let mut list = ListStore::<ListStore<Object>>::new();
        list.extend(&[&item0, &item1]);
        assert_eq!(list.item(0).as_ref(), Some(&item0));
        assert_eq!(list.item(1).as_ref(), Some(&item1));
        list.extend(&[item0.clone(), item1.clone()]);
        assert_eq!(list.item(2).as_ref(), Some(&item0));
        assert_eq!(list.item(3).as_ref(), Some(&item1));
    }
}
