// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::ffi::CStr;
use std::fmt;
use std::num::NonZeroU32;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[repr(transparent)]
#[doc(alias = "GQuark")]
pub struct Quark(NonZeroU32);

impl Quark {
    #[doc(alias = "g_quark_from_string")]
    pub fn from_string(s: &str) -> Quark {
        unsafe { from_glib(ffi::g_quark_from_string(s.to_glib_none().0)) }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[doc(alias = "g_quark_to_string")]
    pub fn to_string<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::g_quark_to_string(self.into_glib()))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_quark_try_string")]
    pub fn try_string(s: &str) -> Option<Quark> {
        unsafe { Self::try_from_glib(ffi::g_quark_try_string(s.to_glib_none().0)).ok() }
    }
}

impl fmt::Debug for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(Quark::to_string(self))
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GQuark> for Quark {
    unsafe fn from_glib(value: ffi::GQuark) -> Self {
        assert_ne!(value, 0);
        Self(NonZeroU32::new_unchecked(value))
    }
}

#[doc(hidden)]
impl TryFromGlib<ffi::GQuark> for Quark {
    type Error = GlibNoneError;
    unsafe fn try_from_glib(value: ffi::GQuark) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(GlibNoneError)
        } else {
            Ok(Self(NonZeroU32::new_unchecked(value)))
        }
    }
}

#[doc(hidden)]
impl IntoGlib for Quark {
    type GlibType = ffi::GQuark;

    fn into_glib(self) -> ffi::GQuark {
        self.0.get()
    }
}

#[doc(hidden)]
impl IntoGlib for Option<Quark> {
    type GlibType = ffi::GQuark;

    fn into_glib(self) -> ffi::GQuark {
        self.map(|s| s.0.get()).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_string() {
        let q1 = Quark::from_string("some-quark");
        let q2 = Quark::try_string("some-quark");
        assert_eq!(Some(q1), q2);
        assert_eq!(q1.to_string(), "some-quark");
    }
}
