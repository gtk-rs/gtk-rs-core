// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Action};
use glib::{
    object::ObjectType as _,
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GSimpleAction")]
    pub struct SimpleAction(Object<ffi::GSimpleAction>) @implements Action;

    match fn {
        type_ => || ffi::g_simple_action_get_type(),
    }
}

impl SimpleAction {
    #[doc(alias = "g_simple_action_new")]
    pub fn new(name: &str, parameter_type: Option<&glib::VariantTy>) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new(
                name.to_glib_none().0,
                parameter_type.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_simple_action_new_stateful")]
    pub fn new_stateful(
        name: &str,
        parameter_type: Option<&glib::VariantTy>,
        state: &glib::Variant,
    ) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(
                name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                state.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_simple_action_set_enabled")]
    #[doc(alias = "enabled")]
    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            ffi::g_simple_action_set_enabled(self.to_glib_none().0, enabled.into_glib());
        }
    }

    #[doc(alias = "g_simple_action_set_state")]
    #[doc(alias = "state")]
    pub fn set_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "g_simple_action_set_state_hint")]
    pub fn set_state_hint(&self, state_hint: Option<&glib::Variant>) {
        unsafe {
            ffi::g_simple_action_set_state_hint(self.to_glib_none().0, state_hint.to_glib_none().0);
        }
    }

    #[doc(alias = "activate")]
    pub fn connect_activate<F: Fn(&Self, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn activate_trampoline<
            F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static,
        >(
            this: *mut ffi::GSimpleAction,
            parameter: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Variant>::from_glib_borrow(parameter)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"activate\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    activate_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "change-state")]
    pub fn connect_change_state<F: Fn(&Self, Option<&glib::Variant>) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn change_state_trampoline<
            F: Fn(&SimpleAction, Option<&glib::Variant>) + 'static,
        >(
            this: *mut ffi::GSimpleAction,
            value: *mut glib::ffi::GVariant,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                Option::<glib::Variant>::from_glib_borrow(value)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"change-state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    change_state_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
