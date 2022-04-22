// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "use_glib")]
use glib::translate::*;
use std::fmt;
#[cfg(feature = "use_glib")]
use std::mem;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
#[doc(alias = "cairo_rectangle_t")]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl Uninitialized for Rectangle {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::cairo_rectangle_t> for Rectangle {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::cairo_rectangle_t, Self> {
        let ptr: *const Rectangle = &*self;
        Stash(ptr as *const ffi::cairo_rectangle_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::cairo_rectangle_t> for Rectangle {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::cairo_rectangle_t, Self> {
        let ptr: *mut Rectangle = &mut *self;
        StashMut(ptr as *mut ffi::cairo_rectangle_t, self)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*const ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_none(ptr: *const ffi::cairo_rectangle_t) -> Self {
        *(ptr as *const Rectangle)
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrBorrow<*mut ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_borrow(ptr: *mut ffi::cairo_rectangle_t) -> crate::Borrowed<Self> {
        crate::Borrowed::new(*(ptr as *mut Rectangle))
    }
}

#[cfg(feature = "use_glib")]
#[doc(hidden)]
impl FromGlibPtrNone<*mut ffi::cairo_rectangle_t> for Rectangle {
    unsafe fn from_glib_none(ptr: *mut ffi::cairo_rectangle_t) -> Self {
        *(ptr as *mut Rectangle)
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl_inline!(
    Rectangle,
    ffi::cairo_rectangle_t,
    ffi::gobject::cairo_gobject_rectangle_get_type
);

impl Rectangle {
    pub fn to_raw_none(&self) -> *mut ffi::cairo_rectangle_t {
        let ptr = &*self as *const Rectangle as usize;
        ptr as *mut ffi::cairo_rectangle_t
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "use_glib")]
    #[test]
    fn rectangle_gvalues() {
        use glib::ToValue;
        let rect = Rectangle {
            x: 1.,
            y: 2.,
            width: 3.,
            height: 4.,
        };
        let value = rect.to_value();
        assert_eq!(value.get::<Rectangle>().unwrap().width, 3.);
        let _ = (&rect).to_value();
        let rect = Some(rect);
        let value = rect.to_value();
        assert_eq!(
            value.get::<Option<Rectangle>>().unwrap().map(|s| s.width),
            Some(3.)
        );
        let _ = rect.as_ref().to_value();
        assert_eq!(
            value.get::<Option<&Rectangle>>().unwrap().map(|s| s.width),
            Some(3.)
        );
    }
}
