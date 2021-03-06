// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::error::ErrorDomain;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::ToValue;
use glib::Quark;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkColorspace")]
pub enum Colorspace {
    #[doc(alias = "GDK_COLORSPACE_RGB")]
    Rgb,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for Colorspace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Colorspace::{}",
            match *self {
                Self::Rgb => "Rgb",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for Colorspace {
    type GlibType = ffi::GdkColorspace;

    fn into_glib(self) -> ffi::GdkColorspace {
        match self {
            Self::Rgb => ffi::GDK_COLORSPACE_RGB,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkColorspace> for Colorspace {
    unsafe fn from_glib(value: ffi::GdkColorspace) -> Self {
        match value {
            ffi::GDK_COLORSPACE_RGB => Self::Rgb,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for Colorspace {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_colorspace_get_type()) }
    }
}

impl glib::value::ValueType for Colorspace {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for Colorspace {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for Colorspace {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkInterpType")]
pub enum InterpType {
    #[doc(alias = "GDK_INTERP_NEAREST")]
    Nearest,
    #[doc(alias = "GDK_INTERP_TILES")]
    Tiles,
    #[doc(alias = "GDK_INTERP_BILINEAR")]
    Bilinear,
    #[doc(alias = "GDK_INTERP_HYPER")]
    Hyper,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for InterpType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "InterpType::{}",
            match *self {
                Self::Nearest => "Nearest",
                Self::Tiles => "Tiles",
                Self::Bilinear => "Bilinear",
                Self::Hyper => "Hyper",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for InterpType {
    type GlibType = ffi::GdkInterpType;

    fn into_glib(self) -> ffi::GdkInterpType {
        match self {
            Self::Nearest => ffi::GDK_INTERP_NEAREST,
            Self::Tiles => ffi::GDK_INTERP_TILES,
            Self::Bilinear => ffi::GDK_INTERP_BILINEAR,
            Self::Hyper => ffi::GDK_INTERP_HYPER,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkInterpType> for InterpType {
    unsafe fn from_glib(value: ffi::GdkInterpType) -> Self {
        match value {
            ffi::GDK_INTERP_NEAREST => Self::Nearest,
            ffi::GDK_INTERP_TILES => Self::Tiles,
            ffi::GDK_INTERP_BILINEAR => Self::Bilinear,
            ffi::GDK_INTERP_HYPER => Self::Hyper,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for InterpType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_interp_type_get_type()) }
    }
}

impl glib::value::ValueType for InterpType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for InterpType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for InterpType {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[cfg_attr(feature = "v2_42", deprecated = "Since 2.42")]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkPixbufAlphaMode")]
pub enum PixbufAlphaMode {
    #[doc(alias = "GDK_PIXBUF_ALPHA_BILEVEL")]
    Bilevel,
    #[doc(alias = "GDK_PIXBUF_ALPHA_FULL")]
    Full,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PixbufAlphaMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PixbufAlphaMode::{}",
            match *self {
                Self::Bilevel => "Bilevel",
                Self::Full => "Full",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for PixbufAlphaMode {
    type GlibType = ffi::GdkPixbufAlphaMode;

    fn into_glib(self) -> ffi::GdkPixbufAlphaMode {
        match self {
            Self::Bilevel => ffi::GDK_PIXBUF_ALPHA_BILEVEL,
            Self::Full => ffi::GDK_PIXBUF_ALPHA_FULL,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufAlphaMode> for PixbufAlphaMode {
    unsafe fn from_glib(value: ffi::GdkPixbufAlphaMode) -> Self {
        match value {
            ffi::GDK_PIXBUF_ALPHA_BILEVEL => Self::Bilevel,
            ffi::GDK_PIXBUF_ALPHA_FULL => Self::Full,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PixbufAlphaMode {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_alpha_mode_get_type()) }
    }
}

impl glib::value::ValueType for PixbufAlphaMode {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PixbufAlphaMode {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PixbufAlphaMode {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkPixbufError")]
pub enum PixbufError {
    #[doc(alias = "GDK_PIXBUF_ERROR_CORRUPT_IMAGE")]
    CorruptImage,
    #[doc(alias = "GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY")]
    InsufficientMemory,
    #[doc(alias = "GDK_PIXBUF_ERROR_BAD_OPTION")]
    BadOption,
    #[doc(alias = "GDK_PIXBUF_ERROR_UNKNOWN_TYPE")]
    UnknownType,
    #[doc(alias = "GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION")]
    UnsupportedOperation,
    #[doc(alias = "GDK_PIXBUF_ERROR_FAILED")]
    Failed,
    #[doc(alias = "GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION")]
    IncompleteAnimation,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PixbufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PixbufError::{}",
            match *self {
                Self::CorruptImage => "CorruptImage",
                Self::InsufficientMemory => "InsufficientMemory",
                Self::BadOption => "BadOption",
                Self::UnknownType => "UnknownType",
                Self::UnsupportedOperation => "UnsupportedOperation",
                Self::Failed => "Failed",
                Self::IncompleteAnimation => "IncompleteAnimation",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for PixbufError {
    type GlibType = ffi::GdkPixbufError;

    fn into_glib(self) -> ffi::GdkPixbufError {
        match self {
            Self::CorruptImage => ffi::GDK_PIXBUF_ERROR_CORRUPT_IMAGE,
            Self::InsufficientMemory => ffi::GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY,
            Self::BadOption => ffi::GDK_PIXBUF_ERROR_BAD_OPTION,
            Self::UnknownType => ffi::GDK_PIXBUF_ERROR_UNKNOWN_TYPE,
            Self::UnsupportedOperation => ffi::GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION,
            Self::Failed => ffi::GDK_PIXBUF_ERROR_FAILED,
            Self::IncompleteAnimation => ffi::GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufError> for PixbufError {
    unsafe fn from_glib(value: ffi::GdkPixbufError) -> Self {
        match value {
            ffi::GDK_PIXBUF_ERROR_CORRUPT_IMAGE => Self::CorruptImage,
            ffi::GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY => Self::InsufficientMemory,
            ffi::GDK_PIXBUF_ERROR_BAD_OPTION => Self::BadOption,
            ffi::GDK_PIXBUF_ERROR_UNKNOWN_TYPE => Self::UnknownType,
            ffi::GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION => Self::UnsupportedOperation,
            ffi::GDK_PIXBUF_ERROR_FAILED => Self::Failed,
            ffi::GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION => Self::IncompleteAnimation,
            value => Self::__Unknown(value),
        }
    }
}

impl ErrorDomain for PixbufError {
    fn domain() -> Quark {
        unsafe { from_glib(ffi::gdk_pixbuf_error_quark()) }
    }

    fn code(self) -> i32 {
        self.into_glib()
    }

    fn from(code: i32) -> Option<Self> {
        match code {
            ffi::GDK_PIXBUF_ERROR_CORRUPT_IMAGE => Some(Self::CorruptImage),
            ffi::GDK_PIXBUF_ERROR_INSUFFICIENT_MEMORY => Some(Self::InsufficientMemory),
            ffi::GDK_PIXBUF_ERROR_BAD_OPTION => Some(Self::BadOption),
            ffi::GDK_PIXBUF_ERROR_UNKNOWN_TYPE => Some(Self::UnknownType),
            ffi::GDK_PIXBUF_ERROR_UNSUPPORTED_OPERATION => Some(Self::UnsupportedOperation),
            ffi::GDK_PIXBUF_ERROR_FAILED => Some(Self::Failed),
            ffi::GDK_PIXBUF_ERROR_INCOMPLETE_ANIMATION => Some(Self::IncompleteAnimation),
            _ => Some(Self::Failed),
        }
    }
}

impl StaticType for PixbufError {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_error_get_type()) }
    }
}

impl glib::value::ValueType for PixbufError {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PixbufError {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PixbufError {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GdkPixbufRotation")]
pub enum PixbufRotation {
    #[doc(alias = "GDK_PIXBUF_ROTATE_NONE")]
    None,
    #[doc(alias = "GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE")]
    Counterclockwise,
    #[doc(alias = "GDK_PIXBUF_ROTATE_UPSIDEDOWN")]
    Upsidedown,
    #[doc(alias = "GDK_PIXBUF_ROTATE_CLOCKWISE")]
    Clockwise,
    #[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for PixbufRotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PixbufRotation::{}",
            match *self {
                Self::None => "None",
                Self::Counterclockwise => "Counterclockwise",
                Self::Upsidedown => "Upsidedown",
                Self::Clockwise => "Clockwise",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for PixbufRotation {
    type GlibType = ffi::GdkPixbufRotation;

    fn into_glib(self) -> ffi::GdkPixbufRotation {
        match self {
            Self::None => ffi::GDK_PIXBUF_ROTATE_NONE,
            Self::Counterclockwise => ffi::GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE,
            Self::Upsidedown => ffi::GDK_PIXBUF_ROTATE_UPSIDEDOWN,
            Self::Clockwise => ffi::GDK_PIXBUF_ROTATE_CLOCKWISE,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GdkPixbufRotation> for PixbufRotation {
    unsafe fn from_glib(value: ffi::GdkPixbufRotation) -> Self {
        match value {
            ffi::GDK_PIXBUF_ROTATE_NONE => Self::None,
            ffi::GDK_PIXBUF_ROTATE_COUNTERCLOCKWISE => Self::Counterclockwise,
            ffi::GDK_PIXBUF_ROTATE_UPSIDEDOWN => Self::Upsidedown,
            ffi::GDK_PIXBUF_ROTATE_CLOCKWISE => Self::Clockwise,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for PixbufRotation {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::gdk_pixbuf_rotation_get_type()) }
    }
}

impl glib::value::ValueType for PixbufRotation {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for PixbufRotation {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for PixbufRotation {
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}
