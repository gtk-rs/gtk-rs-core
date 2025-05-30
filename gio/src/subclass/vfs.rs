// Take a look at the license at the top of the repository in the LICENSE file.

use std::path::PathBuf;

use glib::{prelude::*, subclass::prelude::*, translate::*, CStrV, GString};

use libc::c_char;

use crate::{ffi, File, Vfs};

// Support custom implementation of virtual functions defined in `gio::ffi::GVfsClass`.
pub trait VfsImpl: ObjectImpl + ObjectSubclass<Type: IsA<Vfs>> {
    fn is_active(&self) -> bool {
        self.parent_is_active()
    }

    fn get_file_for_path(&self, path: &std::path::Path) -> File {
        self.parent_get_file_for_path(path)
    }

    fn get_file_for_uri(&self, uri: &str) -> File {
        self.parent_get_file_for_uri(uri)
    }

    unsafe fn get_supported_uri_schemes(&self) -> &'static CStrV {
        self.parent_get_supported_uri_schemes()
    }

    fn parse_name(&self, parse_name: &str) -> File {
        self.parent_parse_name(parse_name)
    }
}

// Support parent implementation of virtual functions defined in `gio::ffi::GVfsClass`.
pub trait VfsImplExt: VfsImpl {
    fn parent_is_active(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .is_active
                .expect("No parent class implementation for \"is_active\"");

            let res = f(self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0);
            from_glib(res)
        }
    }

    fn parent_get_file_for_path(&self, path: &std::path::Path) -> File {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .get_file_for_path
                .expect("No parent class implementation for \"get_file_for_path\"");

            let res = f(
                self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0,
                path.to_glib_none().0,
            );
            from_glib_full(res)
        }
    }

    fn parent_get_file_for_uri(&self, uri: &str) -> File {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .get_file_for_uri
                .expect("No parent class implementation for \"get_file_for_uri\"");

            let res = f(
                self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0,
                uri.to_glib_none().0,
            );
            from_glib_full(res)
        }
    }

    fn parent_get_supported_uri_schemes(&self) -> &'static CStrV {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .get_supported_uri_schemes
                .expect("No parent class implementation for \"get_supported_uri_schemes\"");

            let res = f(self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0);
            CStrV::from_glib_borrow(res)
        }
    }

    fn parent_parse_name(&self, parse_name: &str) -> File {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .parse_name
                .expect("No parent class implementation for \"parse_name\"");

            let res = f(
                self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0,
                parse_name.to_glib_none().0,
            );
            from_glib_full(res)
        }
    }
}

impl<T: VfsImpl> VfsImplExt for T {}

// Implement virtual functions defined in `gio::ffi::GVfsClass`.
unsafe impl<T: VfsImpl> IsSubclassable<T> for Vfs {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.is_active = Some(is_active::<T>);
        klass.get_file_for_path = Some(get_file_for_path::<T>);
        klass.get_file_for_uri = Some(get_file_for_uri::<T>);
        klass.get_supported_uri_schemes = Some(get_supported_uri_schemes::<T>);
        klass.parse_name = Some(parse_name::<T>);
    }
}

unsafe extern "C" fn is_active<T: VfsImpl>(vfs: *mut ffi::GVfs) -> glib::ffi::gboolean {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let res = imp.is_active();

    res.into_glib()
}

unsafe extern "C" fn get_file_for_path<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    path: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.get_file_for_path(&PathBuf::from_glib_none(path));

    file.into_glib_ptr()
}

unsafe extern "C" fn get_file_for_uri<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    uri: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.get_file_for_uri(&GString::from_glib_borrow(uri));

    file.into_glib_ptr()
}

unsafe extern "C" fn get_supported_uri_schemes<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
) -> *const *const c_char {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let supported_uri_schemes = imp.get_supported_uri_schemes();

    supported_uri_schemes.as_ptr()
}

unsafe extern "C" fn parse_name<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    parse_name: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.parse_name(&GString::from_glib_borrow(parse_name));

    file.into_glib_ptr()
}

#[cfg(test)]
mod tests {
    // The following tests rely on a custom type `MyLocalVfs` that extends the existing GIO type `GLocalVfs`.
    // For each virtual method defined in class `gio::ffi::GVfsClass`, a test checks that `MyLocalVfs` and `GLocalVfs` return the same results.
    // Note that a `MyLocalVfs` instance is built explicitly by calling `glib::Object::builder` whereas a a `GLocalVfs` instance is created by calling `gio::auto::Vfs::local`.

    use super::*;
    use crate::prelude::*;

    // Binding of existing GIO type GLocalVfs.
    mod ffi {
        use crate::ffi;

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

    // Defines `MyLocalVfs`, `MyVfs` and `MyCustomVfs`.
    mod imp {
        use super::*;

        // Defines `MyLocalVfs` as a subclass of `GLocalVfs`.
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

        // Defines `MyVfs` as a subclass of `Vfs`.
        #[derive(Default)]
        pub struct MyVfs;

        #[glib::object_subclass]
        impl ObjectSubclass for MyVfs {
            const NAME: &'static str = "MyVfs";
            type Type = super::MyVfs;
            type ParentType = Vfs;
        }

        impl ObjectImpl for MyVfs {}

        // Implements `VfsImpl` with specific implementation of `get_supported_uri_schemes`.
        impl VfsImpl for MyVfs {
            unsafe fn get_supported_uri_schemes(&self) -> &'static CStrV {
                static SUPPORTED_URI_SCHEMES: std::sync::OnceLock<glib::StrV> =
                    std::sync::OnceLock::new();
                SUPPORTED_URI_SCHEMES
                    .get_or_init(|| glib::StrV::from(["myvfs"]))
                    .into()
            }
        }

        // Defines `MyCustomVfs` as a subclass of `MyVfs`.
        #[derive(Default)]
        pub struct MyCustomVfs;

        #[glib::object_subclass]
        impl ObjectSubclass for MyCustomVfs {
            const NAME: &'static str = "MyCustomVfs";
            type Type = super::MyCustomVfs;
            type ParentType = super::MyVfs;
        }

        impl ObjectImpl for MyCustomVfs {}

        // Implements `VfsImpl` with default implementation, which calls the parent's implementation.
        impl VfsImpl for MyCustomVfs {}

        impl MyVfsImpl for MyCustomVfs {}
    }

    glib::wrapper! {
        pub struct MyLocalVfs(ObjectSubclass<imp::MyLocalVfs>) @extends LocalVfs, Vfs;
    }

    glib::wrapper! {
        pub struct MyVfs(ObjectSubclass<imp::MyVfs>) @extends Vfs;
    }

    pub trait MyVfsImpl: ObjectImpl + ObjectSubclass<Type: IsA<MyVfs> + IsA<Vfs>> {}

    // To make this class subclassable we need to implement IsSubclassable
    unsafe impl<T: MyVfsImpl + VfsImpl> IsSubclassable<T> for MyVfs {}

    glib::wrapper! {
        pub struct MyCustomVfs(ObjectSubclass<imp::MyCustomVfs>) @extends MyVfs, Vfs;
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
    fn my_vfs_get_supported_uri_schemes() {
        // invoke `MyLocalVfs` implementation of `gio::ffi::GVfsClass::supported_uri_schemes`
        let my_local_vfs = glib::Object::new::<MyLocalVfs>();
        let my_local_vfs_schemes = my_local_vfs.supported_uri_schemes();

        // invoke `MyCustomVfs` implementation of `gio::ffi::GVfsClass::supported_uri_schemes`
        let my_custom_vfs = glib::Object::new::<MyCustomVfs>();
        let my_custom_vfs_schemes = my_custom_vfs.supported_uri_schemes();
        assert_eq!(my_custom_vfs_schemes, vec!["myvfs", "resource"]);

        // both results should not equal
        assert_ne!(my_local_vfs_schemes, my_custom_vfs_schemes);
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
}
