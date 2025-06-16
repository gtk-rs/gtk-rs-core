// Take a look at the license at the top of the repository in the LICENSE file.
//
// The following tests rely on a custom type `MyLocalVfs` that extends the existing GIO type `GLocalVfs`.
// For each virtual method defined in class `gio::ffi::GVfsClass`, a test checks that `MyLocalVfs` and `GLocalVfs` return the same results.
// Note that a `MyLocalVfs` instance is built explicitly by calling `glib::Object::builder` whereas a a `GLocalVfs` instance is created by calling `gio::auto::Vfs::local`.

use gio::{prelude::*, subclass::prelude::*, File, Vfs};
use glib::translate::ToGlibPtr;

// Binding of existing GIO type GLocalVfs.
mod ffi {
    use gio::ffi;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalVfs {
        pub parent_instance: ffi::GVfs,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalVfsClass {
        pub parent_class: ffi::GVfsClass,
    }
}

glib::wrapper! {
    #[doc(alias = "GLocalVfs")]
    pub struct LocalVfs(Object<ffi::GLocalVfs, ffi::GLocalVfsClass>) @extends Vfs;

    match fn {
        type_ => || {
            use std::sync::Once;
            static ONCE: Once = Once::new();

            // ensure type is initialized by calling `gio::auto::File::for_path` to create a `GLocalFile` instance.
            ONCE.call_once(|| unsafe {
                let _ = File::for_path("path");
            });
            glib::gobject_ffi::g_type_from_name("GLocalVfs".to_glib_none().0)
        },
    }
}

pub trait LocalVfsImpl: ObjectImpl + ObjectSubclass<Type: IsA<LocalVfs> + IsA<Vfs>> {}

unsafe impl<T: LocalVfsImpl + VfsImpl> IsSubclassable<T> for LocalVfs {}

// Define `MyLocalVfs` as a subclass of `GLocalVfs`.
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct MyLocalVfs;

    #[glib::object_subclass]
    impl ObjectSubclass for MyLocalVfs {
        const NAME: &'static str = "MyLocalVfs";
        type Type = super::MyLocalVfs;
        type ParentType = LocalVfs;
    }

    impl ObjectImpl for MyLocalVfs {}

    // Implements `VfsImpl` with default implementation, which calls the parent's implementation.
    impl VfsImpl for MyLocalVfs {}

    impl LocalVfsImpl for MyLocalVfs {}
}

glib::wrapper! {
    pub struct MyLocalVfs(ObjectSubclass<imp::MyLocalVfs>) @extends LocalVfs, Vfs;
}

#[test]
fn vfs_is_active() {
    // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::is_active`
    let my_local_vfs = glib::Object::new::<MyLocalVfs>();
    let active = my_local_vfs.is_active();

    // invoke `LocalVfs` implementation of `gio::ffi::GVfsClass::is_active`
    let expected = Vfs::local().is_active();

    // both results should equal
    assert_eq!(active, expected);
}

#[test]
fn vfs_get_file_for_path() {
    // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::get_file_for_path`
    let my_local_vfs = glib::Object::new::<MyLocalVfs>();
    let file = my_local_vfs.file_for_path("/path");

    // invoke `LocalVfs` implementation of `gio::ffi::GVfsClass::get_file_for_path`
    let expected = Vfs::local().file_for_path("/path");

    // both files should equal
    assert!(file.equal(&expected));
}

#[test]
fn vfs_get_file_for_uri() {
    // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::get_file_for_uri`
    let my_local_vfs = glib::Object::new::<MyLocalVfs>();
    let file = my_local_vfs.file_for_uri("file:///path");

    // invoke `LocalVfs` implementation of `gio::ffi::GVfsClass::get_file_for_uri`
    let expected = Vfs::local().file_for_uri("file:///path");

    // both files should equal
    assert!(file.equal(&expected));
}

#[test]
fn vfs_get_supported_uri_schemes() {
    // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::supported_uri_schemes`
    let my_local_vfs = glib::Object::new::<MyLocalVfs>();
    let schemes = my_local_vfs.supported_uri_schemes();

    // invoke `LocalVfs` implementation of `gio::ffi::GVfsClass::supported_uri_schemes`
    let expected = Vfs::local().supported_uri_schemes();

    // both results should equal
    assert_eq!(schemes, expected);
}

#[test]
fn vfs_parse_name() {
    // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::parse_name`
    let my_local_vfs = glib::Object::new::<MyLocalVfs>();
    let file = my_local_vfs.parse_name("file:///path");

    // invoke `LocalVfs` implementation of `gio::ffi::GVfsClass::parse_name`
    let expected = Vfs::local().parse_name("file:///path");

    // both files should equal
    assert!(file.equal(&expected));
}
