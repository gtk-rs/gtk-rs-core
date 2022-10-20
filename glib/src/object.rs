// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` Object wrapper implementation and `Object` binding.

use crate::types::StaticType;
use crate::PtrSlice;
use crate::{quark::Quark, subclass::signal::SignalQuery};
use crate::{translate::*, value::FromValue};
use std::cmp;
use std::fmt;
use std::hash;
use std::marker::PhantomData;
use std::mem;
use std::mem::ManuallyDrop;
use std::ops;
use std::pin::Pin;
use std::ptr;

use crate::closure::TryFromClosureReturnValue;
use crate::subclass::{prelude::ObjectSubclass, SignalId};
use crate::value::ToValue;
use crate::SignalHandlerId;
use crate::Type;
use crate::Value;
use crate::{Closure, RustClosure};

use crate::thread_guard::thread_id;

// rustdoc-stripper-ignore-next
/// Implemented by types representing `glib::Object` and subclasses of it.
pub unsafe trait ObjectType:
    UnsafeFrom<ObjectRef>
    + Into<ObjectRef>
    + StaticType
    + fmt::Debug
    + Clone
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + hash::Hash
    + crate::value::ValueType
    + crate::value::ToValue
    + crate::value::ToValueOptional
    + crate::value::FromValueOptional<'static>
    + for<'a> ToGlibPtr<'a, *mut <Self as ObjectType>::GlibType>
    + IntoGlibPtr<*mut <Self as ObjectType>::GlibType>
    + 'static
{
    // rustdoc-stripper-ignore-next
    /// type of the FFI Instance structure.
    type GlibType: 'static;
    // rustdoc-stripper-ignore-next
    /// type of the FFI Class structure.
    type GlibClassType: 'static;

    fn as_object_ref(&self) -> &ObjectRef;
    fn as_ptr(&self) -> *mut Self::GlibType;
}

// rustdoc-stripper-ignore-next
/// Declares the "is a" relationship.
///
/// `Self` is said to implement `T`.
///
/// For instance, since originally `GtkWidget` is a subclass of `GObject` and
/// implements the `GtkBuildable` interface, `gtk::Widget` implements
/// `IsA<glib::Object>` and `IsA<gtk::Buildable>`.
///
///
/// The trait can only be implemented if the appropriate `ToGlibPtr`
/// implementations exist.
pub unsafe trait IsA<T: ObjectType>:
    ObjectType + Into<T> + AsRef<T> + std::borrow::Borrow<T>
{
}

// rustdoc-stripper-ignore-next
/// Upcasting and downcasting support.
///
/// Provides conversions up and down the class hierarchy tree.
pub trait Cast: ObjectType {
    // rustdoc-stripper-ignore-next
    /// Upcasts an object to a superclass or interface `T`.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast<T: ObjectType>(self) -> T
    where
        Self: IsA<T>,
    {
        unsafe { self.unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Upcasts an object to a reference of its superclass or interface `T`.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast_ref::<gtk::Widget>();
    /// ```
    #[inline]
    fn upcast_ref<T: ObjectType>(&self) -> &T
    where
        Self: IsA<T>,
    {
        unsafe { self.unsafe_cast_ref() }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to downcast to a subclass or interface implementor `T`.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// assert!(widget.downcast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn downcast<T: ObjectType>(self) -> Result<T, Self>
    where
        Self: CanDowncast<T>,
    {
        if self.is::<T>() {
            Ok(unsafe { self.unsafe_cast() })
        } else {
            Err(self)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to downcast to a reference of its subclass or interface implementor `T`.
    ///
    /// Returns `Some(T)` if the object is an instance of `T` and `None`
    /// otherwise.
    ///
    /// *NOTE*: This statically checks at compile-time if casting is possible. It is not always
    /// known at compile-time, whether a specific object implements an interface or not, in which case
    /// `upcast` would fail to compile. `dynamic_cast` can be used in these circumstances, which
    /// is checking the types at runtime.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.upcast::<gtk::Widget>();
    /// assert!(widget.downcast_ref::<gtk::Button>().is_some());
    /// ```
    #[inline]
    fn downcast_ref<T: ObjectType>(&self) -> Option<&T>
    where
        Self: CanDowncast<T>,
    {
        if self.is::<T>() {
            Some(unsafe { self.unsafe_cast_ref() })
        } else {
            None
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to cast to an object of type `T`. This handles upcasting, downcasting
    /// and casting between interface and interface implementors. All checks are performed at
    /// runtime, while `downcast` and `upcast` will do many checks at compile-time already.
    ///
    /// It is not always known at compile-time, whether a specific object implements an interface or
    /// not, and checking has to be performed at runtime.
    ///
    /// Returns `Ok(T)` if the object is an instance of `T` and `Err(self)`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.dynamic_cast::<gtk::Widget>();
    /// assert!(widget.is_ok());
    /// let widget = widget.unwrap();
    /// assert!(widget.dynamic_cast::<gtk::Button>().is_ok());
    /// ```
    #[inline]
    fn dynamic_cast<T: ObjectType>(self) -> Result<T, Self> {
        if !self.is::<T>() {
            Err(self)
        } else {
            Ok(unsafe { self.unsafe_cast() })
        }
    }

    // rustdoc-stripper-ignore-next
    /// Tries to cast to reference to an object of type `T`. This handles upcasting, downcasting
    /// and casting between interface and interface implementors. All checks are performed at
    /// runtime, while `downcast` and `upcast` will do many checks at compile-time already.
    ///
    /// It is not always known at compile-time, whether a specific object implements an interface or
    /// not, and checking has to be performed at runtime.
    ///
    /// Returns `Some(T)` if the object is an instance of `T` and `None`
    /// otherwise.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let button = gtk::Button::new();
    /// let widget = button.dynamic_cast_ref::<gtk::Widget>();
    /// assert!(widget.is_some());
    /// let widget = widget.unwrap();
    /// assert!(widget.dynamic_cast_ref::<gtk::Button>().is_some());
    /// ```
    #[inline]
    fn dynamic_cast_ref<T: ObjectType>(&self) -> Option<&T> {
        if !self.is::<T>() {
            None
        } else {
            // This cast is safe because all our wrapper types have the
            // same representation except for the name and the phantom data
            // type. IsA<> is an unsafe trait that must only be implemented
            // if this is a valid wrapper type
            Some(unsafe { self.unsafe_cast_ref() })
        }
    }

    // rustdoc-stripper-ignore-next
    /// Casts to `T` unconditionally.
    ///
    /// # Panics
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    ///
    /// # Safety
    ///
    /// If not running with `debug_assertions` enabled, the caller is responsible
    /// for ensuring that the instance implements `T`
    #[track_caller]
    unsafe fn unsafe_cast<T: ObjectType>(self) -> T {
        debug_assert!(self.is::<T>());
        T::unsafe_from(self.into())
    }

    // rustdoc-stripper-ignore-next
    /// Casts to `&T` unconditionally.
    ///
    /// # Panics
    ///
    /// Panics if compiled with `debug_assertions` and the instance doesn't implement `T`.
    ///
    /// # Safety
    ///
    /// If not running with `debug_assertions` enabled, the caller is responsible
    /// for ensuring that the instance implements `T`
    #[track_caller]
    unsafe fn unsafe_cast_ref<T: ObjectType>(&self) -> &T {
        debug_assert!(self.is::<T>());
        // This cast is safe because all our wrapper types have the
        // same representation except for the name and the phantom data
        // type. IsA<> is an unsafe trait that must only be implemented
        // if this is a valid wrapper type
        &*(self as *const Self as *const T)
    }
}

impl<T: ObjectType> Cast for T {}

// rustdoc-stripper-ignore-next
/// Marker trait for the statically known possibility of downcasting from `Self` to `T`.
pub trait CanDowncast<T> {}

impl<Super: IsA<Super>, Sub: IsA<Super>> CanDowncast<Sub> for Super {}

// Manual implementation of glib_shared_wrapper! because of special cases
#[repr(transparent)]
pub struct ObjectRef {
    inner: ptr::NonNull<gobject_ffi::GObject>,
}

impl Clone for ObjectRef {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                inner: ptr::NonNull::new_unchecked(gobject_ffi::g_object_ref(self.inner.as_ptr())),
            }
        }
    }
}

impl Drop for ObjectRef {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_object_unref(self.inner.as_ptr());
        }
    }
}

impl fmt::Debug for ObjectRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_ = unsafe {
            let klass = (*self.inner.as_ptr()).g_type_instance.g_class as *const ObjectClass;
            (*klass).type_()
        };

        f.debug_struct("ObjectRef")
            .field("inner", &self.inner)
            .field("type", &type_)
            .finish()
    }
}

impl PartialOrd for ObjectRef {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl Ord for ObjectRef {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl PartialEq for ObjectRef {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl Eq for ObjectRef {}

impl hash::Hash for ObjectRef {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.inner.hash(state)
    }
}

#[doc(hidden)]
impl GlibPtrDefault for ObjectRef {
    type GlibType = *mut gobject_ffi::GObject;
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *mut gobject_ffi::GObject> for ObjectRef {
    type Storage = &'a ObjectRef;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *mut gobject_ffi::GObject, Self> {
        Stash(self.inner.as_ptr(), self)
    }

    #[inline]
    fn to_glib_full(&self) -> *mut gobject_ffi::GObject {
        unsafe { gobject_ffi::g_object_ref(self.inner.as_ptr()) }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *mut *mut gobject_ffi::GObject> for ObjectRef {
    type Storage = (
        Vec<Stash<'a, *mut gobject_ffi::GObject, ObjectRef>>,
        Option<Vec<*mut gobject_ffi::GObject>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [ObjectRef],
    ) -> (*mut *mut gobject_ffi::GObject, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();
        let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
        v_ptr.push(ptr::null_mut() as *mut gobject_ffi::GObject);

        (
            v_ptr.as_ptr() as *mut *mut gobject_ffi::GObject,
            (v, Some(v_ptr)),
        )
    }

    fn to_glib_container_from_slice(
        t: &'a [ObjectRef],
    ) -> (*mut *mut gobject_ffi::GObject, Self::Storage) {
        let v: Vec<_> = t.iter().map(|s| s.to_glib_none()).collect();

        let v_ptr = unsafe {
            let v_ptr = ffi::g_malloc0(mem::size_of::<*mut gobject_ffi::GObject>() * (t.len() + 1))
                as *mut *mut gobject_ffi::GObject;

            for (i, s) in v.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.0);
            }

            v_ptr
        };

        (v_ptr, (v, None))
    }

    fn to_glib_full_from_slice(t: &[ObjectRef]) -> *mut *mut gobject_ffi::GObject {
        unsafe {
            let v_ptr =
                ffi::g_malloc0(std::mem::size_of::<*mut gobject_ffi::GObject>() * (t.len() + 1))
                    as *mut *mut gobject_ffi::GObject;

            for (i, s) in t.iter().enumerate() {
                ptr::write(v_ptr.add(i), s.to_glib_full());
            }

            v_ptr
        }
    }
}

#[doc(hidden)]
impl<'a> ToGlibContainerFromSlice<'a, *const *mut gobject_ffi::GObject> for ObjectRef {
    type Storage = (
        Vec<Stash<'a, *mut gobject_ffi::GObject, ObjectRef>>,
        Option<Vec<*mut gobject_ffi::GObject>>,
    );

    fn to_glib_none_from_slice(
        t: &'a [ObjectRef],
    ) -> (*const *mut gobject_ffi::GObject, Self::Storage) {
        let (ptr, stash) =
            ToGlibContainerFromSlice::<'a, *mut *mut gobject_ffi::GObject>::to_glib_none_from_slice(
                t,
            );
        (ptr as *const *mut gobject_ffi::GObject, stash)
    }

    fn to_glib_container_from_slice(
        _: &'a [ObjectRef],
    ) -> (*const *mut gobject_ffi::GObject, Self::Storage) {
        // Can't have consumer free a *const pointer
        unimplemented!()
    }

    fn to_glib_full_from_slice(_: &[ObjectRef]) -> *const *mut gobject_ffi::GObject {
        // Can't have consumer free a *const pointer
        unimplemented!()
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*mut gobject_ffi::GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut gobject_ffi::GObject) -> Self {
        assert!(!ptr.is_null());
        assert_ne!((*ptr).ref_count, 0);

        // Attention: This takes ownership of floating references!
        Self {
            inner: ptr::NonNull::new_unchecked(gobject_ffi::g_object_ref_sink(ptr)),
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrNone<*const gobject_ffi::GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_none(ptr: *const gobject_ffi::GObject) -> Self {
        // Attention: This takes ownership of floating references!
        from_glib_none(ptr as *mut gobject_ffi::GObject)
    }
}

#[doc(hidden)]
impl FromGlibPtrFull<*mut gobject_ffi::GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut gobject_ffi::GObject) -> Self {
        assert!(!ptr.is_null());
        assert_ne!((*ptr).ref_count, 0);

        Self {
            inner: ptr::NonNull::new_unchecked(ptr),
        }
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*mut gobject_ffi::GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut gobject_ffi::GObject) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        assert_ne!((*ptr).ref_count, 0);

        Borrowed::new(Self {
            inner: ptr::NonNull::new_unchecked(ptr),
        })
    }
}

#[doc(hidden)]
impl FromGlibPtrBorrow<*const gobject_ffi::GObject> for ObjectRef {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *const gobject_ffi::GObject) -> Borrowed<Self> {
        from_glib_borrow(ptr as *mut gobject_ffi::GObject)
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<*mut gobject_ffi::GObject, *mut *mut gobject_ffi::GObject>
    for ObjectRef
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *mut *mut gobject_ffi::GObject,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            return Vec::new();
        }

        // Attention: This takes ownership of floating references!
        let mut res = Vec::with_capacity(num);
        for i in 0..num {
            res.push(from_glib_none(ptr::read(ptr.add(i))));
        }
        res
    }

    unsafe fn from_glib_container_num_as_vec(
        ptr: *mut *mut gobject_ffi::GObject,
        num: usize,
    ) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        let res = FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
        ffi::g_free(ptr as *mut _);
        res
    }

    unsafe fn from_glib_full_num_as_vec(
        ptr: *mut *mut gobject_ffi::GObject,
        num: usize,
    ) -> Vec<Self> {
        if num == 0 || ptr.is_null() {
            ffi::g_free(ptr as *mut _);
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

#[doc(hidden)]
impl FromGlibPtrArrayContainerAsVec<*mut gobject_ffi::GObject, *mut *mut gobject_ffi::GObject>
    for ObjectRef
{
    unsafe fn from_glib_none_as_vec(ptr: *mut *mut gobject_ffi::GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_container_as_vec(ptr: *mut *mut gobject_ffi::GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, c_ptr_array_len(ptr))
    }

    unsafe fn from_glib_full_as_vec(ptr: *mut *mut gobject_ffi::GObject) -> Vec<Self> {
        FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, c_ptr_array_len(ptr))
    }
}

#[doc(hidden)]
impl FromGlibContainerAsVec<*mut gobject_ffi::GObject, *const *mut gobject_ffi::GObject>
    for ObjectRef
{
    unsafe fn from_glib_none_num_as_vec(
        ptr: *const *mut gobject_ffi::GObject,
        num: usize,
    ) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
    }

    unsafe fn from_glib_container_num_as_vec(
        _: *const *mut gobject_ffi::GObject,
        _: usize,
    ) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_num_as_vec(
        _: *const *mut gobject_ffi::GObject,
        _: usize,
    ) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

#[doc(hidden)]
impl FromGlibPtrArrayContainerAsVec<*mut gobject_ffi::GObject, *const *mut gobject_ffi::GObject>
    for ObjectRef
{
    unsafe fn from_glib_none_as_vec(ptr: *const *mut gobject_ffi::GObject) -> Vec<Self> {
        // Attention: This takes ownership of floating references!
        FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
    }

    unsafe fn from_glib_container_as_vec(_: *const *mut gobject_ffi::GObject) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }

    unsafe fn from_glib_full_as_vec(_: *const *mut gobject_ffi::GObject) -> Vec<Self> {
        // Can't free a *const
        unimplemented!()
    }
}

#[repr(transparent)]
pub struct TypedObjectRef<T, P> {
    inner: ObjectRef,
    imp: PhantomData<T>,
    parent: PhantomData<P>,
}

impl<T, P> TypedObjectRef<T, P> {
    pub unsafe fn new(obj: ObjectRef) -> Self {
        Self {
            inner: obj,
            imp: PhantomData,
            parent: PhantomData,
        }
    }

    pub fn into_inner(self) -> ObjectRef {
        self.inner
    }
}

impl<T, P> Clone for TypedObjectRef<T, P> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            imp: PhantomData,
            parent: PhantomData,
        }
    }
}

impl<T, P> ops::Deref for TypedObjectRef<T, P> {
    type Target = ObjectRef;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T, P> fmt::Debug for TypedObjectRef<T, P> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let type_ = unsafe {
            let klass = (*self.inner.inner.as_ptr()).g_type_instance.g_class as *const ObjectClass;
            (*klass).type_()
        };

        f.debug_struct("TypedObjectRef")
            .field("inner", &self.inner.inner)
            .field("type", &type_)
            .finish()
    }
}

impl<T, P> PartialOrd for TypedObjectRef<T, P> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.inner.partial_cmp(&other.inner)
    }
}

impl<T, P> Ord for TypedObjectRef<T, P> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.inner.cmp(&other.inner)
    }
}

impl<T, P> PartialEq for TypedObjectRef<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T, P> Eq for TypedObjectRef<T, P> {}

impl<T, P> hash::Hash for TypedObjectRef<T, P> {
    fn hash<H>(&self, state: &mut H)
    where
        H: hash::Hasher,
    {
        self.inner.hash(state)
    }
}

unsafe impl<T: Send + Sync, P: Send + Sync> Send for TypedObjectRef<T, P> {}
unsafe impl<T: Send + Sync, P: Send + Sync> Sync for TypedObjectRef<T, P> {}

// rustdoc-stripper-ignore-next
/// ObjectType implementations for Object types. See `wrapper!`.
#[macro_export]
macro_rules! glib_object_wrapper {
    (@generic_impl [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $parent_type:ty, $ffi_name:ty, $ffi_class_name:ty, @type_ $get_type_expr:expr) => {
        $(#[$attr])*
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            inner: $crate::object::TypedObjectRef<$impl_type, $parent_type>,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        // Always implement Clone, Hash, PartialEq, Eq, PartialOrd, Ord, and Debug for object types.
        // Due to inheritance and up/downcasting we must implement these by pointer or otherwise they
        // would potentially give different results for the same object depending on the type we
        // currently know for it.
        // Implement them manually rather than generating #[derive] macros since so that when generics
        // are specified, these traits are not required.

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    inner: std::clone::Clone::clone(&self.inner),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::hash::Hash for $name $(<$($generic),+>)? {
            #[inline]
            fn hash<H>(&self, state: &mut H)
            where
                H: std::hash::Hasher
            {
                std::hash::Hash::hash(&self.inner, state);
            }
        }

        impl<OT: $crate::object::ObjectType $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> std::cmp::PartialEq<OT> for $name $(<$($generic),+>)? {
            #[inline]
            fn eq(&self, other: &OT) -> bool {
                std::cmp::PartialEq::eq(&*self.inner, $crate::object::ObjectType::as_object_ref(other))
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::cmp::Eq for $name $(<$($generic),+>)? {}

        impl<OT: $crate::object::ObjectType $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> std::cmp::PartialOrd<OT> for $name $(<$($generic),+>)? {
            #[inline]
            fn partial_cmp(&self, other: &OT) -> Option<std::cmp::Ordering> {
                std::cmp::PartialOrd::partial_cmp(&*self.inner, $crate::object::ObjectType::as_object_ref(other))
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::cmp::Ord for $name $(<$($generic),+>)? {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                std::cmp::Ord::cmp(&*self.inner, $crate::object::ObjectType::as_object_ref(other))
            }
        }


        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::fmt::Debug for $name $(<$($generic),+>)? {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.debug_struct(stringify!($name)).field("inner", &self.inner).finish()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? From<$name $(<$($generic),+>)?> for $crate::object::ObjectRef {
            fn from(s: $name $(<$($generic),+>)?) -> $crate::object::ObjectRef {
                s.inner.into_inner()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::UnsafeFrom<$crate::object::ObjectRef> for $name $(<$($generic),+>)? {
            unsafe fn unsafe_from(t: $crate::object::ObjectRef) -> Self {
                $name {
                    inner: $crate::object::TypedObjectRef::new(t),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::GlibPtrDefault for $name $(<$($generic),+>)? {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ObjectType for $name $(<$($generic),+>)? {
            type GlibType = $ffi_name;
            type GlibClassType = $ffi_class_name;

            fn as_object_ref(&self) -> &$crate::object::ObjectRef {
                &self.inner
            }

            fn as_ptr(&self) -> *mut Self::GlibType {
                $crate::translate::ToGlibPtr::to_glib_none(&*self.inner).0 as *mut _
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? AsRef<$crate::object::ObjectRef> for $name $(<$($generic),+>)? {
            fn as_ref(&self) -> &$crate::object::ObjectRef {
                &self.inner
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? AsRef<Self> for $name $(<$($generic),+>)? {
            fn as_ref(&self) -> &Self {
                self
            }
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsA<Self> for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::subclass::types::FromObject for $name $(<$($generic),+>)? {
            type FromObjectType = Self;
            fn from_object(obj: &Self::FromObjectType) -> &Self {
                obj
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::gobject_ffi::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&*self.inner);
                $crate::translate::Stash(stash.0 as *const _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&*self.inner) as *const _
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = <$crate::object::ObjectRef as
                $crate::translate::ToGlibPtr<'a, *mut $crate::gobject_ffi::GObject>>::Storage;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&*self.inner);
                $crate::translate::Stash(stash.0 as *mut _, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *mut $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&*self.inner) as *mut _
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::IntoGlibPtr<*mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn into_glib_ptr(self) -> *mut $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
                $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&*s).0 as *mut _
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::IntoGlibPtr<*const $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn into_glib_ptr(self) -> *const $ffi_name {
                let s = std::mem::ManuallyDrop::new(self);
                $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_none(&*s).0 as *const _
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (Vec<$crate::translate::Stash<'a, *mut $ffi_name, Self>>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(std::ptr::null_mut() as *mut $ffi_name);

                (v_ptr.as_ptr() as *mut *mut $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [Self]) -> (*mut *mut $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[Self]) -> *mut *mut $ffi_name {
                unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*mut $ffi_name>() * (t.len() + 1)) as *mut *mut $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (Vec<$crate::translate::Stash<'a, *mut $ffi_name, Self>>, Option<Vec<*mut $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [Self]) -> (*const *mut $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *mut $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *mut $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [Self]) -> (*const *mut $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[Self]) -> *const *mut $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name {
                    inner: $crate::object::TypedObjectRef::new($crate::translate::from_glib_none(ptr as *mut _)),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name {
                    inner: $crate::object::TypedObjectRef::new($crate::translate::from_glib_none(ptr as *mut _)),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $name {
                    inner: $crate::object::TypedObjectRef::new($crate::translate::from_glib_full(ptr as *mut _)),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                debug_assert!($crate::types::instance_of::<Self>(ptr as *const _));
                $crate::translate::Borrowed::new(
                    $name {
                        inner: $crate::object::TypedObjectRef::new($crate::translate::from_glib_borrow::<_, $crate::object::ObjectRef>(ptr as *mut _).into_inner()),
                        phantom: std::marker::PhantomData,
                    }
                )
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            #[allow(clippy::cast_ptr_alignment)]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::from_glib_borrow::<_, Self>(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_none(std::ptr::read(ptr.add(i))));
                }
                res
            }

            unsafe fn from_glib_container_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                let res = $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, num);
                $crate::ffi::g_free(ptr as *mut _);
                res
            }

            unsafe fn from_glib_full_num_as_vec(ptr: *mut *mut $ffi_name, num: usize) -> Vec<Self> {
                if num == 0 || ptr.is_null() {
                    $crate::ffi::g_free(ptr as *mut _);
                    return Vec::new();
                }

                let mut res = Vec::with_capacity(num);
                for i in 0..num {
                    res.push($crate::translate::from_glib_full(std::ptr::read(ptr.add(i))));
                }
                $crate::ffi::g_free(ptr as *mut _);
                res
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *mut *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_container_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_container_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }

            unsafe fn from_glib_full_as_vec(ptr: *mut *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(ptr, $crate::translate::c_ptr_array_len(ptr))
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_num_as_vec(ptr: *const *mut $ffi_name, num: usize) -> Vec<Self> {
                $crate::translate::FromGlibContainerAsVec::from_glib_none_num_as_vec(ptr as *mut *mut _, num)
            }

            unsafe fn from_glib_container_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_num_as_vec(_: *const *mut $ffi_name, _: usize) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrArrayContainerAsVec<*mut $ffi_name, *const *mut $ffi_name> for $name $(<$($generic),+>)? {
            unsafe fn from_glib_none_as_vec(ptr: *const *mut $ffi_name) -> Vec<Self> {
                $crate::translate::FromGlibPtrArrayContainerAsVec::from_glib_none_as_vec(ptr as *mut *mut _)
            }

            unsafe fn from_glib_container_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }

            unsafe fn from_glib_full_as_vec(_: *const *mut $ffi_name) -> Vec<Self> {
                // Can't free a *const
                unimplemented!()
            }
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::types::StaticType for $name $(<$($generic),+>)? {
            fn static_type() -> $crate::types::Type {
                #[allow(unused_unsafe)]
                unsafe { $crate::translate::from_glib($get_type_expr) }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ValueType for $name $(<$($generic),+>)? {
            type Type = $name $(<$($generic),+>)?;
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ValueTypeOptional for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        unsafe impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::value::FromValue<'a> for $name $(<$($generic),+>)? {
            type Checker = $crate::object::ObjectValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let ptr = $crate::gobject_ffi::g_value_dup_object($crate::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                assert_ne!((*ptr).ref_count, 0);
                <Self as $crate::translate::FromGlibPtrFull<*mut $ffi_name>>::from_glib_full(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::value::FromValue<'a> for &'a $name $(<$($generic),+>)? {
            type Checker = $crate::object::ObjectValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                assert_eq!(std::mem::size_of::<Self>(), std::mem::size_of::<$crate::ffi::gpointer>());
                let value = &*(value as *const $crate::Value as *const $crate::gobject_ffi::GValue);
                let ptr = &value.data[0].v_pointer as *const $crate::ffi::gpointer as *const *const $ffi_name;
                assert!(!(*ptr).is_null());
                assert_ne!((**(ptr as *const *const $crate::gobject_ffi::GObject)).ref_count, 0);
                &*(ptr as *const $name $(<$($generic),+>)?)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValue for $name $(<$($generic),+>)? {
            fn to_value(&self) -> $crate::Value {
                unsafe {
                    let mut value = $crate::Value::from_type(<Self as $crate::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_take_object(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_full(self) as *mut _,
                    );
                    value
                }
            }

            fn value_type(&self) -> $crate::Type {
                <Self as $crate::StaticType>::static_type()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValueOptional for $name $(<$($generic),+>)? {
            fn to_value_optional(s: Option<&Self>) -> $crate::Value {
                let mut value = $crate::Value::for_value_type::<Self>();
                unsafe {
                    $crate::gobject_ffi::g_value_take_object(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*mut $ffi_name>::to_glib_full(&s) as *mut _,
                    );
                }

                value
            }
        }

        $crate::glib_object_wrapper!(@weak_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?);
    };

    (@weak_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?) => {
        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::clone::Downgrade for $name $(<$($generic),+>)? {
            type Weak = $crate::object::WeakRef<Self>;

            fn downgrade(&self) -> Self::Weak {
                <Self as $crate::object::ObjectExt>::downgrade(&self)
            }
        }
    };

    (@munch_impls $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, ) => { };

    (@munch_impls $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $super_name:path) => {
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsA<$super_name> for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? From<$name $(<$($generic),+>)?> for $super_name {
            fn from(v: $name $(<$($generic),+>)?) -> Self {
                <$name $(::<$($generic),+>)? as $crate::Cast>::upcast(v)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? AsRef<$super_name> for $name $(<$($generic),+>)? {
            fn as_ref(&self) -> &$super_name {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::borrow::Borrow<$super_name> for $name $(<$($generic),+>)? {
            fn borrow(&self) -> &$super_name {
                $crate::object::Cast::upcast_ref(self)
            }
        }
    };

    (@munch_impls $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $super_name:path, $($implements:tt)*) => {
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $super_name);
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $($implements)*);
    };

    // If there is no parent class, i.e. only glib::Object
    (@munch_first_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, ) => {
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, );
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ParentClassIs for $name $(<$($generic),+>)? {
            type Parent = $crate::object::Object;
        }
    };

    // If there is only one parent class
    (@munch_first_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $super_name:path) => {
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $super_name);
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ParentClassIs for $name $(<$($generic),+>)? {
            type Parent = $super_name;
        }
    };

    // If there is more than one parent class
    (@munch_first_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $super_name:path, $($implements:tt)*) => {
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $super_name);
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ParentClassIs for $name $(<$($generic),+>)? {
            type Parent = $super_name;
        }
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $($implements)*);
    };

    // This case is only for glib::Object itself below. All other cases have glib::Object in its
    // parent class list
    (@object [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $parent_type:ty, $ffi_name:ty, @ffi_class $ffi_class_name:ty, @type_ $get_type_expr:expr) => {
        $crate::glib_object_wrapper!(
            @generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $impl_type, $parent_type, $ffi_name, $ffi_class_name,
            @type_ $get_type_expr);

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsClass for $name $(<$($generic),+>)? { }
    };

    (@object [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $parent_type:ty, $ffi_name:ty,
     @type_ $get_type_expr:expr, @extends [$($extends:tt)*], @implements [$($implements:tt)*]) => {
        $crate::glib_object_wrapper!(
            @object [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $impl_type, $parent_type, $ffi_name, @ffi_class std::os::raw::c_void,
            @type_ $get_type_expr, @extends [$($extends)*], @implements [$($implements)*]
        );
    };

    (@object [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $parent_type:ty, $ffi_name:ty, @ffi_class $ffi_class_name:ty,
     @type_ $get_type_expr:expr, @extends [$($extends:tt)*], @implements [$($implements:tt)*]) => {
        $crate::glib_object_wrapper!(
            @generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $impl_type, $parent_type, $ffi_name, $ffi_class_name,
            @type_ $get_type_expr
        );

        $crate::glib_object_wrapper!(@munch_first_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $($extends)*);

        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $($implements)*);

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? AsRef<$crate::object::Object> for $name $(<$($generic),+>)? {
            fn as_ref(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::borrow::Borrow<$crate::object::Object> for $name $(<$($generic),+>)? {
            fn borrow(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? From<$name $(<$($generic),+>)?> for $crate::object::Object {
            fn from(v: $name $(<$($generic),+>)?) -> Self {
                <$name $(::<$($generic),+>)? as $crate::Cast>::upcast(v)
            }
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsA<$crate::object::Object> for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsClass for $name $(<$($generic),+>)? { }
    };

    // FIXME: Workaround for `glib::Object` not being `Send+Sync` but subclasses of it being both
    // if the impl struct is.
    (@object_subclass [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $subclass:ty,
     @extends [], @implements [$($implements:tt)*]) => {
        $crate::glib_object_wrapper!(
            @object [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?,
            $subclass, (),
            <$subclass as $crate::subclass::types::ObjectSubclass>::Instance,
            @ffi_class <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @type_ $crate::translate::IntoGlib::into_glib(<$subclass as $crate::subclass::types::ObjectSubclassType>::type_()),
            @extends [], @implements [$($implements)*]
        );

        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ObjectSubclassIs for $name $(<$($generic),+>)? {
            type Subclass = $subclass;
        }
    };

    (@object_subclass [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $subclass:ty,
     @extends [$($extends:tt)+], @implements [$($implements:tt)*]) => {
        $crate::glib_object_wrapper!(
            @object [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?,
            $subclass, <$subclass as $crate::subclass::types::ObjectSubclass>::ParentType,
            <$subclass as $crate::subclass::types::ObjectSubclass>::Instance,
            @ffi_class <$subclass as $crate::subclass::types::ObjectSubclass>::Class,
            @type_ $crate::translate::IntoGlib::into_glib(<$subclass as $crate::subclass::types::ObjectSubclassType>::type_()),
            @extends [$($extends)*], @implements [$($implements)*]
        );

        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::ObjectSubclassIs for $name $(<$($generic),+>)? {
            type Subclass = $subclass;
        }
    };

    (@interface [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $ffi_name:ty,
     @type_ $get_type_expr:expr, @requires [$($requires:tt)*]) => {
        $crate::glib_object_wrapper!(
            @interface [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $impl_type, $ffi_name, @ffi_class std::os::raw::c_void,
            @type_ $get_type_expr, @requires [$($requires)*]
        );
    };

    (@interface [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $impl_type:ty, $ffi_name:ty, @ffi_class $ffi_class_name:ty,
     @type_ $get_type_expr:expr, @requires [$($requires:tt)*]) => {
        $crate::glib_object_wrapper!(
            @generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $impl_type, (), $ffi_name, $ffi_class_name,
            @type_ $get_type_expr
        );
        $crate::glib_object_wrapper!(@munch_impls $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $($requires)*);

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? AsRef<$crate::object::Object> for $name $(<$($generic),+>)? {
            fn as_ref(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::borrow::Borrow<$crate::object::Object> for $name $(<$($generic),+>)? {
            fn borrow(&self) -> &$crate::object::Object {
                $crate::object::Cast::upcast_ref(self)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? From<$name $(<$($generic),+>)?> for $crate::object::Object {
            fn from(v: $name $(<$($generic),+>)?) -> Self {
                <$name $(::<$($generic),+>)? as $crate::Cast>::upcast(v)
            }
        }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsA<$crate::object::Object> for $name $(<$($generic),+>)? { }

        #[doc(hidden)]
        unsafe impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::object::IsInterface for $name $(<$($generic),+>)? { }
    };
}

glib_object_wrapper!(@object
    [doc = "The base class in the object hierarchy."]
    pub Object, *mut std::os::raw::c_void, (), gobject_ffi::GObject, @ffi_class gobject_ffi::GObjectClass, @type_ gobject_ffi::g_object_get_type()
);
pub type ObjectClass = Class<Object>;

impl Object {
    pub const NONE: Option<&'static Object> = None;

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object with the given properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[allow(clippy::new_ret_no_self)]
    #[track_caller]
    pub fn new<T: IsA<Object> + IsClass>(properties: &[(&str, &dyn ToValue)]) -> T {
        Object::with_type(T::static_type(), properties)
            .downcast()
            .unwrap()
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object of the given type with the given properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    pub fn with_type(type_: Type, properties: &[(&str, &dyn ToValue)]) -> Object {
        #[cfg(feature = "gio")]
        unsafe {
            let iface_type = from_glib(gio_ffi::g_initable_get_type());
            if type_.is_a(iface_type) {
                panic!("Can't instantiate type '{}' implementing `gio::Initable`. Use `gio::Initable::new()`", type_);
            }
            let iface_type = from_glib(gio_ffi::g_async_initable_get_type());
            if type_.is_a(iface_type) {
                panic!("Can't instantiate type '{}' implementing `gio::AsyncInitable`. Use `gio::AsyncInitable::new()`", type_);
            }
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.to_value()));
        }

        unsafe { Object::new_internal(type_, &mut property_values) }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object of the given type with the given properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    pub fn with_values(type_: Type, properties: &[(&str, Value)]) -> Object {
        #[cfg(feature = "gio")]
        unsafe {
            let iface_type = from_glib(gio_ffi::g_initable_get_type());
            if type_.is_a(iface_type) {
                panic!("Can't instantiate type '{}' implementing `gio::Initable`. Use `gio::Initable::new()`", type_);
            }
            let iface_type = from_glib(gio_ffi::g_async_initable_get_type());
            if type_.is_a(iface_type) {
                panic!("Can't instantiate type '{}' implementing `gio::AsyncInitable`. Use `gio::AsyncInitable::new()`", type_);
            }
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            // FIXME> This cloning can be avoided with GLib 2.74, see below.
            property_values.push((*name, value.clone()));
        }

        unsafe { Object::new_internal(type_, &mut property_values) }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object of the given type with the given properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    ///
    /// Unlike the other constructors this does not panic if the object is implementing
    /// `gio::Initable` or `gio::AsyncInitable` and it might be unsafe to use the returned object
    /// without using the API of those interfaces first.
    #[track_caller]
    pub unsafe fn new_internal(type_: Type, properties: &mut [(&str, Value)]) -> Object {
        if !type_.is_a(Object::static_type()) {
            panic!("Can't instantiate non-GObject type '{}'", type_);
        }

        if gobject_ffi::g_type_test_flags(
            type_.into_glib(),
            gobject_ffi::G_TYPE_FLAG_INSTANTIATABLE,
        ) == ffi::GFALSE
        {
            panic!("Can't instantiate type '{}'", type_);
        }

        if gobject_ffi::g_type_test_flags(type_.into_glib(), gobject_ffi::G_TYPE_FLAG_ABSTRACT)
            != ffi::GFALSE
        {
            panic!("Can't instantiate abstract type '{}'", type_);
        }

        let mut property_names = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());

        if !properties.is_empty() {
            let klass = ObjectClass::from_type(type_)
                .unwrap_or_else(|| panic!("Can't retrieve class for type '{}'", type_));
            let pspecs = klass.list_properties();

            for (idx, (name, value)) in properties.iter_mut().enumerate() {
                let pspec = pspecs
                    .iter()
                    .find(|p| p.name() == *name)
                    .unwrap_or_else(|| {
                        panic!("Can't find property '{}' for type '{}'", name, type_)
                    });

                if (pspec.flags().contains(crate::ParamFlags::CONSTRUCT)
                    || pspec.flags().contains(crate::ParamFlags::CONSTRUCT_ONLY))
                    && property_names[0..idx]
                        .iter()
                        .any(|other_name| pspec.name().as_ptr() == *other_name)
                {
                    panic!(
                        "Can't set construct property '{}' for type '{}' twice",
                        name, type_
                    );
                }

                // FIXME: With GLib 2.74 and GParamSpecClass::value_is_valid() it is possible to
                // not require mutable values here except for when LAX_VALIDATION is provided and a
                // change is needed, or a GObject value needs it's GType changed.
                validate_property_type(type_, true, pspec, value);

                property_names.push(pspec.name().as_ptr());
                property_values.push(*value.to_glib_none().0);
            }
        }

        let ptr = gobject_ffi::g_object_new_with_properties(
            type_.into_glib(),
            properties.len() as u32,
            mut_override(property_names.as_ptr() as *const *const _),
            property_values.as_ptr(),
        );

        if ptr.is_null() {
            panic!("Can't instantiate object for type '{}'", type_);
        } else if type_.is_a(InitiallyUnowned::static_type()) {
            // Attention: This takes ownership of the floating reference
            from_glib_none(ptr)
        } else {
            from_glib_full(ptr)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new object builder for a specific type.
    pub fn builder<'a, O: IsA<Object> + IsClass>() -> ObjectBuilder<'a, O> {
        ObjectBuilder::new(O::static_type())
    }

    // rustdoc-stripper-ignore-next
    /// Create a new object builder for a specific type.
    pub fn builder_with_type<'a>(type_: Type) -> ObjectBuilder<'a, Object> {
        ObjectBuilder::new(type_)
    }
}

#[must_use = "builder doesn't do anything unless built"]
pub struct ObjectBuilder<'a, O> {
    type_: Type,
    properties: Vec<(&'a str, Value)>,
    phantom: PhantomData<O>,
}

impl<'a, O: IsA<Object> + IsClass> ObjectBuilder<'a, O> {
    fn new(type_: Type) -> Self {
        ObjectBuilder {
            type_,
            properties: vec![],
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the type of this builder.
    pub fn type_(&self) -> Type {
        self.type_
    }

    // rustdoc-stripper-ignore-next
    /// Set property `name` to the given value `value`.
    pub fn property<T: ToValue + 'a>(self, name: &'a str, value: T) -> Self {
        let ObjectBuilder {
            type_,
            mut properties,
            ..
        } = self;
        properties.push((name, value.to_value()));

        ObjectBuilder {
            type_,
            properties,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the object with the provided properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    pub fn build(self) -> O {
        let object = Object::with_values(self.type_, &self.properties);
        unsafe { object.unsafe_cast::<O>() }
    }
}

#[must_use = "if unused the property notifications will immediately be thawed"]
pub struct PropertyNotificationFreezeGuard(ObjectRef);

impl Drop for PropertyNotificationFreezeGuard {
    #[doc(alias = "g_object_thaw_notify")]
    fn drop(&mut self) {
        unsafe { gobject_ffi::g_object_thaw_notify(self.0.to_glib_none().0) }
    }
}

pub trait ObjectExt: ObjectType {
    // rustdoc-stripper-ignore-next
    /// Returns `true` if the object is an instance of (can be cast to) `T`.
    fn is<T: StaticType>(&self) -> bool;

    // rustdoc-stripper-ignore-next
    /// Returns the type of the object.
    #[doc(alias = "get_type")]
    fn type_(&self) -> Type;

    // rustdoc-stripper-ignore-next
    /// Returns the [`ObjectClass`] of the object.
    ///
    /// This is equivalent to calling `obj.class().upcast_ref::<ObjectClass>()`.
    #[doc(alias = "get_object_class")]
    fn object_class(&self) -> &ObjectClass;

    /// Returns the class of the object.
    #[doc(alias = "get_class")]
    fn class(&self) -> &Class<Self>
    where
        Self: IsClass;

    // rustdoc-stripper-ignore-next
    /// Returns the class of the object in the given type `T`.
    ///
    /// `None` is returned if the object is not a subclass of `T`.
    #[doc(alias = "get_class_of")]
    fn class_of<T: IsClass>(&self) -> Option<&Class<T>>;

    // rustdoc-stripper-ignore-next
    /// Returns the interface `T` of the object.
    ///
    /// `None` is returned if the object does not implement the interface `T`.
    #[doc(alias = "get_interface")]
    fn interface<T: IsInterface>(&self) -> Option<InterfaceRef<T>>;

    // rustdoc-stripper-ignore-next
    /// Sets the property `property_name` of the object to value `value`.
    ///
    /// # Panics
    ///
    /// If the property does not exist, if the type of the property is different than
    /// the provided value, or if the property is not writable.
    #[doc(alias = "g_object_set_property")]
    fn set_property<V: ToValue>(&self, property_name: &str, value: V);

    // rustdoc-stripper-ignore-next
    /// Sets the property `property_name` of the object to value `value`.
    ///
    /// # Panics
    ///
    /// If the property does not exist, the type of the property is different than the
    /// provided value, or if the property is not writable.
    #[doc(alias = "g_object_set_property")]
    fn set_property_from_value(&self, property_name: &str, value: &Value);

    // rustdoc-stripper-ignore-next
    /// Sets multiple properties of the object at once.
    ///
    /// # Panics
    ///
    /// This does not set any properties if one or more properties don't exist, values of the wrong
    /// type are provided, or if any of the properties is not writable.
    #[doc(alias = "g_object_set")]
    fn set_properties(&self, property_values: &[(&str, &dyn ToValue)]);

    // rustdoc-stripper-ignore-next
    /// Sets multiple properties of the object at once.
    ///
    /// # Panics
    ///
    /// This does not set any properties if one or more properties don't exist, values of the wrong
    /// type are provided, or if any of the properties is not writable.
    #[doc(alias = "g_object_set")]
    fn set_properties_from_value(&self, property_values: &[(&str, Value)]);

    // rustdoc-stripper-ignore-next
    /// Gets the property `property_name` of the object and cast it to the type V.
    ///
    /// # Panics
    ///
    /// If the property doesn't exist or is not readable or of a different type than V.
    #[doc(alias = "get_property")]
    #[doc(alias = "g_object_get_property")]
    fn property<V: for<'b> FromValue<'b> + 'static>(&self, property_name: &str) -> V;

    // rustdoc-stripper-ignore-next
    /// Gets the property `property_name` of the object.
    ///
    /// # Panics
    ///
    /// If the property does not exist or is not writable.
    #[doc(alias = "get_property")]
    #[doc(alias = "g_object_get_property")]
    fn property_value(&self, property_name: &str) -> Value;

    // rustdoc-stripper-ignore-next
    /// Check if the object has a property `property_name` of the given `type_`.
    ///
    /// If no type is provided then only the existence of the property is checked.
    fn has_property(&self, property_name: &str, type_: Option<Type>) -> bool;

    // rustdoc-stripper-ignore-next
    /// Get the type of the property `property_name` of this object.
    ///
    /// This returns `None` if the property does not exist.
    #[doc(alias = "get_property_type")]
    fn property_type(&self, property_name: &str) -> Option<Type>;

    // rustdoc-stripper-ignore-next
    /// Get the [`ParamSpec`](crate::ParamSpec) of the property `property_name` of this object.
    fn find_property(&self, property_name: &str) -> Option<crate::ParamSpec>;

    // rustdoc-stripper-ignore-next
    /// Return all [`ParamSpec`](crate::ParamSpec) of the properties of this object.
    fn list_properties(&self) -> PtrSlice<crate::ParamSpec>;

    // rustdoc-stripper-ignore-next
    /// Freeze all property notifications until the return guard object is dropped.
    ///
    /// This prevents the `notify` signal for all properties of this object to be emitted.
    #[doc(alias = "g_object_freeze_notify")]
    fn freeze_notify(&self) -> PropertyNotificationFreezeGuard;

    // rustdoc-stripper-ignore-next
    /// Set arbitrary data on this object with the given `key`.
    ///
    /// # Safety
    ///
    /// This function doesn't store type information
    unsafe fn set_qdata<QD: 'static>(&self, key: Quark, value: QD);

    // rustdoc-stripper-ignore-next
    /// Return previously set arbitrary data of this object with the given `key`.
    ///
    /// # Safety
    ///
    /// The returned pointer can become invalid by a call to
    /// `set_qdata`, `steal_qdata`, `set_data` or `steal_data`.
    ///
    /// The caller is responsible for ensuring the returned value is of a suitable type
    #[doc(alias = "get_qdata")]
    unsafe fn qdata<QD: 'static>(&self, key: Quark) -> Option<ptr::NonNull<QD>>;

    // rustdoc-stripper-ignore-next
    /// Retrieve previously set arbitrary data of this object with the given `key`.
    ///
    /// The data is not set on the object anymore afterwards.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring the returned value is of a suitable type
    unsafe fn steal_qdata<QD: 'static>(&self, key: Quark) -> Option<QD>;

    // rustdoc-stripper-ignore-next
    /// Set arbitrary data on this object with the given `key`.
    ///
    /// # Safety
    ///
    /// This function doesn't store type information
    unsafe fn set_data<QD: 'static>(&self, key: &str, value: QD);

    // rustdoc-stripper-ignore-next
    /// Return previously set arbitrary data of this object with the given `key`.
    ///
    /// # Safety
    ///
    /// The returned pointer can become invalid by a call to
    /// `set_qdata`, `steal_qdata`, `set_data` or `steal_data`.
    ///
    /// The caller is responsible for ensuring the returned value is of a suitable type
    #[doc(alias = "get_data")]
    unsafe fn data<QD: 'static>(&self, key: &str) -> Option<ptr::NonNull<QD>>;

    // rustdoc-stripper-ignore-next
    /// Retrieve previously set arbitrary data of this object with the given `key`.
    ///
    /// The data is not set on the object anymore afterwards.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring the returned value is of a suitable type
    unsafe fn steal_data<QD: 'static>(&self, key: &str) -> Option<QD>;

    // rustdoc-stripper-ignore-next
    /// Block a given signal handler.
    ///
    /// It will not be called again during signal emissions until it is unblocked.
    #[doc(alias = "g_signal_handler_block")]
    fn block_signal(&self, handler_id: &SignalHandlerId);

    // rustdoc-stripper-ignore-next
    /// Unblock a given signal handler.
    #[doc(alias = "g_signal_handler_unblock")]
    fn unblock_signal(&self, handler_id: &SignalHandlerId);

    // rustdoc-stripper-ignore-next
    /// Stop emission of the currently emitted signal.
    #[doc(alias = "g_signal_stop_emission")]
    fn stop_signal_emission(&self, signal_id: SignalId, detail: Option<Quark>);

    // rustdoc-stripper-ignore-next
    /// Stop emission of the currently emitted signal by the (possibly detailed) signal name.
    #[doc(alias = "g_signal_stop_emission_by_name")]
    fn stop_signal_emission_by_name(&self, signal_name: &str);

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_name` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// # Panics
    ///
    /// If the signal does not exist.
    fn connect<F>(&self, signal_name: &str, after: bool, callback: F) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static;

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_id` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// Same as [`Self::connect`] but takes a `SignalId` instead of a signal name.
    ///
    /// # Panics
    ///
    /// If the signal does not exist.
    fn connect_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static;

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_name` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// Same as [`Self::connect`] but takes a non-`Send+Sync` closure. If the signal is emitted from a
    /// different thread than it was connected to then the signal emission will panic.
    ///
    /// # Panics
    ///
    /// If the signal does not exist.
    fn connect_local<F>(&self, signal_name: &str, after: bool, callback: F) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + 'static;

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_id` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// Same as [`Self::connect_id`] but takes a non-`Send+Sync` closure. If the signal is emitted from a
    /// different thread than it was connected to then the signal emission will panic.
    ///
    /// # Panics
    ///
    /// This panics if the signal does not exist.
    fn connect_local_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + 'static;

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_name` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// Same as [`Self::connect`] but takes a non-`Send+Sync` and non-`'static'` closure. No runtime checks
    /// are performed for ensuring that the closure is called correctly.
    ///
    /// # Safety
    ///
    /// The provided closure must be valid until the signal handler is disconnected, and it must
    /// be allowed to call the closure from the threads the signal is emitted from.
    ///
    /// # Panics
    ///
    /// If the signal does not exist.
    unsafe fn connect_unsafe<F>(
        &self,
        signal_name: &str,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Connect to the signal `signal_id` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    ///
    /// Same as [`Self::connect_id`] but takes a non-`Send+Sync` and non-`'static'` closure. No runtime checks
    /// are performed for ensuring that the closure is called correctly.
    ///
    /// # Safety
    ///
    /// The provided closure must be valid until the signal handler is disconnected, and it must
    /// be allowed to call the closure from the threads the signal is emitted from.
    ///
    /// # Panics
    ///
    /// If the signal does not exist.
    unsafe fn connect_unsafe_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Connect a closure to the signal `signal_name` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// This panics if the signal does not exist.
    ///
    /// A recurring case is connecting a handler which will be automatically disconnected
    /// when an object it refers to is destroyed, as it happens with `g_signal_connect_object`
    /// in C. This can be achieved with a closure that watches an object: see the documentation
    /// of the [`closure!`](crate::closure!) macro for more details.
    ///
    /// Same as [`Self::connect`] but takes a [`Closure`](crate::Closure) instead of a `Fn`.
    #[doc(alias = "g_signal_connect_closure")]
    #[doc(alias = "g_signal_connect_object")]
    fn connect_closure(
        &self,
        signal_name: &str,
        after: bool,
        closure: RustClosure,
    ) -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Connect a closure to the signal `signal_id` on this object.
    ///
    /// If `after` is set to `true` then the callback will be called after the default class
    /// handler of the signal is emitted, otherwise before.
    ///
    /// This panics if the signal does not exist.
    ///
    /// Same as [`Self::connect_closure`] but takes a
    /// [`SignalId`](crate::subclass::signal::SignalId) instead of a signal name.
    #[doc(alias = "g_signal_connect_closure_by_id")]
    fn connect_closure_id(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        closure: RustClosure,
    ) -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Limits the lifetime of `closure` to the lifetime of the object. When
    /// the object's reference count drops to zero, the closure will be
    /// invalidated. An invalidated closure will ignore any calls to
    /// [`invoke_with_values`](crate::closure::Closure::invoke_with_values), or
    /// [`invoke`](crate::closure::RustClosure::invoke) when using Rust closures.
    #[doc(alias = "g_object_watch_closure")]
    fn watch_closure(&self, closure: &impl AsRef<Closure>);

    // rustdoc-stripper-ignore-next
    /// Emit signal by signal id.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the wrong number of arguments is provided, or arguments of the wrong types
    /// were provided.
    #[doc(alias = "g_signal_emitv")]
    fn emit<R: TryFromClosureReturnValue>(&self, signal_id: SignalId, args: &[&dyn ToValue]) -> R;

    // rustdoc-stripper-ignore-next
    /// Same as [`Self::emit`] but takes `Value` for the arguments.
    fn emit_with_values(&self, signal_id: SignalId, args: &[Value]) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Emit signal by its name.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the signal does not exist, the wrong number of arguments is provided, or
    /// arguments of the wrong types were provided.
    #[doc(alias = "g_signal_emit_by_name")]
    fn emit_by_name<R: TryFromClosureReturnValue>(
        &self,
        signal_name: &str,
        args: &[&dyn ToValue],
    ) -> R;

    // rustdoc-stripper-ignore-next
    /// Emit signal by its name.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the signal does not exist, the wrong number of arguments is provided, or
    /// arguments of the wrong types were provided.
    fn emit_by_name_with_values(&self, signal_name: &str, args: &[Value]) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Emit signal by its name with details.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the wrong number of arguments is provided, or arguments of the wrong types
    /// were provided.
    fn emit_by_name_with_details<R: TryFromClosureReturnValue>(
        &self,
        signal_name: &str,
        details: Quark,
        args: &[&dyn ToValue],
    ) -> R;

    // rustdoc-stripper-ignore-next
    /// Emit signal by its name with details.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the wrong number of arguments is provided, or arguments of the wrong types
    /// were provided.
    fn emit_by_name_with_details_and_values(
        &self,
        signal_name: &str,
        details: Quark,
        args: &[Value],
    ) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Emit signal by signal id with details.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the wrong number of arguments is provided, or arguments of the wrong types
    /// were provided.
    fn emit_with_details<R: TryFromClosureReturnValue>(
        &self,
        signal_id: SignalId,
        details: Quark,
        args: &[&dyn ToValue],
    ) -> R;

    // rustdoc-stripper-ignore-next
    /// Emit signal by signal id with details.
    ///
    /// If the signal has a return value then this is returned here.
    ///
    /// # Panics
    ///
    /// If the wrong number of arguments is provided, or arguments of the wrong types
    /// were provided.
    fn emit_with_details_and_values(
        &self,
        signal_id: SignalId,
        details: Quark,
        args: &[Value],
    ) -> Option<Value>;

    // rustdoc-stripper-ignore-next
    /// Disconnect a previously connected signal handler.
    #[doc(alias = "g_signal_handler_disconnect")]
    fn disconnect(&self, handler_id: SignalHandlerId);

    // rustdoc-stripper-ignore-next
    /// Connect to the `notify` signal of the object.
    ///
    /// This is emitted whenever a property is changed. If `name` is provided then the signal
    /// handler is only called for this specific property.
    fn connect_notify<F: Fn(&Self, &crate::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Connect to the `notify` signal of the object.
    ///
    /// This is emitted whenever a property is changed. If `name` is provided then the signal
    /// handler is only called for this specific property.
    ///
    /// This is like `connect_notify` but doesn't require a `Send+Sync` closure. Signal emission
    /// will panic if the signal is emitted from the wrong thread.
    fn connect_notify_local<F: Fn(&Self, &crate::ParamSpec) + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Connect to the `notify` signal of the object.
    ///
    /// This is emitted whenever a property is changed. If `name` is provided then the signal
    /// handler is only called for this specific property.
    ///
    /// This is like `connect_notify` but doesn't require a `Send+Sync` or `'static` closure. No
    /// runtime checks for wrongly calling the closure are performed.
    ///
    /// # Safety
    ///
    /// The provided closure must be valid until the signal handler is disconnected, and it must
    /// be allowed to call the closure from the threads the signal is emitted from.
    unsafe fn connect_notify_unsafe<F: Fn(&Self, &crate::ParamSpec)>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId;

    // rustdoc-stripper-ignore-next
    /// Notify that the given property has changed its value.
    ///
    /// This emits the `notify` signal.
    #[doc(alias = "g_object_notify")]
    fn notify(&self, property_name: &str);

    // rustdoc-stripper-ignore-next
    /// Notify that the given property has changed its value.
    ///
    /// This emits the `notify` signal.
    #[doc(alias = "g_object_notify_by_pspec")]
    fn notify_by_pspec(&self, pspec: &crate::ParamSpec);

    // rustdoc-stripper-ignore-next
    /// Downgrade this object to a weak reference.
    fn downgrade(&self) -> WeakRef<Self>;

    // rustdoc-stripper-ignore-next
    /// Add a callback to be notified when the Object is disposed.
    #[doc(alias = "g_object_weak_ref")]
    #[doc(alias = "connect_drop")]
    fn add_weak_ref_notify<F: FnOnce() + Send + 'static>(&self, f: F) -> WeakRefNotify<Self>;

    // rustdoc-stripper-ignore-next
    /// Add a callback to be notified when the Object is disposed.
    ///
    /// This is like [`add_weak_ref_notify`][`ObjectExt::add_weak_ref_notify`] but doesn't require the closure to be [`Send`].
    /// Object dispose will panic if the object is disposed from the wrong thread.
    #[doc(alias = "g_object_weak_ref")]
    #[doc(alias = "connect_drop")]
    fn add_weak_ref_notify_local<F: FnOnce() + 'static>(&self, f: F) -> WeakRefNotify<Self>;

    // rustdoc-stripper-ignore-next
    /// Bind property `source_property` on this object to the `target_property` on the `target` object.
    ///
    /// This allows keeping the properties of both objects in sync.
    ///
    /// The binding can be unidirectional or bidirectional and optionally it is possible to
    /// transform the property values before they're passed to the other object.
    fn bind_property<'a, 'f, 't, O: ObjectType>(
        &'a self,
        source_property: &'a str,
        target: &'a O,
        target_property: &'a str,
    ) -> BindingBuilder<'a, 'f, 't>;

    // rustdoc-stripper-ignore-next
    /// Returns the strong reference count of this object.
    fn ref_count(&self) -> u32;
}

impl<T: ObjectType> ObjectExt for T {
    fn is<U: StaticType>(&self) -> bool {
        self.type_().is_a(U::static_type())
    }

    fn type_(&self) -> Type {
        self.object_class().type_()
    }

    fn object_class(&self) -> &ObjectClass {
        unsafe {
            let obj: *mut gobject_ffi::GObject = self.as_object_ref().to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *const ObjectClass;
            &*klass
        }
    }

    fn class(&self) -> &Class<Self>
    where
        Self: IsClass,
    {
        unsafe {
            let obj: *mut gobject_ffi::GObject = self.as_object_ref().to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *const Class<Self>;
            &*klass
        }
    }

    fn class_of<U: IsClass>(&self) -> Option<&Class<U>> {
        if !self.is::<U>() {
            return None;
        }

        unsafe {
            let obj: *mut gobject_ffi::GObject = self.as_object_ref().to_glib_none().0;
            let klass = (*obj).g_type_instance.g_class as *const Class<U>;
            Some(&*klass)
        }
    }

    fn interface<U: IsInterface>(&self) -> Option<InterfaceRef<U>> {
        Interface::from_class(self.object_class())
    }

    #[track_caller]
    fn set_property<V: ToValue>(&self, property_name: &str, value: V) {
        let pspec = self.find_property(property_name).unwrap_or_else(|| {
            panic!(
                "property '{}' of type '{}' not found",
                property_name,
                self.type_()
            )
        });

        let mut property_value = value.to_value();
        validate_property_type(self.type_(), false, &pspec, &mut property_value);
        unsafe {
            gobject_ffi::g_object_set_property(
                self.as_object_ref().to_glib_none().0,
                pspec.name().as_ptr() as *const _,
                property_value.to_glib_none().0,
            );
        }
    }

    #[track_caller]
    fn set_property_from_value(&self, property_name: &str, value: &Value) {
        let pspec = match self.find_property(property_name) {
            Some(pspec) => pspec,
            None => {
                panic!(
                    "property '{}' of type '{}' not found",
                    property_name,
                    self.type_()
                );
            }
        };

        // FIXME: With GLib 2.74 and GParamSpecClass::value_is_valid() it is possible to
        // not require mutable values here except for when LAX_VALIDATION is provided and a
        // change is needed, or a GObject value needs it's GType changed.
        let mut property_value = value.clone();
        validate_property_type(self.type_(), false, &pspec, &mut property_value);
        unsafe {
            gobject_ffi::g_object_set_property(
                self.as_object_ref().to_glib_none().0,
                pspec.name().as_ptr() as *const _,
                property_value.to_glib_none().0,
            );
        }
    }

    #[track_caller]
    fn set_properties(&self, property_values: &[(&str, &dyn ToValue)]) {
        let pspecs = self.list_properties();

        let params = property_values
            .iter()
            .map(|&(name, value)| {
                let pspec = pspecs.iter().find(|p| p.name() == name).unwrap_or_else(|| {
                    panic!("Can't find property '{}' for type '{}'", name, self.type_());
                });

                let mut value = value.to_value();
                validate_property_type(self.type_(), false, pspec, &mut value);
                (pspec.name().as_ptr(), value)
            })
            .collect::<smallvec::SmallVec<[_; 10]>>();

        for (name, value) in params {
            unsafe {
                gobject_ffi::g_object_set_property(
                    self.as_object_ref().to_glib_none().0,
                    name as *const _,
                    value.to_glib_none().0,
                );
            }
        }
    }

    #[track_caller]
    fn set_properties_from_value(&self, property_values: &[(&str, Value)]) {
        let pspecs = self.list_properties();

        let params = property_values
            .iter()
            .map(|(name, value)| {
                let pspec = pspecs
                    .iter()
                    .find(|p| p.name() == *name)
                    .unwrap_or_else(|| {
                        panic!("Can't find property '{}' for type '{}'", name, self.type_());
                    });

                let mut value = value.clone();
                validate_property_type(self.type_(), false, pspec, &mut value);
                (pspec.name().as_ptr(), value)
            })
            .collect::<smallvec::SmallVec<[_; 10]>>();

        for (name, value) in params {
            unsafe {
                gobject_ffi::g_object_set_property(
                    self.as_object_ref().to_glib_none().0,
                    name as *const _,
                    value.to_glib_none().0,
                );
            }
        }
    }

    #[track_caller]
    fn property<V: for<'b> FromValue<'b> + 'static>(&self, property_name: &str) -> V {
        let prop = self.property_value(property_name);
        let v = prop
            .get_owned::<V>()
            .unwrap_or_else(|e| panic!("Failed to get cast value to a different type {}", e));

        v
    }

    #[track_caller]
    fn property_value(&self, property_name: &str) -> Value {
        let pspec = self.find_property(property_name).unwrap_or_else(|| {
            panic!(
                "property '{}' of type '{}' not found",
                property_name,
                self.type_()
            )
        });

        if !pspec.flags().contains(crate::ParamFlags::READABLE) {
            panic!(
                "property '{}' of type '{}' is not readable",
                property_name,
                self.type_()
            );
        }

        unsafe {
            let mut value = Value::from_type(pspec.value_type());
            gobject_ffi::g_object_get_property(
                self.as_object_ref().to_glib_none().0,
                pspec.name().as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );

            // This can't really happen unless something goes wrong inside GObject
            if !value.type_().is_valid() {
                panic!(
                    "Failed to get property value for property '{}' of type '{}'",
                    property_name,
                    self.type_()
                )
            }

            value
        }
    }

    fn has_property(&self, property_name: &str, type_: Option<Type>) -> bool {
        self.object_class().has_property(property_name, type_)
    }

    fn property_type(&self, property_name: &str) -> Option<Type> {
        self.object_class().property_type(property_name)
    }

    fn find_property(&self, property_name: &str) -> Option<crate::ParamSpec> {
        self.object_class().find_property(property_name)
    }

    fn list_properties(&self) -> PtrSlice<crate::ParamSpec> {
        self.object_class().list_properties()
    }

    fn freeze_notify(&self) -> PropertyNotificationFreezeGuard {
        unsafe { gobject_ffi::g_object_freeze_notify(self.as_object_ref().to_glib_none().0) };
        PropertyNotificationFreezeGuard(self.as_object_ref().clone())
    }

    unsafe fn set_qdata<QD: 'static>(&self, key: Quark, value: QD) {
        unsafe extern "C" fn drop_value<QD>(ptr: ffi::gpointer) {
            debug_assert!(!ptr.is_null());
            let value: Box<QD> = Box::from_raw(ptr as *mut QD);
            drop(value)
        }

        let ptr = Box::into_raw(Box::new(value)) as ffi::gpointer;
        gobject_ffi::g_object_set_qdata_full(
            self.as_object_ref().to_glib_none().0,
            key.into_glib(),
            ptr,
            Some(drop_value::<QD>),
        );
    }

    unsafe fn qdata<QD: 'static>(&self, key: Quark) -> Option<ptr::NonNull<QD>> {
        ptr::NonNull::new(gobject_ffi::g_object_get_qdata(
            self.as_object_ref().to_glib_none().0,
            key.into_glib(),
        ) as *mut QD)
    }

    unsafe fn steal_qdata<QD: 'static>(&self, key: Quark) -> Option<QD> {
        let ptr = gobject_ffi::g_object_steal_qdata(
            self.as_object_ref().to_glib_none().0,
            key.into_glib(),
        );
        if ptr.is_null() {
            None
        } else {
            let value: Box<QD> = Box::from_raw(ptr as *mut QD);
            Some(*value)
        }
    }

    unsafe fn set_data<QD: 'static>(&self, key: &str, value: QD) {
        self.set_qdata::<QD>(Quark::from_str(key), value)
    }

    unsafe fn data<QD: 'static>(&self, key: &str) -> Option<ptr::NonNull<QD>> {
        self.qdata::<QD>(Quark::from_str(key))
    }

    unsafe fn steal_data<QD: 'static>(&self, key: &str) -> Option<QD> {
        self.steal_qdata::<QD>(Quark::from_str(key))
    }

    fn block_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_block(
                self.as_object_ref().to_glib_none().0,
                handler_id.as_raw(),
            );
        }
    }

    fn unblock_signal(&self, handler_id: &SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_unblock(
                self.as_object_ref().to_glib_none().0,
                handler_id.as_raw(),
            );
        }
    }

    fn stop_signal_emission(&self, signal_id: SignalId, detail: Option<Quark>) {
        unsafe {
            gobject_ffi::g_signal_stop_emission(
                self.as_object_ref().to_glib_none().0,
                signal_id.into_glib(),
                detail.into_glib(),
            );
        }
    }

    fn stop_signal_emission_by_name(&self, signal_name: &str) {
        unsafe {
            gobject_ffi::g_signal_stop_emission_by_name(
                self.as_object_ref().to_glib_none().0,
                signal_name.to_glib_none().0,
            );
        }
    }

    #[track_caller]
    fn connect<F>(&self, signal_name: &str, after: bool, callback: F) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe { self.connect_unsafe(signal_name, after, callback) }
    }

    #[track_caller]
    fn connect_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + Send + Sync + 'static,
    {
        unsafe { self.connect_unsafe_id(signal_id, details, after, callback) }
    }

    #[track_caller]
    fn connect_local<F>(&self, signal_name: &str, after: bool, callback: F) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + 'static,
    {
        let callback = crate::thread_guard::ThreadGuard::new(callback);

        unsafe {
            self.connect_unsafe(signal_name, after, move |values| {
                (callback.get_ref())(values)
            })
        }
    }

    #[track_caller]
    fn connect_local_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value> + 'static,
    {
        let callback = crate::thread_guard::ThreadGuard::new(callback);

        unsafe {
            self.connect_unsafe_id(signal_id, details, after, move |values| {
                (callback.get_ref())(values)
            })
        }
    }

    #[track_caller]
    unsafe fn connect_unsafe<F>(
        &self,
        signal_name: &str,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value>,
    {
        let type_ = self.type_();
        let (signal_id, details) = SignalId::parse_name(signal_name, type_, true)
            .unwrap_or_else(|| panic!("Signal '{}' of type '{}' not found", signal_name, type_));
        self.connect_unsafe_id(signal_id, details, after, callback)
    }

    #[track_caller]
    unsafe fn connect_unsafe_id<F>(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        callback: F,
    ) -> SignalHandlerId
    where
        F: Fn(&[Value]) -> Option<Value>,
    {
        let signal_query = signal_id.query();
        let type_ = self.type_();
        let return_type: Type = signal_query.return_type().into();
        let signal_name = signal_id.name();
        let signal_query_type = signal_query.type_();

        let closure = if return_type == Type::UNIT {
            Closure::new_unsafe(move |values| {
                let ret = callback(values);
                if let Some(ret) = ret {
                    panic!(
                        "Signal '{}' of type '{}' required no return value but got value of type '{}'",
                        signal_name,
                        type_,
                        ret.type_()
                    );
                }
                None
            })
        } else {
            Closure::new_unsafe(move |values| {
                let mut ret = callback(values).unwrap_or_else(|| {
                    panic!(
                        "Signal '{}' of type '{}' required return value of type '{}' but got None",
                        signal_name,
                        type_,
                        return_type.name()
                    );
                });
                let valid_type: bool = from_glib(gobject_ffi::g_type_check_value_holds(
                    mut_override(ret.to_glib_none().0),
                    return_type.into_glib(),
                ));

                if valid_type {
                    return Some(ret);
                }

                if let Err(got) = coerce_object_type(&mut ret, return_type) {
                    panic!(
                        "Signal '{}' of type '{}' required return value of type '{}' but got '{}'",
                        signal_name, type_, return_type, got
                    );
                };
                Some(ret)
            })
        };

        assert!(
            type_.is_a(signal_query_type),
            "Signal '{}' of type '{}' but got type '{}'",
            signal_name,
            type_,
            signal_query_type
        );

        let handler = gobject_ffi::g_signal_connect_closure_by_id(
            self.as_object_ref().to_glib_none().0,
            signal_id.into_glib(),
            details.into_glib(),
            closure.as_ref().to_glib_none().0,
            after.into_glib(),
        );

        if handler == 0 {
            panic!(
                "Failed to connect to signal '{}' of type '{}'",
                signal_name, type_
            );
        }

        from_glib(handler)
    }

    #[track_caller]
    fn connect_closure(
        &self,
        signal_name: &str,
        after: bool,
        closure: RustClosure,
    ) -> SignalHandlerId {
        let type_ = self.type_();
        let (signal_id, details) = SignalId::parse_name(signal_name, type_, true)
            .unwrap_or_else(|| panic!("Signal '{}' of type '{}' not found", signal_name, type_));
        self.connect_closure_id(signal_id, details, after, closure)
    }

    #[track_caller]
    fn connect_closure_id(
        &self,
        signal_id: SignalId,
        details: Option<Quark>,
        after: bool,
        closure: RustClosure,
    ) -> SignalHandlerId {
        let signal_query = signal_id.query();
        let type_ = self.type_();
        let signal_name = signal_id.name();

        let signal_query_type = signal_query.type_();
        assert!(
            type_.is_a(signal_query_type),
            "Signal '{}' of type '{}' but got type '{}'",
            signal_name,
            type_,
            signal_query_type
        );

        unsafe {
            let handler = gobject_ffi::g_signal_connect_closure_by_id(
                self.as_object_ref().to_glib_none().0,
                signal_id.into_glib(),
                details.into_glib(),
                closure.as_ref().to_glib_none().0,
                after.into_glib(),
            );

            if handler == 0 {
                panic!(
                    "Failed to connect to signal '{}' of type '{}'",
                    signal_name, type_
                );
            }

            from_glib(handler)
        }
    }

    fn watch_closure(&self, closure: &impl AsRef<Closure>) {
        let closure = closure.as_ref();
        unsafe {
            gobject_ffi::g_object_watch_closure(
                self.as_object_ref().to_glib_none().0,
                closure.to_glib_none().0,
            );
        }
    }

    #[track_caller]
    fn emit<R: TryFromClosureReturnValue>(&self, signal_id: SignalId, args: &[&dyn ToValue]) -> R {
        let signal_query = signal_id.query();
        unsafe {
            let type_ = self.type_();

            let self_v = {
                let mut v = Value::uninitialized();
                gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.type_().into_glib());
                gobject_ffi::g_value_set_object(
                    v.to_glib_none_mut().0,
                    self.as_object_ref().to_glib_none().0,
                );
                v
            };

            let mut args = Iterator::chain(
                std::iter::once(self_v),
                args.iter().copied().map(ToValue::to_value),
            )
            .collect::<smallvec::SmallVec<[_; 10]>>();

            validate_signal_arguments(type_, &signal_query, &mut args[1..]);

            let mut return_value = if signal_query.return_type() != Type::UNIT {
                Value::from_type(signal_query.return_type().into())
            } else {
                Value::uninitialized()
            };
            let return_value_ptr = if signal_query.return_type() != Type::UNIT {
                return_value.to_glib_none_mut().0
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_ffi::GValue,
                signal_id.into_glib(),
                0,
                return_value_ptr,
            );

            R::try_from_closure_return_value(
                Some(return_value).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT),
            )
            .unwrap()
        }
    }

    #[track_caller]
    fn emit_with_values(&self, signal_id: SignalId, args: &[Value]) -> Option<Value> {
        unsafe {
            let type_ = self.type_();

            let signal_query = signal_id.query();

            let self_v = {
                let mut v = Value::uninitialized();
                gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.type_().into_glib());
                gobject_ffi::g_value_set_object(
                    v.to_glib_none_mut().0,
                    self.as_object_ref().to_glib_none().0,
                );
                v
            };

            let mut args = Iterator::chain(std::iter::once(self_v), args.iter().cloned())
                .collect::<smallvec::SmallVec<[_; 10]>>();

            validate_signal_arguments(type_, &signal_query, &mut args[1..]);

            let mut return_value = if signal_query.return_type() != Type::UNIT {
                Value::from_type(signal_query.return_type().into())
            } else {
                Value::uninitialized()
            };
            let return_value_ptr = if signal_query.return_type() != Type::UNIT {
                return_value.to_glib_none_mut().0
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_ffi::GValue,
                signal_id.into_glib(),
                0,
                return_value_ptr,
            );

            Some(return_value).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT)
        }
    }

    #[track_caller]
    fn emit_by_name<R: TryFromClosureReturnValue>(
        &self,
        signal_name: &str,
        args: &[&dyn ToValue],
    ) -> R {
        let type_ = self.type_();
        let signal_id = SignalId::lookup(signal_name, type_).unwrap_or_else(|| {
            panic!("Signal '{}' of type '{}' not found", signal_name, type_);
        });
        self.emit(signal_id, args)
    }

    #[track_caller]
    fn emit_by_name_with_values(&self, signal_name: &str, args: &[Value]) -> Option<Value> {
        let type_ = self.type_();
        let signal_id = SignalId::lookup(signal_name, type_).unwrap_or_else(|| {
            panic!("Signal '{}' of type '{}' not found", signal_name, type_);
        });
        self.emit_with_values(signal_id, args)
    }

    #[track_caller]
    fn emit_by_name_with_details<R: TryFromClosureReturnValue>(
        &self,
        signal_name: &str,
        details: Quark,
        args: &[&dyn ToValue],
    ) -> R {
        let type_ = self.type_();
        let signal_id = SignalId::lookup(signal_name, type_)
            .unwrap_or_else(|| panic!("Signal '{}' of type '{}' not found", signal_name, type_));
        self.emit_with_details(signal_id, details, args)
    }

    #[track_caller]
    fn emit_by_name_with_details_and_values(
        &self,
        signal_name: &str,
        details: Quark,
        args: &[Value],
    ) -> Option<Value> {
        let type_ = self.type_();
        let signal_id = SignalId::lookup(signal_name, type_)
            .unwrap_or_else(|| panic!("Signal '{}' of type '{}' not found", signal_name, type_));
        self.emit_with_details_and_values(signal_id, details, args)
    }

    #[track_caller]
    fn emit_with_details<R: TryFromClosureReturnValue>(
        &self,
        signal_id: SignalId,
        details: Quark,
        args: &[&dyn ToValue],
    ) -> R {
        let signal_query = signal_id.query();
        assert!(signal_query.flags().contains(crate::SignalFlags::DETAILED));

        unsafe {
            let type_ = self.type_();

            let self_v = {
                let mut v = Value::uninitialized();
                gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.type_().into_glib());
                gobject_ffi::g_value_set_object(
                    v.to_glib_none_mut().0,
                    self.as_object_ref().to_glib_none().0,
                );
                v
            };

            let mut args = Iterator::chain(
                std::iter::once(self_v),
                args.iter().copied().map(ToValue::to_value),
            )
            .collect::<smallvec::SmallVec<[_; 10]>>();

            validate_signal_arguments(type_, &signal_query, &mut args[1..]);

            let mut return_value = if signal_query.return_type() != Type::UNIT {
                Value::from_type(signal_query.return_type().into())
            } else {
                Value::uninitialized()
            };
            let return_value_ptr = if signal_query.return_type() != Type::UNIT {
                return_value.to_glib_none_mut().0
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_ffi::GValue,
                signal_id.into_glib(),
                details.into_glib(),
                return_value_ptr,
            );

            R::try_from_closure_return_value(
                Some(return_value).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT),
            )
            .unwrap()
        }
    }

    #[track_caller]
    fn emit_with_details_and_values(
        &self,
        signal_id: SignalId,
        details: Quark,
        args: &[Value],
    ) -> Option<Value> {
        let signal_query = signal_id.query();
        assert!(signal_query.flags().contains(crate::SignalFlags::DETAILED));

        unsafe {
            let type_ = self.type_();

            let self_v = {
                let mut v = Value::uninitialized();
                gobject_ffi::g_value_init(v.to_glib_none_mut().0, self.type_().into_glib());
                gobject_ffi::g_value_set_object(
                    v.to_glib_none_mut().0,
                    self.as_object_ref().to_glib_none().0,
                );
                v
            };

            let mut args = Iterator::chain(std::iter::once(self_v), args.iter().cloned())
                .collect::<smallvec::SmallVec<[_; 10]>>();

            validate_signal_arguments(type_, &signal_query, &mut args[1..]);

            let mut return_value = if signal_query.return_type() != Type::UNIT {
                Value::from_type(signal_query.return_type().into())
            } else {
                Value::uninitialized()
            };
            let return_value_ptr = if signal_query.return_type() != Type::UNIT {
                return_value.to_glib_none_mut().0
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_signal_emitv(
                mut_override(args.as_ptr()) as *mut gobject_ffi::GValue,
                signal_id.into_glib(),
                details.into_glib(),
                return_value_ptr,
            );

            Some(return_value).filter(|r| r.type_().is_valid() && r.type_() != Type::UNIT)
        }
    }

    fn disconnect(&self, handler_id: SignalHandlerId) {
        unsafe {
            gobject_ffi::g_signal_handler_disconnect(
                self.as_object_ref().to_glib_none().0,
                handler_id.as_raw(),
            );
        }
    }

    fn connect_notify<F: Fn(&Self, &crate::ParamSpec) + Send + Sync + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe { self.connect_notify_unsafe(name, f) }
    }

    fn connect_notify_local<F: Fn(&Self, &crate::ParamSpec) + 'static>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        let f = crate::thread_guard::ThreadGuard::new(f);

        unsafe {
            self.connect_notify_unsafe(name, move |s, pspec| {
                (f.get_ref())(s, pspec);
            })
        }
    }

    unsafe fn connect_notify_unsafe<F: Fn(&Self, &crate::ParamSpec)>(
        &self,
        name: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_trampoline<P, F: Fn(&P, &crate::ParamSpec)>(
            this: *mut gobject_ffi::GObject,
            param_spec: *mut gobject_ffi::GParamSpec,
            f: ffi::gpointer,
        ) where
            P: ObjectType,
        {
            let f: &F = &*(f as *const F);
            f(
                Object::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(param_spec),
            )
        }

        let signal_name = if let Some(name) = name {
            format!("notify::{}\0", name)
        } else {
            "notify\0".into()
        };

        let f: Box<F> = Box::new(f);
        crate::signal::connect_raw(
            self.as_object_ref().to_glib_none().0,
            signal_name.as_ptr() as *const _,
            Some(mem::transmute::<_, unsafe extern "C" fn()>(
                notify_trampoline::<Self, F> as *const (),
            )),
            Box::into_raw(f),
        )
    }

    fn notify(&self, property_name: &str) {
        unsafe {
            gobject_ffi::g_object_notify(
                self.as_object_ref().to_glib_none().0,
                property_name.to_glib_none().0,
            );
        }
    }

    fn notify_by_pspec(&self, pspec: &crate::ParamSpec) {
        unsafe {
            gobject_ffi::g_object_notify_by_pspec(
                self.as_object_ref().to_glib_none().0,
                pspec.to_glib_none().0,
            );
        }
    }

    fn downgrade(&self) -> WeakRef<T> {
        unsafe {
            let w = WeakRef(Box::pin(mem::zeroed()), PhantomData);
            gobject_ffi::g_weak_ref_init(
                mut_override(&*w.0),
                self.as_object_ref().to_glib_none().0,
            );
            w
        }
    }

    fn add_weak_ref_notify<F: FnOnce() + Send + 'static>(&self, f: F) -> WeakRefNotify<T> {
        WeakRefNotify::new(self, f)
    }

    fn add_weak_ref_notify_local<F: FnOnce() + 'static>(&self, f: F) -> WeakRefNotify<T> {
        let callback = crate::thread_guard::ThreadGuard::new(f);

        WeakRefNotify::new(self, move || callback.into_inner()())
    }

    fn bind_property<'a, 'f, 't, O: ObjectType>(
        &'a self,
        source_property: &'a str,
        target: &'a O,
        target_property: &'a str,
    ) -> BindingBuilder<'a, 'f, 't> {
        BindingBuilder::new(self, source_property, target, target_property)
    }

    fn ref_count(&self) -> u32 {
        let stash = self.as_object_ref().to_glib_none();
        let ptr: *mut gobject_ffi::GObject = stash.0;

        unsafe { ffi::g_atomic_int_get(&(*ptr).ref_count as *const u32 as *const i32) as u32 }
    }
}

// Helper struct to avoid creating an extra ref on objects inside closure watches. This is safe
// because `watch_closure` ensures the object has a ref when the closure is called.
#[doc(hidden)]
pub struct WatchedObject<T: ObjectType>(ptr::NonNull<T::GlibType>);

#[doc(hidden)]
unsafe impl<T: ObjectType + Send + Sync> Send for WatchedObject<T> {}

#[doc(hidden)]
unsafe impl<T: ObjectType + Send + Sync> Sync for WatchedObject<T> {}

#[doc(hidden)]
impl<T: ObjectType> WatchedObject<T> {
    pub fn new(obj: &T) -> Self {
        Self(unsafe { ptr::NonNull::new_unchecked(obj.as_ptr()) })
    }
    // rustdoc-stripper-ignore-next
    /// # Safety
    ///
    /// This should only be called from within a closure that was previously attached to `T` using
    /// `Watchable::watch_closure`.
    pub unsafe fn borrow(&self) -> Borrowed<T>
    where
        T: FromGlibPtrBorrow<*mut <T as ObjectType>::GlibType>,
    {
        from_glib_borrow(self.0.as_ptr())
    }
}

#[doc(hidden)]
pub trait Watchable<T: ObjectType> {
    fn watched_object(&self) -> WatchedObject<T>;
    fn watch_closure(&self, closure: &impl AsRef<Closure>);
}

#[doc(hidden)]
impl<T: ObjectType> Watchable<T> for T {
    fn watched_object(&self) -> WatchedObject<T> {
        WatchedObject::new(self)
    }
    fn watch_closure(&self, closure: &impl AsRef<Closure>) {
        ObjectExt::watch_closure(self, closure)
    }
}

#[doc(hidden)]
impl<T: ObjectType> Watchable<T> for &T {
    fn watched_object(&self) -> WatchedObject<T> {
        WatchedObject::new(*self)
    }
    fn watch_closure(&self, closure: &impl AsRef<Closure>) {
        ObjectExt::watch_closure(*self, closure)
    }
}

// Validate that the given property value has an acceptable type for the given property pspec
// and if necessary update the value
#[track_caller]
fn validate_property_type(
    type_: Type,
    allow_construct_only: bool,
    pspec: &crate::ParamSpec,
    property_value: &mut Value,
) {
    if !pspec.flags().contains(crate::ParamFlags::WRITABLE)
        || (!allow_construct_only && pspec.flags().contains(crate::ParamFlags::CONSTRUCT_ONLY))
    {
        panic!(
            "property '{}' of type '{}' is not writable",
            pspec.name(),
            type_
        );
    }

    unsafe {
        // While GLib actually allows all types that can somehow be transformed
        // into the property type, we're more restrictive here to be consistent
        // with Rust's type rules. We only allow the exact same type, or if the
        // value type is a subtype of the property type
        let valid_type: bool = from_glib(gobject_ffi::g_type_check_value_holds(
            mut_override(property_value.to_glib_none().0),
            pspec.value_type().into_glib(),
        ));

        if !valid_type {
            if let Err(got) = coerce_object_type(property_value, pspec.value_type()) {
                panic!(
                        "property '{}' of type '{}' can't be set from the given type (expected: '{}', got: '{}')",
                        pspec.name(),
                        type_,
                        pspec.value_type(),
                        got
                    );
            }
        }

        let changed: bool = from_glib(gobject_ffi::g_param_value_validate(
            pspec.to_glib_none().0,
            property_value.to_glib_none_mut().0,
        ));
        let change_allowed = pspec.flags().contains(crate::ParamFlags::LAX_VALIDATION);
        if changed && !change_allowed {
            panic!(
                "property '{}' of type '{}' can't be set from given value, it is invalid or out of range",
                pspec.name(),
                type_,
            );
        }
    }
}

// If it's not directly a valid type but an object type, we check if the
// actual type of the contained object is compatible and if so create
// a properly typed Value (by mutating the existing Value).
// This can happen if the type field in the Value is set to a more
// generic type than the contained value.
fn coerce_object_type(property_value: &mut Value, type_: Type) -> Result<(), Type> {
    // return early if type coercion is not possible
    match property_value.get::<Option<Object>>() {
        Ok(Some(obj)) if !(obj.type_().is_a(type_)) => Err(obj.type_()),
        Ok(_) => {
            property_value.inner.g_type = type_.into_glib();
            Ok(())
        }
        Err(_) => Err(property_value.type_()),
    }
}

#[track_caller]
fn validate_signal_arguments(type_: Type, signal_query: &SignalQuery, args: &mut [Value]) {
    let signal_name = signal_query.signal_name();

    if signal_query.n_params() != args.len() as u32 {
        panic!(
            "Incompatible number of arguments for signal '{}' of type '{}' (expected {}, got {})",
            signal_name,
            type_,
            signal_query.n_params(),
            args.len(),
        );
    }

    let param_types = Iterator::zip(args.iter_mut(), signal_query.param_types());

    for (i, (arg, param_type)) in param_types.enumerate() {
        let param_type: Type = (*param_type).into();
        if param_type != arg.type_() {
            coerce_object_type(arg, param_type).unwrap_or_else(|got|
                panic!(
                    "Incompatible argument type in argument {} for signal '{}' of type '{}' (expected {}, got {})",
                    i,
                    signal_name,
                    type_,
                    param_type,
                    got
                )
            );
        }
    }
}

impl ObjectClass {
    // rustdoc-stripper-ignore-next
    /// Check if the object class has a property `property_name` of the given `type_`.
    ///
    /// If no type is provided then only the existence of the property is checked.
    pub fn has_property(&self, property_name: &str, type_: Option<Type>) -> bool {
        let ptype = self.property_type(property_name);

        match (ptype, type_) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(ptype), Some(type_)) => ptype == type_,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Get the type of the property `property_name` of this object class.
    ///
    /// This returns `None` if the property does not exist.
    #[doc(alias = "get_property_type")]
    pub fn property_type(&self, property_name: &str) -> Option<Type> {
        self.find_property(property_name)
            .map(|pspec| pspec.value_type())
    }

    // rustdoc-stripper-ignore-next
    /// Get the [`ParamSpec`](crate::ParamSpec) of the property `property_name` of this object class.
    #[doc(alias = "g_object_class_find_property")]
    pub fn find_property(&self, property_name: &str) -> Option<crate::ParamSpec> {
        unsafe {
            let klass = self as *const _ as *const gobject_ffi::GObjectClass;

            from_glib_none(gobject_ffi::g_object_class_find_property(
                klass as *mut _,
                property_name.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return all [`ParamSpec`](crate::ParamSpec) of the properties of this object class.
    #[doc(alias = "g_object_class_list_properties")]
    pub fn list_properties(&self) -> PtrSlice<crate::ParamSpec> {
        unsafe {
            let klass = self as *const _ as *const gobject_ffi::GObjectClass;

            let mut n_properties = 0;

            let props =
                gobject_ffi::g_object_class_list_properties(klass as *mut _, &mut n_properties);
            PtrSlice::from_glib_container_num_static(props, n_properties as usize)
        }
    }
}

wrapper! {
    #[doc(alias = "GInitiallyUnowned")]
    pub struct InitiallyUnowned(Object<gobject_ffi::GInitiallyUnowned, gobject_ffi::GInitiallyUnownedClass>);

    match fn {
        type_ => || gobject_ffi::g_initially_unowned_get_type(),
    }
}

// ManuallyDrop -> The lifetime of the data isn't bound to a Rust value but a GObject. Drop could free data too early.
// Pin          -> Make sure the pointer Box(1) passed to FFI is always valid and never reallocates.
// Box(1)       -> Pointer to Box(2), 64 bits large and compatible with FFI.
// Box(2)       -> Pointer to dyn FnOnce(), 128 bits large and incompatible with FFI (so Box(1) is passed instead).
type WeakRefNotifyData = ManuallyDrop<Pin<Box<Box<dyn FnOnce() + 'static>>>>;

// rustdoc-stripper-ignore-next
/// A handle to disconnect a weak ref notify closure.
pub struct WeakRefNotify<T: ObjectType> {
    object: WeakRef<T>,
    data: WeakRefNotifyData,
}

unsafe extern "C" fn notify_func(data: ffi::gpointer, _obj: *mut gobject_ffi::GObject) {
    // SAFETY: Call to FFI with pointers that must be valid due to Pin and lifetimes.
    //         ManuallyDrop and Pin are elided because the pointer only points to Box<Box<dyn FnOnce()>>.
    let callback: Box<Box<dyn FnOnce()>> = Box::from_raw(data as *mut _);

    // SAFETY: Function must have type FnOnce() due type checks in WeakRefNotify::new.
    //         This callback can only be called once when the object is disposed, to the data can be dropped.
    (*callback)()
}

impl<T: ObjectType> WeakRefNotify<T> {
    fn new<F: FnOnce() + 'static>(obj: &T, f: F) -> WeakRefNotify<T> {
        let data: WeakRefNotifyData = ManuallyDrop::new(Box::pin(Box::new(f)));
        let data_ptr: *const Box<dyn FnOnce()> = Pin::as_ref(&data).get_ref();

        unsafe {
            // SAFETY: Call to FFI with pointers that must be valid due to Pin and lifetimes.
            gobject_ffi::g_object_weak_ref(
                obj.as_ptr() as *mut gobject_ffi::GObject,
                Some(notify_func),
                data_ptr as *mut _,
            );
        }

        let object = obj.downgrade();

        WeakRefNotify { object, data }
    }

    // rustdoc-stripper-ignore-next
    /// Try to upgrade this weak reference to a strong reference.
    ///
    /// If the stored object was already destroyed then `None` is returned.
    pub fn upgrade(&self) -> Option<T> {
        self.object.upgrade()
    }

    #[doc(alias = "g_object_weak_unref")]
    pub fn disconnect(mut self) {
        // Upgrade the object to make sure it's alive and the callback can't be called while it's disconnected.
        if let Some(obj) = self.object.upgrade() {
            let data_ptr: *const Box<dyn FnOnce()> = Pin::as_ref(&self.data).get_ref();

            unsafe {
                // SAFETY: Call to FFI with pointers that must be valid due to Pin and lifetimes.
                gobject_ffi::g_object_weak_unref(
                    obj.as_ptr() as *mut gobject_ffi::GObject,
                    Some(notify_func),
                    data_ptr as *mut _,
                );

                // SAFETY: The data can be dropped because references to GObject have been dropped too.
                //         The callback can't be called before or after because it's disconnected and the object is still alive.
                //         This function can't be called anymore either because it consumes self.
                ManuallyDrop::drop(&mut self.data);
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// A weak reference to an object.
#[derive(Debug)]
#[doc(alias = "GWeakRef")]
pub struct WeakRef<T: ObjectType>(Pin<Box<gobject_ffi::GWeakRef>>, PhantomData<*mut T>);

impl<T: ObjectType> WeakRef<T> {
    // rustdoc-stripper-ignore-next
    /// Create a new empty weak reference.
    ///
    /// `upgrade` will always return `None` until an object is set on it.
    pub fn new() -> WeakRef<T> {
        unsafe {
            let mut w = WeakRef(Box::pin(mem::zeroed()), PhantomData);
            gobject_ffi::g_weak_ref_init(
                Pin::as_mut(&mut w.0).get_unchecked_mut(),
                ptr::null_mut(),
            );
            w
        }
    }

    // rustdoc-stripper-ignore-next
    /// Set this weak reference to the given object.
    #[doc(alias = "g_weak_ref_set")]
    pub fn set(&self, obj: Option<&T>) {
        unsafe {
            gobject_ffi::g_weak_ref_set(
                mut_override(Pin::as_ref(&self.0).get_ref()),
                obj.map_or(std::ptr::null_mut(), |obj| {
                    obj.as_object_ref().to_glib_none().0
                }),
            );
        }
    }

    // rustdoc-stripper-ignore-next
    /// Try to upgrade this weak reference to a strong reference.
    ///
    /// If the stored object was already destroyed or no object was set in this weak reference then
    /// `None` is returned.
    pub fn upgrade(&self) -> Option<T> {
        unsafe {
            let ptr = gobject_ffi::g_weak_ref_get(mut_override(Pin::as_ref(&self.0).get_ref()));
            if ptr.is_null() {
                None
            } else {
                let obj: Object = from_glib_full(ptr);
                Some(T::unsafe_from(obj.into()))
            }
        }
    }
}

impl<T: ObjectType> Drop for WeakRef<T> {
    fn drop(&mut self) {
        unsafe {
            gobject_ffi::g_weak_ref_clear(Pin::as_mut(&mut self.0).get_unchecked_mut());
        }
    }
}

impl<T: ObjectType> Clone for WeakRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            let o = self.upgrade();

            let mut c = WeakRef(Box::pin(mem::zeroed()), PhantomData);
            gobject_ffi::g_weak_ref_init(
                Pin::as_mut(&mut c.0).get_unchecked_mut(),
                o.to_glib_none().0 as *mut gobject_ffi::GObject,
            );

            c
        }
    }
}

impl<T: ObjectType> Default for WeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl<T: ObjectType + Sync + Sync> Sync for WeakRef<T> {}
unsafe impl<T: ObjectType + Send + Sync> Send for WeakRef<T> {}

// rustdoc-stripper-ignore-next
/// A weak reference to the object it was created for that can be sent to
/// different threads even for object types that don't implement `Send`.
///
/// Trying to upgrade the weak reference from another thread than the one
/// where it was created on will panic but dropping or cloning can be done
/// safely from any thread.
#[derive(Debug)]
pub struct SendWeakRef<T: ObjectType>(WeakRef<T>, Option<usize>);

impl<T: ObjectType> SendWeakRef<T> {
    pub fn new() -> SendWeakRef<T> {
        SendWeakRef(WeakRef::new(), None)
    }

    pub fn into_weak_ref(self) -> WeakRef<T> {
        assert!(
            self.1.is_none() || self.1 == Some(thread_id()),
            "SendWeakRef dereferenced on a different thread",
        );

        self.0
    }
}

impl<T: ObjectType> ops::Deref for SendWeakRef<T> {
    type Target = WeakRef<T>;

    fn deref(&self) -> &WeakRef<T> {
        assert!(
            self.1.is_none() || self.1 == Some(thread_id()),
            "SendWeakRef dereferenced on a different thread"
        );

        &self.0
    }
}

// Deriving this gives the wrong trait bounds
impl<T: ObjectType> Clone for SendWeakRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1)
    }
}

impl<T: ObjectType> Default for SendWeakRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ObjectType> From<WeakRef<T>> for SendWeakRef<T> {
    fn from(v: WeakRef<T>) -> SendWeakRef<T> {
        SendWeakRef(v, Some(thread_id()))
    }
}

unsafe impl<T: ObjectType> Sync for SendWeakRef<T> {}
unsafe impl<T: ObjectType> Send for SendWeakRef<T> {}

type TransformFn<'b> =
    Option<Box<dyn Fn(&'b crate::Binding, &'b Value) -> Option<Value> + Send + Sync + 'static>>;

// rustdoc-stripper-ignore-next
/// Builder for object property bindings.
#[must_use = "The builder must be built to be used"]
pub struct BindingBuilder<'a, 'f, 't> {
    source: &'a ObjectRef,
    source_property: &'a str,
    target: &'a ObjectRef,
    target_property: &'a str,
    flags: crate::BindingFlags,
    transform_from: TransformFn<'f>,
    transform_to: TransformFn<'t>,
}

impl<'a, 'f, 't> fmt::Debug for BindingBuilder<'a, 'f, 't> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BindingBuilder")
            .field("source", &self.source)
            .field("source_property", &self.source_property)
            .field("target", &self.target)
            .field("target_property", &self.target_property)
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a, 'f, 't> BindingBuilder<'a, 'f, 't> {
    fn new(
        source: &'a impl ObjectType,
        source_property: &'a str,
        target: &'a impl ObjectType,
        target_property: &'a str,
    ) -> Self {
        Self {
            source: source.as_object_ref(),
            source_property,
            target: target.as_object_ref(),
            target_property,
            flags: crate::BindingFlags::DEFAULT,
            transform_to: None,
            transform_from: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the target object to the source object with the given closure.
    ///
    /// This function operates on `glib::Value`s.
    /// See [`Self::transform_from`] for a version which operates on concrete argument and return types.
    pub fn transform_from_with_values<
        F: Fn(&crate::Binding, &Value) -> Option<Value> + Send + Sync + 'static,
    >(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_from: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the target object to the source object with the given closure.
    ///
    /// This function operates on concrete argument and return types.
    /// See [`Self::transform_from_with_values`] for a version which operates on `glib::Value`s.
    pub fn transform_from<
        S: FromValue<'f>,
        T: ToValue,
        F: Fn(&'f crate::Binding, S) -> Option<T> + Send + Sync + 'static,
    >(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_from: Some(Box::new(move |binding, from_value| {
                let from_value = from_value.get().expect("Wrong value type");
                func(binding, from_value).map(|r| r.to_value())
            })),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the source object to the target object with the given closure.
    ///
    /// This function operates on `glib::Value`s.
    /// See [`Self::transform_to`] for a version which operates on concrete argument and return types.
    pub fn transform_to_with_values<
        F: Fn(&crate::Binding, &Value) -> Option<Value> + Send + Sync + 'static,
    >(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_to: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the source object to the target object with the given closure.
    ///
    /// This function operates on concrete argument and return types.
    /// See [`Self::transform_to_with_values`] for a version which operates on `glib::Value`s.
    pub fn transform_to<
        S: FromValue<'t>,
        T: ToValue,
        F: Fn(&'t crate::Binding, S) -> Option<T> + Send + Sync + 'static,
    >(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_to: Some(Box::new(move |binding, from_value| {
                let from_value = from_value.get().expect("Wrong value type");
                func(binding, from_value).map(|r| r.to_value())
            })),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Bind the properties with the given flags.
    pub fn flags(self, flags: crate::BindingFlags) -> Self {
        Self { flags, ..self }
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to (`BIDIRECTIONAL`)[crate::BindingFlags::BIDIRECTIONAL].
    pub fn bidirectional(mut self) -> Self {
        self.flags |= crate::BindingFlags::BIDIRECTIONAL;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to (`SYNC_CREATE`)[crate::BindingFlags::SYNC_CREATE].
    pub fn sync_create(mut self) -> Self {
        self.flags |= crate::BindingFlags::SYNC_CREATE;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Set the binding flags to (`INVERT_BOOLEAN`)[crate::BindingFlags::INVERT_BOOLEAN].
    pub fn invert_boolean(mut self) -> Self {
        self.flags |= crate::BindingFlags::INVERT_BOOLEAN;
        self
    }

    // rustdoc-stripper-ignore-next
    /// Establish the property binding.
    ///
    /// # Panics
    /// This panics if the provided properties do not exist.
    #[track_caller]
    pub fn build(self) -> crate::Binding {
        unsafe extern "C" fn transform_to_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data = &*(user_data
                as *const (TransformFn, TransformFn, crate::ParamSpec, crate::ParamSpec));

            match (transform_data.0.as_ref().unwrap())(
                &from_glib_borrow(binding),
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    assert!(
                        res.type_().is_a(transform_data.3.value_type()),
                        "Target property {} expected type {} but transform_to function returned {}",
                        transform_data.3.name(),
                        transform_data.3.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn transform_from_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data = &*(user_data
                as *const (TransformFn, TransformFn, crate::ParamSpec, crate::ParamSpec));

            match (transform_data.1.as_ref().unwrap())(
                &from_glib_borrow(binding),
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    assert!(
                        res.type_().is_a(transform_data.2.value_type()),
                        "Source property {} expected type {} but transform_from function returned {}",
                        transform_data.2.name(),
                        transform_data.2.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn free_transform_data(data: ffi::gpointer) {
            let _ = Box::from_raw(
                data as *mut (TransformFn, TransformFn, crate::ParamSpec, crate::ParamSpec),
            );
        }

        unsafe {
            let source = Object {
                inner: TypedObjectRef::new(self.source.clone()),
                phantom: std::marker::PhantomData,
            };
            let target = Object {
                inner: TypedObjectRef::new(self.target.clone()),
                phantom: std::marker::PhantomData,
            };

            let source_property = source
                .find_property(self.source_property)
                .unwrap_or_else(|| {
                    panic!(
                        "Source property {} on type {} not found",
                        self.source_property,
                        source.type_()
                    );
                });
            let target_property = target
                .find_property(self.target_property)
                .unwrap_or_else(|| {
                    panic!(
                        "Target property {} on type {} not found",
                        self.target_property,
                        target.type_()
                    );
                });

            let source_property_name = source_property.name().as_ptr();
            let target_property_name = target_property.name().as_ptr();

            let have_transform_to = self.transform_to.is_some();
            let have_transform_from = self.transform_from.is_some();
            let transform_data = if have_transform_to || have_transform_from {
                Box::into_raw(Box::new((
                    self.transform_to,
                    self.transform_from,
                    source_property,
                    target_property,
                )))
            } else {
                ptr::null_mut()
            };

            from_glib_none(gobject_ffi::g_object_bind_property_full(
                source.to_glib_none().0,
                source_property_name as *const _,
                target.to_glib_none().0,
                target_property_name as *const _,
                self.flags.into_glib(),
                if have_transform_to {
                    Some(transform_to_trampoline)
                } else {
                    None
                },
                if have_transform_from {
                    Some(transform_from_trampoline)
                } else {
                    None
                },
                transform_data as ffi::gpointer,
                if transform_data.is_null() {
                    None
                } else {
                    Some(free_transform_data)
                },
            ))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Class struct of type `T`.
#[repr(transparent)]
pub struct Class<T: IsClass>(T::GlibClassType);

impl<T: IsClass> Class<T> {
    // rustdoc-stripper-ignore-next
    /// Get the type id for this class.
    ///
    /// This is not equivalent to `T::static_type()` but is the type of the subclass of `T` where
    /// this class belongs to.
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Type {
        unsafe {
            // This also works for interfaces because they also have the type
            // as the first struct field.
            let klass = self as *const _ as *const gobject_ffi::GTypeClass;
            from_glib((*klass).g_type)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Casts this class to a reference to a parent type's class.
    pub fn upcast_ref<U: IsClass>(&self) -> &Class<U>
    where
        T: IsA<U>,
    {
        unsafe {
            let klass = self as *const _ as *const Class<U>;
            &*klass
        }
    }

    // rustdoc-stripper-ignore-next
    /// Casts this class to a mutable reference to a parent type's class.
    pub fn upcast_ref_mut<U: IsClass>(&mut self) -> &mut Class<U>
    where
        T: IsA<U>,
    {
        unsafe {
            let klass = self as *mut _ as *mut Class<U>;
            &mut *klass
        }
    }

    // rustdoc-stripper-ignore-next
    /// Casts this class to a reference to a child type's class or
    /// fails if this class is not implementing the child class.
    pub fn downcast_ref<U: IsClass>(&self) -> Option<&Class<U>>
    where
        U: IsA<T>,
    {
        if !self.type_().is_a(U::static_type()) {
            return None;
        }

        unsafe {
            let klass = self as *const _ as *const Class<U>;
            Some(&*klass)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Casts this class to a mutable reference to a child type's class or
    /// fails if this class is not implementing the child class.
    pub fn downcast_ref_mut<U: IsClass>(&mut self) -> Option<&mut Class<U>>
    where
        U: IsA<T>,
    {
        if !self.type_().is_a(U::static_type()) {
            return None;
        }

        unsafe {
            let klass = self as *mut _ as *mut Class<U>;
            Some(&mut *klass)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the class struct for `Self` of `type_`.
    ///
    /// This will return `None` if `type_` is not a subclass of `Self`.
    #[doc(alias = "g_type_class_ref")]
    pub fn from_type(type_: Type) -> Option<ClassRef<'static, T>> {
        if !type_.is_a(T::static_type()) {
            return None;
        }

        unsafe {
            let ptr = gobject_ffi::g_type_class_ref(type_.into_glib());
            if ptr.is_null() {
                None
            } else {
                Some(ClassRef(
                    ptr::NonNull::new_unchecked(ptr as *mut Self),
                    true,
                    PhantomData,
                ))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the parent class struct, if any.
    #[doc(alias = "g_type_class_peek_parent")]
    pub fn parent(&self) -> Option<ClassRef<T>> {
        unsafe {
            let ptr = gobject_ffi::g_type_class_peek_parent(&self.0 as *const _ as *mut _);
            if ptr.is_null() {
                None
            } else {
                Some(ClassRef(
                    ptr::NonNull::new_unchecked(ptr as *mut Self),
                    false,
                    PhantomData,
                ))
            }
        }
    }
}

unsafe impl<T: IsClass> Send for Class<T> {}
unsafe impl<T: IsClass> Sync for Class<T> {}

impl<T: IsClass> AsRef<T::GlibClassType> for Class<T> {
    fn as_ref(&self) -> &T::GlibClassType {
        &self.0
    }
}

impl<T: IsClass> AsMut<T::GlibClassType> for Class<T> {
    fn as_mut(&mut self) -> &mut T::GlibClassType {
        &mut self.0
    }
}

// rustdoc-stripper-ignore-next
/// Reference to the class struct of type `T`.
#[derive(Debug)]
pub struct ClassRef<'a, T: IsClass>(ptr::NonNull<Class<T>>, bool, PhantomData<&'a ()>);

impl<'a, T: IsClass> ops::Deref for ClassRef<'a, T> {
    type Target = Class<T>;

    fn deref(&self) -> &Class<T> {
        unsafe { self.0.as_ref() }
    }
}

impl<'a, T: IsClass> Drop for ClassRef<'a, T> {
    fn drop(&mut self) {
        if self.1 {
            unsafe {
                gobject_ffi::g_type_class_unref(self.0.as_ptr() as *mut _);
            }
        }
    }
}

unsafe impl<'a, T: IsClass> Send for ClassRef<'a, T> {}
unsafe impl<'a, T: IsClass> Sync for ClassRef<'a, T> {}

// This should require Self: IsA<Self::Super>, but that seems to cause a cycle error
pub unsafe trait ParentClassIs: IsClass {
    type Parent: IsClass;
}

// rustdoc-stripper-ignore-next
/// Automatically implemented by `ObjectSubclass` variants of
/// [`wrapper!`][crate::wrapper!]
pub unsafe trait ObjectSubclassIs: IsClass {
    type Subclass: ObjectSubclass;
}

impl<T: ParentClassIs> ops::Deref for Class<T> {
    type Target = Class<T::Parent>;

    fn deref(&self) -> &Self::Target {
        unsafe {
            let klass = self as *const _ as *const Self::Target;
            &*klass
        }
    }
}

impl<T: ParentClassIs> ops::DerefMut for Class<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {
            let klass = self as *mut _ as *mut Self::Target;
            &mut *klass
        }
    }
}

// rustdoc-stripper-ignore-next
/// Trait implemented by class types.
pub unsafe trait IsClass: ObjectType {}

// rustdoc-stripper-ignore-next
/// Interface struct of type `T` for some type.
#[repr(transparent)]
pub struct Interface<T: IsInterface>(T::GlibClassType);

impl<T: IsInterface> Interface<T> {
    // rustdoc-stripper-ignore-next
    /// Get the type id for this interface.
    ///
    /// This is equivalent to `T::static_type()`.
    #[doc(alias = "get_type")]
    pub fn type_(&self) -> Type {
        unsafe {
            let klass = self as *const _ as *const gobject_ffi::GTypeInterface;
            from_glib((*klass).g_type)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Get the type id for the instance type of this interface.
    ///
    /// This is not equivalent to `T::static_type()` but is the type id of the type this specific
    /// interface belongs to.
    #[doc(alias = "get_instance_type")]
    pub fn instance_type(&self) -> Type {
        unsafe {
            // This also works for interfaces because they also have the type
            // as the first struct field.
            let klass = self as *const _ as *const gobject_ffi::GTypeInterface;
            from_glib((*klass).g_instance_type)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the interface struct for `Self` of `klass`.
    ///
    /// This will return `None` if `klass` is not implementing `Self`.
    pub fn from_class<U: IsClass>(klass: &Class<U>) -> Option<InterfaceRef<T>> {
        if !klass.type_().is_a(T::static_type()) {
            return None;
        }

        unsafe {
            let ptr = gobject_ffi::g_type_interface_peek(
                &klass.0 as *const _ as *mut _,
                T::static_type().into_glib(),
            );
            if ptr.is_null() {
                None
            } else {
                Some(InterfaceRef(
                    ptr::NonNull::new_unchecked(ptr as *mut Self),
                    false,
                    PhantomData,
                ))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the default interface struct for `Self`.
    ///
    /// This will return `None` if `type_` is not an interface.
    pub fn from_type(type_: Type) -> Option<InterfaceRef<'static, T>> {
        if !type_.is_a(Type::INTERFACE) {
            return None;
        }

        unsafe {
            let ptr = gobject_ffi::g_type_default_interface_ref(T::static_type().into_glib());
            if ptr.is_null() {
                None
            } else {
                Some(InterfaceRef(
                    ptr::NonNull::new_unchecked(ptr as *mut Self),
                    true,
                    PhantomData,
                ))
            }
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the default interface struct for `Self`.
    #[doc(alias = "g_type_default_interface_ref")]
    pub fn default() -> InterfaceRef<'static, T> {
        unsafe {
            let ptr = gobject_ffi::g_type_default_interface_ref(T::static_type().into_glib());
            assert!(!ptr.is_null());
            InterfaceRef(
                ptr::NonNull::new_unchecked(ptr as *mut Self),
                true,
                PhantomData,
            )
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the parent interface struct, if any.
    ///
    /// This returns the parent interface if a parent type of the instance type also implements the
    /// interface.
    #[doc(alias = "g_type_interface_peek_parent")]
    pub fn parent(&self) -> Option<InterfaceRef<T>> {
        unsafe {
            let ptr = gobject_ffi::g_type_interface_peek_parent(&self.0 as *const _ as *mut _);
            if ptr.is_null() {
                None
            } else {
                Some(InterfaceRef(
                    ptr::NonNull::new_unchecked(ptr as *mut Self),
                    false,
                    PhantomData,
                ))
            }
        }
    }
}

impl<T: IsA<Object> + IsInterface> Interface<T> {
    // rustdoc-stripper-ignore-next
    /// Check if this interface has a property `property_name` of the given `type_`.
    ///
    /// If no type is provided then only the existence of the property is checked.
    pub fn has_property(&self, property_name: &str, type_: Option<Type>) -> bool {
        let ptype = self.property_type(property_name);

        match (ptype, type_) {
            (None, _) => false,
            (Some(_), None) => true,
            (Some(ptype), Some(type_)) => ptype == type_,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Get the type of the property `property_name` of this interface.
    ///
    /// This returns `None` if the property does not exist.
    #[doc(alias = "get_property_type")]
    pub fn property_type(&self, property_name: &str) -> Option<Type> {
        self.find_property(property_name)
            .map(|pspec| pspec.value_type())
    }

    // rustdoc-stripper-ignore-next
    /// Get the [`ParamSpec`](crate::ParamSpec) of the property `property_name` of this interface.
    #[doc(alias = "g_object_interface_find_property")]
    pub fn find_property(&self, property_name: &str) -> Option<crate::ParamSpec> {
        unsafe {
            let interface = self as *const _ as *const gobject_ffi::GTypeInterface;

            from_glib_none(gobject_ffi::g_object_interface_find_property(
                interface as *mut _,
                property_name.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Return all [`ParamSpec`](crate::ParamSpec) of the properties of this interface.
    #[doc(alias = "g_object_interface_list_properties")]
    pub fn list_properties(&self) -> PtrSlice<crate::ParamSpec> {
        unsafe {
            let interface = self as *const _ as *const gobject_ffi::GTypeInterface;

            let mut n_properties = 0;

            let props = gobject_ffi::g_object_interface_list_properties(
                interface as *mut _,
                &mut n_properties,
            );
            PtrSlice::from_glib_container_num_static(props, n_properties as usize)
        }
    }
}

unsafe impl<T: IsInterface> Send for Interface<T> {}
unsafe impl<T: IsInterface> Sync for Interface<T> {}

impl<T: IsInterface> AsRef<T::GlibClassType> for Interface<T> {
    fn as_ref(&self) -> &T::GlibClassType {
        &self.0
    }
}

impl<T: IsInterface> AsMut<T::GlibClassType> for Interface<T> {
    fn as_mut(&mut self) -> &mut T::GlibClassType {
        &mut self.0
    }
}

// rustdoc-stripper-ignore-next
/// Reference to a class struct of type `T`.
#[derive(Debug)]
pub struct InterfaceRef<'a, T: IsInterface>(ptr::NonNull<Interface<T>>, bool, PhantomData<&'a ()>);

impl<'a, T: IsInterface> Drop for InterfaceRef<'a, T> {
    fn drop(&mut self) {
        if self.1 {
            unsafe {
                gobject_ffi::g_type_default_interface_unref(self.0.as_ptr() as *mut _);
            }
        }
    }
}

impl<'a, T: IsInterface> ops::Deref for InterfaceRef<'a, T> {
    type Target = Interface<T>;

    fn deref(&self) -> &Interface<T> {
        unsafe { self.0.as_ref() }
    }
}

unsafe impl<'a, T: IsInterface> Send for InterfaceRef<'a, T> {}
unsafe impl<'a, T: IsInterface> Sync for InterfaceRef<'a, T> {}

// rustdoc-stripper-ignore-next
/// Trait implemented by interface types.
pub unsafe trait IsInterface: ObjectType {}

// rustdoc-stripper-ignore-next
/// `Value` type checker for object types.
pub struct ObjectValueTypeChecker<T>(std::marker::PhantomData<T>);

unsafe impl<T: StaticType> crate::value::ValueTypeChecker for ObjectValueTypeChecker<T> {
    type Error = crate::value::ValueTypeMismatchOrNoneError<crate::value::ValueTypeMismatchError>;

    fn check(value: &Value) -> Result<(), Self::Error> {
        // g_type_check_value_holds() only checks for the GType of the GValue. This might be
        // initialized to a parent type of the expected type and would then fail while it's
        // still valid to retrieve the value.

        unsafe {
            let requested_type = T::static_type().into_glib();
            let type_ = value.inner.g_type;

            // Direct match or value type is a subtype of the requested type.
            if gobject_ffi::g_type_is_a(type_, requested_type) != ffi::GFALSE {
                let obj = gobject_ffi::g_value_get_object(&value.inner);
                if obj.is_null() {
                    return Err(Self::Error::UnexpectedNone);
                } else {
                    return Ok(());
                }
            }

            // If the value type is not a GObject or subtype of GObject then there's a mismatch.
            if gobject_ffi::g_type_is_a(type_, gobject_ffi::G_TYPE_OBJECT) == ffi::GFALSE {
                return Err(crate::value::ValueTypeMismatchError::new(
                    Type::from_glib(type_),
                    T::static_type(),
                )
                .into());
            }

            // Otherwise peek at the actual object and its concrete type.
            let obj = gobject_ffi::g_value_get_object(&value.inner);

            // Allow any types if the object is NULL.
            if obj.is_null() {
                return Err(Self::Error::UnexpectedNone);
            }

            let type_ = (*(*obj).g_type_instance.g_class).g_type;
            // Direct match or concrete type is a subtype of the requested type.
            if gobject_ffi::g_type_is_a(type_, requested_type) != ffi::GFALSE {
                Ok(())
            } else {
                Err(crate::value::ValueTypeMismatchError::new(
                    Type::from_glib(type_),
                    T::static_type(),
                )
                .into())
            }
        }
    }
}

// rustdoc-stripper-ignore-next
/// Borrowed reference to an object of type `T`.
///
/// This dereferences into `&T`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BorrowedObject<'a, T> {
    ptr: ptr::NonNull<gobject_ffi::GObject>,
    phantom: PhantomData<&'a T>,
}

unsafe impl<'a, T: Send + Sync> Send for BorrowedObject<'a, T> {}
unsafe impl<'a, T: Send + Sync> Sync for BorrowedObject<'a, T> {}

impl<'a, T: ObjectType> BorrowedObject<'a, T> {
    // rustdoc-stripper-ignore-next
    /// Creates a new borrowed object reference.
    ///
    /// # SAFETY:
    ///
    /// The pointer needs to be valid for at least the lifetime `'a`.
    pub unsafe fn new(ptr: *mut T::GlibType) -> BorrowedObject<'a, T> {
        BorrowedObject {
            ptr: ptr::NonNull::new_unchecked(ptr as *mut _),
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Downgrade to a weak reference.
    pub fn downgrade(&self) -> <Self as crate::clone::Downgrade>::Weak
    where
        T: crate::clone::Downgrade,
    {
        <T as crate::clone::Downgrade>::downgrade(self)
    }
}

impl<'a, T> ops::Deref for BorrowedObject<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*(&self.ptr as *const _ as *const T) }
    }
}

impl<'a, T> AsRef<T> for BorrowedObject<'a, T> {
    fn as_ref(&self) -> &T {
        unsafe { &*(&self.ptr as *const _ as *const T) }
    }
}

impl<'a, T: PartialEq> PartialEq<T> for BorrowedObject<'a, T> {
    fn eq(&self, other: &T) -> bool {
        <T as PartialEq>::eq(self, other)
    }
}

impl<'a, T: PartialOrd> PartialOrd<T> for BorrowedObject<'a, T> {
    fn partial_cmp(&self, other: &T) -> Option<cmp::Ordering> {
        <T as PartialOrd>::partial_cmp(self, other)
    }
}

impl<'a, T: crate::clone::Downgrade + ObjectType> crate::clone::Downgrade
    for BorrowedObject<'a, T>
{
    type Weak = <T as crate::clone::Downgrade>::Weak;

    fn downgrade(&self) -> Self::Weak {
        <T as crate::clone::Downgrade>::downgrade(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[test]
    fn new() {
        let obj: Object = Object::new(&[]);
        drop(obj);
    }

    #[test]
    fn data() {
        let obj: Object = Object::new(&[]);
        unsafe {
            obj.set_data::<String>("foo", "hello".into());
            let data = obj.data::<String>("foo").unwrap();
            assert_eq!(data.as_ref(), "hello");
            let data2 = obj.steal_data::<String>("foo").unwrap();
            assert_eq!(data2, "hello");
        }
    }

    #[test]
    fn weak_ref() {
        let obj: Object = Object::new(&[]);

        let weakref: WeakRef<Object> = WeakRef::new();
        weakref.set(Some(&obj));
        assert!(weakref.upgrade().is_some());
        weakref.set(None);
        assert!(weakref.upgrade().is_none());

        let weakref = WeakRef::new();
        weakref.set(Some(&obj));
        assert!(weakref.upgrade().is_some());

        drop(obj);
        assert!(weakref.upgrade().is_none());
    }

    #[test]
    fn weak_ref_notify() {
        let obj: Object = Object::new(&[]);

        let handle = obj.add_weak_ref_notify(|| {
            unreachable!();
        });

        handle.disconnect();

        let called = Arc::new(AtomicBool::new(false));
        let called_weak = Arc::downgrade(&called);
        let handle = obj.add_weak_ref_notify(move || {
            called_weak.upgrade().unwrap().store(true, Ordering::SeqCst);
        });

        drop(obj);
        assert!(called.load(Ordering::SeqCst));
        handle.disconnect();

        let obj: Object = Object::new(&[]);

        let called = Arc::new(AtomicBool::new(false));
        let called_weak = Arc::downgrade(&called);
        obj.add_weak_ref_notify(move || {
            called_weak.upgrade().unwrap().store(true, Ordering::SeqCst);
        });

        drop(obj);
        assert!(called.load(Ordering::SeqCst));

        let obj: Object = Object::new(&[]);

        let called = Rc::new(Cell::new(false));
        let called_weak = Rc::downgrade(&called);
        obj.add_weak_ref_notify_local(move || {
            called_weak.upgrade().unwrap().set(true);
        });

        drop(obj);
        assert!(called.get());
    }

    #[test]
    fn test_value() {
        let obj1: Object = Object::new(&[]);
        let v = obj1.to_value();
        let obj2 = v.get::<&Object>().unwrap();

        assert_eq!(obj1.as_ptr(), obj2.as_ptr());
    }

    #[test]
    fn test_borrow_hashing() {
        let mut m = std::collections::HashSet::new();
        let boxed_object = crate::BoxedAnyObject::new("");

        m.insert(boxed_object.clone());

        let object: &Object = std::borrow::Borrow::borrow(&boxed_object);
        assert_eq!(m.get(object), Some(&boxed_object));
    }
}
