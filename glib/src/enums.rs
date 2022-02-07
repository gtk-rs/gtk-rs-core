// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::value::Value;
use crate::Type;
use std::ffi::CStr;
use std::{cmp, fmt, ptr};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum UserDirectory {
    #[doc(alias = "G_USER_DIRECTORY_DESKTOP")]
    Desktop,
    #[doc(alias = "G_USER_DIRECTORY_DOCUMENTS")]
    Documents,
    #[doc(alias = "G_USER_DIRECTORY_DOWNLOAD")]
    Downloads,
    #[doc(alias = "G_USER_DIRECTORY_MUSIC")]
    Music,
    #[doc(alias = "G_USER_DIRECTORY_PICTURES")]
    Pictures,
    #[doc(alias = "G_USER_DIRECTORY_PUBLIC_SHARE")]
    PublicShare,
    #[doc(alias = "G_USER_DIRECTORY_TEMPLATES")]
    Templates,
    #[doc(alias = "G_USER_DIRECTORY_VIDEOS")]
    Videos,
    #[doc(hidden)]
    #[doc(alias = "G_USER_N_DIRECTORIES")]
    NDirectories,
}

#[doc(hidden)]
impl IntoGlib for UserDirectory {
    type GlibType = ffi::GUserDirectory;

    fn into_glib(self) -> ffi::GUserDirectory {
        match self {
            Self::Desktop => ffi::G_USER_DIRECTORY_DESKTOP,
            Self::Documents => ffi::G_USER_DIRECTORY_DOCUMENTS,
            Self::Downloads => ffi::G_USER_DIRECTORY_DOWNLOAD,
            Self::Music => ffi::G_USER_DIRECTORY_MUSIC,
            Self::Pictures => ffi::G_USER_DIRECTORY_PICTURES,
            Self::PublicShare => ffi::G_USER_DIRECTORY_PUBLIC_SHARE,
            Self::Templates => ffi::G_USER_DIRECTORY_TEMPLATES,
            Self::Videos => ffi::G_USER_DIRECTORY_VIDEOS,
            Self::NDirectories => ffi::G_USER_N_DIRECTORIES,
        }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of an `enum` for dynamically, at runtime, querying the values of the enum and
/// using them.
#[doc(alias = "GEnumClass")]
#[repr(transparent)]
pub struct EnumClass(ptr::NonNull<gobject_ffi::GEnumClass>);

unsafe impl Send for EnumClass {}
unsafe impl Sync for EnumClass {}

impl fmt::Debug for EnumClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumClass")
            .field("type", &self.type_())
            .field("values", &self.values())
            .finish()
    }
}

impl EnumClass {
    // rustdoc-stripper-ignore-next
    /// Create a new `EnumClass` from a `Type`.
    ///
    /// Returns `None` if `type_` is not representing an enum.
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_enum: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_ENUM,
            ));
            if !is_enum {
                return None;
            }

            Some(EnumClass(
                ptr::NonNull::new(gobject_ffi::g_type_class_ref(type_.into_glib()) as *mut _)
                    .unwrap(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// `Type` of the enum.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.as_ref().g_type_class.g_type) }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with `value`.
    #[doc(alias = "g_enum_get_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, value: i32) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value(self.0.as_ptr(), value);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with name `name`.
    #[doc(alias = "g_enum_get_value_by_name")]
    #[doc(alias = "get_value_by_name")]
    pub fn value_by_name(&self, name: &str) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_name(self.0.as_ptr(), name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `EnumValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the enum does not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_enum_get_value_by_nick")]
    #[doc(alias = "get_value_by_nick")]
    pub fn value_by_nick(&self, nick: &str) -> Option<&EnumValue> {
        unsafe {
            let v = gobject_ffi::g_enum_get_value_by_nick(self.0.as_ptr(), nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const EnumValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets all `EnumValue` of this `EnumClass`.
    #[doc(alias = "get_values")]
    pub fn values(&self) -> &[EnumValue] {
        unsafe {
            if self.0.as_ref().n_values == 0 {
                return &[];
            }
            std::slice::from_raw_parts(
                self.0.as_ref().values as *const EnumValue,
                self.0.as_ref().n_values as usize,
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts integer `value` to a `Value`, if part of the enum.
    pub fn to_value(&self, value: i32) -> Option<Value> {
        self.value(value).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string name `name` to a `Value`, if part of the enum.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.value_by_name(name).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string nick `nick` to a `Value`, if part of the enum.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.value_by_nick(nick).map(|v| v.to_value(self))
    }
}

impl Drop for EnumClass {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0.as_ptr() as *mut _);
        }
    }
}

impl Clone for EnumClass {
    fn clone(&self) -> Self {
        unsafe {
            Self(ptr::NonNull::new(gobject_ffi::g_type_class_ref(self.type_().into_glib()) as *mut _).unwrap())
        }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a single enum value of an `EnumClass`.
#[doc(alias = "GEnumValue")]
#[repr(transparent)]
pub struct EnumValue(gobject_ffi::GEnumValue);

unsafe impl Send for EnumValue {}
unsafe impl Sync for EnumValue {}

impl fmt::Debug for EnumValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EnumValue")
            .field("value", &self.value())
            .field("name", &self.name())
            .field("nick", &self.nick())
            .finish()
    }
}

impl EnumValue {
    // rustdoc-stripper-ignore-next
    /// Get integer value corresponding to the value.
    #[doc(alias = "get_value")]
    pub fn value(&self) -> i32 {
        self.0.value
    }

    // rustdoc-stripper-ignore-next
    /// Get name corresponding to the value.
    #[doc(alias = "get_name")]
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_name).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Get nick corresponding to the value.
    #[doc(alias = "get_nick")]
    pub fn nick(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_nick).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Convert enum value to a `Value`.
    pub fn to_value(&self, enum_: &EnumClass) -> Value {
        unsafe {
            let mut v = Value::from_type(enum_.type_());
            gobject_ffi::g_value_set_enum(v.to_glib_none_mut().0, self.0.value);
            v
        }
    }

    // rustdoc-stripper-ignore-next
    /// Convert enum value from a `Value`.
    pub fn from_value(value: &Value) -> Option<(EnumClass, &EnumValue)> {
        unsafe {
            let enum_class = EnumClass::new(value.type_())?;
            let v = enum_class.value(gobject_ffi::g_value_get_enum(value.to_glib_none().0))?;
            let v = &*(v as *const EnumValue);
            Some((enum_class, v))
        }
    }
}

impl PartialEq for EnumValue {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for EnumValue {}

impl PartialOrd for EnumValue {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.value().partial_cmp(&other.value())
    }
}

impl Ord for EnumValue {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a `flags` for dynamically, at runtime, querying the values of the enum and
/// using them
#[doc(alias = "GFlagsClass")]
#[repr(transparent)]
pub struct FlagsClass(ptr::NonNull<gobject_ffi::GFlagsClass>);

unsafe impl Send for FlagsClass {}
unsafe impl Sync for FlagsClass {}

impl fmt::Debug for FlagsClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlagsClass")
            .field("type", &self.type_())
            .field("values", &self.values())
            .finish()
    }
}

impl FlagsClass {
    // rustdoc-stripper-ignore-next
    /// Create a new `FlagsClass` from a `Type`
    ///
    /// Returns `None` if `type_` is not representing a flags type.
    pub fn new(type_: Type) -> Option<Self> {
        unsafe {
            let is_flags: bool = from_glib(gobject_ffi::g_type_is_a(
                type_.into_glib(),
                gobject_ffi::G_TYPE_FLAGS,
            ));
            if !is_flags {
                return None;
            }

            Some(FlagsClass(
                ptr::NonNull::new(gobject_ffi::g_type_class_ref(type_.into_glib()) as *mut _)
                    .unwrap(),
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// `Type` of the flags.
    pub fn type_(&self) -> Type {
        unsafe { from_glib(self.0.as_ref().g_type_class.g_type) }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by integer `value`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with `value`.
    #[doc(alias = "g_flags_get_first_value")]
    #[doc(alias = "get_value")]
    pub fn value(&self, value: u32) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_first_value(self.0.as_ptr(), value);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by string name `name`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with name `name`.
    #[doc(alias = "g_flags_get_value_by_name")]
    #[doc(alias = "get_value_by_name")]
    pub fn value_by_name(&self, name: &str) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_name(self.0.as_ptr(), name.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets `FlagsValue` by string nick `nick`, if existing.
    ///
    /// Returns `None` if the flags do not contain any value
    /// with nick `nick`.
    #[doc(alias = "g_flags_get_value_by_nick")]
    #[doc(alias = "get_value_by_nick")]
    pub fn value_by_nick(&self, nick: &str) -> Option<&FlagsValue> {
        unsafe {
            let v = gobject_ffi::g_flags_get_value_by_nick(self.0.as_ptr(), nick.to_glib_none().0);
            if v.is_null() {
                None
            } else {
                Some(&*(v as *const FlagsValue))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets all `FlagsValue` of this `FlagsClass`.
    #[doc(alias = "get_values")]
    pub fn values(&self) -> &[FlagsValue] {
        unsafe {
            if self.0.as_ref().n_values == 0 {
                return &[];
            }
            std::slice::from_raw_parts(
                self.0.as_ref().values as *const FlagsValue,
                self.0.as_ref().n_values as usize,
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Converts integer `value` to a `Value`, if part of the flags.
    pub fn to_value(&self, value: u32) -> Option<Value> {
        self.value(value).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string name `name` to a `Value`, if part of the flags.
    pub fn to_value_by_name(&self, name: &str) -> Option<Value> {
        self.value_by_name(name).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Converts string nick `nick` to a `Value`, if part of the flags.
    pub fn to_value_by_nick(&self, nick: &str) -> Option<Value> {
        self.value_by_nick(nick).map(|v| v.to_value(self))
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to integer `f` is set in `value`.
    pub fn is_set(&self, value: &Value, f: u32) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            flags & f != 0
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to string name `name` is set in `value`.
    pub fn is_set_by_name(&self, value: &Value, name: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.value() != 0
            } else {
                false
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Checks if the flags corresponding to string nick `nick` is set in `value`.
    pub fn is_set_by_nick(&self, value: &Value, nick: &str) -> bool {
        unsafe {
            if self.type_() != value.type_() {
                return false;
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                flags & f.value() != 0
            } else {
                false
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to integer `f` in `value`, if part of that flags. If the
    /// flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    #[doc(alias = "g_value_set_flags")]
    pub fn set(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already set, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag set if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn set_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags | f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to integer `f` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset(&self, mut value: Value, f: u32) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value(f) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to string name `name` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_name(&self, mut value: Value, name: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_name(name) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags value corresponding to string nick `nick` in `value`, if part of that flags.
    /// If the flag is already unset, it will succeed without doing any changes.
    ///
    /// Returns `Ok(value)` with the flag unset if successful, or `Err(value)` with the original
    /// value otherwise.
    pub fn unset_by_nick(&self, mut value: Value, nick: &str) -> Result<Value, Value> {
        unsafe {
            if self.type_() != value.type_() {
                return Err(value);
            }

            if let Some(f) = self.value_by_nick(nick) {
                let flags = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
                gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, flags & !f.value());
                Ok(value)
            } else {
                Err(value)
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`.
    pub fn builder(&self) -> FlagsBuilder {
        FlagsBuilder::new(self)
    }

    // rustdoc-stripper-ignore-next
    /// Returns a new `FlagsBuilder` for conveniently setting/unsetting flags
    /// and building a `Value`. The `Value` is initialized with `value`.
    pub fn builder_with_value(&self, value: Value) -> Option<FlagsBuilder> {
        if self.type_() != value.type_() {
            return None;
        }

        Some(FlagsBuilder::with_value(self, value))
    }
}

impl Drop for FlagsClass {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_type_class_unref(self.0.as_ptr() as *mut _);
        }
    }
}

impl Clone for FlagsClass {
    fn clone(&self) -> Self {
        unsafe {
            Self(ptr::NonNull::new(gobject_ffi::g_type_class_ref(self.type_().into_glib()) as *mut _).unwrap())
        }
    }
}

// rustdoc-stripper-ignore-next
/// Representation of a single flags value of a `FlagsClass`.
#[doc(alias = "GFlagsValue")]
#[repr(transparent)]
pub struct FlagsValue(gobject_ffi::GFlagsValue);

unsafe impl Send for FlagsValue {}
unsafe impl Sync for FlagsValue {}

impl fmt::Debug for FlagsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FlagsValue")
            .field("value", &self.value())
            .field("name", &self.name())
            .field("nick", &self.nick())
            .finish()
    }
}

impl FlagsValue {
    // rustdoc-stripper-ignore-next
    /// Get integer value corresponding to the value.
    #[doc(alias = "get_value")]
    pub fn value(&self) -> u32 {
        self.0.value
    }

    // rustdoc-stripper-ignore-next
    /// Get name corresponding to the value.
    #[doc(alias = "get_name")]
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_name).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Get nick corresponding to the value.
    #[doc(alias = "get_nick")]
    pub fn nick(&self) -> &str {
        unsafe { CStr::from_ptr(self.0.value_nick).to_str().unwrap() }
    }

    // rustdoc-stripper-ignore-next
    /// Convert flags value to a `Value`.
    pub fn to_value(&self, flags: &FlagsClass) -> Value {
        unsafe {
            let mut v = Value::from_type(flags.type_());
            gobject_ffi::g_value_set_flags(v.to_glib_none_mut().0, self.0.value);
            v
        }
    }

    // rustdoc-stripper-ignore-next
    /// Convert flags values from a `Value`. This returns all flags that are set.
    pub fn from_value(value: &Value) -> Option<(FlagsClass, Vec<&FlagsValue>)> {
        unsafe {
            let flags_class = FlagsClass::new(value.type_())?;
            let mut res = Vec::new();
            let f = gobject_ffi::g_value_get_flags(value.to_glib_none().0);
            for v in flags_class.values() {
                if v.value() & f != 0 {
                    res.push(&*(v as *const FlagsValue));
                }
            }
            Some((flags_class, res))
        }
    }
}

impl PartialEq for FlagsValue {
    fn eq(&self, other: &Self) -> bool {
        self.value().eq(&other.value())
    }
}

impl Eq for FlagsValue {}

// rustdoc-stripper-ignore-next
/// Builder for conveniently setting/unsetting flags and returning a `Value`.
///
/// Example for getting a flags property, unsetting some flags and setting the updated flags on the
/// object again:
///
/// ```ignore
/// let flags = obj.property("flags").unwrap();
/// let flags_class = FlagsClass::new(flags.type_()).unwrap();
/// let flags = flags_class.builder_with_value(flags).unwrap()
///     .unset_by_nick("some-flag")
///     .unset_by_nick("some-other-flag")
///     .build()
///     .unwrap();
/// obj.set_property("flags", &flags).unwrap();
/// ```
///
/// If setting/unsetting any value fails, `build()` returns `None`.
#[must_use = "The builder must be built to be used"]
pub struct FlagsBuilder<'a>(&'a FlagsClass, Option<Value>);
impl<'a> FlagsBuilder<'a> {
    fn new(flags_class: &FlagsClass) -> FlagsBuilder {
        let value = Value::from_type(flags_class.type_());
        FlagsBuilder(flags_class, Some(value))
    }

    fn with_value(flags_class: &FlagsClass, value: Value) -> FlagsBuilder {
        FlagsBuilder(flags_class, Some(value))
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to integer value `f`.
    pub fn set(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set(value, f).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to string name `name`.
    pub fn set_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_name(value, name).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Set flags corresponding to string nick `nick`.
    pub fn set_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.set_by_nick(value, nick).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unsets flags corresponding to integer value `f`.
    pub fn unset(mut self, f: u32) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset(value, f).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags corresponding to string name `name`.
    pub fn unset_by_name(mut self, name: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_name(value, name).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Unset flags corresponding to string nick `nick`.
    pub fn unset_by_nick(mut self, nick: &str) -> Self {
        if let Some(value) = self.1.take() {
            self.1 = self.0.unset_by_nick(value, nick).ok();
        }

        self
    }

    // rustdoc-stripper-ignore-next
    /// Converts to the final `Value`, unless any previous setting/unsetting of flags failed.
    #[must_use = "Value returned from the builder should probably be used"]
    pub fn build(self) -> Option<Value> {
        self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::StaticType;

    #[test]
    fn test_flags() {
        let flags = FlagsClass::new(crate::BindingFlags::static_type()).unwrap();
        let values = flags.values();
        let def1 = values
            .iter()
            .find(|v| v.name() == "G_BINDING_DEFAULT")
            .unwrap();
        let def2 = flags.value_by_name("G_BINDING_DEFAULT").unwrap();
        assert!(ptr::eq(def1, def2));

        assert_eq!(def1.value(), crate::BindingFlags::DEFAULT.bits());
    }
}
