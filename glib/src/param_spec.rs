// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::utils::is_canonical_pspec_name;
use crate::IsA;
use crate::ParamFlags;
use crate::StaticType;
use crate::Type;
use crate::Value;
use crate::{
    object::{Interface, InterfaceRef, IsClass, IsInterface, ObjectClass},
    Object,
};

use std::char::CharTryFromError;
use std::convert::TryFrom;
use std::ffi::CStr;

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
        assert!(!ptr.is_null());
        from_glib_full(ptr as *mut gobject_ffi::GParamSpec)
    }
}

#[doc(hidden)]
unsafe impl<'a> crate::value::FromValue<'a> for &'a ParamSpec {
    type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

    unsafe fn from_value(value: &'a crate::Value) -> Self {
        assert_eq!(
            std::mem::size_of::<Self>(),
            std::mem::size_of::<crate::ffi::gpointer>()
        );
        let value = &*(value as *const crate::Value as *const crate::gobject_ffi::GValue);
        let ptr = &value.data[0].v_pointer as *const crate::ffi::gpointer
            as *const *const gobject_ffi::GParamSpec;
        assert!(!(*ptr).is_null());
        &*(ptr as *const ParamSpec)
    }
}

#[doc(hidden)]
impl crate::value::ToValue for ParamSpec {
    fn to_value(&self) -> crate::Value {
        unsafe {
            let mut value = crate::Value::from_type(ParamSpec::static_type());
            gobject_ffi::g_value_take_param(
                value.to_glib_none_mut().0,
                self.to_glib_full() as *mut _,
            );
            value
        }
    }

    fn value_type(&self) -> crate::Type {
        ParamSpec::static_type()
    }
}

#[doc(hidden)]
impl crate::value::ToValueOptional for ParamSpec {
    fn to_value_optional(s: Option<&Self>) -> crate::Value {
        let mut value = crate::Value::for_value_type::<Self>();
        unsafe {
            gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full() as *mut _);
        }

        value
    }
}

impl AsRef<ParamSpec> for ParamSpec {
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
                Ok(from_glib_full(self.to_glib_full()))
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
    pub fn type_(&self) -> Type {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib((*(*ptr).g_type_instance.g_class).g_type)
        }
    }

    #[doc(alias = "get_value_type")]
    pub fn value_type(&self) -> crate::Type {
        unsafe { from_glib((*self.to_glib_none().0).value_type) }
    }

    #[doc(alias = "get_owner_type")]
    pub fn owner_type(&self) -> crate::Type {
        unsafe { from_glib((*self.to_glib_none().0).owner_type) }
    }

    #[doc(alias = "get_flags")]
    pub fn flags(&self) -> ParamFlags {
        unsafe { from_glib((*self.to_glib_none().0).flags) }
    }

    #[doc(alias = "g_param_spec_get_blurb")]
    #[doc(alias = "get_blurb")]
    pub fn blurb(&self) -> &str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_blurb(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_param_spec_get_default_value")]
    #[doc(alias = "get_default_value")]
    pub fn default_value(&self) -> &Value {
        unsafe {
            &*(gobject_ffi::g_param_spec_get_default_value(self.to_glib_none().0)
                as *const crate::Value)
        }
    }

    #[doc(alias = "g_param_spec_get_name")]
    #[doc(alias = "get_name")]
    pub fn name<'a>(&self) -> &'a str {
        unsafe {
            CStr::from_ptr(gobject_ffi::g_param_spec_get_name(self.to_glib_none().0))
                .to_str()
                .unwrap()
        }
    }

    #[doc(alias = "g_param_spec_get_name_quark")]
    #[doc(alias = "get_name_quark")]
    pub fn name_quark(&self) -> crate::Quark {
        unsafe {
            from_glib(gobject_ffi::g_param_spec_get_name_quark(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_param_spec_get_nick")]
    #[doc(alias = "get_nick")]
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
                assert!(!ptr.is_null());
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a> crate::value::FromValue<'a> for &'a $rust_type {
            type Checker = crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a crate::Value) -> Self {
                assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<crate::ffi::gpointer>());
                let value = &*(value as *const crate::Value as *const crate::gobject_ffi::GValue);
                let ptr = &value.data[0].v_pointer as *const crate::ffi::gpointer as *const *const gobject_ffi::GParamSpec;
                assert!(!(*ptr).is_null());
                &*(ptr as *const $rust_type)
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValue for $rust_type {
            fn to_value(&self) -> crate::Value {
                unsafe {
                    let mut value = crate::Value::from_type($rust_type::static_type());
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, self.to_glib_full() as *mut _);
                    value
                }
            }

            fn value_type(&self) -> crate::Type {
                $rust_type::static_type()
            }
        }

        #[doc(hidden)]
        impl crate::value::ToValueOptional for $rust_type {
            fn to_value_optional(s: Option<&Self>) -> crate::Value {
                let mut value = crate::Value::for_value_type::<Self>();
                unsafe {
                    gobject_ffi::g_value_take_param(value.to_glib_none_mut().0, s.to_glib_full() as *mut _);
                }

                value
            }
        }

        unsafe impl Send for $rust_type {}
        unsafe impl Sync for $rust_type {}

        impl std::ops::Deref for $rust_type {
            type Target = ParamSpec;

            fn deref(&self) -> &Self::Target {
                unsafe {
                    &*(self as *const $rust_type as *const ParamSpec)
                }
            }
        }

        unsafe impl ParamSpecType for $rust_type {}

        #[doc(hidden)]
        impl FromGlibPtrFull<*mut gobject_ffi::GParamSpec> for $rust_type {
            unsafe fn from_glib_full(ptr: *mut gobject_ffi::GParamSpec) -> Self {
                from_glib_full(ptr as *mut $ffi_type)
            }
        }

        impl $rust_type {
            pub fn upcast(self) -> ParamSpec {
                unsafe {
                    from_glib_full(self.to_glib_full() as *mut gobject_ffi::GParamSpec)
                }
            }

            pub fn upcast_ref(&self) -> &ParamSpec {
                &*self
            }
        }

        impl AsRef<ParamSpec> for $rust_type {
            fn as_ref(&self) -> &ParamSpec {
                &self
            }
        }
    };
}

macro_rules! define_param_spec_default {
    ($rust_type:ident, $value_type:ty, $from_glib:expr) => {
        impl $rust_type {
            pub fn default_value(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    $from_glib((*ptr).default_value)
                }
            }
        }
    };
}

macro_rules! define_param_spec_min_max {
    ($rust_type:ident, $value_type:ty) => {
        impl $rust_type {
            pub fn minimum(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    (*ptr).minimum
                }
            }

            pub fn maximum(&self) -> $value_type {
                unsafe {
                    let ptr = self.to_glib_none().0;
                    (*ptr).maximum
                }
            }
        }
    };
}

macro_rules! define_param_spec_numeric {
    ($rust_type:ident, $ffi_type:path, $value_type:ty, $rust_type_offset:expr, $ffi_fun:ident, $alias:literal) => {
        define_param_spec!($rust_type, $ffi_type, $rust_type_offset);
        define_param_spec_default!($rust_type, $value_type, |x| x);
        define_param_spec_min_max!($rust_type, $value_type);

        impl $rust_type {
            #[allow(clippy::new_ret_no_self)]
            #[doc(alias = $alias)]
            pub fn new(
                name: &str,
                nick: &str,
                blurb: &str,
                minimum: $value_type,
                maximum: $value_type,
                default_value: $value_type,
                flags: ParamFlags,
            ) -> ParamSpec {
                assert_param_name(name);
                unsafe {
                    from_glib_none(gobject_ffi::$ffi_fun(
                        name.to_glib_none().0,
                        nick.to_glib_none().0,
                        blurb.to_glib_none().0,
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

macro_rules! define_builder {
    (@constructors $rust_type:ident, $builder_type:ident $(($($req_ident:ident: $req_ty:ty,)*))?) => {
        impl<'a> $builder_type<'a> {
            fn new(name: &'a str, $($($req_ident: $req_ty)*)?) -> Self {
                Self {
                    name,
                    $($($req_ident: Some($req_ident),)*)?
                    ..Default::default()
                }
            }
        }

        impl $rust_type {
            pub fn builder<'a>(name: &'a str, $($($req_ident: $req_ty),*)?) -> $builder_type<'a> {
                $builder_type::new(name, $($($req_ident),*)?)
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
            /// Default: `self.name`
            pub fn nick(mut self, nick: &'a str) -> Self {
                self.nick = Some(nick);
                self
            }
            /// Default: `self.name`
            pub fn blurb(mut self, blurb: &'a str) -> Self {
                self.blurb = Some(blurb);
                self
            }

            /// Default: `glib::ParamFlags::READWRITE`
            pub fn flags(mut self, flags: crate::ParamFlags) -> Self {
                self.flags = flags;
                self
            }

            $(
            $(#[doc = concat!("Default: `", stringify!($field_expr), "`")])?
            pub fn $field_id(mut self, value: $field_ty) -> Self {
                self.$field_id = Some(value);
                self
            }
            )*

            #[must_use]
            pub fn build(self) -> ParamSpec {
                $rust_type::new(
                    self.name,
                    self.nick.unwrap_or(self.name),
                    self.blurb.unwrap_or(self.name),
                    $(self
                        .$field_id
                        $(.or(Some($field_expr)))?
                        .expect("impossible: missing parameter in ParamSpec*Builder")
                    ,)*
                    self.flags
                )
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
        "{} is not a valid canonical parameter name",
        name
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

define_param_spec_default!(ParamSpecBoolean, bool, |x| from_glib(x));

impl ParamSpecBoolean {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_boolean")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: bool,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boolean(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
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
define_param_spec_default!(ParamSpecUnichar, Result<char, CharTryFromError>, TryFrom::try_from);

impl ParamSpecUnichar {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_unichar")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: char,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_unichar(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
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

define_param_spec_default!(ParamSpecEnum, i32, |x| x);

impl ParamSpecEnum {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_enum")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        enum_type: crate::Type,
        default_value: i32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_enum(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                enum_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_enum_class")]
    pub fn enum_class(&self) -> crate::EnumClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).enum_class.is_null());

            crate::EnumClass::new(from_glib((*(*ptr).enum_class).g_type_class.g_type))
                .expect("Invalid enum class")
        }
    }
}

define_builder!(
    ParamSpecEnum,
    ParamSpecEnumBuilder {
        enum_type: crate::Type,
        default_value: i32 = 0i32,
    }
    requires (enum_type: crate::Type,)
);
define_param_spec!(ParamSpecFlags, gobject_ffi::GParamSpecFlags, 11);

define_param_spec_default!(ParamSpecFlags, u32, |x| x);

impl ParamSpecFlags {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_flags")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        flags_type: crate::Type,
        default_value: u32,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_flags(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                flags_type.into_glib(),
                default_value,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_flags_class")]
    pub fn flags_class(&self) -> crate::FlagsClass {
        unsafe {
            let ptr = self.to_glib_none().0;

            assert!(!(*ptr).flags_class.is_null());

            crate::FlagsClass::new(from_glib((*(*ptr).flags_class).g_type_class.g_type))
                .expect("Invalid flags class")
        }
    }
}

define_builder!(
    ParamSpecFlags,
    ParamSpecFlagsBuilder {
        flags_type: crate::Type,
        default_value: u32 = 0u32,
    }
    requires (flags_type: crate::Type,)
);

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

define_param_spec_default!(ParamSpecString, Option<&str>, |x: *mut libc::c_char| {
    use std::ffi::CStr;

    if x.is_null() {
        None
    } else {
        Some(CStr::from_ptr(x).to_str().unwrap())
    }
});

impl ParamSpecString {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_string")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        default_value: Option<&str>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        let default_value = default_value.to_glib_none();
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_string(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                default_value.0,
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecString,
    ParamSpecStringBuilder {
        default_value: Option<&'a str> = None,
    }
);

define_param_spec!(ParamSpecParam, gobject_ffi::GParamSpecParam, 15);

impl ParamSpecParam {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_param")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        param_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_param(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
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
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        boxed_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_boxed(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                boxed_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecBoxed,
    ParamSpecBoxedBuilder {
        boxed_type: crate::Type,
    }
    requires (boxed_type: crate::Type,)
);

define_param_spec!(ParamSpecPointer, gobject_ffi::GParamSpecPointer, 17);

impl ParamSpecPointer {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_pointer")]
    pub fn new(name: &str, nick: &str, blurb: &str, flags: ParamFlags) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_pointer(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
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
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        element_spec: &ParamSpec,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_value_array(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                element_spec.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_element_spec")]
    pub fn element_spec(&self) -> Option<ParamSpec> {
        unsafe {
            let ptr = self.to_glib_none().0;

            from_glib_none((*ptr).element_spec)
        }
    }

    #[doc(alias = "get_fixed_n_elements")]
    pub fn fixed_n_elements(&self) -> u32 {
        unsafe {
            let ptr = self.to_glib_none().0;

            (*ptr).fixed_n_elements
        }
    }
}

define_builder!(
    ParamSpecValueArray,
    ParamSpecValueArrayBuilder {
        element_spec: &'a ParamSpec,
    }
    requires (element_spec: &'a ParamSpec,)
);

define_param_spec!(ParamSpecObject, gobject_ffi::GParamSpecObject, 19);

impl ParamSpecObject {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_object")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        object_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_object(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                object_type.into_glib(),
                flags.into_glib(),
            ))
        }
    }
}

define_builder!(
    ParamSpecObject,
    ParamSpecObjectBuilder {
        object_type: crate::Type,
    }
    requires (object_type: crate::Type,)
);
define_param_spec!(ParamSpecOverride, gobject_ffi::GParamSpecOverride, 20);

impl ParamSpecOverride {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_override")]
    pub fn new(name: &str, overridden: &ParamSpec) -> ParamSpec {
        assert_param_name(name);
        unsafe { Self::new_unchecked(name, overridden) }
    }
    unsafe fn new_unchecked(name: &str, overridden: &ParamSpec) -> ParamSpec {
        from_glib_none(gobject_ffi::g_param_spec_override(
            name.to_glib_none().0,
            overridden.to_glib_none().0,
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
            .unwrap_or_else(|| panic!("Couldn't find a property named `{}` to override", name));

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
            .unwrap_or_else(|| panic!("Couldn't find a property named `{}` to override", name));

        unsafe { Self::new_unchecked(name, &pspec) }
    }

    #[doc(alias = "get_overridden")]
    pub fn overridden(&self) -> ParamSpec {
        unsafe {
            let ptr = self.to_glib_none().0;

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
        Self { name, overridden }
    }
    pub fn overridden(mut self, spec: &'a ParamSpec) -> Self {
        self.overridden = spec;
        self
    }
    #[must_use]
    pub fn build(self) -> ParamSpec {
        ParamSpecOverride::new(self.name, self.overridden)
    }
}

define_param_spec!(ParamSpecGType, gobject_ffi::GParamSpecGType, 21);

impl ParamSpecGType {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_gtype")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        is_a_type: crate::Type,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_gtype(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
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
    Option<crate::Variant>,
    |x: *mut ffi::GVariant| from_glib_none(x)
);

impl ParamSpecVariant {
    #[allow(clippy::new_ret_no_self)]
    #[doc(alias = "g_param_spec_variant")]
    pub fn new(
        name: &str,
        nick: &str,
        blurb: &str,
        type_: &crate::VariantTy,
        default_value: Option<&crate::Variant>,
        flags: ParamFlags,
    ) -> ParamSpec {
        assert_param_name(name);
        unsafe {
            from_glib_none(gobject_ffi::g_param_spec_variant(
                name.to_glib_none().0,
                nick.to_glib_none().0,
                blurb.to_glib_none().0,
                type_.to_glib_none().0,
                default_value.to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Option<&crate::VariantTy> {
        unsafe {
            let ptr = self.to_glib_none().0;

            if (*ptr).type_.is_null() {
                None
            } else {
                Some(crate::VariantTy::from_ptr((*ptr).type_))
            }
        }
    }
}

define_builder!(
    ParamSpecVariant,
    ParamSpecVariantBuilder {
        type_: &'a crate::VariantTy,
        default_value: Option<&'a crate::Variant> = None,
    }
    requires (type_: &'a crate::VariantTy,)
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_spec_string() {
        let pspec = ParamSpecString::new(
            "name",
            "nick",
            "blurb",
            Some("default"),
            ParamFlags::READWRITE,
        );

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.nick(), "nick");
        assert_eq!(pspec.blurb(), "blurb");
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
            .build();

        assert_eq!(pspec.name(), "name");
        assert_eq!(pspec.nick(), "name");
        assert_eq!(pspec.blurb(), "Simple int parameter");
        assert_eq!(pspec.flags(), ParamFlags::READWRITE);
    }
}
