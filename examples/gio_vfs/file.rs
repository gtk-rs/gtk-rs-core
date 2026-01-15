// Take a look at the license at the top of the repository in the LICENSE file.

use std::path::PathBuf;

use gio::File;
use gio::Vfs;
use gio::prelude::*;
use gio::subclass::prelude::*;
use glib::Object;
use glib::g_debug;

// Define `MyFile` as an implementation of `File`.
pub mod imp {
    use std::{path::PathBuf, sync::OnceLock};

    use gio::{
        Cancellable, FileAttributeInfoList, FileAttributeValue, FileCopyFlags, FileCreateFlags,
        FileEnumerator, FileInfo, FileInputStream, FileMonitor, FileMonitorFlags, FileOutputStream,
        FileQueryInfoFlags, IOErrorEnum,
    };
    use glib::{Error, Properties, translate::ToGlibPtr};

    use crate::{
        file_enumerator::MyFileEnumerator,
        file_monitor::MyFileMonitor,
        {SCHEME, update_file_info},
    };

    use super::*;

    #[derive(Properties, Default, Debug)]
    #[properties(wrapper_type = super::MyFile)]
    pub struct MyFile {
        #[property(get, set)]
        virtual_path: OnceLock<PathBuf>,
        #[property(get, set)]
        local_file: OnceLock<File>,
    }

    impl MyFile {
        fn virtual_path(&self) -> &PathBuf {
            self.virtual_path.get().unwrap()
        }

        fn local_file(&self) -> &File {
            self.local_file.get().unwrap()
        }
    }

    #[glib::object_subclass]
    #[object_subclass_dynamic(lazy_registration = true)]
    impl ObjectSubclass for MyFile {
        const NAME: &'static str = "MyFile";
        type Type = super::MyFile;
        type Interfaces = (File,);
    }

    #[glib::derived_properties]
    impl ObjectImpl for MyFile {}

    impl FileImpl for MyFile {
        fn dup(&self) -> File {
            g_debug!("MyVfs", "MyFile::dup({:?})", self);
            Self::Type::new(self.virtual_path().clone(), self.local_file().clone()).upcast()
        }

        fn hash(&self) -> u32 {
            g_debug!("MyVfs", "MyFile::hash({:?})", self);
            unsafe {
                gio::ffi::g_file_hash(
                    ToGlibPtr::<*const gio::ffi::GFile>::to_glib_none(self.local_file()).0
                        as *const _,
                )
            }
        }

        fn equal(&self, file2: &File) -> bool {
            g_debug!("MyVfs", "MyFile::equal({:?},{:?})", self, file2);
            match file2.downcast_ref::<Self::Type>() {
                Some(file2) => self.local_file().equal(file2),
                None => false,
            }
        }

        fn is_native(&self) -> bool {
            g_debug!("MyVfs", "MyFile::is_native({:?})", self);
            false
        }

        fn has_uri_scheme(&self, uri_scheme: &str) -> bool {
            g_debug!("MyVfs", "MyFile::has_uri_scheme({:?},{})", self, uri_scheme);
            uri_scheme == SCHEME
        }

        fn uri_scheme(&self) -> Option<String> {
            g_debug!("MyVfs", "MyFile::uri_scheme({:?})", self);
            Some(SCHEME.to_owned())
        }

        fn basename(&self) -> Option<PathBuf> {
            g_debug!("MyVfs", "MyFile::basename({:?})", self);
            self.local_file().basename()
        }

        fn path(&self) -> Option<PathBuf> {
            g_debug!("MyVfs", "MyFile::path({:?})", self);
            self.local_file().path()
        }

        fn uri(&self) -> String {
            g_debug!("MyVfs", "MyFile::uri({:?})", self);
            format!(
                "{}://{}",
                SCHEME,
                self.local_file().path().unwrap().to_string_lossy()
            )
        }

        fn parse_name(&self) -> String {
            g_debug!("MyVfs", "MyFile::parse_name({:?})", self);
            self.uri()
        }

        fn parent(&self) -> Option<File> {
            g_debug!("MyVfs", "MyFile::parent({:?})", self);
            match (self.virtual_path().parent(), self.local_file().parent()) {
                (Some(virtual_path), Some(local_file)) => {
                    Some(Self::Type::new(virtual_path.to_path_buf(), local_file).upcast())
                }
                _ => None,
            }
        }

        fn has_prefix(&self, prefix: &File) -> bool {
            g_debug!("MyVfs", "MyFile::has_prefix({:?},{:?})", self, prefix);
            self.local_file().has_prefix(prefix)
        }

        fn relative_path(&self, descendant: &File) -> Option<PathBuf> {
            g_debug!(
                "MyVfs",
                "MyFile::relative_path({:?},{:?})",
                self,
                descendant
            );
            match descendant.downcast_ref::<Self::Type>() {
                Some(descendant) => descendant
                    .virtual_path()
                    .strip_prefix(self.virtual_path())
                    .ok()
                    .map(PathBuf::from),
                None => None,
            }
        }

        fn resolve_relative_path(&self, relative_path: impl AsRef<std::path::Path>) -> File {
            g_debug!(
                "MyVfs",
                "MyFile::resolve_relative_path({:?},{:?})",
                self,
                relative_path.as_ref()
            );
            let relative_path_as_pb = PathBuf::from(relative_path.as_ref());
            let (virtual_path, local_file) = if relative_path_as_pb.is_absolute() {
                (
                    relative_path_as_pb,
                    Vfs::local().file_for_path(relative_path),
                )
            } else {
                (
                    self.virtual_path().join(relative_path_as_pb),
                    self.local_file().resolve_relative_path(relative_path),
                )
            };
            Self::Type::new(virtual_path, local_file).upcast()
        }

        fn child_for_display_name(&self, display_name: &str) -> Result<File, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::child_for_display_name({:?},{})",
                self,
                display_name
            );
            let virtual_path = self.virtual_path().join(display_name);
            let local_file = self.local_file().child_for_display_name(display_name)?;
            Ok(Self::Type::new(virtual_path, local_file).upcast())
        }

        fn enumerate_children(
            &self,
            attributes: &str,
            flags: FileQueryInfoFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileEnumerator, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::enumerate_children({:?},{},{:?},{:?})",
                self,
                attributes,
                flags,
                cancellable.map(|_| "_")
            );
            MyFileEnumerator::new(self.local_file(), attributes, flags, cancellable)
                .map(|my_file_enumerator| my_file_enumerator.upcast())
        }

        fn query_info(
            &self,
            attributes: &str,
            flags: FileQueryInfoFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileInfo, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::query_info({:?},{},{:?},{:?})",
                self,
                attributes,
                flags,
                cancellable.map(|_| "_")
            );
            let info = self
                .local_file()
                .query_info(attributes, flags, cancellable)?;
            update_file_info(&info);
            Ok(info)
        }

        fn query_filesystem_info(
            &self,
            attributes: &str,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileInfo, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::query_filesystem_info({:?},{},{:?})",
                self,
                attributes,
                cancellable.map(|_| "_")
            );
            let info = self
                .local_file()
                .query_filesystem_info(attributes, cancellable)?;
            update_file_info(&info);
            Ok(info)
        }

        fn set_display_name(
            &self,
            display_name: &str,
            cancellable: Option<&Cancellable>,
        ) -> Result<File, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::set_display_name({:?},{},{:?})",
                self,
                display_name,
                cancellable.map(|_| "_")
            );
            let mut virtual_path = self.virtual_path().clone();
            let local_file = self
                .local_file()
                .set_display_name(display_name, cancellable)?;
            let basename = local_file.basename().ok_or(Error::new(
                IOErrorEnum::InvalidFilename,
                &format!(
                    "failed to rename {} to {}",
                    virtual_path.file_name().unwrap().to_string_lossy(),
                    display_name
                ),
            ))?;
            virtual_path.set_file_name(basename);
            Ok(Self::Type::new(virtual_path, local_file).upcast())
        }

        fn query_settable_attributes(
            &self,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileAttributeInfoList, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::query_settable_attributes({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().query_settable_attributes(cancellable)
        }

        fn query_writable_namespaces(
            &self,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileAttributeInfoList, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::query_writable_namespaces({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().query_writable_namespaces(cancellable)
        }

        fn set_attribute<'a>(
            &self,
            attribute: &str,
            value: impl Into<FileAttributeValue<'a>>,
            flags: FileQueryInfoFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<(), Error> {
            let value: FileAttributeValue<'a> = value.into();
            g_debug!(
                "MyVfs",
                "MyFile::set_attribute({:?},{},{:?},{:?},{:?})",
                self,
                attribute,
                value,
                flags,
                cancellable.map(|_| "_")
            );
            self.local_file()
                .set_attribute(attribute, value, flags, cancellable)
        }

        fn read_fn(&self, cancellable: Option<&Cancellable>) -> Result<FileInputStream, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::read_fn({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().read(cancellable)
        }

        fn append_to(
            &self,
            flags: FileCreateFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileOutputStream, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::append_to({:?},{:?},{:?})",
                self,
                flags,
                cancellable.map(|_| "_")
            );
            self.local_file().append_to(flags, cancellable)
        }

        fn create(
            &self,
            flags: FileCreateFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileOutputStream, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::create({:?},{:?},{:?})",
                self,
                flags,
                cancellable.map(|_| "_")
            );
            self.local_file().create(flags, cancellable)
        }

        fn replace(
            &self,
            etag: Option<&str>,
            make_backup: bool,
            flags: FileCreateFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileOutputStream, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::replace({:?},{:?},{},{:?},{:?})",
                self,
                etag,
                make_backup,
                flags,
                cancellable.map(|_| "_")
            );
            self.local_file().replace(
                etag.map(AsRef::<str>::as_ref),
                make_backup,
                flags,
                cancellable,
            )
        }

        fn delete(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
            g_debug!(
                "MyVfs",
                "MyFile::delete({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().delete(cancellable)
        }

        fn trash(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
            g_debug!(
                "MyVfs",
                "MyFile::trash({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().trash(cancellable)
        }

        fn make_directory(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
            g_debug!(
                "MyVfs",
                "MyFile::make_directory({:?},{:?})",
                self,
                cancellable.map(|_| "_")
            );
            self.local_file().make_directory(cancellable)
        }

        fn copy(
            source: &File,
            destination: &File,
            flags: FileCopyFlags,
            cancellable: Option<&Cancellable>,
            progress_callback: Option<&mut dyn FnMut(i64, i64)>,
        ) -> Result<(), Error> {
            g_debug!(
                "MyVfs",
                "MyFile::copy({:?},{:?},{:?},{:?},{:?})",
                source,
                destination,
                flags,
                cancellable.map(|_| "_"),
                progress_callback.as_ref().map(|_| "_")
            );
            let source = source
                .downcast_ref::<Self::Type>()
                .map(|my_file| my_file.imp().local_file())
                .unwrap_or(source);
            let destination = destination
                .downcast_ref::<Self::Type>()
                .map(|my_file| my_file.imp().local_file())
                .unwrap_or(destination);
            source.copy(destination, flags, cancellable, progress_callback)
        }

        fn move_(
            source: &File,
            destination: &File,
            flags: FileCopyFlags,
            cancellable: Option<&Cancellable>,
            progress_callback: Option<&mut dyn FnMut(i64, i64)>,
        ) -> Result<(), Error> {
            g_debug!(
                "MyVfs",
                "MyFile::move_({:?},{:?},{:?},{:?},{:?})",
                source,
                destination,
                flags,
                cancellable.map(|_| "_"),
                progress_callback.as_ref().map(|_| "_")
            );
            let source = source
                .downcast_ref::<Self::Type>()
                .map(|my_file| my_file.imp().local_file())
                .unwrap_or(source);
            let destination = destination
                .downcast_ref::<Self::Type>()
                .map(|my_file| my_file.imp().local_file())
                .unwrap_or(destination);
            source.move_(destination, flags, cancellable, progress_callback)
        }

        fn monitor_dir(
            &self,
            flags: FileMonitorFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileMonitor, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::monitor_dir({:?},{:?},{:?})",
                self,
                flags,
                cancellable.map(|_| "_")
            );
            MyFileMonitor::for_directory(self.local_file(), flags, cancellable)
                .map(|my_file_monitor| my_file_monitor.upcast())
        }

        fn monitor_file(
            &self,
            flags: FileMonitorFlags,
            cancellable: Option<&Cancellable>,
        ) -> Result<FileMonitor, Error> {
            g_debug!(
                "MyVfs",
                "MyFile::monitor_file({:?},{:?},{:?})",
                self,
                flags,
                cancellable.map(|_| "_")
            );
            MyFileMonitor::for_file(self.local_file(), flags, cancellable)
                .map(|my_file_monitor| my_file_monitor.upcast())
        }
    }
}

glib::wrapper! {
    pub struct MyFile(ObjectSubclass<imp::MyFile>) @implements File;
}

impl MyFile {
    pub fn new(virtual_path: PathBuf, local_file: File) -> Self {
        g_debug!("MyVfs", "MyFile::new({:?},{:?})", virtual_path, local_file);
        Object::builder()
            .property("virtual_path", virtual_path)
            .property("local_file", local_file)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test() {
        let f = MyFile::new(PathBuf::from("/"), Vfs::local().file_for_path("/"));
        assert_eq!(f.path(), Some(PathBuf::from("/")));
    }
}
