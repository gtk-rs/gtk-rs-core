// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::PageRange;
use crate::PrintSettings;

impl PrintSettings {
    pub fn set_page_ranges(&self, page_ranges: &[PageRange]) {
        let num_ranges = page_ranges.len() as i32;
        unsafe {
            ffi::gtk_print_settings_set_page_ranges(
                self.to_glib_none().0,
                mut_override(page_ranges.as_ptr() as *const _),
                num_ranges,
            );
        }
    }
}
