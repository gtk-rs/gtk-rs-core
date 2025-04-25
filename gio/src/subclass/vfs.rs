// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*, GString};

use libc::c_char;

use crate::{ffi, File, Vfs};

// Support custom implementation of virtual functions defined in `gio::ffi::GVfsClass`.
pub trait VfsImpl: ObjectImpl + ObjectSubclass<Type: IsA<Vfs>> {
    fn is_active(&self) -> bool {
        self.parent_is_active()
    }

    fn get_file_for_path(&self, path: &str) -> File {
        self.parent_get_file_for_path(path)
    }

    fn get_file_for_uri(&self, uri: &str) -> File {
        self.parent_get_file_for_uri(uri)
    }

    fn get_supported_uri_schemes(&self) -> Vec<String> {
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

    fn parent_get_file_for_path(&self, path: &str) -> File {
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

    fn parent_get_supported_uri_schemes(&self) -> Vec<String> {
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

    let file = imp.get_file_for_path(&GString::from_glib_borrow(path));

    file.to_glib_full()
}

unsafe extern "C" fn get_file_for_uri<T: VfsImpl>(
    vfs: *mut ffi::GVfs,
    uri: *const c_char,
) -> *mut ffi::GFile {
    let instance = &*(vfs as *mut T::Instance);
    let imp = instance.imp();

    let file = imp.get_file_for_uri(&GString::from_glib_borrow(uri));

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

    let file = imp.parse_name(&GString::from_glib_borrow(parse_name));

    file.to_glib_full()
}
