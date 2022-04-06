// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::Object;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib::wrapper! {
    pub struct ListModel<T: (IsA<Object>)>(Interface<ffi::GListModel, ffi::GListModelInterface>),
        @requires_generic
            <Super: (IsA<Super>) + (IsA<Object>), Sub: (IsA<Super>) + (IsA<Object>)> ListModel<Super> for ListModel<Sub>,
        @default_casts false,
        @checkers ListModelCastChecker<T>, ListModelValueChecker<T>;

    match fn {
        type_ => || ffi::g_list_model_get_type(),
    }
}

#[doc(hidden)]
pub struct ListModelCastChecker<T>(std::marker::PhantomData<T>);

impl<T: IsA<Object>> glib::object::ObjectCastChecker<ListModel<T>> for ListModelCastChecker<T> {
    fn check<U: glib::ObjectType>(obj: &U) -> bool {
        if glib::object::GenericObjectCastChecker::<ListModel<T>>::check(obj) {
            let item_type: glib::Type =
                unsafe { from_glib(ffi::g_list_model_get_item_type(obj.as_ptr() as *mut _)) };
            if item_type.is_a(T::static_type()) {
                return true;
            }
        }
        false
    }
}

#[doc(hidden)]
pub struct ListModelValueChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: IsA<Object>> glib::value::ValueTypeChecker for ListModelValueChecker<T> {
    type Error = glib::value::ValueTypeMismatchOrNoneError<glib::value::ValueTypeMismatchError>;

    fn check(value: &glib::Value) -> Result<(), Self::Error> {
        glib::object::ObjectValueTypeChecker::<ListModel<Object>>::check(value)?;
        let model: &ListModel<Object> = unsafe { glib::value::FromValue::from_value(value) };
        let model_type = model.item_type();
        let expected = T::static_type();
        if !model_type.is_a(expected) {
            return Err(glib::value::ValueTypeMismatchError::new(model_type, expected).into());
        }

        Ok(())
    }
}

impl<T: IsA<Object>> ListModel<T> {
    pub const NONE: Option<&'static ListModel<T>> = None;
}

impl<T: IsA<Object>> std::fmt::Display for ListModel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("ListModel")
    }
}

pub trait ListModelExt: 'static {
    #[doc(alias = "g_list_model_get_item_type")]
    #[doc(alias = "get_item_type")]
    fn item_type(&self) -> glib::types::Type;

    #[doc(alias = "g_list_model_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> u32;

    #[doc(alias = "g_list_model_get_object")]
    #[doc(alias = "get_object")]
    fn object(&self, position: u32) -> Option<Object>;

    #[doc(alias = "g_list_model_get_item")]
    #[doc(alias = "get_item")]
    fn item<U: IsA<Object>>(&self, position: u32) -> Option<U>
    where
        Self: IsA<ListModel<U>>;

    #[doc(alias = "g_list_model_items_changed")]
    fn items_changed(&self, position: u32, removed: u32, added: u32);

    #[doc(alias = "items-changed")]
    fn connect_items_changed<F: Fn(&Self, u32, u32, u32) + 'static>(&self, f: F)
        -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Get an immutable snapshot of the container inside the `ListModel`.
    /// Any modification done to the returned container `Vec` will not be
    /// reflected on the `ListModel`.
    fn snapshot<U: IsA<Object>>(&self) -> Vec<U>
    where
        Self: IsA<ListModel<U>>;
}

impl<O: IsA<ListModel<Object>>> ListModelExt for O {
    #[inline]
    fn item_type(&self) -> glib::types::Type {
        unsafe {
            from_glib(ffi::g_list_model_get_item_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[inline]
    fn n_items(&self) -> u32 {
        unsafe { ffi::g_list_model_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[inline]
    fn object(&self, position: u32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::g_list_model_get_object(
                self.upcast_ref::<ListModel<Object>>().to_glib_none().0,
                position,
            ))
        }
    }

    fn item<U: IsA<Object>>(&self, position: u32) -> Option<U>
    where
        Self: IsA<ListModel<U>>,
    {
        self.object(position).map(|o| {
            o.downcast().unwrap_or_else(|o| {
                panic!(
                    "List model type mismatch. Actual {:?}, requested {:?}",
                    o.type_(),
                    U::static_type()
                );
            })
        })
    }

    #[inline]
    fn items_changed(&self, position: u32, removed: u32, added: u32) {
        unsafe {
            ffi::g_list_model_items_changed(
                self.as_ref().to_glib_none().0,
                position,
                removed,
                added,
            );
        }
    }

    fn connect_items_changed<F: Fn(&Self, u32, u32, u32) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn items_changed_trampoline<
            P: IsA<ListModel<Object>>,
            F: Fn(&P, u32, u32, u32) + 'static,
        >(
            this: *mut ffi::GListModel,
            position: libc::c_uint,
            removed: libc::c_uint,
            added: libc::c_uint,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ListModel::<Object>::from_glib_borrow(this).unsafe_cast_ref(),
                position,
                removed,
                added,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"items-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    items_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn snapshot<U: IsA<Object>>(&self) -> Vec<U>
    where
        Self: IsA<ListModel<U>>,
    {
        let count = self.n_items();
        let mut res = Vec::with_capacity(count as usize);
        for i in 0..count {
            res.push(self.item(i).unwrap())
        }
        res
    }
}

impl<T: IsA<Object> + IsA<T>> std::iter::IntoIterator for ListModel<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    // rustdoc-stripper-ignore-next
    /// Returns an iterator with the elements returned by `ListModel::snapshot`
    fn into_iter(self) -> Self::IntoIter {
        self.snapshot::<T>().into_iter()
    }
}
