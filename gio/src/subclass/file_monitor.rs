// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{ffi, FileMonitor};

pub trait FileMonitorImpl: ObjectImpl + ObjectSubclass<Type: IsA<FileMonitor>> {
    fn cancel(&self) -> bool {
        self.parent_cancel()
    }
}

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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;
    use crate::{prelude::*, File, FileMonitorEvent};

    mod imp {
        use super::*;

        #[derive(Default)]
        pub struct MyFileMonitor {
            pub index: RefCell<i8>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MyFileMonitor {
            const NAME: &'static str = "MyFileMonitor";
            type Type = super::MyFileMonitor;
            type ParentType = FileMonitor;
        }

        impl ObjectImpl for MyFileMonitor {}

        impl FileMonitorImpl for MyFileMonitor {
            fn cancel(&self) -> bool {
                self.index.replace(-1);
                true
            }
        }
    }

    glib::wrapper! {
        pub struct MyFileMonitor(ObjectSubclass<imp::MyFileMonitor>) @extends FileMonitor;
    }

    impl MyFileMonitor {
        pub async fn tick(&self) {
            glib::timeout_future(std::time::Duration::from_millis(10)).await;
            let mut i = *(self.imp().index.borrow());
            while i != -1 {
                let (child, other_file, event_type) = match i % 3 {
                    0 => (
                        File::for_parse_name(&format!("file{}", i)),
                        None,
                        FileMonitorEvent::Created,
                    ),
                    1 => (
                        File::for_parse_name(&format!("file{}", i - 1)),
                        Some(File::for_parse_name(&format!("file{}", i))),
                        FileMonitorEvent::Renamed,
                    ),
                    2 => (
                        File::for_parse_name(&format!("file{}", i)),
                        None,
                        FileMonitorEvent::Deleted,
                    ),
                    _ => unimplemented!("cannot occur"),
                };
                self.emit_event(&child, other_file.as_ref(), event_type);
                glib::timeout_future(std::time::Duration::from_millis(10)).await;
                i = *(self.imp().index.borrow());
                if i != -1 {
                    i += 1;
                    self.imp().index.replace(i);
                }
            }
        }
    }

    #[test]
    fn test_cancel() {
        let file_monitor = glib::Object::new::<MyFileMonitor>();
        let index = RefCell::new(0i8);
        file_monitor.connect_changed(
            move |self_: &MyFileMonitor,
                  child: &File,
                  other_file: Option<&File>,
                  event_type: FileMonitorEvent| {
                let i = *(index.borrow());
                let (expected_child, expected_other_file, expected_event_type) = match i % 3 {
                    0 => (
                        File::for_parse_name(&format!("file{}", i)),
                        None,
                        FileMonitorEvent::Created,
                    ),
                    1 => (
                        File::for_parse_name(&format!("file{}", i - 1)),
                        Some(File::for_parse_name(&format!("file{}", i))),
                        FileMonitorEvent::Renamed,
                    ),
                    2 => (
                        File::for_parse_name(&format!("file{}", i)),
                        None,
                        FileMonitorEvent::Deleted,
                    ),
                    _ => unimplemented!("cannot occur"),
                };
                assert_eq!(child.path(), expected_child.path());
                assert_eq!(
                    other_file.and_then(FileExt::path),
                    expected_other_file.as_ref().and_then(FileExt::path)
                );
                assert_eq!(event_type, expected_event_type);
                index.replace(i + 1);
                if i >= 8 {
                    self_.cancel();
                }
            },
        );

        glib::MainContext::new().block_on(file_monitor.tick())
    }
}
