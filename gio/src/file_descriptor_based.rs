// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

use crate::ffi;
use glib::{prelude::*, translate::*};
#[cfg(all(not(unix), docsrs))]
use socket::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

glib::wrapper! {
    #[doc(alias = "GFileDescriptorBased")]
    pub struct FileDescriptorBased(Interface<ffi::GFileDescriptorBased, ffi::GFileDescriptorBasedIface>);

    match fn {
        type_ => || ffi::g_file_descriptor_based_get_type(),
    }
}

impl FileDescriptorBased {
    pub const NONE: Option<&'static FileDescriptorBased> = None;
}

impl AsRawFd for FileDescriptorBased {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::g_file_descriptor_based_get_fd(self.to_glib_none().0) as _ }
    }
}

pub trait FileDescriptorBasedExtManual: IsA<FileDescriptorBased> + 'static {
    #[doc(alias = "g_file_descriptor_based_get_fd")]
    #[doc(alias = "get_fd")]
    fn fd<T: FromRawFd>(&self) -> T {
        unsafe {
            T::from_raw_fd(ffi::g_file_descriptor_based_get_fd(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<FileDescriptorBased>> FileDescriptorBasedExtManual for O {}
