// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Script;
use glib::translate::*;
use glib::GString;

#[doc(alias = "PangoLanguage")]
pub struct Language(*mut ffi::PangoLanguage);

unsafe impl Send for Language {}
unsafe impl Sync for Language {}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut ffi::PangoLanguage> for &'a Language {
    type Storage = &'a Language;

    fn to_glib_none(&self) -> Stash<'a, *mut ffi::PangoLanguage, Self> {
        Stash(self.0, *self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::PangoLanguage> for Language {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::PangoLanguage, Self> {
        StashMut(self.0, self)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *mut ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr)
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_none(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*const ffi::PangoLanguage> for Language {
    unsafe fn from_glib_full(ptr: *const ffi::PangoLanguage) -> Self {
        assert!(!ptr.is_null());
        Self(ptr as *mut _)
    }
}

impl Default for Language {
    #[doc(alias = "pango_language_get_default")]
    fn default() -> Self {
        unsafe { from_glib_full(ffi::pango_language_get_default()) }
    }
}

impl Language {
    #[doc(alias = "pango_language_from_string")]
    pub fn from_string(language: &str) -> Self {
        unsafe { from_glib_full(ffi::pango_language_from_string(language.to_glib_none().0)) }
    }

    #[doc(alias = "pango_language_to_string")]
    pub fn to_string(&self) -> GString {
        unsafe { from_glib_none(ffi::pango_language_to_string(self.to_glib_none().0)) }
    }

    #[doc(alias = "pango_language_matches")]
    pub fn matches(&self, range_list: &str) -> bool {
        unsafe {
            from_glib(ffi::pango_language_matches(
                self.to_glib_none().0,
                range_list.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "pango_language_includes_script")]
    pub fn includes_script(&self, script: Script) -> bool {
        unsafe {
            from_glib(ffi::pango_language_includes_script(
                self.to_glib_none().0,
                script.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_scripts")]
    #[doc(alias = "pango_language_get_scripts")]
    pub fn scripts(&self) -> Vec<Script> {
        let mut num_scripts = 0;
        let mut ret = Vec::new();

        unsafe {
            let scripts: *const ffi::PangoScript =
                ffi::pango_language_get_scripts(self.to_glib_none().0, &mut num_scripts);
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

    #[doc(alias = "get_sample_string")]
    #[doc(alias = "pango_language_get_sample_string")]
    pub fn sample_string(&self) -> GString {
        unsafe { from_glib_none(ffi::pango_language_get_sample_string(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v1_48", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_48")))]
    #[doc(alias = "get_preferred")]
    #[doc(alias = "pango_language_get_preferred")]
    pub fn preferred(&self) -> Vec<Language> {
        unsafe {
            let langs = ffi::pango_language_get_preferred();
            let mut ptr = langs;

            let mut ret = vec![];

            while !(*ptr).is_null() {
                ret.push(Language(*ptr));
                ptr = ptr.add(1);
            }

            ret
        }
    }
}
