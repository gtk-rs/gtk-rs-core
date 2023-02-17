// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{subclass::prelude::*, File, FileMonitor, FileMonitorEvent};
use glib::{prelude::*, translate::*};

pub trait FileMonitorImpl: ObjectImpl {
    fn changed(&self, file: &File, other_file: &File, event_type: FileMonitorEvent) {
        self.parent_changed(file, other_file, event_type)
    }
    fn cancel(&self) -> bool {
        self.parent_cancel()
    }
}

pub trait FileMonitorImplExt: ObjectSubclass {
    fn parent_changed(&self, file: &File, other_file: &File, event_type: FileMonitorEvent);
    fn parent_cancel(&self) -> bool;
}

impl<T: FileMonitorImpl> FileMonitorImplExt for T {
    fn parent_changed(&self, file: &File, other_file: &File, event_type: FileMonitorEvent) {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GFileMonitorClass;
            if let Some(f) = (*parent_class).changed {
                f(
                    self.obj().unsafe_cast_ref::<FileMonitor>().to_glib_none().0,
                    file.to_glib_none().0,
                    other_file.to_glib_none().0,
                    event_type.into_glib(),
                )
            }
        }
    }

    fn parent_cancel(&self) -> bool {
        unsafe {
            let data = T::type_data();
            let parent_class = data.as_ref().parent_class() as *mut ffi::GFileMonitorClass;
            let f = (*parent_class)
                .cancel
                .expect("No parent class implementation for \"cancel\"");
            from_glib(f(self
                .obj()
                .unsafe_cast_ref::<FileMonitor>()
                .to_glib_none()
                .0))
        }
    }
}

unsafe impl<T: FileMonitorImpl> IsSubclassable<T> for FileMonitor {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.changed = Some(file_monitor_changed::<T>);
        klass.cancel = Some(file_monitor_cancel::<T>);
    }
}

unsafe extern "C" fn file_monitor_changed<T: FileMonitorImpl>(
    ptr: *mut ffi::GFileMonitor,
    file: *mut ffi::GFile,
    other_file: *mut ffi::GFile,
    event: ffi::GFileMonitorEvent,
) {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.changed(
        &from_glib_borrow(file),
        &from_glib_borrow(other_file),
        from_glib(event),
    )
}

unsafe extern "C" fn file_monitor_cancel<T: FileMonitorImpl>(
    ptr: *mut ffi::GFileMonitor,
) -> glib::ffi::gboolean {
    let instance = &*(ptr as *mut T::Instance);
    let imp = instance.imp();

    imp.cancel().into_glib()
}
