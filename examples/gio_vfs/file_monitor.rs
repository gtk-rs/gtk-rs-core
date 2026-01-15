// Take a look at the license at the top of the repository in the LICENSE file.

use gio::Cancellable;
use gio::File;
use gio::FileMonitor;
use gio::FileMonitorFlags;
use gio::prelude::*;
use gio::subclass::prelude::*;
use glib::Error;
use glib::Object;
use glib::g_debug;

use crate::file::MyFile;
use crate::resolve_local_path;

// Define `MyFileMonitor` as an implementation of `FileMonitor`.
pub mod imp {
    use std::{path::PathBuf, sync::OnceLock};

    use glib::Properties;

    use super::*;

    // #[derive(Default)]
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::MyFileMonitor)]
    pub struct MyFileMonitor {
        #[property(get, set)]
        virtual_path: OnceLock<PathBuf>,
        #[property(get, set)]
        local_file_monitor: OnceLock<FileMonitor>,
    }

    impl MyFileMonitor {
        pub(super) fn local_file_monitor(&self) -> &FileMonitor {
            self.local_file_monitor.get().unwrap()
        }
    }

    #[glib::object_subclass]
    #[object_subclass_dynamic(lazy_registration = true)]
    impl ObjectSubclass for MyFileMonitor {
        const NAME: &'static str = "MyFileMonitor";
        type Type = super::MyFileMonitor;
        type ParentType = FileMonitor;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MyFileMonitor {}

    impl FileMonitorImpl for MyFileMonitor {
        fn cancel(&self) {
            self.local_file_monitor().cancel();
        }
    }
}

glib::wrapper! {
    pub struct MyFileMonitor(ObjectSubclass<imp::MyFileMonitor>) @extends FileMonitor;
}

impl MyFileMonitor {
    pub fn for_directory(
        local_file: &File,
        flags: FileMonitorFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Self, Error> {
        g_debug!(
            "MyVfs",
            "MyFileMonitor::for_directory({:?},{:?},{:?})",
            local_file,
            flags,
            cancellable.map(|_| "_")
        );
        let local_file_monitor = local_file.monitor_directory(flags, cancellable)?;
        Self::new(local_file_monitor)
    }

    pub fn for_file(
        local_file: &File,
        flags: FileMonitorFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Self, Error> {
        g_debug!(
            "MyVfs",
            "MyFileMonitor::for_file({:?},{:?},{:?})",
            local_file,
            flags,
            cancellable.map(|_| "_")
        );
        let local_file_monitor = local_file.monitor_file(flags, cancellable)?;
        Self::new(local_file_monitor)
    }

    pub fn new(local_file_monitor: FileMonitor) -> Result<Self, Error> {
        let file_monitor = Object::builder::<Self>()
            .property("local_file_monitor", local_file_monitor)
            .build();
        file_monitor
            .imp()
            .local_file_monitor()
            .connect_changed(glib::clone!(
                #[weak]
                file_monitor,
                move |_, local_file, other_local_file, event_type| {
                    let file = MyFile::new(
                        resolve_local_path(local_file.path().unwrap().to_string_lossy()).into(),
                        local_file.clone(),
                    );
                    let other_file = other_local_file.map(|local_file| {
                        MyFile::new(
                            resolve_local_path(local_file.path().unwrap().to_string_lossy()).into(),
                            local_file.clone(),
                        )
                    });
                    file_monitor.emit_event(&file, other_file.as_ref(), event_type);
                }
            ));
        Ok(file_monitor)
    }
}
