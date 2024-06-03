// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(unix)]
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};
use std::{mem, ptr};

use glib::{prelude::*, translate::*};
#[cfg(all(not(unix), docsrs))]
use socket::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, OwnedFd, RawFd};

use crate::{ffi, UnixFDList};

impl UnixFDList {
    #[doc(alias = "g_unix_fd_list_new_from_array")]
    pub fn from_array(fds: impl IntoIterator<Item = impl IntoRawFd>) -> UnixFDList {
        let fds = fds.into_iter().map(|t| t.into_raw_fd()).collect::<Vec<_>>();
        unsafe {
            from_glib_full(ffi::g_unix_fd_list_new_from_array(
                fds.to_glib_none().0,
                fds.len() as i32,
            ))
        }
    }
}

pub trait UnixFDListExtManual: IsA<UnixFDList> + Sized {
    #[doc(alias = "g_unix_fd_list_append")]
    fn append(&self, fd: impl AsFd) -> Result<i32, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_unix_fd_list_append(
                self.as_ref().to_glib_none().0,
                fd.as_fd().as_raw_fd(),
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_unix_fd_list_get")]
    fn get(&self, index_: i32) -> Result<OwnedFd, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let raw_fd =
                ffi::g_unix_fd_list_get(self.as_ref().to_glib_none().0, index_, &mut error);
            if error.is_null() {
                let fd = OwnedFd::from_raw_fd(raw_fd);
                Ok(fd)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_unix_fd_list_peek_fds")]
    fn peek_fds(&self) -> Vec<RawFd> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();
            let ret = FromGlibContainer::from_glib_none_num(
                ffi::g_unix_fd_list_peek_fds(self.as_ref().to_glib_none().0, length.as_mut_ptr()),
                length.assume_init() as usize,
            );
            ret
        }
    }

    #[doc(alias = "g_unix_fd_list_steal_fds")]
    fn steal_fds(&self) -> FdArray {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();

            let ptr =
                ffi::g_unix_fd_list_steal_fds(self.as_ref().to_glib_none().0, length.as_mut_ptr());

            FdArray {
                ptr: ptr::NonNull::new(ptr).unwrap(),
                len: length.assume_init() as usize,
            }
        }
    }
}

impl<O: IsA<UnixFDList>> UnixFDListExtManual for O {}

pub struct FdArray {
    ptr: ptr::NonNull<libc::c_int>,
    len: usize,
}

pub struct FdIterator {
    ptr: ptr::NonNull<libc::c_int>,
    len: usize,
}

impl Iterator for FdIterator {
    type Item = OwnedFd;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let current = self.ptr.as_ptr();
            if self.len > 1 {
                let next = unsafe { self.ptr.as_ptr().add(1) };
                self.ptr = ptr::NonNull::new(next).unwrap();
            }
            self.len -= 1;
            Some(unsafe { OwnedFd::from_raw_fd(*current) })
        } else {
            None
        }
    }
}

impl Drop for FdArray {
    fn drop(&mut self) {
        while self.len > 0 {
            unsafe {
                let current = self.ptr.as_ptr();
                libc::close(*current);
            }
            if self.len > 1 {
                let next = unsafe { self.ptr.as_ptr().add(1) };
                self.ptr = ptr::NonNull::new(next).unwrap();
            }
            self.len -= 1;
        }
    }
}

impl std::iter::IntoIterator for FdArray {
    type Item = OwnedFd;
    type IntoIter = FdIterator;

    fn into_iter(mut self) -> Self::IntoIter {
        let len = std::mem::take(&mut self.len);
        FdIterator { len, ptr: self.ptr }
    }
}

impl FdArray {
    pub fn as_slice(&self) -> &[BorrowedFd<'_>] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr() as *const BorrowedFd, self.len) }
    }
}
