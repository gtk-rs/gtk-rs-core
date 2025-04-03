// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*, GString};

use libc::c_char;

use crate::{ffi, File, Vfs};

pub trait VfsImpl: ObjectImpl + ObjectSubclass<Type: IsA<Vfs>> {
    fn is_active(&self) -> bool {
        self.parent_is_active()
    }

    fn get_file_for_path(&self, path: &GString) -> File {
        self.parent_get_file_for_path(path)
    }

    fn get_file_for_uri(&self, uri: &GString) -> File {
        self.parent_get_file_for_uri(uri)
    }

    fn get_supported_uri_schemes(&self) -> Vec<GString> {
        self.parent_get_supported_uri_schemes()
    }

    fn parse_name(&self, parse_name: &GString) -> File {
        self.parent_parse_name(parse_name)
    }
}

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

    fn parent_get_file_for_path(&self, path: &GString) -> File {
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

    fn parent_get_file_for_uri(&self, uri: &GString) -> File {
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

    fn parent_get_supported_uri_schemes(&self) -> Vec<GString> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GVfsClass;

            let f = (*parent_class)
                .get_supported_uri_schemes
                .expect("No parent class implementation for \"get_supported_uri_schemes\"");

            let res = f(self.obj().unsafe_cast_ref::<Vfs>().to_glib_none().0);
            FromGlibPtrContainer::from_glib_none(res)
        }
    }

    fn parent_parse_name(&self, parse_name: &GString) -> File {
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

    let file = imp.get_file_for_path(&from_glib_borrow(path));

    file.to_glib_full()
}

unsafe extern "C" fn get_file_for_uri<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    uri: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.get_file_for_uri(&from_glib_borrow(uri));

    file.to_glib_full()
}

unsafe extern "C" fn get_supported_uri_schemes<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
) -> *const *const c_char {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let supported_uri_schemes = imp.get_supported_uri_schemes();

    supported_uri_schemes.to_glib_full()
}

unsafe extern "C" fn parse_name<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    parse_name: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.parse_name(&from_glib_borrow(parse_name));

    file.to_glib_full()
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::path::PathBuf;

    use glib::object::ObjectSubclassIs;

    use super::*;
    use crate::prelude::*;

    mod imp {
        use super::*;

        #[derive(glib::Properties, Default)]
        #[properties(wrapper_type = super::MyFile)]
        pub struct MyFile {
            #[property(get, set)]
            path: RefCell<Option<std::path::PathBuf>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MyFile {
            const NAME: &'static str = "MyFile";
            const ABSTRACT: bool = false;
            type Type = super::MyFile;
            type Interfaces = (File,);
        }

        #[glib::derived_properties]
        impl ObjectImpl for MyFile {}

        impl MyFile {
            fn uri_scheme(&self) -> Option<glib::GString> {
                Some(GString::from(MyVfs::NAME))
            }

            fn uri(&self) -> glib::GString {
                GString::from(format!(
                    "{}:{}",
                    MyVfs::NAME,
                    self.obj()
                        .path()
                        .unwrap_or(std::path::PathBuf::from("no_name"))
                        .to_str()
                        .unwrap()
                ))
            }
        }

        unsafe impl IsImplementable<MyFile> for File {
            fn interface_init(iface: &mut glib::Interface<Self>) {
                let iface = iface.as_mut();
                iface.get_uri_scheme = Some(my_file_get_uri_scheme);
                iface.get_path = Some(my_file_get_path);
                iface.get_uri = Some(my_file_get_uri);
            }
        }

        unsafe extern "C" fn my_file_get_uri_scheme(file: *mut ffi::GFile) -> *mut c_char {
            let instance = &*(file as *mut <MyFile as ObjectSubclass>::Instance);
            let imp = instance.imp();

            let res = imp.uri_scheme();
            if let Some(uri_scheme) = res {
                uri_scheme.to_glib_full()
            } else {
                std::ptr::null_mut()
            }
        }

        unsafe extern "C" fn my_file_get_path(file: *mut ffi::GFile) -> *mut c_char {
            let instance = &*(file as *mut <MyFile as ObjectSubclass>::Instance);
            let imp = instance.imp();

            let res = imp.obj().path();
            if let Some(path) = res {
                path.to_glib_full()
            } else {
                std::ptr::null_mut()
            }
        }

        unsafe extern "C" fn my_file_get_uri(file: *mut ffi::GFile) -> *mut c_char {
            let instance = &*(file as *mut <MyFile as ObjectSubclass>::Instance);
            let imp = instance.imp();

            let uri = imp.uri();
            uri.to_glib_full()
        }

        #[derive(Default)]
        pub struct MyVfs {
            pub active: RefCell<bool>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MyVfs {
            const NAME: &'static str = "MyVfs";
            type Type = super::MyVfs;
            type ParentType = Vfs;
        }

        impl ObjectImpl for MyVfs {}

        impl VfsImpl for MyVfs {
            fn is_active(&self) -> bool {
                *self.active.borrow()
            }

            fn get_file_for_path(&self, path: &GString) -> File {
                glib::Object::builder::<super::MyFile>()
                    .property("path", Some(std::path::PathBuf::from(&path)))
                    .build()
                    .into()
            }

            fn get_file_for_uri(&self, uri: &GString) -> File {
                if let Some(path) = uri.to_string().strip_prefix(&format!("{}:", Self::NAME)) {
                    self.get_file_for_path(&GString::from(path))
                } else {
                    File::for_uri(uri)
                }
            }

            fn get_supported_uri_schemes(&self) -> Vec<GString> {
                vec![GString::from(Self::NAME)]
            }

            fn parse_name(&self, parse_name: &GString) -> File {
                self.get_file_for_path(parse_name)
            }
        }
    }

    glib::wrapper! {
        pub struct MyFile(ObjectSubclass<imp::MyFile>) @implements File;
    }

    glib::wrapper! {
        pub struct MyVfs(ObjectSubclass<imp::MyVfs>) @extends Vfs;
    }

    impl MyVfs {
        pub fn set_active(&self, active: bool) {
            self.imp().active.replace(active);
        }

        pub fn uri_scheme(&self) -> &'static str {
            <Self as ObjectSubclassIs>::Subclass::NAME
        }
    }

    #[test]
    fn test_is_active() {
        let vfs = glib::Object::new::<MyVfs>();
        vfs.set_active(true);
        assert!(vfs.is_active());
        vfs.set_active(false);
        assert!(!vfs.is_active());
    }

    #[test]
    fn test_get_file_for_path() {
        let vfs = glib::Object::new::<MyVfs>();
        let file = vfs.file_for_path("my_file");
        assert_eq!(file.uri_scheme(), Some(GString::from(vfs.uri_scheme())));
        assert_eq!(file.path(), Some(PathBuf::from("my_file")));
        assert_eq!(
            file.uri(),
            GString::from(format!("{}:my_file", vfs.uri_scheme()))
        );
    }

    #[test]
    fn test_get_file_for_uri() {
        let vfs = glib::Object::new::<MyVfs>();
        let file = vfs.file_for_uri(&format!("{}:my_file", vfs.uri_scheme()));
        assert_eq!(file.uri_scheme(), Some(GString::from(vfs.uri_scheme())));
        assert_eq!(file.path(), Some(PathBuf::from("my_file")));
        assert_eq!(
            file.uri(),
            GString::from(format!("{}:my_file", vfs.uri_scheme()))
        );
    }

    #[test]
    fn test_get_supported_uri_schemes() {
        let vfs = glib::Object::new::<MyVfs>();
        let uri_schemes = vfs.supported_uri_schemes();
        assert!(uri_schemes.contains(&GString::from(vfs.uri_scheme())));
    }

    #[test]
    fn test_parse_name() {
        let vfs = glib::Object::new::<MyVfs>();
        let file = vfs.parse_name("my_file");
        assert_eq!(file.uri_scheme(), Some(GString::from(vfs.uri_scheme())));
        assert_eq!(file.path(), Some(PathBuf::from("my_file")));
        assert_eq!(
            file.uri(),
            GString::from(format!("{}:my_file", vfs.uri_scheme()))
        );
    }
}
