// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{translate::*, IntoGStr, MatchInfo};
use std::{mem, ptr};

impl MatchInfo {
    #[doc(alias = "g_match_info_expand_references")]
    pub fn expand_references(
        &self,
        string_to_expand: impl IntoGStr,
    ) -> Result<Option<crate::GString>, crate::Error> {
        string_to_expand.run_with_gstr(|string_to_expand| unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_match_info_expand_references(
                self.to_glib_none().0,
                string_to_expand.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        })
    }

    #[doc(alias = "g_match_info_fetch_named")]
    pub fn fetch_named(&self, name: impl IntoGStr) -> Option<crate::GString> {
        name.run_with_gstr(|name| unsafe {
            from_glib_full(ffi::g_match_info_fetch_named(
                self.to_glib_none().0,
                name.to_glib_none().0,
            ))
        })
    }

    #[doc(alias = "g_match_info_fetch_named_pos")]
    pub fn fetch_named_pos(&self, name: impl IntoGStr) -> Option<(i32, i32)> {
        name.run_with_gstr(|name| unsafe {
            let mut start_pos = mem::MaybeUninit::uninit();
            let mut end_pos = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::g_match_info_fetch_named_pos(
                self.to_glib_none().0,
                name.to_glib_none().0,
                start_pos.as_mut_ptr(),
                end_pos.as_mut_ptr(),
            ));
            if ret {
                Some((start_pos.assume_init(), end_pos.assume_init()))
            } else {
                None
            }
        })
    }
}
