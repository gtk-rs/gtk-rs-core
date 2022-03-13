// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{AttrClass, AttrType, Attribute};
use glib::translate::*;

impl Attribute {
    #[doc(alias = "get_attr_class")]
    pub fn attr_class(&self) -> AttrClass {
        unsafe { from_glib_full((*self.to_glib_none().0).klass) }
    }

    pub fn type_(&self) -> AttrType {
        unsafe { from_glib((*(*self.to_glib_none().0).klass).type_) }
    }

    #[doc(alias = "get_start_index")]
    pub fn start_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).start_index
        }
    }

    #[doc(alias = "get_end_index")]
    pub fn end_index(&self) -> u32 {
        unsafe {
            let stash = self.to_glib_none();
            (*stash.0).end_index
        }
    }

    pub fn set_start_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).start_index = index;
        }
    }

    pub fn set_end_index(&mut self, index: u32) {
        unsafe {
            let stash = self.to_glib_none_mut();
            (*stash.0).end_index = index;
        }
    }
    pub fn downcast<T: IsAttribute>(self) -> Result<T, Attribute> {
        unsafe {
            if T::ATTR_TYPES.contains(&self.attr_class().type_()) {
                Ok(from_glib_full(self.to_glib_full()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: IsAttribute>(&self) -> Option<&T> {
        unsafe {
            if T::ATTR_TYPES.contains(&self.attr_class().type_()) {
                Some(&*(self as *const Attribute as *const T))
            } else {
                None
            }
        }
    }
}

pub unsafe trait IsAttribute:
    FromGlibPtrFull<*const ffi::PangoAttribute>
    + FromGlibPtrFull<*mut ffi::PangoAttribute>
    + std::convert::AsRef<crate::Attribute>
    + 'static
{
    const ATTR_TYPES: &'static [AttrType];
    fn upcast(self) -> Attribute;
    fn upcast_ref(&self) -> &Attribute;
}

macro_rules! define_attribute_struct {
    ($rust_type:ident, $ffi_type:path, $attr_types:expr) => {

        #[cfg(any(feature = "v1_44", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
        glib::wrapper! {
            #[derive(Debug)]
            pub struct $rust_type(Boxed<$ffi_type>);

            match fn {
                copy => |ptr| ffi::pango_attribute_copy(ptr as *const ffi::PangoAttribute) as *mut $ffi_type,
                free => |ptr| ffi::pango_attribute_destroy(ptr as *mut ffi::PangoAttribute),
                type_ => || ffi::pango_attribute_get_type(),
            }
        }

        unsafe impl Send for $rust_type {}
        unsafe impl Sync for $rust_type {}

        #[cfg(not(any(feature = "v1_44", feature = "dox")))]
        glib::wrapper! {
            #[derive(Debug)]
            pub struct $rust_type(Boxed<$ffi_type>);

            match fn {
                copy => |ptr| ffi::pango_attribute_copy(ptr as *const ffi::PangoAttribute) as *mut $ffi_type,
                free => |ptr| ffi::pango_attribute_destroy(ptr as *mut ffi::PangoAttribute),
            }
        }

        impl $rust_type {
            #[doc(alias = "pango_attribute_equal")]
            fn equal<'a, T:  crate::attribute::IsAttribute>(&self, attr2: &'a T) -> bool {
                unsafe {
                    glib::translate::from_glib(ffi::pango_attribute_equal(
                       glib::translate::ToGlibPtr::to_glib_none(self).0 as *const ffi::PangoAttribute,
                       glib::translate::ToGlibPtr::to_glib_none(attr2.upcast_ref()).0 as *const ffi::PangoAttribute,
                    ))
                }
            }
        }

        impl PartialEq for $rust_type {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                self.equal(other)
            }
        }

        impl Eq for $rust_type {}

        unsafe impl crate::attribute::IsAttribute for $rust_type {
            const ATTR_TYPES: &'static [crate::AttrType] = $attr_types;

            fn upcast(self) -> crate::Attribute {
                unsafe { glib::translate::from_glib_full(glib::translate::ToGlibPtr::to_glib_full(&self) as *mut ffi::PangoAttribute) }
            }

            fn upcast_ref(&self) -> &crate::Attribute {
                &*self
            }
        }

        #[doc(hidden)]
        impl glib::translate::FromGlibPtrFull<*mut ffi::PangoAttribute> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut ffi::PangoAttribute) -> Self {
                glib::translate::from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        impl glib::translate::FromGlibPtrFull<*const ffi::PangoAttribute> for $rust_type {
            unsafe fn from_glib_full(ptr: *const ffi::PangoAttribute) -> Self {
                glib::translate::from_glib_full(ptr as *const $ffi_type)
            }
        }

        impl std::convert::AsRef<crate::Attribute> for $rust_type {
            fn as_ref(&self) -> &crate::Attribute {
                &*self
            }
        }

        impl From<$rust_type> for crate::Attribute {
            fn from(attr: $rust_type) -> crate::Attribute {
                crate::IsAttribute::upcast(attr)
            }
        }

        impl std::ops::Deref for $rust_type {
            type Target = crate::Attribute;

            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const $rust_type as *const crate::Attribute) }
            }
        }

        impl std::ops::DerefMut for $rust_type {
            fn deref_mut(&mut self) -> &mut crate::Attribute {
                unsafe { &mut *(self as *mut $rust_type as *mut crate::Attribute) }
            }
        }
    }
}
