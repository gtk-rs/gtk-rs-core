// Take a look at the license at the top of the repository in the LICENSE file.

use gio::{File, Vfs, prelude::*, subclass::prelude::*};
use glib::g_debug;

use crate::SCHEME;

// Define `MyVfs` as a subclass of `Vfs`.
pub mod imp {
    use std::{path::PathBuf, sync::LazyLock};

    use glib::{StrVRef, object::Cast};

    use crate::{file::MyFile, resolve_local_path};

    use super::*;

    #[derive(Default, Debug)]
    pub struct MyVfs;

    #[glib::object_subclass]
    #[object_subclass_dynamic(lazy_registration = true)]
    impl ObjectSubclass for MyVfs {
        const NAME: &'static str = "MyVfs";
        type Type = super::MyVfs;
        type ParentType = Vfs;
    }

    impl ObjectImpl for MyVfs {}

    impl VfsImpl for MyVfs {
        fn is_active(&self) -> bool {
            true
        }

        fn get_file_for_path(&self, path: &std::path::Path) -> File {
            g_debug!("MyVfs", "MyVfs::get_file_for_path({:?},{:?})", self, path);
            Vfs::local().file_for_path(path)
        }

        fn get_file_for_uri(&self, uri: &str) -> File {
            g_debug!("MyVfs", "MyVfs::get_file_for_uri({:?},{})", self, uri);
            if let Some(path) = uri.strip_prefix(&format!("{SCHEME}://")) {
                MyFile::new(
                    PathBuf::from(path),
                    Vfs::local().file_for_path(resolve_local_path(path)),
                )
                .upcast()
            } else {
                Vfs::local().file_for_uri(uri)
            }
        }

        fn get_supported_uri_schemes(&self) -> &'static StrVRef {
            g_debug!("MyVfs", "MyVfs::get_supported_uri_schemes({:?})", self);
            static SUPPORTED_URI_SCHEMES: LazyLock<glib::StrV> = LazyLock::new(|| {
                let mut schemes: glib::StrV = Vfs::local().supported_uri_schemes().into();
                schemes.push(SCHEME.into());
                schemes
            });
            &SUPPORTED_URI_SCHEMES
        }

        fn parse_name(&self, parse_name: &str) -> File {
            g_debug!("MyVfs", "MyVfs::parse_name({:?},{})", self, parse_name);
            if let Some(path) = parse_name.strip_prefix(&format!("{SCHEME}://")) {
                MyFile::new(
                    PathBuf::from(path),
                    Vfs::local().parse_name(&resolve_local_path(path)),
                )
                .upcast()
            } else {
                Vfs::local().parse_name(parse_name)
            }
        }
    }
}

glib::wrapper! {
    pub struct MyVfs(ObjectSubclass<imp::MyVfs>) @extends Vfs;
}
