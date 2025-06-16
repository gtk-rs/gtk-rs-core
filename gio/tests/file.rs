// Take a look at the license at the top of the repository in the LICENSE file.
//
// The following tests rely on a custom type `MyLocalFile` that extends the existing GIO type `GLocalFile`. Both types implement the interface `gio::auto::File`.
// For each virtual method defined in interface `gio::ffi::GFileIface`, a test checks that `MyLocalFile` and `GLocalFile` return the same results.
// Note that a `MyLocalFile` instance is built explicitly by calling `glib::Object::builder` whereas a a `GLocalFile` instance is created by calling `gio::auto::File::for_path`.

use std::path::PathBuf;

use futures_channel::oneshot;

use gio::{
    prelude::*, subclass::prelude::*, Cancellable, DriveStartFlags, File, FileCopyFlags,
    FileCreateFlags, FileMeasureFlags, FileMonitor, FileMonitorEvent, FileMonitorFlags,
    FileQueryInfoFlags, IOErrorEnum, MountMountFlags, MountOperation, MountUnmountFlags,
};
use glib::translate::ToGlibPtr;

// Binding of existing GIO type GLocalFile.
pub mod ffi {
    use libc::c_char;

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalFile {
        pub parent_instance: glib::gobject_ffi::GObject,
        pub filename: *mut c_char,
    }

    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct GLocalFileClass {
        pub parent_class: glib::gobject_ffi::GObjectClass,
    }
}

glib::wrapper! {
    #[doc(alias = "GLocalFile")]
    pub struct LocalFile(Object<ffi::GLocalFile, ffi::GLocalFileClass>) @implements File;

    match fn {
        type_ => || {
            use std::sync::Once;
            static ONCE: Once = Once::new();

            // ensure type is initialized by calling `gio::auto::File::for_path` to create a `GLocalFile` instance.
            ONCE.call_once(|| unsafe {
                let _ = File::for_path("path");
            });
            glib::gobject_ffi::g_type_from_name("GLocalFile".to_glib_none().0)
        },
    }
}

pub trait LocalFileImpl: ObjectImpl + ObjectSubclass<Type: IsA<LocalFile> + IsA<File>> {}

unsafe impl<T: LocalFileImpl> IsSubclassable<T> for LocalFile {}

// Define `MyLocalFile` as a subclass of `GLocalFile`.
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct MyLocalFile;

    #[glib::object_subclass]
    impl ObjectSubclass for MyLocalFile {
        const NAME: &'static str = "MyLocalFile";
        type Type = super::MyLocalFile;
        type ParentType = super::LocalFile;
        type Interfaces = (File,);
    }

    // Handle property `path` to properly initialize `GLocalFile` field `filename` in the parent instance at creation time.
    impl DerivedObjectProperties for MyLocalFile {
        fn derived_properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: ::std::sync::OnceLock<[glib::ParamSpec; 1]> =
                ::std::sync::OnceLock::new();
            PROPERTIES.get_or_init(||[<<PathBuf as glib::property::Property> ::Value as HasParamSpec> ::param_spec_builder()("path").write_only().build(),])
        }

        fn derived_property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            panic!("missing getter for property {}", pspec.name())
        }

        #[allow(unreachable_code)]
        fn derived_set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            match id {
                1 => unsafe {
                    (*<Self::ParentType as ToGlibPtr<
                        *mut <Self::ParentType as ObjectType>::GlibType,
                    >>::to_glib_none(
                        self.obj().upcast_ref::<Self::ParentType>()
                    )
                    .0)
                        .filename = value.get::<PathBuf>().unwrap().to_glib_full()
                },
                _ => panic!("missing setter for property {}", pspec.name()),
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for MyLocalFile {}

    // Implements `FileImpl` with default implementation, which calls the parent's implementation.
    impl FileImpl for MyLocalFile {}

    impl LocalFileImpl for MyLocalFile {}
}

glib::wrapper! {
    pub struct MyLocalFile(ObjectSubclass<imp::MyLocalFile>) @extends LocalFile, @implements File;
}

#[allow(dead_code)]
mod file_utilities;
use file_utilities::Temp;

#[cfg(unix)]
const MY_FILE: &str = "/my_file";

#[cfg(windows)]
const MY_FILE: &str = "c:\\my_file";

#[test]
fn file_dup() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::dup`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let dup = my_local_file.dup();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::dup`
    let expected = File::for_path(MY_FILE).dup();

    // both results should equal
    assert!(dup.equal(&expected));
}

// checker-ignore-item
#[test]
fn file_hash() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::hash`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let hash = unsafe {
        gio::ffi::g_file_hash(
            <MyLocalFile as ToGlibPtr<
                *mut glib::subclass::basic::InstanceStruct<imp::MyLocalFile>,
            >>::to_glib_none(&my_local_file)
            .0 as glib::ffi::gconstpointer,
        )
    };

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::hash`
    let expected = unsafe {
        gio::ffi::g_file_hash(
            <File as ToGlibPtr<*mut gio::ffi::GFile>>::to_glib_none(&File::for_path(MY_FILE)).0
                as glib::ffi::gconstpointer,
        )
    };

    // both hash values should equal
    assert_eq!(hash, expected);
}

#[test]
fn file_equal() {
    // 2 instances of `MyLocalFile` with same path should equal
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let expected = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    assert!(my_local_file.equal(&expected));

    // instances of `MyLocalFile` and of `LocalFile` with same path should not equal (because type is different)
    let expected = File::for_path(MY_FILE);
    assert!(!my_local_file.equal(&expected));
}

#[test]
fn file_is_native() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::is_native`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let is_native = my_local_file.is_native();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::is_native`
    let expected = File::for_path(MY_FILE).is_native();

    // both results should equal
    assert_eq!(is_native, expected);
}

#[test]
fn file_has_uri_scheme() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::has_uri_scheme`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let has_file = my_local_file.has_uri_scheme("file");
    let has_foo = my_local_file.has_uri_scheme("foo");

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::has_uri_scheme`
    let file = File::for_path(MY_FILE);
    let expected_file = file.has_uri_scheme("file");
    let expected_foo = file.has_uri_scheme("foo");

    // both results should equal
    assert_eq!(has_file, expected_file);
    assert_eq!(has_foo, expected_foo);
}

#[test]
fn file_uri_scheme() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::uri_scheme`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let uri_scheme = my_local_file.uri_scheme();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::uri_scheme`
    let expected = File::for_path(MY_FILE).uri_scheme();

    // both uri schemes should equal
    assert_eq!(uri_scheme, expected);
}

#[test]
fn file_basename() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::basename`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let basename = my_local_file.basename();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::basename`
    let expected = File::for_path(MY_FILE).basename();

    // both basenames should equal
    assert_eq!(basename, expected);
}

#[test]
fn file_path() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::path`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let path = my_local_file.path();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::path`
    let expected = File::for_path(MY_FILE).path();

    // both paths should equal
    assert_eq!(path, expected);
}

#[test]
fn file_uri() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::uri`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let uri = my_local_file.uri();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::uri`
    let expected = File::for_path(MY_FILE).uri();

    // both uris should equal
    assert_eq!(uri, expected);
}

#[test]
fn file_parse_name() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::parse_name`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", MY_FILE)
        .build();
    let parse_name = my_local_file.parse_name();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::parse_name`
    let expected = File::for_path(MY_FILE).parse_name();

    // both parse names should equal
    assert_eq!(parse_name, expected);
}

#[test]
fn file_parent() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::parent`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_parent/my_file")
        .build();
    let res = my_local_file.parent();
    assert!(res.is_some(), "unexpected None");
    let parent = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::parent`
    let res = File::for_path("/my_parent/my_file").parent();
    assert!(res.is_some(), "unexpected None");
    let expected = res.unwrap();

    // both parents should equal
    assert!(parent.equal(&expected));
}

#[test]
fn file_has_prefix() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::has_prefix`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_prefix/my_file")
        .build();
    let my_local_prefix = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_prefix")
        .build();
    let has_prefix = my_local_file.has_prefix(&my_local_prefix);

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::has_prefix`
    let expected = File::for_path("/my_prefix/my_file").has_prefix(&File::for_path("/my_prefix"));

    // both results should equal
    assert_eq!(has_prefix, expected);
}

#[test]
fn file_relative_path() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::relative_path`
    let my_local_parent = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_parent")
        .build();
    let my_local_descendant = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_parent/my_descendant")
        .build();
    let relative_path = my_local_parent.relative_path(&my_local_descendant);

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::relative_path`
    let expected =
        File::for_path("/my_parent").relative_path(&File::for_path("/my_parent/my_descendant"));

    // both relative paths should equal
    assert_eq!(relative_path, expected);
}

#[test]
fn file_resolve_relative_path() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::resolve_relative_path`
    let my_local_prefix = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_prefix")
        .build();
    let resolved_path = my_local_prefix.resolve_relative_path("my_file");

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::resolve_relative_path`
    let expected = File::for_path("/my_prefix").resolve_relative_path("my_file");

    // both resolved path result should equal
    assert!(resolved_path.equal(&expected));
}

#[test]
fn file_child_for_display_name() {
    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::child_for_display_name`
    let my_local_parent = glib::Object::builder::<MyLocalFile>()
        .property("path", "/my_parent")
        .build();
    let res = my_local_parent.child_for_display_name("my_file");
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let child = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::child_for_display_name`
    let res = File::for_path("/my_parent").child_for_display_name("my_file");
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected = res.unwrap();

    // both children should equal
    assert!(child.equal(&expected))
}

#[test]
fn file_enumerate_children() {
    // temporary dir and file are deleted when variables go out of scope
    let my_temp_dir = Temp::make_dir("enumerate_children_XXXXXX");
    let _my_temp_file = my_temp_dir.create_file_child("my_file_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::enumerate_children`
    let my_local_dir = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_dir.path)
        .build();
    let res = my_local_dir.enumerate_children("*", FileQueryInfoFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let mut enumerator = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::enumerate_children`
    let res = File::for_path(&my_temp_dir.path).enumerate_children(
        "*",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected_enumerator = res.unwrap();

    // for each expected child
    for res in expected_enumerator {
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let expected = res.unwrap();

        // get next child from MyLocalFile's implementation
        let res = enumerator.next();
        assert!(res.is_some(), "unexpected None");
        let res = res.unwrap();
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let file_info = res.unwrap();

        // both file infos should have the same attributes
        for attr in expected.list_attributes(None) {
            if attr != "standard::icon" && attr != "standard::symbolic-icon" {
                assert_eq!(
                    file_info.attribute_as_string(&attr),
                    expected.attribute_as_string(&attr),
                    "attribute: {}",
                    &attr
                );
            }
        }
    }
}

#[test]
fn file_query_info() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("query_info_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::query_info`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.query_info("*", FileQueryInfoFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let my_local_file_info = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_info`
    let res = File::for_path(&my_temp_file.path).query_info(
        "*",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected = res.unwrap();

    // both file infos should have the same attributes
    for attr in expected.list_attributes(None) {
        if attr != "standard::icon" && attr != "standard::symbolic-icon" {
            assert_eq!(
                my_local_file_info.attribute_as_string(&attr),
                expected.attribute_as_string(&attr),
                "attribute: {}",
                &attr
            );
        }
    }
}

#[test]
fn file_query_filesystem_info() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("query_filesystem_info_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::query_filesystem_info`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.query_filesystem_info("*", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let file_info = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_filesystem_info`
    let res = File::for_path(&my_temp_file.path).query_filesystem_info("*", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected = res.unwrap();

    // both file infos should have the same attributes
    for attr in expected.list_attributes(None) {
        if attr != "filesystem::free" && attr != "filesystem::used" {
            assert_eq!(
                file_info.attribute_as_string(&attr),
                expected.attribute_as_string(&attr),
                "attribute: {}",
                &attr
            );
        }
    }
}

#[test]
fn file_find_enclosing_mount() {
    // temporary dir is deleted when the variable goes out of scope
    let my_temp_dir = Temp::make_dir("find_enclosing_mount_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::find_enclosing_mount`
    let my_local_dir = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_dir.path)
        .build();
    let res = my_local_dir.find_enclosing_mount(Cancellable::NONE);

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::find_enclosing_mount`
    let expected = File::for_path(&my_temp_dir.path).find_enclosing_mount(Cancellable::NONE);

    // both results should equal
    assert_eq!(
        res.map(|mount| mount.name())
            .map_err(|err| err.kind::<IOErrorEnum>()),
        expected
            .map(|mount| mount.name())
            .map_err(|err| err.kind::<IOErrorEnum>())
    );
}

#[test]
fn file_set_display_name() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("set_display_name_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::set_display_name`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let new_name = format!("{}_new_name", my_temp_file.basename);
    let res = my_local_file.set_display_name(&new_name, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let file_renamed = res.unwrap();
    assert_eq!(
        file_renamed.path(),
        Some(PathBuf::from(format!("{}_new_name", my_temp_file.path)))
    );

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::set_display_name`
    let res = File::for_path(format!("{}_new_name", my_temp_file.path))
        .set_display_name(&my_temp_file.basename, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // filename should be the original one
    assert_eq!(res.unwrap().path(), Some(PathBuf::from(&my_temp_file.path)));
}

#[test]
fn file_query_settable_attributes() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("query_settable_attributes_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::query_settable_attributes`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.query_settable_attributes(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let file_attr_infos = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_settable_attributes`
    let res = File::for_path(&my_temp_file.path).query_settable_attributes(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected = res.unwrap();

    // both file attribute info lists should have the same attributes
    for (expected, my_local_file_attr_info) in expected
        .attributes()
        .iter()
        .zip(file_attr_infos.attributes().iter())
    {
        assert_eq!(my_local_file_attr_info.name(), expected.name());
    }
}

#[test]
fn file_query_writable_namespaces() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("query_writable_namespaces_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::query_writable_namespaces`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.query_writable_namespaces(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let file_attr_infos = res.unwrap();

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_writable_namespaces`
    let res = File::for_path(&my_temp_file.path).query_writable_namespaces(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let expected = res.unwrap();

    // both file attribute info lists should have the same attributes
    for (expected, my_local_file_attr_info) in expected
        .attributes()
        .iter()
        .zip(file_attr_infos.attributes().iter())
    {
        assert_eq!(my_local_file_attr_info.name(), expected.name());
    }
}

#[test]
fn file_set_attribute() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("set_attribute_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::set_attribute`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    #[cfg(unix)]
    {
        let res = my_local_file.set_attribute(
            "xattr::my_string",
            "value",
            FileQueryInfoFlags::NONE,
            Cancellable::NONE,
        );
        assert!(res.is_ok(), "{}", res.err().unwrap());
    }

    let res = my_local_file.set_attribute(
        "time::access",
        1u64,
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());

    #[cfg(unix)]
    {
        let res = my_local_file.set_attribute(
            "time::access-nsec",
            1u32,
            FileQueryInfoFlags::NONE,
            Cancellable::NONE,
        );
        assert!(res.is_ok(), "{}", res.err().unwrap());
    }

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_info`
    let res = File::for_path(&my_temp_file.path).query_info(
        "time::access,time::modified,time::access-nsec,xattr::my_string",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let file_info = res.unwrap();

    // attributes should be the expected ones
    #[cfg(unix)]
    {
        let my_string = file_info.attribute_string("xattr::my_string");
        assert_eq!(my_string.as_ref().map(glib::GString::as_str), Some("value"));
    }

    let time_access = file_info.attribute_uint64("time::access");
    assert_eq!(time_access, 1);

    #[cfg(unix)]
    {
        let time_access_nsec = file_info.attribute_uint32("time::access-nsec");
        assert_eq!(time_access_nsec, 1);
    }
}

#[test]
fn file_set_attributes_from_info() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("set_attributes_from_info_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::set_attributes_from_info`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.query_info("time::access", FileQueryInfoFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let my_local_file_info = res.unwrap();
    my_local_file_info.set_attribute_uint64("time::access", 1);
    let res = my_local_file.set_attributes_from_info(
        &my_local_file_info,
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_info`
    let res = File::for_path(&my_temp_file.path).query_info(
        "time::access",
        FileQueryInfoFlags::NONE,
        Cancellable::NONE,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let time_access = res.unwrap().attribute_uint64("time::access");

    // time access should be 1
    assert_eq!(time_access, 1);
}

#[test]
fn file_read_fn() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, my_temp_file_io_stream) = Temp::create_file("read_fn_XXXXXX");
    {
        // temporary output stream is closed when the variable output go out of scope
        let output = my_temp_file_io_stream;
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
    }

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::read`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.capacity());
    let res = input.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let mut expected = vec![0; b"foo".len()];
    let res = File::for_path(&my_temp_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let res = input.read_all(&mut expected, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, expected.capacity());
    let res = input.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // both read contents should equal
    assert_eq!(buffer, expected);
}

#[test]
fn file_append_to() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("append_to_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::append_to`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.append_to(FileCreateFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let output = res.unwrap();
    let res = output.write(b"foo", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"foo".len());
    let res = output.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let res = File::for_path(&my_temp_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());

    // read and appended contents should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_create() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("create_XXXXXX");
    // delete temporary file so that we can recreate it
    let res = my_temp_file.as_ref().unwrap().delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::create`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.create(FileCreateFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let output = res.unwrap();
    let res = output.write(b"foo", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"foo".len());
    let res = output.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let res = File::for_path(&my_temp_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());

    // read and created contents should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_replace() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, my_temp_file_io_stream) = Temp::create_file("replace_XXXXXX");
    {
        // temporary output stream is closed when the variable output go out of scope
        let output = my_temp_file_io_stream;
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
    }

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::replace`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.replace(None, false, FileCreateFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let output = res.unwrap();
    let res = output.write(b"bar", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"bar".len());
    let res = output.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let res = File::for_path(&my_temp_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"bar".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());

    // read and replaced contents should equal
    assert_eq!(buffer, Vec::from(b"bar"));
}

#[test]
fn file_delete() {
    // temporary file should be deleted when the variable goes out of scope...
    let (mut my_temp_file, _) = Temp::create_file("delete_XXXXXX");
    // ... but we consume its file so that it won't be deleted when the variable goes out of scope
    let _ = my_temp_file.take_file();

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::delete`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_exists`
    let exist = File::for_path(&my_temp_file.path).query_exists(Cancellable::NONE);

    // file should not exist
    assert!(!exist);
}

#[test]
fn file_trash() {
    // temporary file should be deleted when the variable goes out of scope...
    let (mut my_temp_file, _) = Temp::create_file("trash_XXXXXX");
    // ... but we consume its file so that it won't be deleted when the variable goes out of scope
    let _ = my_temp_file.take_file();

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::trash`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.trash(Cancellable::NONE);
    assert!(
        res.is_ok()
            || res.as_ref().is_err_and(|err| err
                .kind::<IOErrorEnum>()
                .is_some_and(|err| err == IOErrorEnum::NotSupported)),
        "{}",
        res.err().unwrap()
    );
    if res.is_ok() {
        // continue test only if trashing on system is supported
        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_exists`
        let my_file = File::for_path(&my_temp_file.path);
        let exist = my_file.query_exists(Cancellable::NONE);

        // file should not exist
        assert!(!exist);
    }
}

#[test]
fn file_make_directory() {
    // temporary directory is deleted when the variable goes out of scope
    let my_temp_dir = Temp::make_dir("make_directory_XXXXXX");
    // delete temporary directory so that we can recreate it
    let res = my_temp_dir.as_ref().unwrap().delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::make_directory`
    let my_local_dir = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_dir.path)
        .build();
    let res = my_local_dir.make_directory(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_exists`
    let exist = File::for_path(&my_temp_dir.path).query_exists(Cancellable::NONE);

    // file should exist
    assert!(exist);
}

#[test]
fn file_make_symbolic_link() {
    // temporary symbolic link is deleted when the variable goes out of scope
    let (my_temp_symbolic_link_target, _) = Temp::create_file("make_symbolic_link_target_XXXXXX");
    let (mut my_temp_symbolic_link, _) = Temp::create_file("make_symbolic_link_XXXXXX");
    // delete temporary file so that we can recreate it
    let res = my_temp_symbolic_link
        .as_ref()
        .unwrap()
        .delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::make_symbolic_link`
    let my_local_symbolic_link = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_symbolic_link.path)
        .build();
    let res = my_local_symbolic_link
        .make_symbolic_link(&my_temp_symbolic_link_target.path, Cancellable::NONE);
    assert!(
        res.is_ok()
            || res.as_ref().is_err_and(|err| err
                .kind::<IOErrorEnum>()
                .is_some_and(|err| err == IOErrorEnum::NotSupported)),
        "{}",
        res.err().unwrap()
    );
    if res.is_err() {
        // if operation is not supported, temporary symbolic link is not created,
        // so we consume its file so that it won't be deleted when the variable goes out of scope
        my_temp_symbolic_link.take_file();
    }

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_exists`
    let exist = File::for_path(&my_temp_symbolic_link.path).query_exists(Cancellable::NONE);

    // file should exist (or not) according to the operation result
    assert_eq!(res.is_ok(), exist);
}

#[test]
fn file_copy() {
    // temporary source file is deleted when the variable goes out of scope
    let (my_temp_source_file, my_source_file_io_stream) = Temp::create_file("copy_XXXXXX");
    {
        // temporary output stream is closed when the variable output go out of scope
        let output = my_source_file_io_stream;
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
    }
    // temporary destination file is deleted when the variable goes out of scope
    let (my_temp_destination_file, _) = Temp::create_file("copy_XXXXXX");
    // delete temporary destination file so that we can recreate it
    let res = my_temp_destination_file
        .as_ref()
        .unwrap()
        .delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::copy`
    let my_local_source_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_source_file.path)
        .build();
    let res = my_local_source_file.copy(
        &File::for_path(&my_temp_destination_file.path),
        FileCopyFlags::NONE,
        Cancellable::NONE,
        None,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let res = File::for_path(&my_temp_destination_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());

    // read and copied content should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_move_() {
    // temporary source file should be deleted when the variable goes out of scope...
    let (mut my_temp_source_file, my_temp_source_file_io_stream) = Temp::create_file("move_XXXXXX");
    // ... but we consume its file so that it won't be deleted when the variable goes out of scope
    let _ = my_temp_source_file.take_file();
    {
        // temporary output stream is closed when the variable output go out of scope
        let output = my_temp_source_file_io_stream;
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
    }
    // temporary destination file is deleted when the variable goes out of scope
    let (my_temp_destination_file, _) = Temp::create_file("move_XXXXXX");
    // delete temporary destination file so that we can recreate it by renaming the temporary source file
    let res = my_temp_destination_file
        .as_ref()
        .unwrap()
        .delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::move_`
    let my_local_source_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_source_file.path)
        .build();
    let res = my_local_source_file.move_(
        &File::for_path(&my_temp_destination_file.path),
        FileCopyFlags::NONE,
        Cancellable::NONE,
        None,
    );
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::query_exists`
    let exist = File::for_path(&my_temp_source_file.path).query_exists(Cancellable::NONE);
    // source file should not exist
    assert!(!exist);
    let exist = File::for_path(&my_temp_destination_file.path).query_exists(Cancellable::NONE);
    // destination file should exist
    assert!(exist);

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::read`
    let res = File::for_path(&my_temp_destination_file.path).read(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let input = res.unwrap();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());

    // read and moved content should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_mount_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("mount_mountable_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::mount_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.mount_mountable(
                MountMountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::mount_mountable`
        let (tc, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).mount_mountable(
                MountMountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tc.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

// checker-ignore-item
#[test]
fn file_unmount_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("unmount_mountable_XXXXXX");

        // implement the deprecated function unmount_mountable which is useful for this test
        fn unmount_mountable<T: IsA<File>, P: FnOnce(Result<(), glib::Error>) + 'static>(
            file: &T,
            flags: MountUnmountFlags,
            cancellable: Option<&impl IsA<Cancellable>>,
            callback: P,
        ) {
            use glib::translate::{from_glib_full, IntoGlib};
            use std::boxed::Box as Box_;
            let main_context = glib::MainContext::ref_thread_default();
            let is_main_context_owner = main_context.is_owner();
            let has_acquired_main_context = (!is_main_context_owner)
                .then(|| main_context.acquire().ok())
                .flatten();
            assert!(
                is_main_context_owner || has_acquired_main_context.is_some(),
                "Async operations only allowed if the thread is owning the MainContext"
            );

            let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::new(glib::thread_guard::ThreadGuard::new(callback));
            unsafe extern "C" fn unmount_mountable_trampoline<
                P: FnOnce(Result<(), glib::Error>) + 'static,
            >(
                _source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) {
                let mut error = std::ptr::null_mut();
                gio::ffi::g_file_unmount_mountable_finish(
                    _source_object as *mut _,
                    res,
                    &mut error,
                );
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                    Box_::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
            let callback = unmount_mountable_trampoline::<P>;
            unsafe {
                gio::ffi::g_file_unmount_mountable(
                    file.as_ref().to_glib_none().0,
                    flags.into_glib(),
                    cancellable.map(|p| p.as_ref()).to_glib_none().0,
                    Some(callback),
                    Box_::into_raw(user_data) as *mut _,
                );
            }
        }

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::unmount_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            unmount_mountable(
                &my_local_dir,
                MountUnmountFlags::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::unmount_mountable`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            unmount_mountable(
                &File::for_path(&my_temp_dir.path),
                MountUnmountFlags::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

// checker-ignore-item
#[test]
fn file_eject_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("eject_mountable_XXXXXX");

        // implement the deprecated function eject_mountable which is useful for this test
        fn eject_mountable<T: IsA<File>, P: FnOnce(Result<(), glib::Error>) + 'static>(
            file: &T,
            flags: MountUnmountFlags,
            cancellable: Option<&impl IsA<Cancellable>>,
            callback: P,
        ) {
            use glib::translate::{from_glib_full, IntoGlib};
            use std::boxed::Box as Box_;
            let main_context = glib::MainContext::ref_thread_default();
            let is_main_context_owner = main_context.is_owner();
            let has_acquired_main_context = (!is_main_context_owner)
                .then(|| main_context.acquire().ok())
                .flatten();
            assert!(
                is_main_context_owner || has_acquired_main_context.is_some(),
                "Async operations only allowed if the thread is owning the MainContext"
            );

            let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::new(glib::thread_guard::ThreadGuard::new(callback));
            unsafe extern "C" fn eject_mountable_trampoline<
                P: FnOnce(Result<(), glib::Error>) + 'static,
            >(
                _source_object: *mut glib::gobject_ffi::GObject,
                res: *mut gio::ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) {
                let mut error = std::ptr::null_mut();
                gio::ffi::g_file_eject_mountable_finish(_source_object as *mut _, res, &mut error);
                let result = if error.is_null() {
                    Ok(())
                } else {
                    Err(from_glib_full(error))
                };
                let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                    Box_::from_raw(user_data as *mut _);
                let callback: P = callback.into_inner();
                callback(result);
            }
            let callback = eject_mountable_trampoline::<P>;
            unsafe {
                gio::ffi::g_file_eject_mountable(
                    file.as_ref().to_glib_none().0,
                    flags.into_glib(),
                    cancellable.map(|p| p.as_ref()).to_glib_none().0,
                    Some(callback),
                    Box_::into_raw(user_data) as *mut _,
                );
            }
        }

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::eject_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            eject_mountable(
                &my_local_dir,
                MountUnmountFlags::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::eject_mountable`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            eject_mountable(
                &File::for_path(&my_temp_dir.path),
                MountUnmountFlags::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_mount_enclosing_volume() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("mount_enclosing_volume_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::mount_enclosing_volume`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.mount_enclosing_volume(
                MountMountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });
        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::mount_enclosing_volume`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).mount_enclosing_volume(
                MountMountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_monitor_dir() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("monitor_dir_XXXXXX");

        // utility function to create a directory monitor and a channel receiver
        fn monitor_dir<T: IsA<File>>(
            directory: &T,
        ) -> (
            FileMonitor,
            async_channel::Receiver<(FileMonitorEvent, glib::GString, Option<glib::GString>)>,
        ) {
            let res = directory.monitor_directory(FileMonitorFlags::NONE, Cancellable::NONE);
            assert!(res.is_ok(), "{}", res.err().unwrap());
            let monitor = res.unwrap();
            let rx = {
                let (tx, rx) = async_channel::unbounded();
                monitor.connect_changed(
                    move |_: &FileMonitor,
                          file: &File,
                          other_file: Option<&File>,
                          event_type: FileMonitorEvent| {
                        let res = glib::MainContext::ref_thread_default().block_on(tx.send((
                            event_type,
                            file.parse_name(),
                            other_file.map(File::parse_name),
                        )));
                        assert!(res.is_ok(), "{}", res.err().unwrap());
                        if event_type == FileMonitorEvent::Deleted {
                            tx.close();
                        }
                    },
                );
                rx
            };
            (monitor, rx)
        }

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::monitor_dir`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (monitor, rx) = monitor_dir(&my_local_dir);

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::monitor_dir`
        let my_dir = File::for_path(&my_temp_dir.path);
        let (expected_monitor, expected_rx) = monitor_dir(&my_dir);

        {
            // temporary file is deleted when the variable goes out of scope
            let _my_temp_file1 = my_temp_dir.create_file_child("my_file_1_XXXXXX");
        }

        glib::MainContext::ref_thread_default().block_on(async {
            let mut n = 0;
            // for each expected event (from monitor returned by ffi::GLocalFile's implementation)
            while let Ok(expected) = expected_rx.recv().await {
                n += 1;
                // get next event from monitor returned by MyLocalFile's implementation
                let res = rx.recv().await;
                assert!(res.is_ok(), "{}", res.err().unwrap());
                let event = res.unwrap();

                // both events should equal
                assert_eq!(event, expected);
            }

            // at least 1 file event expected
            assert!(n > 0, "at least 1 file event is expected");
        });

        // cancel both monitors
        assert!(monitor.cancel());
        assert!(expected_monitor.cancel());
    });
}

#[test]
fn file_monitor_file() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary file is deleted when variables go out of scope
        let (my_temp_file, _) = Temp::create_file("monitor_file_XXXXXX");

        // utility function to create a file monitor and a channel receiver
        fn monitor_file<T: IsA<File>>(
            file: &T,
        ) -> (
            FileMonitor,
            async_channel::Receiver<(FileMonitorEvent, glib::GString, Option<glib::GString>)>,
        ) {
            let res = file.monitor_file(FileMonitorFlags::NONE, Cancellable::NONE);
            assert!(res.is_ok(), "{}", res.err().unwrap());
            let monitor = res.unwrap();
            let rx = {
                let (tx, rx) = async_channel::unbounded();
                monitor.connect_changed(
                    move |_: &FileMonitor,
                          file: &File,
                          other_file: Option<&File>,
                          event_type: FileMonitorEvent| {
                        let res = glib::MainContext::ref_thread_default().block_on(tx.send((
                            event_type,
                            file.parse_name(),
                            other_file.map(File::parse_name),
                        )));
                        assert!(res.is_ok(), "{}", res.err().unwrap());
                        if event_type == FileMonitorEvent::ChangesDoneHint {
                            tx.close();
                        }
                    },
                );
                rx
            };
            (monitor, rx)
        }

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::monitor_file`
        let my_local_file = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_file.path)
            .build();
        let (monitor, rx) = monitor_file(&my_local_file);

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::monitor_file`
        let my_file = File::for_path(&my_temp_file.path);
        let (expected_monitor, expected_rx) = monitor_file(&my_file);

        // modify the file
        let res = my_local_file.append_to(FileCreateFlags::NONE, Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let output = res.unwrap();
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
        let res = output.close(Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());

        glib::MainContext::ref_thread_default().block_on(async {
            let mut n = 0;
            // for each expected event (from monitor returned by ffi::GLocalFile's implementation)
            while let Ok(expected) = expected_rx.recv().await {
                n += 1;
                // get next event from monitor returned by MyLocalFile's implementation
                let res = rx.recv().await;
                assert!(res.is_ok(), "{}", res.err().unwrap());
                let event = res.unwrap();

                // both event should equal
                assert_eq!(event, expected);
            }

            // at least 1 file event expected
            assert!(n > 0, "at least 1 file event is expected");
        });

        // cancel both monitors
        assert!(monitor.cancel());
        assert!(expected_monitor.cancel());
    });
}

#[test]
fn file_open_readwrite() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("open_readwrite_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::open_readwrite`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.open_readwrite(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let output = io.output_stream();
    let res = output.write(b"foo", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"foo".len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::open_readwrite`
    let res = File::for_path(&my_temp_file.path).open_readwrite(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let input = io.input_stream();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // read and written contents should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_create_readwrite() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, _) = Temp::create_file("create_readwrite_XXXXXX");
    // delete temporary file so that we can recreate it
    let res = my_temp_file.as_ref().unwrap().delete(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::create_readwrite`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res = my_local_file.create_readwrite(FileCreateFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let output = io.output_stream();
    let res = output.write(b"foo", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"foo".len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::open_readwrite`
    let res = File::for_path(&my_temp_file.path).open_readwrite(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let input = io.input_stream();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // read and written contents should equal
    assert_eq!(buffer, Vec::from(b"foo"));
}

#[test]
fn file_replace_readwrite() {
    // temporary file is deleted when the variable goes out of scope
    let (my_temp_file, my_temp_file_io_stream) = Temp::create_file("replace_readwrite_XXXXXX");
    {
        // temporary output stream is closed when the variable output goes out of scope
        let output = my_temp_file_io_stream;
        let res = output.write(b"foo", Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        assert_eq!(res.unwrap() as usize, b"foo".len());
    }

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::replace_readwrite`
    let my_local_file = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_file.path)
        .build();
    let res =
        my_local_file.replace_readwrite(None, false, FileCreateFlags::NONE, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let output = io.output_stream();
    let res = output.write(b"bar", Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap() as usize, b"bar".len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::open_readwrite`
    let res = File::for_path(&my_temp_file.path).open_readwrite(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    let io = res.unwrap();
    let input = io.input_stream();
    let mut buffer = vec![0; b"foo".len()];
    let res = input.read_all(&mut buffer, Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());
    assert_eq!(res.unwrap().0, buffer.len());
    let res = io.close(Cancellable::NONE);
    assert!(res.is_ok(), "{}", res.err().unwrap());

    // read and replaced contents should equal
    assert_eq!(buffer, Vec::from(b"bar"));
}

#[test]
fn file_start_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("start_mountable_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::start_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.start_mountable(
                DriveStartFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::start_mountable`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).start_mountable(
                DriveStartFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_stop_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("stop_mountable_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::stop_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.stop_mountable(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::stop_mountable`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).stop_mountable(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_unmount_mountable_with_operation() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("unmount_mountable_with_operation_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::unmount_mountable_with_operation`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.unmount_mountable_with_operation(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::unmount_mountable_with_operation`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).unmount_mountable_with_operation(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_eject_mountable_with_operation() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("eject_mountable_with_operation_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::eject_mountable_with_operation`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.eject_mountable_with_operation(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::eject_mountable_with_operation`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path).eject_mountable_with_operation(
                MountUnmountFlags::NONE,
                MountOperation::NONE,
                Cancellable::NONE,
                move |res| tx.send(res).unwrap(),
            );
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_poll_mountable() {
    // run test in a main context dedicated and configured as the thread default one
    let _ = glib::MainContext::new().with_thread_default(|| {
        // temporary dir is deleted when the variable goes out of scope
        let my_temp_dir = Temp::make_dir("poll_mountable_XXXXXX");

        // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::poll_mountable`
        let my_local_dir = glib::Object::builder::<MyLocalFile>()
            .property("path", &my_temp_dir.path)
            .build();
        let (tx, rx) = oneshot::channel();
        let res = glib::MainContext::ref_thread_default().block_on(async {
            my_local_dir.poll_mountable(Cancellable::NONE, move |res| tx.send(res).unwrap());
            rx.await.unwrap()
        });

        // invoke `LocalFile` implementation of `gio::ffi::GFileIface::poll_mountable`
        let (tx, rx) = oneshot::channel();
        let expected = glib::MainContext::ref_thread_default().block_on(async {
            File::for_path(&my_temp_dir.path)
                .poll_mountable(Cancellable::NONE, move |res| tx.send(res).unwrap());
            rx.await.unwrap()
        });

        // both results should equal
        assert_eq!(
            res.map_err(|err| err.kind::<IOErrorEnum>()),
            expected.map_err(|err| err.kind::<IOErrorEnum>())
        );
    });
}

#[test]
fn file_measure_disk_usage() {
    // temporary dir is deleted when the variable goes out of scope
    let my_temp_dir = Temp::make_dir("measure_disk_usage_XXXXXX");

    // invoke `MyLocalFile` implementation of `gio::ffi::GFileIface::measure_disk_usage`
    let my_local_dir = glib::Object::builder::<MyLocalFile>()
        .property("path", &my_temp_dir.path)
        .build();
    let res = my_local_dir.measure_disk_usage(FileMeasureFlags::all(), Cancellable::NONE, None);

    // invoke `LocalFile` implementation of `gio::ffi::GFileIface::measure_disk_usage`
    let expected = File::for_path(&my_temp_dir.path).measure_disk_usage(
        FileMeasureFlags::all(),
        Cancellable::NONE,
        None,
    );

    // both results should equal
    assert_eq!(
        res.map_err(|err| err.kind::<IOErrorEnum>()),
        expected.map_err(|err| err.kind::<IOErrorEnum>())
    );
}
