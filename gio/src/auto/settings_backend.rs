// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GSettingsBackend")]
    pub struct SettingsBackend(Object<ffi::GSettingsBackend, ffi::GSettingsBackendClass>);

    match fn {
        type_ => || ffi::g_settings_backend_get_type(),
    }
}

impl SettingsBackend {
    pub const NONE: Option<&'static SettingsBackend> = None;

    //#[doc(alias = "g_settings_backend_flatten_tree")]
    //pub fn flatten_tree(tree: /*Ignored*/&glib::Tree) -> (glib::GString, Vec<glib::GString>, Vec<glib::Variant>) {
    //    unsafe { TODO: call ffi:g_settings_backend_flatten_tree() }
    //}

    #[doc(alias = "g_settings_backend_get_default")]
    #[doc(alias = "get_default")]
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> SettingsBackend {
        unsafe { from_glib_full(ffi::g_settings_backend_get_default()) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::SettingsBackend>> Sealed for T {}
}

pub trait SettingsBackendExt: IsA<SettingsBackend> + sealed::Sealed + 'static {
    //#[doc(alias = "g_settings_backend_changed")]
    //fn changed(&self, key: &str, origin_tag: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:g_settings_backend_changed() }
    //}

    //#[doc(alias = "g_settings_backend_changed_tree")]
    //fn changed_tree(&self, tree: /*Ignored*/&glib::Tree, origin_tag: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:g_settings_backend_changed_tree() }
    //}

    //#[doc(alias = "g_settings_backend_keys_changed")]
    //fn keys_changed(&self, path: &str, items: &[&str], origin_tag: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:g_settings_backend_keys_changed() }
    //}

    //#[doc(alias = "g_settings_backend_path_changed")]
    //fn path_changed(&self, path: &str, origin_tag: /*Unimplemented*/Option<Basic: Pointer>) {
    //    unsafe { TODO: call ffi:g_settings_backend_path_changed() }
    //}

    #[doc(alias = "g_settings_backend_path_writable_changed")]
    fn path_writable_changed(&self, path: &str) {
        unsafe {
            ffi::g_settings_backend_path_writable_changed(
                self.as_ref().to_glib_none().0,
                path.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_settings_backend_writable_changed")]
    fn writable_changed(&self, key: &str) {
        unsafe {
            ffi::g_settings_backend_writable_changed(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            );
        }
    }
}

impl<O: IsA<SettingsBackend>> SettingsBackendExt for O {}

impl fmt::Display for SettingsBackend {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("SettingsBackend")
    }
}
