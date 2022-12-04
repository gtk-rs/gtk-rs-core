// Take a look at the license at the top of the repository in the LICENSE file.

use std::str::FromStr;

use crate::{Language, Script};
use glib::translate::*;

unsafe impl Send for Language {}
unsafe impl Sync for Language {}

impl Language {
    #[doc(alias = "get_scripts")]
    #[doc(alias = "pango_language_get_scripts")]
    pub fn scripts(&self) -> Vec<Script> {
        let mut num_scripts = 0;
        let mut ret = Vec::new();

        unsafe {
            let scripts: *const ffi::PangoScript = ffi::pango_language_get_scripts(
                mut_override(self.to_glib_none().0),
                &mut num_scripts,
            );
            if num_scripts > 0 {
                for x in 0..num_scripts {
                    ret.push(from_glib(
                        *(scripts.offset(x as isize) as *const ffi::PangoScript),
                    ));
                }
            }
            ret
        }
    }

    #[cfg(any(feature = "v1_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_48")))]
    #[doc(alias = "get_preferred")]
    #[doc(alias = "pango_language_get_preferred")]
    pub fn preferred() -> Vec<Self> {
        unsafe {
            let ptr = ffi::pango_language_get_preferred();
            Self::from_glib_full_as_vec(ptr)
        }
    }

    pub fn to_string(&self) -> glib::GString {
        self.to_str()
    }
}

impl FromStr for Language {
    type Err = std::convert::Infallible;
    fn from_str(language: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_string(language))
    }
}

impl Default for Language {
    fn default() -> Self {
        Self::default()
    }
}
