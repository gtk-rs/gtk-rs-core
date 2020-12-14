// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::IsA;
use glib::translate::ToGlibPtr;
use std::convert::TryFrom;

use crate::Entry;

pub trait EntryExtManual: 'static {
    fn get_invisible_char(&self) -> Option<char>;
}

impl<O: IsA<Entry>> EntryExtManual for O {
    fn get_invisible_char(&self) -> Option<char> {
        let ret = unsafe { ffi::gtk_entry_get_invisible_char(self.as_ref().to_glib_none().0) };

        if ret == 0 {
            return None;
        }

        Some(TryFrom::try_from(ret).expect("conversion from an invalid Unicode value attempted"))
    }
}
