// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::borrow::Borrow;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::slice;

glib_wrapper! {
    /// A shared immutable byte slice (the equivalent of `Rc<[u8]>`).
    ///
    /// `From` implementations that take references (e.g. `&[u8]`) copy the
    /// data. The `from_static` constructor avoids copying static data.
    ///
    /// ```
    /// use glib::Bytes;
    ///
    /// let v = vec![1, 2, 3];
    /// let b = Bytes::from(&v);
    /// assert_eq!(v, b);
    ///
    /// let s = b"xyz";
    /// let b = Bytes::from_static(s);
    /// assert_eq!(&s[..], b);
    /// ```
    pub struct Bytes(Shared<ffi::GBytes>);

    match fn {
        ref => |ptr| ffi::g_bytes_ref(ptr),
        unref => |ptr| ffi::g_bytes_unref(ptr),
        get_type => || ffi::g_bytes_get_type(),
    }
}

impl Bytes {
    /// Copies `data` into a new shared slice.
    fn new<T: AsRef<[u8]>>(data: T) -> Bytes {
        let data = data.as_ref();
        unsafe { from_glib_full(ffi::g_bytes_new(data.as_ptr() as *const _, data.len())) }
    }

    /// Creates a view into static `data` without copying.
    pub fn from_static(data: &'static [u8]) -> Bytes {
        unsafe {
            from_glib_full(ffi::g_bytes_new_static(
                data.as_ptr() as *const _,
                data.len(),
            ))
        }
    }

    /// Takes ownership of `data` and creates a new `Bytes` without copying.
    pub fn from_owned<T: AsRef<[u8]> + Send + 'static>(data: T) -> Bytes {
        let data: Box<T> = Box::new(data);
        let (size, data_ptr) = {
            let data = (*data).as_ref();
            (data.len(), data.as_ptr())
        };

        unsafe extern "C" fn drop_box<T: AsRef<[u8]> + Send + 'static>(b: ffi::gpointer) {
            let _: Box<T> = Box::from_raw(b as *mut _);
        }

        unsafe {
            from_glib_full(ffi::g_bytes_new_with_free_func(
                data_ptr as *const _,
                size,
                Some(drop_box::<T>),
                Box::into_raw(data) as *mut _,
            ))
        }
    }
}

unsafe impl Send for Bytes {}
unsafe impl Sync for Bytes {}

impl<'a, T: ?Sized + Borrow<[u8]> + 'a> From<&'a T> for Bytes {
    fn from(value: &'a T) -> Bytes {
        Bytes::new(value.borrow())
    }
}

impl fmt::Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Bytes")
            .field("ptr", &self.to_glib_none().0)
            .field("data", &&self[..])
            .finish()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &*self
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        unsafe {
            let mut len = 0;
            let ptr = ffi::g_bytes_get_data(self.to_glib_none().0, &mut len);
            debug_assert!(!ptr.is_null() || len == 0);
            slice::from_raw_parts(ptr as *const u8, len)
        }
    }
}

impl PartialEq for Bytes {
    fn eq(&self, other: &Self) -> bool {
        unsafe {
            from_glib(ffi::g_bytes_equal(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            ))
        }
    }
}

impl Eq for Bytes {}

impl PartialOrd for Bytes {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            let ret = ffi::g_bytes_compare(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            );
            ret.partial_cmp(&0)
        }
    }
}

impl Ord for Bytes {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            let ret = ffi::g_bytes_compare(
                self.to_glib_none().0 as *const _,
                other.to_glib_none().0 as *const _,
            );
            ret.cmp(&0)
        }
    }
}

macro_rules! impl_cmp {
    ($lhs:ty, $rhs: ty) => {
        impl<'a, 'b> PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        impl<'a, 'b> PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                self[..].eq(&other[..])
            }
        }

        impl<'a, 'b> PartialOrd<$rhs> for $lhs {
            #[inline]
            fn partial_cmp(&self, other: &$rhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }

        impl<'a, 'b> PartialOrd<$lhs> for $rhs {
            #[inline]
            fn partial_cmp(&self, other: &$lhs) -> Option<Ordering> {
                self[..].partial_cmp(&other[..])
            }
        }
    };
}

impl_cmp!(Bytes, [u8]);
impl_cmp!(Bytes, &'a [u8]);
impl_cmp!(&'a Bytes, [u8]);
impl_cmp!(Bytes, Vec<u8>);
impl_cmp!(&'a Bytes, Vec<u8>);

impl Hash for Bytes {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        Hash::hash_slice(self, state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn eq() {
        let abc: &[u8] = b"abc";
        let def: &[u8] = b"def";
        let a1 = Bytes::from(abc);
        let a2 = Bytes::from(abc);
        let d = Bytes::from(def);
        assert_eq!(a1, a2);
        assert_eq!(def, d);
        assert!(a1 != d);
        assert!(a1 != def);
    }

    #[test]
    fn ord() {
        let abc: &[u8] = b"abc";
        let def: &[u8] = b"def";
        let a = Bytes::from(abc);
        let d = Bytes::from(def);
        assert!(a < d);
        assert!(a < def);
        assert!(abc < d);
        assert!(d > a);
        assert!(d > abc);
        assert!(def > a);
    }

    #[test]
    fn hash() {
        let b1 = Bytes::from(b"this is a test");
        let b2 = Bytes::from(b"this is a test");
        let b3 = Bytes::from(b"test");
        let mut set = HashSet::new();
        set.insert(b1);
        assert!(set.contains(&b2));
        assert!(!set.contains(&b3));
    }

    #[test]
    fn from_static() {
        let b1 = Bytes::from_static(b"this is a test");
        let b2 = Bytes::from(b"this is a test");
        assert_eq!(b1, b2);
    }

    #[test]
    fn from_owned() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        assert_eq!(b, [1u8, 2u8, 3u8].as_ref());
    }
}
