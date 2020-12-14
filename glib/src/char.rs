// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::FromGlib;
use crate::translate::ToGlib;
use libc::{c_char, c_uchar};

/// Wrapper for values where C functions expect a plain C `char`
///
/// Consider the following C function prototype from glib:
///
/// ```C
/// void g_key_file_set_list_separator (GKeyFile *key_file, gchar separator);
/// ```
///
/// This function plainly expects a byte as the `separator` argument.  However,
/// having this function exposed to Rust as the following would be inconvenient:
///
/// ```ignore
/// impl KeyFile {
///     pub fn set_list_separator(&self, separator: libc:c_char) { }
/// }
/// ```
///
/// This would be inconvenient because users would have to do the conversion from a Rust `char` to an `libc::c_char` by hand, which is just a type alias
/// for `i8` on most system.
///
/// This `Char` type is a wrapper over an `libc::c_char`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `libc::c_char` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `libc::c_char` (which is equivalent to `i8` can be extracted with `.0`, or
/// by calling `my_char.to_glib()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Char(pub c_char);

impl Char {
    /// Creates a `Some(Char)` if the given `char` is representable as an `libc::c_char`
    ///
    /// # Example
    /// ```ignore
    /// extern "C" fn have_a_byte(b: libc::c_char);
    ///
    /// let a = Char::new('a').unwrap();
    /// assert!(a.0 == 65);
    /// have_a_byte(a.to_glib());
    ///
    /// let not_representable = Char::new('☔');
    /// assert!(not_representable.is_none());
    /// ```
    pub fn new(c: char) -> Option<Char> {
        if c as u32 > 255 {
            None
        } else {
            Some(Char(c as c_char))
        }
    }
}

impl From<Char> for char {
    fn from(c: Char) -> char {
        c.0 as u8 as char
    }
}

#[doc(hidden)]
impl FromGlib<c_char> for Char {
    unsafe fn from_glib(value: c_char) -> Self {
        Char(value)
    }
}

#[doc(hidden)]
impl ToGlib for Char {
    type GlibType = c_char;

    fn to_glib(&self) -> c_char {
        self.0
    }
}

/// Wrapper for values where C functions expect a plain C `unsigned char`
///
/// This `UChar` type is a wrapper over an `libc::c_uchar`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `libc::c_uchar` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `libc::c_uchar` (which is equivalent to `u8` can be extracted with `.0`, or
/// by calling `my_char.to_glib()`.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UChar(pub c_uchar);

impl UChar {
    /// Creates a `Some(UChar)` if the given `char` is representable as an `libc::c_uchar`
    ///
    /// # Example
    /// ```ignore
    /// extern "C" fn have_a_byte(b: libc::c_uchar);
    ///
    /// let a = Char::new('a').unwrap();
    /// assert!(a.0 == 65);
    /// have_a_byte(a.to_glib());
    ///
    /// let not_representable = Char::new('☔');
    /// assert!(not_representable.is_none());
    /// ```
    pub fn new(c: char) -> Option<UChar> {
        if c as u32 > 255 {
            None
        } else {
            Some(UChar(c as c_uchar))
        }
    }
}

impl From<UChar> for char {
    fn from(c: UChar) -> char {
        c.0 as char
    }
}

#[doc(hidden)]
impl FromGlib<c_uchar> for UChar {
    unsafe fn from_glib(value: c_uchar) -> Self {
        UChar(value)
    }
}

#[doc(hidden)]
impl ToGlib for UChar {
    type GlibType = c_uchar;

    fn to_glib(&self) -> c_uchar {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::translate::from_glib;

    #[test]
    fn converts_single_byte_chars() {
        assert_eq!(Char::new(0 as char), Some(Char(0_i8)));
        assert_eq!(UChar::new(0 as char), Some(UChar(0_u8)));
        assert_eq!(UChar::new(255 as char), Some(UChar(255_u8)));
        assert_eq!(UChar::new('ñ'), Some(UChar(241_u8)));
    }

    #[test]
    fn refuses_multibyte_chars() {
        assert_eq!(Char::new('☔'), None); // no umbrella for you
        assert_eq!(UChar::new('☔'), None);
    }

    #[test]
    fn into_i8() {
        assert_eq!(Char::new('A').unwrap().to_glib(), 65_i8);
    }

    #[test]
    fn into_u8() {
        assert_eq!(UChar::new('A').unwrap().to_glib(), 65_u8);
    }

    #[test]
    fn into_char() {
        assert_eq!(char::from(Char(65_i8)), 'A');
        assert_eq!('ñ', UChar(241_u8).into());
    }

    #[test]
    fn convert_from_glib() {
        assert_eq!(Char(65_i8), unsafe { from_glib(65_i8) });
        assert_eq!(UChar(241_u8), unsafe { from_glib(241_u8) });
    }
}
