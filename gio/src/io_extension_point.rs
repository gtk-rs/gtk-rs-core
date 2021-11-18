// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;
use glib::Type;
use std::fmt;
use std::ptr;

use crate::io_extension::IOExtension;

/// Builder for extension points.
#[derive(Debug)]
#[must_use = "Builder doesn't do anything unless built"]
pub struct IOExtensionPointBuilder<'a> {
    name: &'a str,
    required_type: Option<Type>,
}

impl<'a> IOExtensionPointBuilder<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            required_type: None,
        }
    }

    #[doc(alias = "g_io_extension_point_set_required_type")]
    pub fn required_type(self, required_type: Type) -> Self {
        Self {
            required_type: Some(required_type),
            ..self
        }
    }

    #[must_use = "Builder needs to be built to be used"]
    pub fn build(self) -> IOExtensionPoint {
        unsafe {
            let ep = IOExtensionPoint::from_glib_none(ffi::g_io_extension_point_register(
                self.name.to_glib_none().0,
            ));
            if let Some(t) = self.required_type {
                ffi::g_io_extension_point_set_required_type(ep.0.as_ptr(), t.into_glib());
            }
            ep
        }
    }
}

/// An extension point provides a mechanism to extend the functionality of a library or application.
/// Each extension point is identified by a name, and it may optionally require that any implementation
/// must be of a certain type.
#[doc(alias = "GIOExtensionPoint")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct IOExtensionPoint(ptr::NonNull<ffi::GIOExtensionPoint>);

impl fmt::Display for IOExtensionPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "IOExtensionPoint")
    }
}

impl FromGlibPtrNone<*mut ffi::GIOExtensionPoint> for IOExtensionPoint {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut ffi::GIOExtensionPoint) -> Self {
        assert!(!ptr.is_null());
        IOExtensionPoint(ptr::NonNull::new_unchecked(ptr))
    }
}

impl<'a> ToGlibPtr<'a, *mut ffi::GIOExtensionPoint> for &'a IOExtensionPoint {
    type Storage = &'a IOExtensionPoint;

    #[inline]
    fn to_glib_none(&self) -> Stash<'a, *mut ffi::GIOExtensionPoint, &'a IOExtensionPoint> {
        Stash(self.0.as_ptr() as *mut ffi::GIOExtensionPoint, *self)
    }
}

impl IOExtensionPoint {
    /// Create a new builder for an extension point.
    #[doc(alias = "g_io_extension_point_register")]
    pub fn builder(name: &str) -> IOExtensionPointBuilder {
        IOExtensionPointBuilder::new(name)
    }

    #[doc(alias = "g_io_extension_point_lookup")]
    pub fn lookup(name: &str) -> Option<Self> {
        unsafe {
            let ep = ffi::g_io_extension_point_lookup(name.to_glib_none().0);
            from_glib_none(ep)
        }
    }

    #[doc(alias = "g_io_extension_point_get_extensions")]
    pub fn extensions(&self) -> Vec<IOExtension> {
        let mut res = Vec::new();
        unsafe {
            let mut l = ffi::g_io_extension_point_get_extensions(self.0.as_ptr());
            while !l.is_null() {
                let e: *mut ffi::GIOExtension = Ptr::from((*l).data);
                res.push(from_glib_none(e));
                l = (*l).next;
            }
        }
        res
    }

    #[doc(alias = "g_io_extension_point_get_extension_by_name")]
    pub fn extension_by_name(&self, name: &str) -> Option<IOExtension> {
        unsafe {
            let e = ffi::g_io_extension_point_get_extension_by_name(
                self.0.as_ptr(),
                name.to_glib_none().0,
            );
            from_glib_none(e)
        }
    }

    #[doc(alias = "g_io_extension_point_get_required_type")]
    pub fn required_type(&self) -> Type {
        unsafe { from_glib(ffi::g_io_extension_point_get_required_type(self.0.as_ptr())) }
    }

    #[doc(alias = "g_io_extension_point_implement")]
    pub fn implement(
        extension_point_name: &str,
        type_: Type,
        extension_name: &str,
        priority: i32,
    ) -> Option<IOExtension> {
        unsafe {
            let e = ffi::g_io_extension_point_implement(
                extension_point_name.to_glib_none().0,
                type_.into_glib(),
                extension_name.to_glib_none().0,
                priority,
            );
            from_glib_none(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glib::StaticType;

    #[test]
    fn extension_point() {
        let ep = IOExtensionPoint::lookup("test-extension-point");
        assert!(ep.is_none());

        let ep = IOExtensionPoint::builder("test-extension-point").build();
        let ep2 = IOExtensionPoint::lookup("test-extension-point");
        assert_eq!(ep2, Some(ep));

        let req = ep.required_type();
        assert_eq!(req, Type::INVALID);

        let ep = IOExtensionPoint::builder("test-extension-point")
            .required_type(Type::OBJECT)
            .build();
        let req = ep.required_type();
        assert_eq!(req, Type::OBJECT);

        let v = ep.extensions();
        assert!(v.is_empty());

        let e = IOExtensionPoint::implement(
            "test-extension-point",
            <crate::Vfs as StaticType>::static_type(),
            "extension1",
            10,
        );
        assert!(e.is_some());

        let e = IOExtensionPoint::implement("test-extension-point", Type::OBJECT, "extension2", 20);
        assert!(e.is_some());

        let v = ep.extensions();
        assert_eq!(v.len(), 2);
        assert_eq!(v[0].name(), "extension2");
        assert_eq!(v[0].type_(), Type::OBJECT);
        assert_eq!(v[0].priority(), 20);
        assert_eq!(v[1].name(), "extension1");
        assert_eq!(v[1].type_(), <crate::Vfs as StaticType>::static_type());
        assert_eq!(v[1].priority(), 10);
    }
}
