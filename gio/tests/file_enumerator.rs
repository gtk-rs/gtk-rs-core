// Take a look at the license at the top of the repository in the LICENSE file.
//
// The following tests rely on a custom type `MyLocalFileEnumerator` that extends the existing GIO type `GLocalFileEnumerator`.
// For each virtual method defined in class `gio::ffi::GFileEnumeratorClass`, a test checks that `MyLocalFileEnumerator` and `GLocalFileEnumerator` return the same results.
// Note that a `MyLocalFileEnumerator` instance is built explicitly by calling `glib::Object::builder` whereas a a `GLocalFileEnumerator` instance is created by calling `gio::auto::File::for_path`.

use gio::{
    prelude::*, subclass::prelude::*, Cancellable, File, FileAttributeMatcher, FileEnumerator,
    FileInfo, FileQueryInfoFlags, IOErrorEnum,
};
use glib::translate::{IntoGlib, ToGlibPtr};

// Binding of existing GIO type GLocalFileEnumerator.
pub mod ffi {
    #[cfg(not(windows))]
    use libc::DIR;
    use libc::{c_char, c_int, dev_t, ino_t};

    use gio::ffi;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalParentFileInfo {
        pub writable: glib::ffi::gboolean,
        pub is_sticky: glib::ffi::gboolean,
        pub has_trash_dir: glib::ffi::gboolean,
        pub owner: c_int,
        pub device: dev_t,
        pub inode: ino_t,
        pub extra_data: glib::ffi::gpointer,
        pub free_extra_data: glib::ffi::GDestroyNotify,
    }

    #[cfg(not(windows))]
    #[repr(C)]
    pub struct DirEntry {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalFileEnumerator {
        pub parent_instance: ffi::GFileEnumerator,
        pub matcher: *mut ffi::GFileAttributeMatcher,
        pub reduced_matcher: *mut ffi::GFileAttributeMatcher,
        pub filename: *mut c_char,
        pub attributes: *mut c_char,
        pub flags: ffi::GFileQueryInfoFlags,
        pub got_parent_info: glib::ffi::gboolean,
        pub parent_info: GLocalParentFileInfo,
        #[cfg(windows)]
        pub dir: *mut glib::ffi::GDir,
        #[cfg(not(windows))]
        pub dir: *mut DIR,
        #[cfg(not(windows))]
        pub entries: *mut DirEntry,
        #[cfg(not(windows))]
        pub entries_pos: c_int,
        #[cfg(not(windows))]
        pub at_end: glib::ffi::gboolean,
        pub follow_symlinks: glib::ffi::gboolean,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalFileEnumeratorClass {
        pub parent_class: ffi::GFileEnumeratorClass,
    }
}

glib::wrapper! {
    #[doc(alias = "GLocalFileEnumerator")]
    pub struct LocalFileEnumerator(Object<ffi::GLocalFileEnumerator, ffi::GLocalFileEnumeratorClass>) @extends FileEnumerator;

    match fn {
        type_ => || {
            use std::sync::Once;
            static ONCE: Once = Once::new();

            // ensure type is initialized by calling `gio::auto::File::for_path` to create a `GLocalFile` instance
            // and then by calling `gio::auto::file::FileExt::monitor_file` to create a `GLocalFileMonitor` instance.
            ONCE.call_once(|| unsafe {
                use crate::{File, FileQueryInfoFlags};
                let _ = File::for_path("/").enumerate_children("*", FileQueryInfoFlags::NONE, Cancellable::NONE);
            });
            glib::gobject_ffi::g_type_from_name("GLocalFileEnumerator".to_glib_none().0)
        },
    }
}

pub trait LocalFileEnumeratorImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<LocalFileEnumerator> + IsA<FileEnumerator>>
{
}

unsafe impl<T: LocalFileEnumeratorImpl + FileEnumeratorImpl> IsSubclassable<T>
    for LocalFileEnumerator
{
}

// Define `MyLocalFileEnumerator` as a subclass of `GLocalFileEnumerator`.
mod imp {
    #[cfg(not(windows))]
    use libc::opendir;

    use crate::FileAttributeMatcher;

    use super::*;

    #[derive(Default)]
    pub struct MyLocalFileEnumerator;

    #[glib::object_subclass]
    impl ObjectSubclass for MyLocalFileEnumerator {
        const NAME: &'static str = "MyLocalFileEnumerator";
        type Type = super::MyLocalFileEnumerator;
        type ParentType = LocalFileEnumerator;
    }

    // Handle properties `container`, `matcher` and `flags` to properly initialize `GLocalFileEnumerator` fields `dir`, `filename`, `matcher` and `flags` in the parent instance at creation time.
    impl glib::subclass::object::DerivedObjectProperties for MyLocalFileEnumerator {
        fn derived_properties() -> &'static [glib::ParamSpec] {
            use glib::prelude::ParamSpecBuilderExt;
            static PROPERTIES: ::std::sync::OnceLock<[glib::ParamSpec; 3]> =
                ::std::sync::OnceLock::new();
            PROPERTIES.get_or_init(||[
                <<File as glib::property::Property> ::Value as glib::prelude::HasParamSpec> ::param_spec_builder()("container").write_only().build(),
                <<FileAttributeMatcher as glib::property::Property> ::Value as glib::prelude::HasParamSpec> ::param_spec_builder()("matcher").write_only().build(),
                <<FileQueryInfoFlags as glib::property::Property> ::Value as glib::prelude::HasParamSpec> ::param_spec_builder()("flags").write_only().build(),
            ])
        }

        fn derived_property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            panic!("missing getter for property {}", pspec.name())
        }

        #[allow(unreachable_code)]
        fn derived_set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            let obj = self.obj();
            let parent = obj.upcast_ref::<Self::ParentType>();
            let glocal_file_enumerator = <Self::ParentType as ToGlibPtr<
                *mut <Self::ParentType as ObjectType>::GlibType,
            >>::to_glib_none(parent);
            match id {
                1 => {
                    let file = value.get::<File>().unwrap();
                    let filename = file.path().unwrap();
                    unsafe {
                        #[cfg(windows)]
                        {
                            (*glocal_file_enumerator.0).dir = glib::ffi::g_dir_open(
                                filename.to_glib_none().0,
                                0,
                                std::ptr::null_mut(),
                            );
                        }
                        #[cfg(not(windows))]
                        {
                            (*glocal_file_enumerator.0).dir = opendir(filename.to_glib_none().0);
                        }
                        (*glocal_file_enumerator.0).filename = filename.as_path().to_glib_full();
                    }
                }
                2 => {
                    let matcher = value.get::<FileAttributeMatcher>().unwrap();
                    unsafe {
                        (*glocal_file_enumerator.0).matcher = matcher.to_glib_full();
                    }
                }
                3 => {
                    let flags = value.get::<FileQueryInfoFlags>().unwrap();
                    unsafe {
                        (*glocal_file_enumerator.0).flags = flags.into_glib();
                    }
                }
                _ => panic!("missing setter for property {}", pspec.name()),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MyLocalFileEnumerator {}

    // Implements `FileEnumeratorImpl` with default implementation, which calls the parent's implementation.
    impl FileEnumeratorImpl for MyLocalFileEnumerator {}

    impl LocalFileEnumeratorImpl for MyLocalFileEnumerator {}
}

glib::wrapper! {
    pub struct MyLocalFileEnumerator(ObjectSubclass<imp::MyLocalFileEnumerator>) @extends LocalFileEnumerator, FileEnumerator;
}

impl Iterator for MyLocalFileEnumerator {
    type Item = Result<FileInfo, glib::Error>;

    fn next(&mut self) -> Option<Result<FileInfo, glib::Error>> {
        match self.imp().next_file(crate::Cancellable::NONE) {
            Err(err) => Some(Err(err)),
            Ok(file_info) => file_info.map(Ok),
        }
    }
}

#[allow(dead_code)]
mod file_utilities;
use file_utilities::Temp;

#[test]
fn file_enumerator_next_file() {
    // temporary dir and file are deleted when variables go out of scope
    let my_temp_dir = Temp::make_dir("next_file_XXXXXX");
    let _my_temp_file = my_temp_dir.create_file_child("my_file_XXXXXX");
    let _my_temp_file2 = my_temp_dir.create_file_child("my_file2_XXXXXX");

    // build a new `MyLocalFileEnumerator`
    let mut enumerator = glib::Object::builder::<MyLocalFileEnumerator>()
        .property("container", File::for_path(&my_temp_dir.path))
        .property("matcher", FileAttributeMatcher::new("*"))
        .property("flags", FileQueryInfoFlags::NONE)
        .build();

    // build a new `LocalFileEnumerator`
    let res = File::for_path(&my_temp_dir.path).enumerate_children(
        "*",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected_enumerator = res.unwrap();

    // iterate over `LocalFileEnumerator` to invoke its implementation of `gio::ffi::GFileEnumeratorClass::next_file`
    let mut n = 0;
    for res in expected_enumerator {
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let expected = res.unwrap();

        // invoke `MyLocalFileEnumerator` implementation of `gio::ffi::GFileEnumeratorClass::next_file`
        let res = enumerator.next();
        assert!(res.is_some(), "unexpected None");
        let res = res.unwrap();
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let file_info = res.unwrap();

        // both filenames should equal
        assert_eq!(file_info.name(), expected.name());
        n += 1;
    }
    assert_eq!(n, 2);
}

#[test]
fn file_enumerator_close() {
    // temporary dir and file are deleted when variables go out of scope
    let my_temp_dir = Temp::make_dir("close_XXXXXX");

    // build a new `MyLocalFileEnumerator`
    let mut enumerator = glib::Object::builder::<MyLocalFileEnumerator>()
        .property("container", File::for_path(&my_temp_dir.path))
        .property("matcher", FileAttributeMatcher::new("*"))
        .property("flags", FileQueryInfoFlags::NONE)
        .build();

    // build a new `LocalFileEnumerator`
    let res = File::for_path(&my_temp_dir.path).enumerate_children(
        "*",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let mut expected_enumerator = res.unwrap();

    // invoke `MyLocalFileEnumerator` implementation of `gio::ffi::GFileEnumeratorClass::close`
    let res = enumerator.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFileEnumerator` implementation of `gio::ffi::GFileEnumeratorClass::close`
    let res = expected_enumerator.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFileEnumerator` implementation of `gio::ffi::GFileEnumeratorClass::next`
    let res = enumerator.next();

    // invoke `LocalFileEnumerator` implementation of `gio::ffi::GFileEnumeratorClass::next`
    let expected = expected_enumerator.next();

    // both next results should equal
    assert_eq!(
        res.map(|res| res.map_err(|err| err.kind::<IOErrorEnum>())),
        expected.map(|res| res.map_err(|err| err.kind::<IOErrorEnum>()))
    );
}
