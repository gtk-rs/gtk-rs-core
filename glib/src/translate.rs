// Take a look at the license at the top of the repository in the LICENSE file.

//! Translation between GLib/GLib-based FFI types and their Rust counterparts.
//!
//! This module allows library bindings authors to decouple type translation
//! logic and use unified idioms at FFI boundaries. It also implements
//! translation of GLib core data types.
//!
//! `FromGlib`, `from_glib` and `ToGlib` translate simple types like `bool`.
//!
//! ```ignore
//!     pub fn set_accept_focus(&self, accept_focus: bool) {
//!         unsafe { gdk_sys::gdk_window_set_accept_focus(self.pointer, accept_focus.to_glib()) }
//!     }
//!
//!     pub fn get_accept_focus(&self) -> bool {
//!         unsafe { from_glib(gdk_sys::gdk_window_get_accept_focus(self.pointer)) }
//!     }
//! ```
//!
//! Implementing [`OptionToGlib`] on a Rust type `T` allows specifying a sentinel to indicate
//! a `None` value and auto-implementing [`FromGlib`] for `Option<T>`, which would not be
//! possible in dependent crates due to the [orphan rule](https://doc.rust-lang.org/book/ch10-02-traits.html#implementing-a-trait-on-a-type).
//! In the example below, [`ToGlib`] is auto-implemented for `Option<SpecialU32>`.
//!
//! ```
//! # use glib::translate::*;
//! struct SpecialU32(u32);
//! impl ToGlib for SpecialU32 {
//!     type GlibType = libc::c_uint;
//!     fn to_glib(&self) -> libc::c_uint {
//!         self.0 as libc::c_uint
//!     }
//! }
//! impl OptionToGlib for SpecialU32 {
//!     const GLIB_NONE: Self::GlibType = 0xFFFFFF;
//! }
//! ```
//!
//! In order to auto-implement [`FromGlib`] for `Option<SpecialU32>`, proceed as follows:
//!
//! ```
//! # use glib::translate::*;
//! # struct SpecialU32(u32);
//! # impl ToGlib for SpecialU32 {
//! #     type GlibType = libc::c_uint;
//! #     fn to_glib(&self) -> libc::c_uint {
//! #         self.0 as libc::c_uint
//! #     }
//! # }
//! # impl OptionToGlib for SpecialU32 {
//! #     const GLIB_NONE: Self::GlibType = 0xFFFFFF;
//! # }
//! impl TryFromGlib<libc::c_uint> for SpecialU32 {
//!     type Error = GlibNoneError;
//!     fn try_from_glib(val: libc::c_uint) -> Result<Self, GlibNoneError> {
//!         if val == SpecialU32::GLIB_NONE {
//!             return Err(GlibNoneError);
//!         }
//!         Ok(SpecialU32(val as u32))
//!     }
//! }
//! ```
//!
//! The [`TryFromGlib`] trait can also be implemented when the Glib type range is larger than the
//! target Rust type's range. In the example below, the Rust type `U32` can be built from a signed
//! [`libc::c_long`], which means that the negative range is not valid.
//!
//! ```
//! # use std::convert::TryFrom;
//! # use std::num::TryFromIntError;
//! # use glib::translate::*;
//! struct U32(u32);
//! impl TryFromGlib<libc::c_long> for U32 {
//!     type Error = TryFromIntError;
//!     fn try_from_glib(val: libc::c_long) -> Result<Self, TryFromIntError> {
//!         Ok(U32(u32::try_from(val)?))
//!     }
//! }
//! ```
//!
//! Finally, you can define [`TryFromGlib`] with both `None` and `Invalid` alternatives by setting
//! the associated `type Error = GlibNoneOrInvalidError<I>` (where `I` is the `Error` type
//! when the value is invalid), which results in auto-implementing [`FromGlib`] for
//! `Result<Option<T>, I>`.
//!
//! `ToGlibPtr`, `FromGlibPtrNone`, `FromGlibPtrFull` and `FromGlibPtrBorrow` work on `gpointer`s
//! and ensure correct ownership of values
//! according to [Glib ownership transfer rules](https://gi.readthedocs.io/en/latest/annotations/giannotations.html).
//!
//! `FromGlibPtrNone` and `FromGlibPtrFull`
//! must be called on values obtained from C,
//! according to their `transfer` annotations.
//! They acquire non-gobject types,
//! as well as turning floating references to strong ones,
//! which are the only ones properly handled by the Rust bindings.
//!
//! For more information about floating references, please refer to the "Floating references" section
//! of [the gobject reference](https://developer.gnome.org/gobject/stable/gobject-The-Base-Object-Type.html).
//!
//! ```ignore
//!     fn get_title(&self) -> Option<String> {
//!         unsafe {
//!             let title = gtk_sys::gtk_window_get_title(self.pointer);
//!             from_glib_none(title)
//!         }
//!     }
//!     fn create_bool(value: gboolean) -> Variant {
//!         unsafe {
//!             let variant = ffi::g_variant_new_boolean(value);
//!             // g_variant_new_boolean has `transfer none`
//!             from_glib_none(variant)
//!         }
//!     }
//! ```
//!
//! Letting the foreign library borrow pointers from the Rust side often
//! requires having a temporary variable of an intermediate type (e.g. `CString`).
//! A `Stash` contains the temporary storage and a pointer into it that
//! is valid for the lifetime of the `Stash`. As the lifetime of the `Stash` returned
//! from `to_glib_none` is at least the enclosing statement, you can avoid explicitly
//! binding the stash in most cases and just take the pointer out of it:
//!
//! ```ignore
//!     pub fn set_icon_name(&self, name: &str) {
//!         unsafe {
//!             gdk_sys::gdk_window_set_icon_name(self.pointer, name.to_glib_none().0)
//!         }
//!     }
//! ```

use libc::{c_char, size_t};
use std::char;
use std::cmp::{Eq, Ordering, PartialEq};
use std::collections::HashMap;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::mem;
#[cfg(not(windows))]
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};
use std::ptr;

/// A pointer
pub trait Ptr: Copy + 'static {
    fn is_null(&self) -> bool;
    fn from<X>(ptr: *mut X) -> Self;
    fn to<X>(self) -> *mut X;
}

impl<T: 'static> Ptr for *const T {
    #[inline]
    fn is_null(&self) -> bool {
        (*self).is_null()
    }

    #[inline]
    fn from<X>(ptr: *mut X) -> *const T {
        ptr as *const T
    }

    #[inline]
    fn to<X>(self) -> *mut X {
        self as *mut X
    }
}

impl<T: 'static> Ptr for *mut T {
    #[inline]
    fn is_null(&self) -> bool {
        (*self).is_null()
    }

    #[inline]
    fn from<X>(ptr: *mut X) -> *mut T {
        ptr as *mut T
    }

    #[inline]
    fn to<X>(self) -> *mut X {
        self as *mut X
    }
}

/// Overrides pointer mutability.
///
/// Use when the C API should be specifying a const pointer but doesn't.
pub fn mut_override<T>(ptr: *const T) -> *mut T {
    ptr as *mut T
}

/// Overrides pointer constness.
///
/// Use when the C API need const pointer, but function with `IsA<T>` constraint,
/// that usaly don't have const pointer conversion.
pub fn const_override<T>(ptr: *mut T) -> *const T {
    ptr as *const T
}

/// A trait for creating an uninitialized value. Handy for receiving outparams.
pub trait Uninitialized {
    /// Returns an uninitialized value.
    unsafe fn uninitialized() -> Self;
}

/// Returns an uninitialized value.
#[inline]
pub unsafe fn uninitialized<T: Uninitialized>() -> T {
    T::uninitialized()
}

/// Helper type that stores temporary values used for translation.
///
/// `P` is the foreign type pointer and the first element of the tuple.
///
/// `T` is the Rust type that is translated.
///
/// The second element of the tuple is the temporary storage defined
/// by the implementation of `ToGlibPtr<P> for T`
///
/// Say you want to pass a `*mut GdkWindowAttr` to a foreign function. The `Stash`
/// will own a `GdkWindowAttr` and a `CString` that `GdkWindowAttr::title` points into.
///
/// ```ignore
/// impl <'a> ToGlibPtr<'a, *mut ffi::GdkWindowAttr> for WindowAttr {
///     type Storage = (Box<ffi::GdkWindowAttr>, Stash<'a, *const c_char, Option<String>>);
///
///     fn to_glib_none(&'a self) -> Stash<*mut ffi::GdkWindowAttr, WindowAttr> {
///         let title = self.title.to_glib_none();
///
///         let mut attrs = Box::new(ffi::GdkWindowAttr {
///             title: title.0,
///             // ....
///         });
///
///         Stash(&mut *attrs, (attrs, title))
///     }
/// }
/// ```
pub struct Stash<'a, P: Copy, T: ?Sized + ToGlibPtr<'a, P>>(
    pub P,
    pub <T as ToGlibPtr<'a, P>>::Storage,
);

pub struct StashMut<'a, P: Copy, T: ?Sized>(pub P, pub <T as ToGlibPtrMut<'a, P>>::Storage)
where
    T: ToGlibPtrMut<'a, P>;

/// Wrapper around values representing borrowed C memory.
///
/// This is returned by `from_glib_borrow()` and ensures that the wrapped value
/// is never dropped when going out of scope.
///
/// Borrowed values must never be passed by value or mutable reference to safe Rust code and must
/// not leave the C scope in which they are valid.
#[derive(Debug)]
pub struct Borrowed<T>(mem::ManuallyDrop<T>);

impl<T> Borrowed<T> {
    /// Creates a new borrowed value.
    pub fn new(val: T) -> Self {
        Self(mem::ManuallyDrop::new(val))
    }

    /// Extracts the contained value.
    ///
    /// # Safety
    ///
    /// The returned value must never be dropped and instead has to be passed to `mem::forget()` or
    /// be directly wrapped in `mem::ManuallyDrop` or another `Borrowed` wrapper.
    pub unsafe fn into_inner(self) -> T {
        mem::ManuallyDrop::into_inner(self.0)
    }
}

impl<T> AsRef<T> for Borrowed<T> {
    fn as_ref(&self) -> &T {
        &*self.0
    }
}

impl<T> std::ops::Deref for Borrowed<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}

/// Translate a simple type.
pub trait ToGlib {
    type GlibType: Copy;

    fn to_glib(&self) -> Self::GlibType;
}

impl ToGlib for bool {
    type GlibType = ffi::gboolean;

    #[inline]
    fn to_glib(&self) -> ffi::gboolean {
        if *self {
            ffi::GTRUE
        } else {
            ffi::GFALSE
        }
    }
}

impl ToGlib for char {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        *self as u32
    }
}

impl ToGlib for Option<char> {
    type GlibType = u32;

    #[inline]
    fn to_glib(&self) -> u32 {
        self.as_ref().map(|&c| c as u32).unwrap_or(0)
    }
}

impl ToGlib for Ordering {
    type GlibType = i32;

    #[inline]
    fn to_glib(&self) -> i32 {
        match *self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

/// A Rust type `T` for which `Option<T>` translates to the same glib type as T.
pub trait OptionToGlib: ToGlib {
    const GLIB_NONE: Self::GlibType;
}

impl<T: OptionToGlib> ToGlib for Option<T> {
    type GlibType = T::GlibType;

    #[inline]
    fn to_glib(&self) -> Self::GlibType {
        match self {
            Some(t) => t.to_glib(),
            None => T::GLIB_NONE,
        }
    }
}

/// Provides the default pointer type to be used in some container conversions.
///
/// It's `*mut c_char` for `String`, `*mut GtkButton` for `gtk::Button`, etc.
pub trait GlibPtrDefault {
    type GlibType: Ptr;
}

impl<'a, T: ?Sized + GlibPtrDefault> GlibPtrDefault for &'a T {
    type GlibType = <T as GlibPtrDefault>::GlibType;
}

/// Translate to a pointer.
pub trait ToGlibPtr<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn to_glib_none(&'a self) -> Stash<'a, P, Self>;

    /// Transfer: container.
    ///
    /// We transfer the container ownership to the foreign library retaining
    /// the elements ownership.
    fn to_glib_container(&'a self) -> Stash<'a, P, Self> {
        unimplemented!();
    }

    /// Transfer: full.
    ///
    /// We transfer the ownership to the foreign library.
    fn to_glib_full(&self) -> P {
        unimplemented!();
    }
}
///
/// Translate to a pointer with a mutable borrow.
pub trait ToGlibPtrMut<'a, P: Copy> {
    type Storage;

    /// Transfer: none.
    ///
    /// The pointer in the `Stash` is only valid for the lifetime of the `Stash`.
    fn to_glib_none_mut(&'a mut self) -> StashMut<P, Self>;
}

impl<'a, P: Ptr, T: ToGlibPtr<'a, P>> ToGlibPtr<'a, P> for Option<T> {
    type Storage = Option<<T as ToGlibPtr<'a, P>>::Storage>;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Option<T>> {
        self.as_ref()
            .map_or(Stash(Ptr::from::<()>(ptr::null_mut()), None), |s| {
                let s = s.to_glib_none();
                Stash(s.0, Some(s.1))
            })
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        self.as_ref()
            .map_or(Ptr::from::<()>(ptr::null_mut()), ToGlibPtr::to_glib_full)
    }
}

impl<'a, 'opt: 'a, P: Ptr, T: ToGlibPtrMut<'a, P>> ToGlibPtrMut<'a, P> for Option<&'opt mut T> {
    type Storage = Option<<T as ToGlibPtrMut<'a, P>>::Storage>;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, P, Option<&'opt mut T>> {
        self.as_mut()
            .map_or(StashMut(Ptr::from::<()>(ptr::null_mut()), None), |s| {
                let s = s.to_glib_none_mut();
                StashMut(s.0, Some(s.1))
            })
    }
}

impl<'a, P: Ptr, T: ?Sized + ToGlibPtr<'a, P>> ToGlibPtr<'a, P> for &'a T {
    type Storage = <T as ToGlibPtr<'a, P>>::Storage;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Self> {
        let s = (*self).to_glib_none();
        Stash(s.0, s.1)
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        (*self).to_glib_full()
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp =
            CString::new(self).expect("str::ToGlibPtr<*const c_char>: unexpected '\0' character");
        Stash(tmp.as_ptr(), tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t) as *const c_char
        }
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for str {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp =
            CString::new(self).expect("str::ToGlibPtr<*mut c_char>: unexpected '\0' character");
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        unsafe { ffi::g_strndup(self.as_ptr() as *mut c_char, self.len() as size_t) }
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for String {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *const c_char, String> {
        let tmp = CString::new(&self[..])
            .expect("String::ToGlibPtr<*const c_char>: unexpected '\0' character");
        Stash(tmp.as_ptr(), tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *const c_char {
        unsafe {
            ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t) as *const c_char
        }
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for String {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut c_char, String> {
        let tmp = CString::new(&self[..])
            .expect("String::ToGlibPtr<*mut c_char>: unexpected '\0' character");
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut c_char {
        unsafe {
            ffi::g_strndup(self.as_ptr() as *const c_char, self.len() as size_t) as *mut c_char
        }
    }
}

impl GlibPtrDefault for str {
    type GlibType = *mut c_char;
}

impl GlibPtrDefault for String {
    type GlibType = *mut c_char;
}

#[cfg(not(windows))]
fn path_to_c(path: &Path) -> CString {
    // GLib paths on UNIX are always in the local encoding, just like in Rust
    //
    // Paths on UNIX must not contain NUL bytes, in which case the conversion
    // to a CString would fail. The only thing we can do then is to panic, as passing
    // NULL or the empty string to GLib would cause undefined behaviour.
    CString::new(path.as_os_str().as_bytes()).expect("Invalid path with NUL bytes")
}

#[cfg(windows)]
fn path_to_c(path: &Path) -> CString {
    // GLib paths are always UTF-8 strings on Windows, while in Rust they are
    // WTF-8. As such, we need to convert to a UTF-8 string. This conversion can
    // fail, see https://simonsapin.github.io/wtf-8/#converting-wtf-8-utf-8
    //
    // It's not clear what we're supposed to do if it fails: the path is not
    // representable in UTF-8 and thus can't possibly be passed to GLib.
    // Passing NULL or the empty string to GLib can lead to undefined behaviour, so
    // the only safe option seems to be to simply panic here.
    let path_str = path
        .to_str()
        .expect("Path can't be represented as UTF-8")
        .to_owned();

    // On Windows, paths can have \\?\ prepended for long-path support. See
    // MSDN documentation about CreateFile
    //
    // We have to get rid of this and let GLib take care of all these
    // weirdnesses later
    if path_str.starts_with("\\\\?\\") {
        CString::new(path_str[4..].as_bytes())
    } else {
        CString::new(path_str.as_bytes())
    }
    .expect("Invalid path with NUL bytes")
}

#[cfg(not(windows))]
fn os_str_to_c(s: &OsStr) -> CString {
    // GLib OS string (environment strings) on UNIX are always in the local encoding,
    // just like in Rust
    //
    // OS string on UNIX must not contain NUL bytes, in which case the conversion
    // to a CString would fail. The only thing we can do then is to panic, as passing
    // NULL or the empty string to GLib would cause undefined behaviour.
    CString::new(s.as_bytes()).expect("Invalid OS String with NUL bytes")
}

#[cfg(windows)]
fn os_str_to_c(s: &OsStr) -> CString {
    // GLib OS string (environment strings) are always UTF-8 strings on Windows,
    // while in Rust they are WTF-8. As such, we need to convert to a UTF-8 string.
    // This conversion can fail, see https://simonsapin.github.io/wtf-8/#converting-wtf-8-utf-8
    //
    // It's not clear what we're supposed to do if it fails: the OS string is not
    // representable in UTF-8 and thus can't possibly be passed to GLib.
    // Passing NULL or the empty string to GLib can lead to undefined behaviour, so
    // the only safe option seems to be to simply panic here.
    let os_str = s
        .to_str()
        .expect("OS String can't be represented as UTF-8")
        .to_owned();

    CString::new(os_str.as_bytes()).expect("Invalid OS string with NUL bytes")
}

impl<'a> ToGlibPtr<'a, *const c_char> for Path {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for Path {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for PathBuf {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for PathBuf {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = path_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl GlibPtrDefault for Path {
    type GlibType = *mut c_char;
}

impl GlibPtrDefault for PathBuf {
    type GlibType = *mut c_char;
}

impl<'a> ToGlibPtr<'a, *const c_char> for OsStr {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = os_str_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for OsStr {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = os_str_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl<'a> ToGlibPtr<'a, *const c_char> for OsString {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const c_char, Self> {
        let tmp = os_str_to_c(self);
        Stash(tmp.as_ptr(), tmp)
    }
}

impl<'a> ToGlibPtr<'a, *mut c_char> for OsString {
    type Storage = CString;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut c_char, Self> {
        let tmp = os_str_to_c(self);
        Stash(tmp.as_ptr() as *mut c_char, tmp)
    }
}

impl GlibPtrDefault for OsStr {
    type GlibType = *mut c_char;
}

impl GlibPtrDefault for OsString {
    type GlibType = *mut c_char;
}

pub trait ToGlibContainerFromSlice<'a, P>
where
    Self: Sized,
{
    type Storage;

    fn to_glib_none_from_slice(t: &'a [Self]) -> (P, Self::Storage);
    fn to_glib_container_from_slice(t: &'a [Self]) -> (P, Self::Storage);
    fn to_glib_full_from_slice(t: &[Self]) -> P;
}

macro_rules! impl_to_glib_container_from_slice_fundamental {
    ($name:ty) => {
        impl<'a> ToGlibContainerFromSlice<'a, *mut $name> for $name {
            type Storage = &'a [$name];

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut $name, &'a [$name]) {
                (t.as_ptr() as *mut $name, t)
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut $name, &'a [$name]) {
                (ToGlibContainerFromSlice::to_glib_full_from_slice(t), t)
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut $name {
                if t.len() == 0 {
                    return ptr::null_mut();
                }

                unsafe {
                    let res = ffi::g_malloc(mem::size_of::<$name>() * t.len()) as *mut $name;
                    ptr::copy_nonoverlapping(t.as_ptr(), res, t.len());
                    res
                }
            }
        }
    };
}

impl_to_glib_container_from_slice_fundamental!(u8);
impl_to_glib_container_from_slice_fundamental!(i8);
impl_to_glib_container_from_slice_fundamental!(u16);
impl_to_glib_container_from_slice_fundamental!(i16);
impl_to_glib_container_from_slice_fundamental!(u32);
impl_to_glib_container_from_slice_fundamental!(i32);
impl_to_glib_container_from_slice_fundamental!(u64);
impl_to_glib_container_from_slice_fundamental!(i64);
impl_to_glib_container_from_slice_fundamental!(f32);
impl_to_glib_container_from_slice_fundamental!(f64);

macro_rules! impl_to_glib_container_from_slice_string {
    ($name:ty, $ffi_name:ty) => {
        impl<'a> ToGlibContainerFromSlice<'a, *mut $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, $ffi_name, $name>>, Option<Vec<$ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(ptr::null_mut() as $ffi_name);

                (v_ptr.as_ptr() as *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();

                let v_ptr = unsafe {
                    let v_ptr = ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1))
                        as *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *mut $ffi_name {
                unsafe {
                    let v_ptr = ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1))
                        as *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ptr::write(v_ptr.add(i), s.to_glib_full());
                    }

                    v_ptr
                }
            }
        }
        impl<'a> ToGlibContainerFromSlice<'a, *const $ffi_name> for $name {
            type Storage = (Vec<Stash<'a, $ffi_name, $name>>, Option<Vec<$ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name]) -> (*const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(ptr::null_mut() as $ffi_name);

                (v_ptr.as_ptr() as *const $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name]) -> (*const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();

                let v_ptr = unsafe {
                    let v_ptr = ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1))
                        as *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr as *const $ffi_name
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name]) -> *const $ffi_name {
                unsafe {
                    let v_ptr = ffi::g_malloc0(mem::size_of::<$ffi_name>() * (t.len() + 1))
                        as *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        ptr::write(v_ptr.add(i), s.to_glib_full());
                    }

                    v_ptr as *const $ffi_name
                }
            }
        }
    };
}

impl_to_glib_container_from_slice_string!(&'a str, *mut c_char);
impl_to_glib_container_from_slice_string!(&'a str, *const c_char);
impl_to_glib_container_from_slice_string!(String, *mut c_char);
impl_to_glib_container_from_slice_string!(String, *const c_char);
impl_to_glib_container_from_slice_string!(&'a Path, *mut c_char);
impl_to_glib_container_from_slice_string!(&'a Path, *const c_char);
impl_to_glib_container_from_slice_string!(PathBuf, *mut c_char);
impl_to_glib_container_from_slice_string!(PathBuf, *const c_char);
impl_to_glib_container_from_slice_string!(&'a OsStr, *mut c_char);
impl_to_glib_container_from_slice_string!(&'a OsStr, *const c_char);
impl_to_glib_container_from_slice_string!(OsString, *mut c_char);
impl_to_glib_container_from_slice_string!(OsString, *const c_char);

impl<'a, T> ToGlibContainerFromSlice<'a, *mut ffi::GList> for T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<List>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*mut ffi::GList, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().rev().map(ToGlibPtr::to_glib_none).collect();
        let mut list: *mut ffi::GList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = ffi::g_list_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (Some(List(list)), stash_vec))
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [T]) -> (*mut ffi::GList, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().rev().map(ToGlibPtr::to_glib_none).collect();
        let mut list: *mut ffi::GList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = ffi::g_list_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (None, stash_vec))
    }

    #[inline]
    fn to_glib_full_from_slice(t: &[T]) -> *mut ffi::GList {
        let mut list: *mut ffi::GList = ptr::null_mut();
        unsafe {
            for ptr in t.iter().rev().map(ToGlibPtr::to_glib_full) {
                list = ffi::g_list_prepend(list, Ptr::to(ptr));
            }
        }
        list
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const ffi::GList> for T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<List>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*const ffi::GList, Self::Storage) {
        let (list, stash) = ToGlibContainerFromSlice::<*mut ffi::GList>::to_glib_none_from_slice(t);
        (list as *const ffi::GList, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [T]) -> (*const ffi::GList, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[T]) -> *const ffi::GList {
        unimplemented!()
    }
}

pub struct List(*mut ffi::GList);

impl Drop for List {
    fn drop(&mut self) {
        unsafe { ffi::g_list_free(self.0) }
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *mut ffi::GSList> for &'a T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<SList>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, &'a T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [&'a T]) -> (*mut ffi::GSList, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().rev().map(ToGlibPtr::to_glib_none).collect();
        let mut list: *mut ffi::GSList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (Some(SList(list)), stash_vec))
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [&'a T]) -> (*mut ffi::GSList, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().rev().map(ToGlibPtr::to_glib_none).collect();
        let mut list: *mut ffi::GSList = ptr::null_mut();
        unsafe {
            for stash in &stash_vec {
                list = ffi::g_slist_prepend(list, Ptr::to(stash.0));
            }
        }
        (list, (None, stash_vec))
    }

    #[inline]
    fn to_glib_full_from_slice(t: &[&'a T]) -> *mut ffi::GSList {
        let mut list: *mut ffi::GSList = ptr::null_mut();
        unsafe {
            for ptr in t.iter().rev().map(ToGlibPtr::to_glib_full) {
                list = ffi::g_slist_prepend(list, Ptr::to(ptr));
            }
        }
        list
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const ffi::GSList> for &'a T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<SList>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, &'a T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [&'a T]) -> (*const ffi::GSList, Self::Storage) {
        let (list, stash) =
            ToGlibContainerFromSlice::<*mut ffi::GSList>::to_glib_none_from_slice(t);
        (list as *const ffi::GSList, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [&'a T]) -> (*const ffi::GSList, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[&'a T]) -> *const ffi::GSList {
        unimplemented!()
    }
}

pub struct SList(*mut ffi::GSList);

impl Drop for SList {
    fn drop(&mut self) {
        unsafe { ffi::g_slist_free(self.0) }
    }
}

impl<'a, P: Ptr, T: ToGlibContainerFromSlice<'a, P>> ToGlibPtr<'a, P> for [T] {
    type Storage = T::Storage;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, P, Self> {
        let result = ToGlibContainerFromSlice::to_glib_none_from_slice(self);
        Stash(result.0, result.1)
    }

    #[inline]
    fn to_glib_container(&'a self) -> Stash<'a, P, Self> {
        let result = ToGlibContainerFromSlice::to_glib_container_from_slice(self);
        Stash(result.0, result.1)
    }

    #[inline]
    fn to_glib_full(&self) -> P {
        ToGlibContainerFromSlice::to_glib_full_from_slice(self)
    }
}

#[allow(clippy::implicit_hasher)]
impl<'a> ToGlibPtr<'a, *mut ffi::GHashTable> for HashMap<String, String> {
    type Storage = HashTable;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::GHashTable, Self> {
        let ptr = self.to_glib_full();
        Stash(ptr, HashTable(ptr))
    }

    #[inline]
    fn to_glib_full(&self) -> *mut ffi::GHashTable {
        unsafe {
            let ptr = ffi::g_hash_table_new_full(
                Some(ffi::g_str_hash),
                Some(ffi::g_str_equal),
                Some(ffi::g_free),
                Some(ffi::g_free),
            );
            for (k, v) in self {
                let k: *mut c_char = k.to_glib_full();
                let v: *mut c_char = v.to_glib_full();
                ffi::g_hash_table_insert(ptr, k as *mut _, v as *mut _);
            }
            ptr
        }
    }
}

pub struct HashTable(*mut ffi::GHashTable);

impl Drop for HashTable {
    fn drop(&mut self) {
        unsafe { ffi::g_hash_table_unref(self.0) }
    }
}

pub struct PtrArray(*mut ffi::GPtrArray);

impl Drop for PtrArray {
    fn drop(&mut self) {
        unsafe {
            ffi::g_ptr_array_unref(self.0);
        }
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *mut ffi::GPtrArray> for T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<PtrArray>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*mut ffi::GPtrArray, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();
        let arr = unsafe { ffi::g_ptr_array_sized_new(t.len() as _) };
        unsafe {
            for stash in &stash_vec {
                ffi::g_ptr_array_add(arr, Ptr::to(stash.0));
            }
        }
        (arr, (Some(PtrArray(arr)), stash_vec))
    }

    #[inline]
    fn to_glib_container_from_slice(t: &'a [T]) -> (*mut ffi::GPtrArray, Self::Storage) {
        let stash_vec: Vec<_> = t.iter().map(ToGlibPtr::to_glib_none).collect();
        let arr = unsafe { ffi::g_ptr_array_sized_new(t.len() as _) };
        unsafe {
            for stash in &stash_vec {
                ffi::g_ptr_array_add(arr, Ptr::to(stash.0));
            }
        }
        (arr, (None, stash_vec))
    }

    #[inline]
    fn to_glib_full_from_slice(t: &[T]) -> *mut ffi::GPtrArray {
        let arr = unsafe { ffi::g_ptr_array_sized_new(t.len() as _) };
        unsafe {
            for ptr in t.iter().map(ToGlibPtr::to_glib_full) {
                ffi::g_ptr_array_add(arr, Ptr::to(ptr));
            }
        }
        arr
    }
}

impl<'a, T> ToGlibContainerFromSlice<'a, *const ffi::GPtrArray> for T
where
    T: GlibPtrDefault + ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
{
    type Storage = (
        Option<PtrArray>,
        Vec<Stash<'a, <T as GlibPtrDefault>::GlibType, T>>,
    );

    #[inline]
    fn to_glib_none_from_slice(t: &'a [T]) -> (*const ffi::GPtrArray, Self::Storage) {
        let (arr, stash) =
            ToGlibContainerFromSlice::<*mut ffi::GPtrArray>::to_glib_none_from_slice(t);
        (arr as *const ffi::GPtrArray, stash)
    }

    #[inline]
    fn to_glib_container_from_slice(_t: &'a [T]) -> (*const ffi::GPtrArray, Self::Storage) {
        unimplemented!()
    }

    #[inline]
    fn to_glib_full_from_slice(_t: &[T]) -> *const ffi::GPtrArray {
        unimplemented!()
    }
}

/// Translate a simple type.
pub trait FromGlib<G: Copy>: Sized {
    unsafe fn from_glib(val: G) -> Self;
}

/// Translate a simple type.
#[inline]
pub unsafe fn from_glib<G: Copy, T: FromGlib<G>>(val: G) -> T {
    FromGlib::from_glib(val)
}

impl FromGlib<ffi::gboolean> for bool {
    #[inline]
    unsafe fn from_glib(val: ffi::gboolean) -> bool {
        val != ffi::GFALSE
    }
}

impl FromGlib<i32> for Ordering {
    #[inline]
    unsafe fn from_glib(val: i32) -> Ordering {
        val.cmp(&0)
    }
}

/// Translate from a Glib type which can result in an undefined and/or invalid value.
pub trait TryFromGlib<G: Copy>: Sized {
    type Error;
    fn try_from_glib(val: G) -> Result<Self, Self::Error>;
}

/// Error type for [`TryFromGlib`] when the Glib value is None.
#[derive(Debug, PartialEq, Eq)]
pub struct GlibNoneError;

impl fmt::Display for GlibNoneError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "glib value is None")
    }
}

impl std::error::Error for GlibNoneError {}

impl<G: Copy, T: TryFromGlib<G, Error = GlibNoneError>> FromGlib<G> for Option<T> {
    #[inline]
    unsafe fn from_glib(val: G) -> Option<T> {
        T::try_from_glib(val).ok()
    }
}

/// Error type for [`TryFromGlib`] when the Glib value can be None or invalid.
#[derive(Debug, Eq, PartialEq)]
pub enum GlibNoneOrInvalidError<I: Error> {
    Invalid(I),
    None,
}

impl<I: Error> GlibNoneOrInvalidError<I> {
    /// Builds the `None` variant.
    pub fn none() -> Self {
        GlibNoneOrInvalidError::None
    }

    /// Returns `true` if `self` is the `None` variant.
    pub fn is_none(&self) -> bool {
        matches!(self, GlibNoneOrInvalidError::None)
    }

    /// Returns `true` if `self` is the `Invalid` variant.
    pub fn is_invalid(&self) -> bool {
        matches!(self, GlibNoneOrInvalidError::Invalid(_))
    }
}

impl<I: Error> From<I> for GlibNoneOrInvalidError<I> {
    fn from(invalid: I) -> Self {
        GlibNoneOrInvalidError::Invalid(invalid)
    }
}

impl<I: Error> fmt::Display for GlibNoneOrInvalidError<I> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlibNoneOrInvalidError::Invalid(err) => {
                write!(fmt, "glib value is invalid: ")?;
                fmt::Display::fmt(err, fmt)
            }
            GlibNoneOrInvalidError::None => write!(fmt, "glib value is None"),
        }
    }
}

impl<I: Error> Error for GlibNoneOrInvalidError<I> {}

impl<G: Copy, I: Error, T: TryFromGlib<G, Error = GlibNoneOrInvalidError<I>>> FromGlib<G>
    for Result<Option<T>, I>
{
    #[inline]
    unsafe fn from_glib(val: G) -> Result<Option<T>, I> {
        match T::try_from_glib(val) {
            Ok(value) => Ok(Some(value)),
            Err(GlibNoneOrInvalidError::None) => Ok(None),
            Err(GlibNoneOrInvalidError::Invalid(err)) => Err(err),
        }
    }
}

/// Translate from a pointer type which is annotated with `transfer none`.
/// The resulting value is referenced at least once, by the bindings.
///
/// This is suitable for floating references, which become strong references.
/// It is also suitable for acquiring non-gobject values, like `gchar*`.
///
/// <a name="safety_points"></a>
/// # Safety
///
/// The implementation of this trait should acquire a reference to the value
/// in a way appropriate to the type,
/// e.g. by increasing the reference count or copying.
/// Values obtained using this trait must be properly released on `drop()`
/// by the implementing type.
///
/// For more information, refer to module level documentation.
pub trait FromGlibPtrNone<P: Ptr>: Sized {
    /// # Safety
    ///
    /// See trait level [notes on safety](#safety_points)
    unsafe fn from_glib_none(ptr: P) -> Self;
}

/// Translate from a pointer type which is annotated with `transfer full`.
/// This transfers the ownership of the value to the Rust side.
///
/// Because ownership can only be transferred if something is already referenced,
/// this is unsuitable for floating references.
///
/// <a name="safety_points"></a>
/// # Safety
///
/// The implementation of this trait should not alter the reference count
/// or make copies of the underlying value.
/// Values obtained using this trait must be properly released on `drop()`
/// by the implementing type.
///
/// For more information, refer to module level documentation.
pub trait FromGlibPtrFull<P: Ptr>: Sized {
    /// # Safety
    ///
    /// See trait level [notes on safety](#safety_points)
    unsafe fn from_glib_full(ptr: P) -> Self;
}

/// Translate from a pointer type by borrowing, without affecting the refcount.
///
/// The purpose of this trait is to access values inside callbacks
/// without changing their reference status.
/// The obtained borrow must not be accessed outside of the scope of the callback,
/// and called procedures must not store any references to the underlying data.
/// Safe Rust code must never obtain a mutable Rust reference.
///
/// <a name="safety_points"></a>
/// # Safety
///
/// The implementation of this trait as well as the returned type
/// must satisfy the same constraints together.
/// They must not take ownership of the underlying value, copy it,
/// and should not change its rerefence count.
/// If it does, it must properly release obtained references.
///
/// The returned value, when dropped,
/// must leave the underlying value in the same state
/// as before from_glib_borrow was called:
/// - it must not be dropped,
/// - it must be the same type of reference, e.g. still floating.
///
/// For more information, refer to module level documentation.
pub trait FromGlibPtrBorrow<P: Ptr>: Sized {
    /// # Safety
    ///
    /// See trait level [notes on safety](#safety_points)
    unsafe fn from_glib_borrow(_ptr: P) -> Borrowed<Self> {
        unimplemented!();
    }
}

/// Translate from a pointer type, transfer: none.
///
/// See [`FromGlibPtrNone`](trait.FromGlibPtrNone.html).
#[inline]
pub unsafe fn from_glib_none<P: Ptr, T: FromGlibPtrNone<P>>(ptr: P) -> T {
    FromGlibPtrNone::from_glib_none(ptr)
}

/// Translate from a pointer type, transfer: full (assume ownership).
///
/// See [`FromGlibPtrFull`](trait.FromGlibPtrFull.html).
#[inline]
pub unsafe fn from_glib_full<P: Ptr, T: FromGlibPtrFull<P>>(ptr: P) -> T {
    FromGlibPtrFull::from_glib_full(ptr)
}

/// Translate from a pointer type, borrowing the pointer.
///
/// See [`FromGlibPtrBorrow`](trait.FromGlibPtrBorrow.html).
#[inline]
pub unsafe fn from_glib_borrow<P: Ptr, T: FromGlibPtrBorrow<P>>(ptr: P) -> Borrowed<T> {
    FromGlibPtrBorrow::from_glib_borrow(ptr)
}

impl<P: Ptr, T: FromGlibPtrNone<P>> FromGlibPtrNone<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_none(ptr: P) -> Option<T> {
        if ptr.is_null() {
            None
        } else {
            Some(from_glib_none(ptr))
        }
    }
}

impl<P: Ptr, T: FromGlibPtrBorrow<P>> FromGlibPtrBorrow<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: P) -> Borrowed<Option<T>> {
        if ptr.is_null() {
            Borrowed::new(None)
        } else {
            let val = T::from_glib_borrow(ptr);
            Borrowed::new(Some(val.into_inner()))
        }
    }
}

impl<P: Ptr, T: FromGlibPtrFull<P>> FromGlibPtrFull<P> for Option<T> {
    #[inline]
    unsafe fn from_glib_full(ptr: P) -> Option<T> {
        if ptr.is_null() {
            None
        } else {
            Some(from_glib_full(ptr))
        }
    }
}

impl FromGlibPtrNone<*const c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

// TODO: Deprecate this
impl FromGlibPtrFull<*const c_char> for String {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

// TODO: Deprecate this
impl FromGlibPtrNone<*mut c_char> for String {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).into_owned()
    }
}

// TODO: Deprecate this
impl FromGlibPtrFull<*mut c_char> for String {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

#[cfg(not(windows))]
unsafe fn c_to_path_buf(ptr: *const c_char) -> PathBuf {
    assert!(!ptr.is_null());

    // GLib paths on UNIX are always in the local encoding, which can be
    // UTF-8 or anything else really, but is always a NUL-terminated string
    // and must not contain any other NUL bytes
    OsString::from_vec(CStr::from_ptr(ptr).to_bytes().to_vec()).into()
}

#[cfg(windows)]
unsafe fn c_to_path_buf(ptr: *const c_char) -> PathBuf {
    assert!(!ptr.is_null());

    // GLib paths on Windows are always UTF-8, as such we can convert to a String
    // first and then go to a PathBuf from there. Unless there is a bug
    // in the C library, the conversion from UTF-8 can never fail so we can
    // safely panic here if that ever happens
    String::from_utf8(CStr::from_ptr(ptr).to_bytes().into())
        .expect("Invalid, non-UTF8 path")
        .into()
}

#[cfg(not(windows))]
unsafe fn c_to_os_string(ptr: *const c_char) -> OsString {
    assert!(!ptr.is_null());

    // GLib OS string (environment strings) on UNIX are always in the local encoding,
    // which can be UTF-8 or anything else really, but is always a NUL-terminated string
    // and must not contain any other NUL bytes
    OsString::from_vec(CStr::from_ptr(ptr).to_bytes().to_vec())
}

#[cfg(windows)]
unsafe fn c_to_os_string(ptr: *const c_char) -> OsString {
    assert!(!ptr.is_null());

    // GLib OS string (environment strings) on Windows are always UTF-8,
    // as such we can convert to a String
    // first and then go to a OsString from there. Unless there is a bug
    // in the C library, the conversion from UTF-8 can never fail so we can
    // safely panic here if that ever happens
    String::from_utf8(CStr::from_ptr(ptr).to_bytes().into())
        .expect("Invalid, non-UTF8 path")
        .into()
}

impl FromGlibPtrNone<*const c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_path_buf(ptr)
    }
}

impl FromGlibPtrFull<*const c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrNone<*mut c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_path_buf(ptr)
    }
}

impl FromGlibPtrFull<*mut c_char> for PathBuf {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrNone<*const c_char> for OsString {
    #[inline]
    unsafe fn from_glib_none(ptr: *const c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_os_string(ptr)
    }
}

impl FromGlibPtrFull<*const c_char> for OsString {
    #[inline]
    unsafe fn from_glib_full(ptr: *const c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

impl FromGlibPtrNone<*mut c_char> for OsString {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut c_char) -> Self {
        assert!(!ptr.is_null());
        c_to_os_string(ptr)
    }
}

impl FromGlibPtrFull<*mut c_char> for OsString {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut c_char) -> Self {
        let res = from_glib_none(ptr);
        ffi::g_free(ptr as *mut _);
        res
    }
}

/// Translate from a container.
pub trait FromGlibContainer<T, P: Ptr>: Sized {
    /// Transfer: none.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_none_num(ptr: P, num: usize) -> Self;

    /// Transfer: container.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_container_num(ptr: P, num: usize) -> Self;

    /// Transfer: full.
    ///
    /// `num` is the advised number of elements.
    unsafe fn from_glib_full_num(ptr: P, num: usize) -> Self;
}

/// Translate from a container of pointers.
pub trait FromGlibPtrContainer<P: Ptr, PP: Ptr>: FromGlibContainer<P, PP> + Sized {
    /// Transfer: none.
    unsafe fn from_glib_none(ptr: PP) -> Self;

    /// Transfer: container.
    unsafe fn from_glib_container(ptr: PP) -> Self;

    /// Transfer: full.
    unsafe fn from_glib_full(ptr: PP) -> Self;
}

pub unsafe fn c_ptr_array_len<P: Ptr>(mut ptr: *const P) -> usize {
    let mut len = 0;

    if !ptr.is_null() {
        while !(*ptr).is_null() {
            len += 1;
            ptr = ptr.offset(1);
        }
    }
    len
}

pub trait FromGlibContainerAsVec<T, P: Ptr>
where
    Self: Sized,
{
    unsafe fn from_glib_none_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
    unsafe fn from_glib_container_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
    unsafe fn from_glib_full_num_as_vec(ptr: P, num: usize) -> Vec<Self>;
}

pub trait FromGlibPtrArrayContainerAsVec<P: Ptr, PP: Ptr>: FromGlibContainerAsVec<P, PP>
where
    Self: Sized,
{
    unsafe fn from_glib_none_as_vec(ptr: PP) -> Vec<Self>;
    unsafe fn from_glib_container_as_vec(ptr: PP) -> Vec<Self>;
    unsafe fn from_glib_full_as_vec(ptr: PP) -> Vec<Self>;
}

impl FromGlibContainerAsVec<bool, *const ffi::gboolean> for bool {
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::gboolean, num: usize) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::gboolean, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::gboolean, _: usize) -> Vec<Self> {
        // Can't really free a *const
        unimplemented!();
    }
}

impl FromGlibContainerAsVec<bool, *mut ffi::gboolean> for bool {
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::gboolean, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::gboolean, num: usize) -> Vec<Self> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::gboolean, num: usize) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }
}

macro_rules! impl_from_glib_container_as_vec_fundamental {
    ($name:ty) => {
        impl FromGlibContainerAsVec<$name, *const $name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *const $name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(ptr::read(ptr.add(i)));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(_: *const $name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }

            unsafe fn from_glib_full_num_as_vec(_: *const $name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }
        }

        impl FromGlibContainerAsVec<$name, *mut $name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut $name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
            }
        }
    };
}

impl_from_glib_container_as_vec_fundamental!(u8);
impl_from_glib_container_as_vec_fundamental!(i8);
impl_from_glib_container_as_vec_fundamental!(u16);
impl_from_glib_container_as_vec_fundamental!(i16);
impl_from_glib_container_as_vec_fundamental!(u32);
impl_from_glib_container_as_vec_fundamental!(i32);
impl_from_glib_container_as_vec_fundamental!(u64);
impl_from_glib_container_as_vec_fundamental!(i64);
impl_from_glib_container_as_vec_fundamental!(f32);
impl_from_glib_container_as_vec_fundamental!(f64);

macro_rules! impl_from_glib_container_as_vec_string {
    ($name:ty, $ffi_name:ty) => {
        impl FromGlibContainerAsVec<$ffi_name, *const $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *const $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_none(ptr::read(ptr.add(i)) as $ffi_name));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(_: *const $ffi_name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }

            unsafe fn from_glib_full_num_as_vec(_: *const $ffi_name, _: usize) -> Vec<Self> {
                // Can't really free a *const
                unimplemented!();
            }
        }

        impl FromGlibContainerAsVec<$ffi_name, *mut $ffi_name> for $name {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *const _, num)
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push(from_glib_full(ptr::read(ptr.add(i))));
                }
                ffi::g_free(ptr as *mut _);
                res
            }
        }

        impl FromGlibPtrArrayContainerAsVec<$ffi_name, *mut $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
            }
        }

        impl FromGlibPtrArrayContainerAsVec<$ffi_name, *const $ffi_name> for $name {
            unsafe fn from_glib_none_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *const $ffi_name) -> Vec<Self> {
                FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
            }
        }
    };
}

// TODO: Deprecate this
impl_from_glib_container_as_vec_string!(String, *const c_char);
impl_from_glib_container_as_vec_string!(String, *mut c_char);

impl_from_glib_container_as_vec_string!(PathBuf, *const c_char);
impl_from_glib_container_as_vec_string!(PathBuf, *mut c_char);
impl_from_glib_container_as_vec_string!(OsString, *const c_char);
impl_from_glib_container_as_vec_string!(OsString, *mut c_char);

impl<P, PP: Ptr, T: FromGlibContainerAsVec<P, PP>> FromGlibContainer<P, PP> for Vec<T> {
    unsafe fn from_glib_none_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num)
    }

    unsafe fn from_glib_container_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, num)
    }

    unsafe fn from_glib_full_num(ptr: PP, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, num)
    }
}

impl<P: Ptr, PP: Ptr, T: FromGlibPtrArrayContainerAsVec<P, PP>> FromGlibPtrContainer<P, PP>
    for Vec<T>
{
    unsafe fn from_glib_none(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr)
    }

    unsafe fn from_glib_container(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_container_as_vec(ptr)
    }

    unsafe fn from_glib_full(ptr: PP) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_full_as_vec(ptr)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GSList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(mut ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            if ptr.is_null() {
                break;
            }

            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_slist_free(ptr);
        res
    }

    unsafe fn from_glib_full_num_as_vec(mut ptr: *mut ffi::GSList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            if ptr.is_null() {
                break;
            }

            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        ffi::g_slist_free(orig_ptr);
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GSList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(mut ptr: *mut ffi::GSList) -> Vec<T> {
        let mut res = Vec::new();
        while !ptr.is_null() {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::GSList) -> Vec<T> {
        let res = FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr);
        ffi::g_slist_free(ptr);
        res
    }

    unsafe fn from_glib_full_as_vec(mut ptr: *mut ffi::GSList) -> Vec<T> {
        let orig_ptr = ptr;
        let mut res = Vec::new();
        while !ptr.is_null() {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        ffi::g_slist_free(orig_ptr);
        res
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(mut ptr: *mut ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            if ptr.is_null() {
                break;
            }

            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GList, num: usize) -> Vec<T> {
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_list_free(ptr);
        res
    }

    unsafe fn from_glib_full_num_as_vec(mut ptr: *mut ffi::GList, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let orig_ptr = ptr;
        let mut res = Vec::with_capacity(num);
        for _ in 0..num {
            if ptr.is_null() {
                break;
            }

            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        ffi::g_list_free(orig_ptr);
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(mut ptr: *mut ffi::GList) -> Vec<T> {
        let mut res = Vec::new();
        while !ptr.is_null() {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
            ptr = (*ptr).next;
        }
        res
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::GList) -> Vec<T> {
        let res = FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr);
        ffi::g_list_free(ptr);
        res
    }

    unsafe fn from_glib_full_as_vec(mut ptr: *mut ffi::GList) -> Vec<T> {
        let orig_ptr = ptr;
        let mut res = Vec::new();
        while !ptr.is_null() {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from((*ptr).data);
            if !item_ptr.is_null() {
                res.push(from_glib_full(item_ptr));
            }
            ptr = (*ptr).next;
        }
        ffi::g_list_free(orig_ptr);
        res
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GList, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(ptr: *const ffi::GList) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const ffi::GList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const ffi::GList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GSList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GSList, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GSList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GSList, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GSList> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(ptr: *const ffi::GSList) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const ffi::GSList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const ffi::GSList) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

#[allow(clippy::implicit_hasher)]
impl FromGlibContainer<*const c_char, *mut ffi::GHashTable> for HashMap<String, String> {
    unsafe fn from_glib_none_num(ptr: *mut ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_none(ptr)
    }

    unsafe fn from_glib_container_num(ptr: *mut ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_full_num(ptr: *mut ffi::GHashTable, _: usize) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }
}

#[allow(clippy::implicit_hasher)]
impl FromGlibPtrContainer<*const c_char, *mut ffi::GHashTable> for HashMap<String, String> {
    unsafe fn from_glib_none(ptr: *mut ffi::GHashTable) -> Self {
        unsafe extern "C" fn read_string_hash_table(
            key: ffi::gpointer,
            value: ffi::gpointer,
            hash_map: ffi::gpointer,
        ) {
            let key: String = from_glib_none(key as *const c_char);
            let value: String = from_glib_none(value as *const c_char);
            let hash_map: &mut HashMap<String, String> =
                &mut *(hash_map as *mut HashMap<String, String>);
            hash_map.insert(key, value);
        }
        let mut map = HashMap::new();
        ffi::g_hash_table_foreach(
            ptr,
            Some(read_string_hash_table),
            &mut map as *mut HashMap<String, String> as *mut _,
        );
        map
    }

    unsafe fn from_glib_container(ptr: *mut ffi::GHashTable) -> Self {
        FromGlibPtrContainer::from_glib_full(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut ffi::GHashTable) -> Self {
        let map = FromGlibPtrContainer::from_glib_none(ptr);
        ffi::g_hash_table_unref(ptr);
        map
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GPtrArray> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(ptr: *mut ffi::GPtrArray, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let pdata = (*ptr).pdata;
        assert!((*ptr).len as usize >= num);
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from(ptr::read(pdata.add(i)));
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(ptr: *mut ffi::GPtrArray, num: usize) -> Vec<T> {
        let res = FromGlibContainer::from_glib_none_num(ptr, num);
        if !ptr.is_null() {
            ffi::g_ptr_array_unref(ptr);
        }
        res
    }

    unsafe fn from_glib_full_num_as_vec(ptr: *mut ffi::GPtrArray, num: usize) -> Vec<T> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }
        let pdata = (*ptr).pdata;
        assert!((*ptr).len as usize >= num);
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            let item_ptr: <T as GlibPtrDefault>::GlibType = Ptr::from(ptr::read(pdata.add(i)));
            if !item_ptr.is_null() {
                res.push(from_glib_none(item_ptr));
            }
        }
        ffi::g_ptr_array_unref(ptr);
        res
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *mut ffi::GPtrArray> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(ptr: *mut ffi::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_none_num(ptr, num)
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut ffi::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut ffi::GPtrArray) -> Vec<T> {
        let num = (*ptr).len as usize;
        FromGlibContainer::from_glib_full_num(ptr, num)
    }
}

impl<T> FromGlibContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GPtrArray> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_num_as_vec(ptr: *const ffi::GPtrArray, num: usize) -> Vec<T> {
        FromGlibContainerAsVec::from_glib_none_num_as_vec(mut_override(ptr), num)
    }

    unsafe fn from_glib_container_num_as_vec(_: *const ffi::GPtrArray, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(_: *const ffi::GPtrArray, _: usize) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

impl<T> FromGlibPtrArrayContainerAsVec<<T as GlibPtrDefault>::GlibType, *const ffi::GPtrArray> for T
where
    T: GlibPtrDefault
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>,
{
    unsafe fn from_glib_none_as_vec(ptr: *const ffi::GPtrArray) -> Vec<T> {
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(mut_override(ptr))
    }

    unsafe fn from_glib_container_as_vec(_: *const ffi::GPtrArray) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const ffi::GPtrArray) -> Vec<T> {
        // Can't really free a *const
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;

    use super::*;
    use crate::GString;
    use std::collections::HashMap;

    #[test]
    fn string_hash_map() {
        let mut map = HashMap::new();
        map.insert("A".into(), "1".into());
        map.insert("B".into(), "2".into());
        map.insert("C".into(), "3".into());
        let ptr: *mut ffi::GHashTable = map.to_glib_full();
        let map = unsafe { HashMap::from_glib_full(ptr) };
        assert_eq!(map.get("A"), Some(&"1".into()));
        assert_eq!(map.get("B"), Some(&"2".into()));
        assert_eq!(map.get("C"), Some(&"3".into()));
    }

    #[test]
    fn string_array() {
        let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let stash = v.to_glib_none();
        let ptr: *mut *mut c_char = stash.0;
        let ptr_copy = unsafe { ffi::g_strdupv(ptr) };

        let actual: Vec<String> = unsafe { FromGlibPtrContainer::from_glib_full(ptr_copy) };
        assert_eq!(v, actual);
    }

    #[test]
    fn gstring_array() {
        let v = vec!["A".to_string(), "B".to_string(), "C".to_string()];
        let stash = v.to_glib_none();
        let ptr: *mut *mut c_char = stash.0;
        let ptr_copy = unsafe { ffi::g_strdupv(ptr) };

        let actual: Vec<GString> = unsafe { FromGlibPtrContainer::from_glib_full(ptr_copy) };
        assert_eq!(v, actual);
    }

    #[test]
    fn ptr_array() {
        let strings = &["A", "B", "C"];
        let (ptr, _stash) =
            ToGlibContainerFromSlice::<*mut ffi::GPtrArray>::to_glib_none_from_slice(strings);
        let v: Vec<GString> = unsafe { FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr) };
        assert_eq!(&v, strings);
    }

    #[test]
    #[cfg(not(target_os = "macos"))]
    fn test_paths() {
        let tmp_dir = tempdir().unwrap();

        // Test if passing paths to GLib and getting them back
        // gives us useful results
        let dir_1 = tmp_dir.path().join("abcd");
        fs::create_dir(&dir_1).unwrap();
        assert_eq!(crate::path_get_basename(&dir_1), Path::new("abcd"));
        assert_eq!(
            crate::path_get_basename(dir_1.canonicalize().unwrap()),
            Path::new("abcd")
        );
        assert_eq!(
            crate::path_get_dirname(dir_1.canonicalize().unwrap()),
            tmp_dir.path()
        );
        assert!(crate::file_test(
            &dir_1,
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));
        assert!(crate::file_test(
            &dir_1.canonicalize().unwrap(),
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));

        // And test with some non-ASCII characters
        let dir_2 = tmp_dir.as_ref().join("øäöü");
        fs::create_dir(&dir_2).unwrap();
        assert_eq!(crate::path_get_basename(&dir_2), Path::new("øäöü"));
        assert_eq!(
            crate::path_get_basename(dir_2.canonicalize().unwrap()),
            Path::new("øäöü")
        );
        assert_eq!(
            crate::path_get_dirname(dir_2.canonicalize().unwrap()),
            tmp_dir.path()
        );
        assert!(crate::file_test(
            &dir_2,
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));
        assert!(crate::file_test(
            &dir_2.canonicalize().unwrap(),
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_paths() {
        let t_dir = tempdir().unwrap();
        let tmp_dir = t_dir.path().canonicalize().unwrap();

        // Test if passing paths to GLib and getting them back
        // gives us useful results
        let dir_1 = tmp_dir.join("abcd");
        fs::create_dir(&dir_1).unwrap();
        assert_eq!(crate::path_get_basename(&dir_1), Path::from("abcd"));
        assert_eq!(
            crate::path_get_basename(dir_1.canonicalize().unwrap()),
            Path::from("abcd")
        );
        assert_eq!(
            crate::path_get_dirname(dir_1.canonicalize().unwrap()),
            tmp_dir
        );
        assert!(crate::file_test(
            &dir_1,
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));
        assert!(crate::file_test(
            &dir_1.canonicalize().unwrap(),
            crate::FileTest::EXISTS | crate::FileTest::IS_DIR
        ));
    }

    #[test]
    fn none_value() {
        const CLONG_NONE: libc::c_long = -1;

        #[derive(Debug, PartialEq, Eq)]
        struct SpecialU32(u32);
        impl ToGlib for SpecialU32 {
            type GlibType = libc::c_uint;
            fn to_glib(&self) -> libc::c_uint {
                self.0 as libc::c_uint
            }
        }
        impl OptionToGlib for SpecialU32 {
            const GLIB_NONE: Self::GlibType = CLONG_NONE as libc::c_uint;
        }

        assert_eq!(SpecialU32(0).to_glib(), 0);
        assert_eq!(SpecialU32(42).to_glib(), 42);
        assert_eq!(Some(SpecialU32(0)).to_glib(), 0);
        assert_eq!(Some(SpecialU32(42)).to_glib(), 42);
        assert_eq!(Option::None::<SpecialU32>.to_glib(), SpecialU32::GLIB_NONE);

        impl TryFromGlib<libc::c_uint> for SpecialU32 {
            type Error = GlibNoneError;
            fn try_from_glib(val: libc::c_uint) -> Result<Self, GlibNoneError> {
                if val == SpecialU32::GLIB_NONE {
                    return Err(GlibNoneError);
                }

                Ok(SpecialU32(val as u32))
            }
        }

        assert_eq!(SpecialU32::try_from_glib(0), Ok(SpecialU32(0)));
        assert_eq!(SpecialU32::try_from_glib(42), Ok(SpecialU32(42)));
        assert_eq!(
            SpecialU32::try_from_glib(SpecialU32::GLIB_NONE),
            Err(GlibNoneError)
        );

        assert_eq!(
            unsafe { Option::<SpecialU32>::from_glib(0) },
            Some(SpecialU32(0))
        );
        assert_eq!(
            unsafe { Option::<SpecialU32>::from_glib(42) },
            Some(SpecialU32(42))
        );
        assert!(unsafe { Option::<SpecialU32>::from_glib(SpecialU32::GLIB_NONE) }.is_none());
    }

    #[test]
    fn invalid_value() {
        use std::convert::TryFrom;
        use std::num::TryFromIntError;

        #[derive(Debug, PartialEq, Eq)]
        struct U32(u32);

        impl TryFromGlib<libc::c_long> for U32 {
            type Error = TryFromIntError;
            fn try_from_glib(val: libc::c_long) -> Result<Self, TryFromIntError> {
                Ok(U32(u32::try_from(val)?))
            }
        }

        assert_eq!(U32::try_from_glib(0), Ok(U32(0)));
        assert_eq!(U32::try_from_glib(42), Ok(U32(42)));
        assert!(U32::try_from_glib(-1).is_err());
        assert!(U32::try_from_glib(-42).is_err());
    }

    #[test]
    fn none_or_invalid_value() {
        use std::convert::TryFrom;
        use std::num::TryFromIntError;

        #[derive(Debug, PartialEq, Eq)]
        struct SpecialU32(u32);
        impl ToGlib for SpecialU32 {
            type GlibType = libc::c_long;
            fn to_glib(&self) -> libc::c_long {
                self.0 as libc::c_long
            }
        }
        impl OptionToGlib for SpecialU32 {
            const GLIB_NONE: Self::GlibType = -1;
        }

        assert_eq!(SpecialU32(0).to_glib(), 0);
        assert_eq!(SpecialU32(42).to_glib(), 42);
        assert_eq!(Some(SpecialU32(42)).to_glib(), 42);
        assert_eq!(Option::None::<SpecialU32>.to_glib(), SpecialU32::GLIB_NONE);

        impl TryFromGlib<libc::c_long> for SpecialU32 {
            type Error = GlibNoneOrInvalidError<TryFromIntError>;
            fn try_from_glib(
                val: libc::c_long,
            ) -> Result<Self, GlibNoneOrInvalidError<TryFromIntError>> {
                if val == SpecialU32::GLIB_NONE {
                    return Err(GlibNoneOrInvalidError::None);
                }

                Ok(SpecialU32(u32::try_from(val)?))
            }
        }

        assert_eq!(SpecialU32::try_from_glib(0), Ok(SpecialU32(0)));
        assert_eq!(SpecialU32::try_from_glib(42), Ok(SpecialU32(42)));
        assert!(SpecialU32::try_from_glib(SpecialU32::GLIB_NONE)
            .unwrap_err()
            .is_none());
        assert!(SpecialU32::try_from_glib(-42).unwrap_err().is_invalid());

        assert_eq!(
            unsafe { Result::<Option<SpecialU32>, _>::from_glib(0) },
            Ok(Some(SpecialU32(0)))
        );
        assert_eq!(
            unsafe { Result::<Option<SpecialU32>, _>::from_glib(42) },
            Ok(Some(SpecialU32(42)))
        );
        assert_eq!(
            unsafe { Result::<Option<SpecialU32>, _>::from_glib(SpecialU32::GLIB_NONE) },
            Ok(None)
        );
        assert!(unsafe { Result::<Option<SpecialU32>, _>::from_glib(-42) }.is_err());
    }
}
