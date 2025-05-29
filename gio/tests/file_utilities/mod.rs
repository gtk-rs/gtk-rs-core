// Take a look at the license at the top of the repository in the LICENSE file.

use std::ops::Deref;

use gio::{prelude::*, Cancellable, File};
use glib::translate::{FromGlibPtrFull, ToGlibPtr};

// Temp is a test utility that creates a new temporary file (or directory) and delete it at drop time.
#[derive(Clone)]
pub struct Temp {
    pub file: Option<File>,
    pub path: String,
    pub basename: String,
}

impl Temp {
    // Make a new temporary directory.
    pub fn make_dir(tmpl: &str) -> Self {
        unsafe {
            let res = glib::ffi::g_dir_make_tmp(tmpl.to_glib_none().0, std::ptr::null_mut());
            assert!(!res.is_null());
            let path = glib::GString::from_glib_full(res).as_str().to_owned();
            let file = File::for_parse_name(&path);
            let res = file.basename();
            assert!(res.is_some());
            let basename = res.unwrap().as_path().to_str().unwrap().to_owned();
            Self {
                file: Some(file),
                path,
                basename,
            }
        }
    }

    // Create a new temporary file under a temporary directory.
    pub fn create_file_child(&self, tmpl: &str) -> Self {
        unsafe {
            let tmpl = glib::gformat!("{}/{}", self.path, tmpl);
            let fd = glib::ffi::g_mkstemp(tmpl.to_glib_none().0);
            assert_ne!(fd, -1, "file not created");
            {
                // close file
                use std::os::fd::FromRawFd;
                let _ = std::fs::File::from_raw_fd(fd);
            }
            let path = tmpl.as_str().to_owned();
            let file = File::for_parse_name(&path);
            let res = file.basename();
            assert!(res.is_some());
            let basename = res.unwrap().as_path().to_str().unwrap().to_owned();
            Self {
                file: Some(file),
                path,
                basename,
            }
        }
    }
}

impl Deref for Temp {
    type Target = Option<File>;

    // Dereference self to the inner temporary file.
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl Temp {
    // Take ownership of the inner file so it won't be deleted when self goes out of scope.
    pub fn take_file(&mut self) -> Option<File> {
        self.file.take()
    }
}

impl Drop for Temp {
    // Delete the inner temporary file (if it has not been taken).
    fn drop(&mut self) {
        if let Some(ref file) = self.file {
            let res = file.delete(Cancellable::NONE);
            assert!(res.is_ok(), "{}", res.err().unwrap());
        }
    }
}
