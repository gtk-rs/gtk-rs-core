// Take a look at the license at the top of the repository in the LICENSE file.

use std::{char::CharTryFromError, convert::TryFrom, ffi::CStr};

use crate::{
    object::{Interface, InterfaceRef, IsClass, IsInterface, ObjectClass},
    prelude::*,
    translate::*,
    utils::is_canonical_pspec_name,
    Object, ParamFlags, Type, Value,
};
// Can't use get_type here as this is not a boxed type but another fundamental type
wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[doc(alias = "GParamSpec")]
    pub struct ParamSpec(Shared<gobject_ffi::GParamSpec>);

    match fn {
        ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr),
        unref => |ptr| gobject_ffi::g_param_spec_unref(ptr),
    }
}

impl StaticType for ParamSpec {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(gobject_ffi::G_TYPE_PARAM) }
    }
}

#[doc(hidden)]
impl crate::value::ValueType for ParamSpec {
    type Type = ParamSpec;
}

#[doc(hidden)]
impl crate::value::ValueTypeOptional for ParamSpec {}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for ParamSpec {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        let ptr = gobject_ffi::g_value_dup_param(value.to_glib_none().0);
        debug_assert!(!ptr.is_null());
        from_glib_full(ptr as *mut gobject_ffi::GParamSpec)
    }
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for &'a ParamSpec {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        debug_assert_eq!(
            std::mem::size_of::<Self>(),
            std::mem::size_of::<crate::ffi::gpointer>()
        );
        let value = &*(value as *const crate::Value as *const crate::gobject_ffi::GValue);
        let ptr = &value.data[0].v_pointer as *const crate::ffi::gpointer
            as *const *const gobject_ffi::GParamSpec;
        debug_assert!(!(*ptr).is_null());
        &*(ptr as *const ParamSpec)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for ParamSpec {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(ParamSpec::static_type());
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, self.to_glib_full());
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        ParamSpec::static_type()
    }
}

#[doc(hidden)]
impl From<ParamSpec> for crate::Value {
    #[inline]
    fn from(s: ParamSpec) -> Self {
        unsafe {
            let mut value = crate::Value::from_type_unchecked(ParamSpec::static_type());
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.into_glib_ptr());
            value
        }
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for ParamSpec {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full());
        }

        value
    }
}

impl AsRef<ParamSpec> for ParamSpec {
    #[inline]
    fn as_ref(&self) -> &ParamSpec {
        self
    }
}

unsafe impl Send for ParamSpec {}
unsafe impl Sync for ParamSpec {}

impl ParamSpec {
    pub fn downcast<T: ParamSpecType>(self) -> Result<T, ParamSpec> {
        unsafe {
            if self.type_() == T::static_type() {
                Ok(from_glib_full(self.into_glib_ptr()))
            } else {
                Err(self)
            }
        }
    }

    pub fn downcast_ref<T: ParamSpecType>(&self) -> Option<&T> {
        unsafe {
            if self.type_() == T::static_type() {
                Some(&*(self as *const ParamSpec as *const T))
            } else {
                None
            }
        }
    }

    #[doc(alias = "get_type")]
    #[inline]
    pub fn type_(&self) -> Type {
        unsafe {
            from_glib(
                (*(*(<Self as ToGlibPtr<*const _>>::to_glib_none(self).0))
                    .g_type_instance
                    .g_class)
                    .g_type,
            )
        }
    }

    #[inline]
    pub fn is<T: StaticType>(&self) -> bool {
        self.type_().is_a(T::static_type())
    }

    #[doc(alias = "get_value_type")]
    #[inline]
    pub fn value_type(&self) -> crate::Type {
        unsafe { from_glib((*(<Self as ToGlibPtr<*const _>>::to_glib_none(self).0)).value_type) }
    }

    #[cfg(any(feature = "v2_74", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_74")))]
    #[doc(alias = "g_param_value_is_valid")]
    #[inline]
    pub fn value_is_valid(&self, value: &Value) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_param_value_is_valid(
                self.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "get_owner_type")]
    #[inline]
    pub fn owner_type(&self) -> crate::Type {
        unsafe { from_glib((*(<Self as ToGlibPtr<*const _>>::to_glib_none(self).0)).owner_type) }
    }

    #[doc(alias = "get_flags")]
    #[inline]
    pub fn flags(&self) -> ParamFlags {
        unsafe { from_glib((*(<Self as ToGlibPtr<*const _>>::to_glib_none(self).0)).flags) }
    }

    #[doc(alias = "g_param_spec_get_blurb")]
    #[doc(alias = "get_blurb")]
    #[inline]
    pub fn blurb(&self) -> Option<&str> {
        unsafe {
            let ptr = gobject_ffi::g_param_spec_get_blurb(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                CStr::from_ptr(ptr).to_str().ok()
            }
        }
    }

    #[doc(alias = "g_param_spec_get_default_value")]
    #[doc(alias = "get_default_value")]
    #[inline]
    pub fn default_value(&self) -> &Value {
        unsafe {
            &*(gobject_ffi::g_param_spec_get_default_value(self.to_glib_none().0)
                as *const crate::Value)
        }
    }

    #[doc(alias = "g_param_spec_get_name")]
    #[doc(alias = "get_name")]
    #[inline]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_name(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_param_spec_get_name_quark")]
    #[doc(alias = "get_name_quark")]
    #[inline]
    pub fn name_quark(&self) -> crate::Quark {
        unsafe {
            from_glib(gobject_ffi::g_param_spec_get_name_quark(
                self.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns the nickname of this `ParamSpec`.
    ///
    /// If this `ParamSpec` does not have a nickname, the nickname of its redirect target is returned if it has one.
    /// Otherwise, `self.name()` is returned.
    #[doc(alias = "g_param_spec_get_nick")]
    #[doc(alias = "get_nick")]
    #[inline]
    pub fn nick(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_nick(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    //pub fn get_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_get_qdata() }
    //}

    #[doc(alias = "g_param_spec_get_redirect_target")]
    #[doc(alias = "get_redirect_target")]
    #[inline]
    pub fn redirect_target(&self) -> Option<ParamSpec> {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_get_redirect_target(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn set_qdata(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>) {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_set_qdata() }
    //}

    //pub fn set_qdata_full(&self, quark: /*Ignored*/glib::Quark, data: Option</*Unimplemented*/Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_set_qdata_full() }
    //}

    //pub fn steal_qdata(&self, quark: /*Ignored*/glib::Quark) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gobject_ffi::g_param_spec_steal_qdata() }
    //}

    #[cfg(any(feature = "v2_66", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_param_spec_is_valid_name")]
    #[inline]
    pub fn is_valid_name(name: &str) -> bool {
        unsafe {
            from_glib(gobject_ffi::g_param_spec_is_valid_name(
                name.to_glib_none().0,
            ))
        }
    }
}

pub unsafe trait ParamSpecType:
    StaticType + FromGlibPtrFull<*mut gobject_ffi::GParamSpec> + 'static
{
}

#[link(name = "gobject-2.0")]
extern "C" {
    pub static g_param_spec_types: *const ffi::GType;
}

macro_rules! define_param_spec {
    ($rust_type:ident, $ffi_type:path, $rust_type_offset:expr) => {
        // Can't use get_type here as this is not a boxed type but another fundamental type
        wrapper! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
            pub struct $rust_type(Shared<$ffi_type>);

            match fn {
                ref => |ptr| gobject_ffi::g_param_spec_ref_sink(ptr as *mut gobject_ffi::GParamSpec) as *mut $ffi_type,
                unref => |ptr| gobject_ffi::g_param_spec_unref(ptr as *mut gobject_ffi::GParamSpec),
            }
        }

        impl StaticType for $rust_type {
            #[inline]
            fn static_type() -> Type {
                unsafe {
                    from_glib(*g_param_spec_types.add($rust_type_offset))
                }
            }
        }

        #[doc(hidden)]
        impl crate::value::ValueType for $rust_type {
            type Type = $rust_type;
        }

        #[doc(hidden)]
        impl crate::value::ValueTypeOptional for $rust_type {}

        #[doc(hidden)]
        unsafe impl<'a> crate::value::FromValue<'a> for $rust_type {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a crate::Value) -> Self {
                let ptr = gobject_ffi::g_value_dup_param(value.to_glib_none().0);
                debug_assert!(!ptr.is_null());
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a> crate::value::FromValue<'a> for &'a $rust_type {
            type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a crate::Value) -> Self {
                debug_assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<crate::ffi::gpointer>());
                let value = &*(value as *const crate::Value as *const crate::gobject_ffi::GValue);
                let ptr = &value.data[0].v_pointer as *const crate::ffi::gpointer as *const *const gobject_ffi::GParamSpec;
                debug_assert!(!(*ptr).is_null());
                &*(ptr as *const $rust_type)
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValue for $rust_type {
            fn to_value(&self) -> crate::Value {
                unsafe {
                    let mut value = crate::Value::from_type_unchecked($rust_type::static_type());
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_full(self) as *mut _);
                    value
                }
            }

            fn value_type(&self) -> crate::Type {
                $rust_type::static_type()
            }
        }

        #[doc(hidden)]
        impl From<$rust_type> for crate::Value {
            #[inline]
            fn from(s: $rust_type) -> Self {
                unsafe {
                    let mut value = crate::Value::from_type_unchecked($rust_type::static_type());
                    gobject_ffi::g_value_take_param(
                        value.to_glib_none_mut().0,
                        $crate::translate::IntoGlibPtr::<*mut gobject_ffi::GParamSpec>::into_glib_ptr(s),
                    );
                    value
                }
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValueOptional for $rust_type {
            fn to_value_optional(s: Option<&Self>) -> crate::Value {
                let mut value = crate::Value::for_value_type::<Self>();
                unsafe {
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_full(&s) as *mut _);
                }

                value
            }
        }

        unsafe impl Send for $rust_type {}
        unsafe impl Sync for $rust_type {}

        impl std::ops::Deref for $rust_type {
            type Target = ParamSpec;

            #[inline]
            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const $rust_type as *const ParamSpec)
                }
            }
        }

        unsafe impl ParamSpecType for $rust_type {}

        #[doc(hidden)]
        impl<'a> ToGlibPtr<'a, *const gobject_ffi::GParamSpec> for $rust_type {
            type Storage = std::marker::PhantomData<&'a $crate::shared::Shared<$ffi_type, $rust_type>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const gobject_ffi::GParamSpec, Self> {
                let stash = $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_none(self);
                $crate::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const gobject_ffi::GParamSpec {
                $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_full(self) as *const _
            }
        }

        #[doc(hidden)]
        impl<'a> ToGlibPtr<'a, *mut gobject_ffi::GParamSpec> for $rust_type {
            type Storage = std::marker::PhantomData<&'a $crate::shared::Shared<$ffi_type, $rust_type>>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut gobject_ffi::GParamSpec, Self> {
                let stash = $crate::translate::ToGlibPtr::<*mut $ffi_type>::to_glib_none(self);
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut gobject_ffi::GParamSpec {
                $crate::translate::ToGlibPtr::<*mut $ffi_type>::to_glib_full(self) as *mut _
            }
        }

        #[doc(hidden)]
        impl IntoGlibPtr<*mut gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn into_glib_ptr(self) -> *mut gobject_ffi::GParamSpec {
                let s = std::mem::ManuallyDrop::new(self);
                s.to_glib_none().0
            }
        }

        #[doc(hidden)]
        impl IntoGlibPtr<*const gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn into_glib_ptr(self) -> *const gobject_ffi::GParamSpec {
                let s = std::mem::ManuallyDrop::new(self);
                s.to_glib_none().0
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrNone<*const gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn from_glib_none(ptr: *const gobject_ffi::GParamSpec) -> Self {
                from_glib_none(ptr as *const $ffi_type)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrNone<*mut gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut gobject_ffi::GParamSpec) -> Self {
                from_glib_none(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrBorrow<*const gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const gobject_ffi::GParamSpec) -> Borrowed<Self> {
                from_glib_borrow(ptr as *const $ffi_type)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrBorrow<*mut gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut gobject_ffi::GParamSpec) -> Borrowed<Self> {
                from_glib_borrow(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for $rust_type {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            #[inline]
            pub fn upcast(self) -> ParamSpec {
                unsafe {
                    from_glib_full(IntoGlibPtr::<*mut $ffi_type>::into_glib_ptr(self) as *mut gobject_ffi::GParamSpec)
                }
            }

            #[inline]
            pub fn upcast_ref(&self) -> &ParamSpec {
                &*self
            }
        }

        impl AsRef<ParamSpec> for $rust_type {
            #[inline]
            fn as_ref(&self) -> &ParamSpec {
                &self
            }
        }
    };
}

macro_rules! define_param_spec_default {
    ($rust_type:ident, $ffi_type:path, $value_type:ty, $from_glib:expr) => {
        impl $rust_type {
            #[inline]
            pub fn default_value(&self) -> $value_type {
                unsafe {
                    let ptr =
                        $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_none(self).0;
                    $from_glib((*ptr).default_value)
                }
            }
        }
    };
}

macro_rules! define_param_spec_min_max {
    ($rust_type:ident, $ffi_type:path, $value_type:ty) => {
        impl $rust_type {
            #[inline]
            pub fn minimum(&self) -> $value_type {
                unsafe {
                    let ptr =
                        $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_none(self).0;
                    (*ptr).minimum
                }
            }

            #[inline]
            pub fn maximum(&self) -> $value_type {
                unsafe {
                    let ptr =
                        $crate::translate::ToGlibPtr::<*const $ffi_type>::to_glib_none(self).0;
                    (*ptr).maximum
                }
            }
        }
    };
}

macro_rules! define_param_spec_numeric {
    ($rust_type:ident, $ffi_type:path, $value_type:ty, $rust_type_offset:expr, $ffi_fun:ident, $alias:literal) => {
        define_param_spec!($rust_type, $ffi_type, $rust_type_offset);
        define_param_spec_default!($rust_type, $ffi_type, $value_type, |x| x);
        define_param_spec_min_max!($rust_type, $ffi_type, $value_type);

        impl $rust_type {
            #[allow(clippy::new_ret_no_self)]
            #[doc(alias = $alias)]
            #[deprecated = "Use builder() instead"]
            pub fn new<'a>(
                name: &str,
                nick: impl Into<Option<&'a str>>,
                blurb: impl Into<Option<&'a str>>,
                minimum: $value_type,
                maximum: $value_type,
                default_value: $value_type,
                flags: ParamFlags,
            ) -> ParamSpec {
                assert_param_name(name);
                unsafe {
                    Self::new_unchecked(name, nick, blurb, minimum, maximum, default_value, flags)
                }
            }

            unsafe fn new_unchecked<'a>(
                name: &str,
                nick: impl Into<Option<&'a str>>,
                blurb: impl Into<Option<&'a str>>,
                minimum: $value_type,
                maximum: $value_type,
                default_value: $value_type,
                flags: ParamFlags,
            ) -> ParamSpec {
                unsafe {
                    from_glib_none(gobject_ffi::$ffi_fun(
                        name.to_glib_none().0,
                        nick.into().to_glib_none().0,
                        blurb.into().to_glib_none().0,
                        minimum,
                        maximum,
                        default_value,
                        flags.into_glib(),
                    ))
                }
            }
        }
    };
}

/// A trait implemented by the various [`ParamSpec`] builder types.
///
/// It is useful for providing a builder pattern for [`ParamSpec`] defined
/// outside of GLib like in GStreamer or GTK 4.
pub trait ParamSpecBuilderExt<'a>: Sized {
    /// Implementation detail.
    fn set_nick(&mut self, nick: Option<&'a str>);
    /// Implementation detail.
    fn set_blurb(&mut self, blurb: Option<&'a str>);
    /// Implementation detail.
    fn set_flags(&mut self, flags: crate::ParamFlags);
    /// Implementation detail.
    fn current_flags(&self) -> crate::ParamFlags;

    /// By default, the nickname of its redirect target will be used if it has one.
    /// Otherwise, `self.name` will be used.
    fn nick(mut self, nick: &'a str) -> Self {
        self.set_nick(Some(nick));
        self
    }

    /// Default: `None`
    fn blurb(mut self, blurb: &'a str) -> Self {
        self.set_blurb(Some(blurb));
        self
    }

    /// Default: `glib::ParamFlags::READWRITE`
    fn flags(mut self, flags: crate::ParamFlags) -> Self {
        self.set_flags(flags);
        self
    }

    /// Mark the property as read only and drops the READWRITE flag set by default.
    fn read_only(self) -> Self {
        let flags =
            (self.current_flags() - crate::ParamFlags::WRITABLE) | crate::ParamFlags::READABLE;
        self.flags(flags)
    }

    /// Mark the property as write only and drops the READWRITE flag set by default.
    fn write_only(self) -> Self {
        let flags =
            (self.current_flags() - crate::ParamFlags::READABLE) | crate::ParamFlags::WRITABLE;
        self.flags(flags)
    }

    /// Mark the property as readwrite, it is the default value.
    fn readwrite(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::READWRITE;
        self.flags(flags)
    }

    /// Mark the property as construct
    fn construct(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::CONSTRUCT;
        self.flags(flags)
    }

    /// Mark the property as construct only
    fn construct_only(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::CONSTRUCT_ONLY;
        self.flags(flags)
    }

    /// Mark the property as lax validation
    fn lax_validation(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::LAX_VALIDATION;
        self.flags(flags)
    }

    /// Mark the property as explicit notify
    fn explicit_notify(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::EXPLICIT_NOTIFY;
        self.flags(flags)
    }

    /// Mark the property as deprecated
    fn deprecated(self) -> Self {
        let flags = self.current_flags() | crate::ParamFlags::DEPRECATED;
        self.flags(flags)
    }
}

macro_rules! define_builder {
    (@constructors $rust_type:ident, $builder_type:ident $(($($req_ident:ident: $req_ty:ty,)*))?) => {
        impl<'a> $builder_type<'a> {
            fn new(name: &'a str, $($($req_ident: $req_ty)*)?) -> Self {
                assert_param_name(name);
                Self {
                    name,
                    $($($req_ident: Some($req_ident),)*)?
                    ..Default::default()
                }
            }
        }

        impl $rust_type {
            pub fn builder(name: &str, $($($req_ident: $req_ty),*)?) -> $builder_type<'_> {
                $builder_type::new(name, $($($req_ident),*)?)
            }
        }

        impl<'a> crate::prelude::ParamSpecBuilderExt<'a> for $builder_type<'a> {
            fn set_nick(&mut self, nick: Option<&'a str>) {
                self.nick = nick;
            }
            fn set_blurb(&mut self, blurb: Option<&'a str>) {
                self.blurb = blurb;
            }
            fn set_flags(&mut self, flags: crate::ParamFlags) {
                self.flags = flags;
            }
            fn current_flags(&self) -> crate::ParamFlags {
                self.flags
            }
        }
    };
    (
        $rust_type:ident, $builder_type:ident {
            $($field_id:ident: $field_ty:ty $(= $field_expr:expr)?,)*
        }
        $(requires $required_tt:tt)?
    ) => {
        #[derive(Default)]
        #[must_use]
        pub struct $builder_type<'a> {
            name: &'a str,
            nick: Option<&'a str>,
            blurb: Option<&'a str>,
            flags: crate::ParamFlags,
            $($field_id: Option<$field_ty>),*
        }
        impl<'a> $builder_type<'a> {
            $(
            $(#[doc = concat!("Default: `", stringify!($field_expr), "`")])?
            pub fn $field_id(mut self, value: $field_ty) -> Self {
                self.$field_id = Some(value);
                self
            }
            )*

            #[must_use]
            pub fn build(self) -> ParamSpec {
                unsafe {
                    $rust_type::new_unchecked(
                        self.name,
                        self.nick,
                        self.blurb,
                        $(self
                            .$field_id
                            $(.or(Some($field_expr)))?
                            .expect("impossible: missing parameter in ParamSpec*Builder")
                        ,)*
                        self.flags
                    )
                }
            }
        }
        define_builder!(@constructors $rust_type, $builder_type $($required_tt)?);
    }
}
macro_rules! define_builder_numeric {
    ($rust_type:ident, $builder_type:ident, $n_ty:ty) => {
        define_builder!(
            $rust_type,
            $builder_type {
                minimum: $n_ty = <$n_ty>::MIN,
                maximum: $n_ty = <$n_ty>::MAX,
                default_value: $n_ty = <$n_ty as Default>::default(),
            }
        );
    };
}

#[track_caller]
// the default panic formatter will use its caller as the location in its error message
fn assert_param_name(name: &str) {
    assert!(
        is_canonical_pspec_name(name),
        "{name} is not a valid canonical parameter name",
    );
}
define_param_spec_numeric!(
    ParamSpecChar,
    gobject_ffi::GParamSpecChar,
    i8,
    0,
    g_param_spec_char,
    "g_param_spec_char"
);

define_builder_numeric!(ParamSpecChar, ParamSpecCharBuilder, i8);

define_param_spec_numeric!(
    ParamSpecUChar,
    gobject_ffi::GParamSpecUChar,
    u8,
    1,
    g_param_spec_uchar,
    "g_param_spec_uchar"
);

define_builder_numeric!(ParamSpecUChar, ParamSpecUCharBuilder, u8);

define_param_spec!(ParamSpecBoolean, gobject_ffi::GParamSpecBoolean, 2);

define_param_spec_default!(
    ParamSpecBoolean,
    gobject_ffi::GParamSpecBoolean,
    bool,
    |x| from_glib(x)
);

impl ParamSpecBoolean {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_boolean")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boolean(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                default_value.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecBoolean,
    ParamSpecBooleanBuilder {
        default_value: bool = false,
    }
);

define_param_spec_numeric!(
    ParamSpecInt,
    gobject_ffi::GParamSpecInt,
    i32,
    3,
    g_param_spec_int,
    "g_param_spec_int"
);

define_builder_numeric!(ParamSpecInt, ParamSpecIntBuilder, i32);

define_param_spec_numeric!(
    ParamSpecUInt,
    gobject_ffi::GParamSpecUInt,
    u32,
    4,
    g_param_spec_uint,
    "g_param_spec_uint"
);

define_builder_numeric!(ParamSpecUInt, ParamSpecUIntBuilder, u32);

define_param_spec_numeric!(
    ParamSpecLong,
    gobject_ffi::GParamSpecLong,
    libc::c_long,
    5,
    g_param_spec_long,
    "g_param_spec_long"
);

define_builder_numeric!(ParamSpecLong, ParamSpecLongBuilder, libc::c_long);

define_param_spec_numeric!(
    ParamSpecULong,
    gobject_ffi::GParamSpecULong,
    libc::c_ulong,
    6,
    g_param_spec_ulong,
    "g_param_spec_ulong"
);

define_builder_numeric!(ParamSpecULong, ParamSpecULongBuilder, libc::c_ulong);

define_param_spec_numeric!(
    ParamSpecInt64,
    gobject_ffi::GParamSpecInt64,
    i64,
    7,
    g_param_spec_int64,
    "g_param_spec_int64"
);

define_builder_numeric!(ParamSpecInt64, ParamSpecInt64Builder, i64);

define_param_spec_numeric!(
    ParamSpecUInt64,
    gobject_ffi::GParamSpecUInt64,
    u64,
    8,
    g_param_spec_uint64,
    "g_param_spec_uint64"
);

define_builder_numeric!(ParamSpecUInt64, ParamSpecUInt64Builder, u64);

define_param_spec!(ParamSpecUnichar, gobject_ffi::GParamSpecUnichar, 9);
define_param_spec_default!(ParamSpecUnichar, gobject_ffi::GParamSpecUnichar, Result<char, CharTryFromError>, TryFrom::try_from);

impl ParamSpecUnichar {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_unichar")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: char,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: char,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_unichar(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                default_value.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecUnichar,
    ParamSpecUnicharBuilder {
        default_value: char,
    }
    requires (default_value: char,)
);

define_param_spec!(ParamSpecEnum, gobject_ffi::GParamSpecEnum, 10);

impl ParamSpecEnum {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_enum")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        enum_type: crate::Type,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        assert!(enum_type.is_a(Type::ENUM));
        unsafe { Self::new_unchecked(name, nick, blurb, enum_type, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        enum_type: crate::Type,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_enum(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                enum_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_enum_class")]
    #[inline]
    pub fn enum_class(&self) -> crate::EnumClass {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecEnum>::to_glib_none(self).0;

            debug_assert!(!(*ptr).enum_class.is_null());

            crate::EnumClass::new(from_glib((*(*ptr).enum_class).g_type_class.g_type))
                .expect("Invalid enum class")
        }
    }

    #[inline]
    pub fn default_value<T: StaticType + FromGlib<i32>>(&self) -> Result<T, crate::BoolError> {
        unsafe {
            if !self.enum_class().type_().is_a(T::static_type()) {
                return Err(bool_error!(
                    "Wrong type -- expected {} got {}",
                    self.enum_class().type_(),
                    T::static_type()
                ));
            }
            Ok(from_glib(self.default_value_as_i32()))
        }
    }

    #[inline]
    pub fn default_value_as_i32(&self) -> i32 {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecEnum>::to_glib_none(self).0;
            (*ptr).default_value
        }
    }

    pub fn builder_with_default<T: StaticType + FromGlib<i32> + IntoGlib<GlibType = i32>>(
        name: &str,
        default_value: T,
    ) -> ParamSpecEnumBuilder<T> {
        ParamSpecEnumBuilder::new(name, default_value)
    }

    pub fn builder<T: StaticType + FromGlib<i32> + IntoGlib<GlibType = i32> + Default>(
        name: &str,
    ) -> ParamSpecEnumBuilder<T> {
        ParamSpecEnumBuilder::new(name, T::default())
    }
}

#[must_use]
pub struct ParamSpecEnumBuilder<'a, T: StaticType + FromGlib<i32> + IntoGlib<GlibType = i32>> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    default_value: T,
}

impl<'a, T: StaticType + FromGlib<i32> + IntoGlib<GlibType = i32>> ParamSpecEnumBuilder<'a, T> {
    fn new(name: &'a str, default_value: T) -> Self {
        assert_param_name(name);
        assert!(T::static_type().is_a(Type::ENUM));

        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            default_value,
        }
    }

    pub fn default_value(mut self, default: T) -> Self {
        self.default_value = default;
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecEnum::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                T::static_type(),
                self.default_value.into_glib(),
                self.flags,
            )
        }
    }
}

impl<'a, T: StaticType + FromGlib<i32> + IntoGlib<GlibType = i32>>
    crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecEnumBuilder<'a, T>
{
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec!(ParamSpecFlags, gobject_ffi::GParamSpecFlags, 11);

impl ParamSpecFlags {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_flags")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        flags_type: crate::Type,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        assert!(flags_type.is_a(Type::FLAGS));
        unsafe { Self::new_unchecked(name, nick, blurb, flags_type, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        flags_type: crate::Type,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_flags(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                flags_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_flags_class")]
    #[inline]
    pub fn flags_class(&self) -> crate::FlagsClass {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecFlags>::to_glib_none(self).0;

            debug_assert!(!(*ptr).flags_class.is_null());

            crate::FlagsClass::new(from_glib((*(*ptr).flags_class).g_type_class.g_type))
                .expect("Invalid flags class")
        }
    }

    #[inline]
    pub fn default_value<T: StaticType + FromGlib<u32>>(&self) -> Result<T, crate::BoolError> {
        unsafe {
            if !self.flags_class().type_().is_a(T::static_type()) {
                return Err(bool_error!(
                    "Wrong type -- expected {} got {}",
                    self.flags_class().type_(),
                    T::static_type()
                ));
            }
            Ok(from_glib(self.default_value_as_u32()))
        }
    }

    #[inline]
    pub fn default_value_as_u32(&self) -> u32 {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecFlags>::to_glib_none(self).0;
            (*ptr).default_value
        }
    }

    pub fn builder<T: StaticType + FromGlib<u32> + IntoGlib<GlibType = u32>>(
        name: &str,
    ) -> ParamSpecFlagsBuilder<T> {
        ParamSpecFlagsBuilder::new(name)
    }
}

#[must_use]
pub struct ParamSpecFlagsBuilder<'a, T: StaticType + FromGlib<u32> + IntoGlib<GlibType = u32>> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    default_value: T,
}

impl<'a, T: StaticType + FromGlib<u32> + IntoGlib<GlibType = u32>> ParamSpecFlagsBuilder<'a, T> {
    fn new(name: &'a str) -> Self {
        assert_param_name(name);
        assert!(T::static_type().is_a(Type::FLAGS));

        unsafe {
            Self {
                name,
                nick: None,
                blurb: None,
                flags: crate::ParamFlags::default(),
                default_value: from_glib(0),
            }
        }
    }

    #[doc = "Default: 0`"]
    pub fn default_value(mut self, value: T) -> Self {
        self.default_value = value;
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecFlags::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                T::static_type(),
                self.default_value.into_glib(),
                self.flags,
            )
        }
    }
}

impl<'a, T: StaticType + FromGlib<u32> + IntoGlib<GlibType = u32>>
    crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecFlagsBuilder<'a, T>
{
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec_numeric!(
    ParamSpecFloat,
    gobject_ffi::GParamSpecFloat,
    f32,
    12,
    g_param_spec_float,
    "g_param_spec_float"
);

define_builder_numeric!(ParamSpecFloat, ParamSpecFloatBuilder, f32);

define_param_spec_numeric!(
    ParamSpecDouble,
    gobject_ffi::GParamSpecDouble,
    f64,
    13,
    g_param_spec_double,
    "g_param_spec_double"
);

define_builder_numeric!(ParamSpecDouble, ParamSpecDoubleBuilder, f64);

define_param_spec!(ParamSpecString, gobject_ffi::GParamSpecString, 14);

define_param_spec_default!(
    ParamSpecString,
    gobject_ffi::GParamSpecString,
    Option<&str>,
    |x: *mut libc::c_char| {
        use std::ffi::CStr;

        if x.is_null() {
            None
        } else {
            Some(CStr::from_ptr(x).to_str().unwrap())
        }
    }
);

impl ParamSpecString {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_string")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: Option<&str>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        default_value: Option<&str>,
        flags: ParamFlags,
    ) -> ParamSpec {
        let default_value = default_value.to_glib_none();
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_string(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                default_value.0,
                flags.into_glib(),
            ))
        }
    }

    pub fn builder(name: &str) -> ParamSpecStringBuilder {
        ParamSpecStringBuilder::new(name)
    }
}

#[must_use]
pub struct ParamSpecStringBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    default_value: Option<&'a str>,
}

impl<'a> ParamSpecStringBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_param_name(name);
        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            default_value: None,
        }
    }

    #[doc = "Default: None`"]
    pub fn default_value(mut self, value: impl Into<Option<&'a str>>) -> Self {
        self.default_value = value.into();
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecString::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                self.default_value,
                self.flags,
            )
        }
    }
}

impl<'a> crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecStringBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec!(ParamSpecParam, gobject_ffi::GParamSpecParam, 15);

impl ParamSpecParam {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_param")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        param_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, param_type, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        param_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert!(param_type.is_a(crate::Type::PARAM_SPEC));
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_param(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                param_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecParam,
    ParamSpecParamBuilder {
        param_type: crate::Type,
    }
    requires (param_type: crate::Type,)
);

define_param_spec!(ParamSpecBoxed, gobject_ffi::GParamSpecBoxed, 16);

impl ParamSpecBoxed {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_boxed")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        boxed_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        assert!(boxed_type.is_a(Type::BOXED));
        unsafe { Self::new_unchecked(name, nick, blurb, boxed_type, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        boxed_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boxed(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                boxed_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    pub fn builder<T: StaticType>(name: &str) -> ParamSpecBoxedBuilder<T> {
        ParamSpecBoxedBuilder::new(name)
    }
}

#[must_use]
pub struct ParamSpecBoxedBuilder<'a, T: StaticType> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: StaticType> ParamSpecBoxedBuilder<'a, T> {
    fn new(name: &'a str) -> Self {
        assert_param_name(name);
        assert!(T::static_type().is_a(Type::BOXED));
        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            phantom: Default::default(),
        }
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecBoxed::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                T::static_type(),
                self.flags,
            )
        }
    }
}

impl<'a, T: StaticType> crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecBoxedBuilder<'a, T> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec!(ParamSpecPointer, gobject_ffi::GParamSpecPointer, 17);

impl ParamSpecPointer {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_pointer")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_pointer(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(ParamSpecPointer, ParamSpecPointerBuilder {});

define_param_spec!(ParamSpecValueArray, gobject_ffi::GParamSpecValueArray, 18);

impl ParamSpecValueArray {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_value_array")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        element_spec: Option<impl AsRef<ParamSpec>>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, element_spec, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        element_spec: Option<impl AsRef<ParamSpec>>,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_value_array(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                element_spec.as_ref().map(|p| p.as_ref()).to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_element_spec")]
    #[inline]
    pub fn element_spec(&self) -> Option<&ParamSpec> {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecValueArray>::to_glib_none(self).0;

            if (*ptr).element_spec.is_null() {
                None
            } else {
                Some(
                    &*(&(*ptr).element_spec as *const *mut gobject_ffi::GParamSpec
                        as *const ParamSpec),
                )
            }
        }
    }

    #[doc(alias = "get_fixed_n_elements")]
    #[inline]
    pub fn fixed_n_elements(&self) -> u32 {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecValueArray>::to_glib_none(self).0;

            (*ptr).fixed_n_elements
        }
    }

    pub fn builder(name: &str) -> ParamSpecValueArrayBuilder {
        ParamSpecValueArrayBuilder::new(name)
    }
}

#[must_use]
pub struct ParamSpecValueArrayBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    element_spec: Option<&'a ParamSpec>,
}

impl<'a> ParamSpecValueArrayBuilder<'a> {
    fn new(name: &'a str) -> Self {
        assert_param_name(name);
        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            element_spec: None,
        }
    }

    #[doc = "Default: None`"]
    pub fn element_spec(mut self, value: impl Into<Option<&'a ParamSpec>>) -> Self {
        self.element_spec = value.into();
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecValueArray::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                self.element_spec,
                self.flags,
            )
        }
    }
}

impl<'a> crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecValueArrayBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec!(ParamSpecObject, gobject_ffi::GParamSpecObject, 19);

impl ParamSpecObject {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_object")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        object_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        assert!(object_type.is_a(Type::OBJECT));

        unsafe { Self::new_unchecked(name, nick, blurb, object_type, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        object_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_object(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                object_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }

    pub fn builder<T: StaticType + IsA<Object>>(name: &str) -> ParamSpecObjectBuilder<T> {
        ParamSpecObjectBuilder::new(name)
    }
}

#[must_use]
pub struct ParamSpecObjectBuilder<'a, T: StaticType> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T: StaticType> ParamSpecObjectBuilder<'a, T> {
    fn new(name: &'a str) -> Self {
        assert_param_name(name);

        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            phantom: Default::default(),
        }
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecObject::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                T::static_type(),
                self.flags,
            )
        }
    }
}

impl<'a, T: StaticType> crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecObjectBuilder<'a, T> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

define_param_spec!(ParamSpecOverride, gobject_ffi::GParamSpecOverride, 20);

impl ParamSpecOverride {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_override")]
    #[deprecated = "Use builder() instead"]
    pub fn new(name: &str, overridden: impl AsRef<ParamSpec>) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, overridden) }
    }

    unsafe fn new_unchecked(name: &str, overridden: impl AsRef<ParamSpec>) -> ParamSpec {
        from_glib_none(gobject_ffi::g_param_spec_override(
            name.to_glib_none().0,
            overridden.as_ref().to_glib_none().0,
        ))
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`ParamSpecOverride::new`] but specific for an interface.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let pspec = ParamSpecOverride::for_interface::<gtk::Scrollable>("vadjustment");
    /// ```
    ///
    /// # Panics
    ///
    /// If the property `name` doesn't exist in the interface.
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_override")]
    pub fn for_interface<T: IsA<Object> + IsInterface>(name: &str) -> ParamSpec {
        assert_param_name(name);
        // in case it's an interface
        let interface_ref: InterfaceRef<T> = Interface::from_type(T::static_type()).unwrap();
        let pspec = interface_ref
            .find_property(name)
            .unwrap_or_else(|| panic!("Couldn't find a property named `{name}` to override"));

        unsafe { Self::new_unchecked(name, &pspec) }
    }

    // rustdoc-stripper-ignore-next
    /// Similar to [`ParamSpecOverride::new`] but specific for a class.
    ///
    /// # Examples
    ///
    /// ```rust, ignore
    /// let pspec = ParamSpecOverride::for_class::<gtk::Button>("label");
    /// ```
    ///
    /// # Panics
    ///
    /// If the property `name` doesn't exist in the class.
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_override")]
    pub fn for_class<T: IsA<Object> + IsClass>(name: &str) -> ParamSpec {
        assert_param_name(name);
        let pspec = ObjectClass::from_type(T::static_type())
            .unwrap()
            .find_property(name)
            .unwrap_or_else(|| panic!("Couldn't find a property named `{name}` to override"));

        unsafe { Self::new_unchecked(name, &pspec) }
    }

    #[doc(alias = "get_overridden")]
    #[inline]
    pub fn overridden(&self) -> ParamSpec {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecOverride>::to_glib_none(self).0;

            from_glib_none((*ptr).overridden)
        }
    }

    pub fn builder<'a>(name: &'a str, overridden: &'a ParamSpec) -> ParamSpecOverrideBuilder<'a> {
        ParamSpecOverrideBuilder::new(name, overridden)
    }
}

// This builder is not autogenerated because it's the only one that doesn't take
// `nick`, `blurb` and `flags` as parameters.
#[must_use]
pub struct ParamSpecOverrideBuilder<'a> {
    name: &'a str,
    overridden: &'a ParamSpec,
}

impl<'a> ParamSpecOverrideBuilder<'a> {
    fn new(name: &'a str, overridden: &'a ParamSpec) -> Self {
        assert_param_name(name);
        Self { name, overridden }
    }
    pub fn overridden(mut self, spec: &'a ParamSpec) -> Self {
        self.overridden = spec;
        self
    }
    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe { ParamSpecOverride::new_unchecked(self.name, self.overridden) }
    }
}

define_param_spec!(ParamSpecGType, gobject_ffi::GParamSpecGType, 21);

impl ParamSpecGType {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_gtype")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        is_a_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, is_a_type, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        is_a_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_gtype(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                is_a_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecGType,
    ParamSpecGTypeBuilder {
        is_a_type: crate::Type = crate::Type::UNIT,
    }
);

define_param_spec!(ParamSpecVariant, gobject_ffi::GParamSpecVariant, 22);

define_param_spec_default!(
    ParamSpecVariant,
    gobject_ffi::GParamSpecVariant,
    Option<crate::Variant>,
    |x: *mut ffi::GVariant| from_glib_none(x)
);

impl ParamSpecVariant {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_variant")]
    #[deprecated = "Use builder() instead"]
    pub fn new<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        type_: &crate::VariantTy,
        default_value: Option<&crate::Variant>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, nick, blurb, type_, default_value, flags) }
    }

    unsafe fn new_unchecked<'a>(
        name: &str,
        nick: impl Into<Option<&'a str>>,
        blurb: impl Into<Option<&'a str>>,
        type_: &crate::VariantTy,
        default_value: Option<&crate::Variant>,
        flags: ParamFlags,
    ) -> ParamSpec {
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_variant(
                name.to_glib_none().0,
                nick.into().to_glib_none().0,
                blurb.into().to_glib_none().0,
                type_.to_glib_none().0,
                default_value.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_type")]
    #[inline]
    pub fn type_(&self) -> Option<&crate::VariantTy> {
        unsafe {
            let ptr = ToGlibPtr::<*const gobject_ffi::GParamSpecVariant>::to_glib_none(self).0;

            if (*ptr).type_.is_null() {
                None
            } else {
                Some(crate::VariantTy::from_ptr((*ptr).type_))
            }
        }
    }

    pub fn builder<'a>(name: &'a str, type_: &'a crate::VariantTy) -> ParamSpecVariantBuilder<'a> {
        ParamSpecVariantBuilder::new(name, type_)
    }
}

#[must_use]
pub struct ParamSpecVariantBuilder<'a> {
    name: &'a str,
    nick: Option<&'a str>,
    blurb: Option<&'a str>,
    flags: crate::ParamFlags,
    type_: &'a crate::VariantTy,
    default_value: Option<&'a crate::Variant>,
}

impl<'a> ParamSpecVariantBuilder<'a> {
    fn new(name: &'a str, type_: &'a crate::VariantTy) -> Self {
        assert_param_name(name);
        Self {
            name,
            nick: None,
            blurb: None,
            flags: crate::ParamFlags::default(),
            type_,
            default_value: None,
        }
    }

    #[doc = "Default: None`"]
    pub fn default_value(mut self, value: impl Into<Option<&'a crate::Variant>>) -> Self {
        self.default_value = value.into();
        self
    }

    #[must_use]
    pub fn build(self) -> ParamSpec {
        unsafe {
            ParamSpecVariant::new_unchecked(
                self.name,
                self.nick,
                self.blurb,
                self.type_,
                self.default_value,
                self.flags,
            )
        }
    }
}

impl<'a> crate::prelude::ParamSpecBuilderExt<'a> for ParamSpecVariantBuilder<'a> {
    fn set_nick(&mut self, nick: Option<&'a str>) {
        self.nick = nick;
    }
    fn set_blurb(&mut self, blurb: Option<&'a str>) {
        self.blurb = blurb;
    }
    fn set_flags(&mut self, flags: crate::ParamFlags) {
        self.flags = flags;
    }
    fn current_flags(&self) -> crate::ParamFlags {
        self.flags
    }
}

pub trait HasParamSpec {
    type ParamSpec;

    // rustdoc-stripper-ignore-next
    /// Preferred value to be used as setter for the associated ParamSpec.
    type SetValue: ?Sized;
    type BuilderFn;
    fn param_spec_builder() -> Self::BuilderFn;
}

impl<T: crate::value::ToValueOptional + HasParamSpec> HasParamSpec for Option<T> {
    type ParamSpec = T::ParamSpec;
    type SetValue = T::SetValue;
    type BuilderFn = T::BuilderFn;

    fn param_spec_builder() -> Self::BuilderFn {
        T::param_spec_builder()
    }
}
impl<T: HasParamSpec + ?Sized> HasParamSpec for &T {
    type ParamSpec = T::ParamSpec;
    type SetValue = T::SetValue;
    type BuilderFn = T::BuilderFn;

    fn param_spec_builder() -> Self::BuilderFn {
        T::param_spec_builder()
    }
}
impl HasParamSpec for crate::GString {
    type ParamSpec = ParamSpecString;
    type SetValue = str;
    type BuilderFn = fn(&str) -> ParamSpecStringBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for str {
    type ParamSpec = ParamSpecString;
    type SetValue = str;
    type BuilderFn = fn(&str) -> ParamSpecStringBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for String {
    type ParamSpec = ParamSpecString;
    type SetValue = str;
    type BuilderFn = fn(&str) -> ParamSpecStringBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for Box<str> {
    type ParamSpec = ParamSpecString;
    type SetValue = str;
    type BuilderFn = fn(&str) -> ParamSpecStringBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for crate::StrV {
    type ParamSpec = ParamSpecBoxed;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecBoxedBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for Vec<String> {
    type ParamSpec = ParamSpecBoxed;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecBoxedBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for char {
    type ParamSpec = ParamSpecUnichar;
    type SetValue = Self;
    type BuilderFn = fn(&str, char) -> ParamSpecUnicharBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for f64 {
    type ParamSpec = ParamSpecDouble;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecDoubleBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for f32 {
    type ParamSpec = ParamSpecFloat;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecFloatBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for i64 {
    type ParamSpec = ParamSpecInt64;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecInt64Builder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for i32 {
    type ParamSpec = ParamSpecInt;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecIntBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for i8 {
    type ParamSpec = ParamSpecChar;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecCharBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for u64 {
    type ParamSpec = ParamSpecUInt64;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecUInt64Builder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for u32 {
    type ParamSpec = ParamSpecUInt;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecUIntBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for u8 {
    type ParamSpec = ParamSpecUChar;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecUCharBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}
impl HasParamSpec for bool {
    type ParamSpec = ParamSpecBoolean;
    type SetValue = Self;
    type BuilderFn = fn(&str) -> ParamSpecBooleanBuilder;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

impl HasParamSpec for crate::Variant {
    type ParamSpec = ParamSpecVariant;
    type SetValue = Self;
    type BuilderFn =
        fn(&'static str, ty: &'static crate::VariantTy) -> ParamSpecVariantBuilder<'static>;

    fn param_spec_builder() -> Self::BuilderFn {
        Self::ParamSpec::builder
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn test_param_spec_string() {
        let pspec =
            ParamSpecString::new("name", None, None, Some("default"), ParamFlags::READWRITE);

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.nick(), "name");
        assert_eq!(pspec.blurb(), None);
        let default_value = pspec.default_value();
        assert_eq!(default_value.get::<&str>().unwrap(), "default");
        assert_eq!(pspec.flags(), ParamFlags::READWRITE);
        assert_eq!(pspec.value_type(), Type::STRING);
        assert_eq!(pspec.type_(), ParamSpecString::static_type());

        let pspec_ref = pspec
            .downcast_ref::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec_ref.default_value(), Some("default"));

        let pspec = pspec
            .downcast::<ParamSpecString>()
            .expect("Not a string param spec");
        assert_eq!(pspec.default_value(), Some("default"));
    }

    #[test]
    fn test_param_spec_int_builder() {
        let pspec = ParamSpecInt::builder("name")
            .blurb("Simple int parameter")
            .minimum(-2)
            .explicit_notify()
            .build();

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.nick(), "name");
        assert_eq!(pspec.blurb(), Some("Simple int parameter"));
        assert_eq!(
            pspec.flags(),
            ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY
        );
    }

    #[test]
    fn test_param_spec_builder_flags() {
        let pspec = ParamSpecInt::builder("name")
            .minimum(-2)
            .read_only()
            .build()
            .downcast::<ParamSpecInt>()
            .unwrap();
        assert_eq!(pspec.minimum(), -2);
        assert_eq!(pspec.flags(), ParamFlags::READABLE);

        let pspec = ParamSpecInt::builder("name")
            .read_only()
            .write_only()
            .minimum(-2)
            .build()
            .downcast::<ParamSpecInt>()
            .unwrap();
        assert_eq!(pspec.minimum(), -2);
        assert_eq!(pspec.flags(), ParamFlags::WRITABLE);

        let pspec = ParamSpecInt::builder("name")
            .read_only()
            .write_only()
            .readwrite()
            .minimum(-2)
            .build()
            .downcast::<ParamSpecInt>()
            .unwrap();
        assert_eq!(pspec.minimum(), -2);
        assert_eq!(pspec.flags(), ParamFlags::READWRITE);
    }

    #[test]
    fn test_has_param_spec() {
        let pspec = <i32 as HasParamSpec>::param_spec_builder()("name")
            .blurb("Simple int parameter")
            .minimum(-2)
            .explicit_notify()
            .build();

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.blurb(), Some("Simple int parameter"));
        assert_eq!(
            pspec.flags(),
            ParamFlags::READWRITE | ParamFlags::EXPLICIT_NOTIFY
        );
    }
}
