// Take a look at the license at the top of the repository in the LICENSE file.

use gio::Cancellable;
use gio::File;
use gio::FileEnumerator;
use gio::FileQueryInfoFlags;
use gio::prelude::*;
use gio::subclass::prelude::*;
use glib::Error;
use glib::Object;
use glib::g_debug;

// Define `MyFileEnumerator` as an implementation of `FileEnumerator`.
pub mod imp {
    use std::{path::PathBuf, sync::OnceLock};

    use glib::Properties;

    use crate::update_file_info;

    use super::*;

    // #[derive(Default)]
    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::MyFileEnumerator)]
    pub struct MyFileEnumerator {
        #[property(get, set)]
        virtual_path: OnceLock<PathBuf>,
        #[property(get, set)]
        local_file_enumerator: OnceLock<FileEnumerator>,
    }

    impl MyFileEnumerator {
        fn local_file_enumerator(&self) -> &FileEnumerator {
            self.local_file_enumerator.get().unwrap()
        }
    }

    #[glib::object_subclass]
    #[object_subclass_dynamic(lazy_registration = true)]
    impl ObjectSubclass for MyFileEnumerator {
        const NAME: &'static str = "MyFileEnumerator";
        type Type = super::MyFileEnumerator;
        type ParentType = FileEnumerator;
    }

    #[glib::derived_properties]
    impl ObjectImpl for MyFileEnumerator {}

    impl FileEnumeratorImpl for MyFileEnumerator {
        fn next_file(
            &self,
            cancellable: Option<&gio::Cancellable>,
        ) -> Result<Option<gio::FileInfo>, glib::Error> {
            if let Some(info) = self.local_file_enumerator().next_file(cancellable)? {
                update_file_info(&info);
                Ok(Some(info))
            } else {
                Ok(None)
            }
        }

        fn close(&self, cancellable: Option<&gio::Cancellable>) -> (bool, Option<glib::Error>) {
            self.local_file_enumerator().close(cancellable)
        }
    }
}

glib::wrapper! {
    pub struct MyFileEnumerator(ObjectSubclass<imp::MyFileEnumerator>) @extends FileEnumerator;
}

impl MyFileEnumerator {
    pub fn new(
        local_file: &File,
        attributes: &str,
        flags: FileQueryInfoFlags,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Self, Error> {
        g_debug!(
            "MyVfs",
            "MyFileEnumerator::new({:?},{},{:?},{:?})",
            local_file,
            attributes,
            flags,
            cancellable.map(|_| "_")
        );
        let local_file_enumerator =
            local_file.enumerate_children(attributes, flags, cancellable)?;
        Ok(Object::builder()
            .property("local_file_enumerator", local_file_enumerator)
            .build())
    }
}
