// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ffi, FileMonitor};

// Support custom implementation of virtual functions defined in `gio::ffi::GFileMonitorClass`.
pub trait FileMonitorImpl: ObjectImpl + ObjectSubclass<Type: IsA<FileMonitor>> {
    fn cancel(&self) -> bool {
        let res = self.parent_cancel();
        // cancel should always return true as specified in documentation
        // https://docs.gtk.org/gio/vfunc.FileMonitor.cancel.html
        debug_assert!(res, "FileMonitor.cancel should always return true");
        res
    }
}

// Support parent implementation of virtual functions defined in `gio::ffi::GFileMonitorClass`.
pub trait FileMonitorImplExt: FileMonitorImpl {
    fn parent_cancel(&self) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileMonitorClass;

            let f = (*parent_class)
                .cancel
                .expect("No parent class implementation for \"cancel\"");

            let res = f(self.obj().unsafe_cast_ref::<FileMonitor>().to_glib_none().0);
            from_glib(res)
        }
    }
}

impl<T: FileMonitorImpl> FileMonitorImplExt for T {}

// Implement virtual functions defined in `gio::ffi::GFileMonitorClass`.
unsafe impl<T: FileMonitorImpl> IsSubclassable<T> for FileMonitor {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.cancel = Some(cancel::<T>);
    }
}

unsafe extern "C" fn cancel<T: FileMonitorImpl>(
    monitor: *mut ffi::GFileMonitor,
) -> glib::ffi::gboolean {
    let instance = &*(monitor as *mut T::Instance);
    let imp = instance.imp();

    let res = imp.cancel();

    res.into_glib()
}
