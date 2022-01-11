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
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Quark {
        unsafe { from_glib(ffi::g_quark_from_string(s.to_glib_none().0)) }
    }

    #[allow(clippy::trivially_copy_pass_by_ref)]
    #[doc(alias = "g_quark_to_string")]
    pub fn as_str<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(ffi::g_quark_to_string(self.into_glib()))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_quark_try_string")]
    pub fn try_from_str(s: &str) -> Option<Quark> {
        unsafe { Self::try_from_glib(ffi::g_quark_try_string(s.to_glib_none().0)).ok() }
    }
}

impl fmt::Debug for Quark {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(Quark::as_str(self))
    }
}

impl<'a> From<&'a str> for Quark {
    fn from(s: &'a str) -> Self {
        Self::from_str(s)
    }
}

impl std::str::FromStr for Quark {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str(s))
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
    fn test_from_str() {
        let q1 = Quark::from_str("some-quark");
        let q2 = Quark::try_from_str("some-quark");
        assert_eq!(Some(q1), q2);
        assert_eq!(q1.as_str(), "some-quark");
    }
}
