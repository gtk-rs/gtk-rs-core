// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! `IMPL` Boxed wrapper implementation.

use crate::translate::*;
use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr;

// rustdoc-stripper-ignore-next
/// Wrapper implementations for Boxed types. See `wrapper!`.
#[macro_export]
macro_rules! glib_boxed_wrapper {
    ([$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty,
     @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr
     $(, @type_ $get_type_expr:expr)?) => {
        $crate::glib_boxed_wrapper!(@generic_impl [$($attr)*] $visibility $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name);

        $crate::glib_boxed_wrapper!(
            @memory_manager_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name,
            @copy $copy_arg $copy_expr, @free $free_arg $free_expr
        );

        $crate::glib_boxed_wrapper!(@value_impl $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)?, $ffi_name $(, @type_ $get_type_expr)?);
    };

    (@generic_impl [$($attr:meta)*] $visibility:vis $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => {
        $(#[$attr])*
        #[repr(transparent)]
        $visibility struct $name $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? {
            inner: $crate::boxed::Boxed<$ffi_name, $name $(<$($generic),+>)?>,
            phantom: std::marker::PhantomData<($($($generic),+)?)>,
        }

        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? std::clone::Clone for $name $(<$($generic),+>)? {
            #[inline]
            fn clone(&self) -> Self {
                Self {
                    inner: std::clone::Clone::clone(&self.inner),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::GlibPtrDefault for $name $(<$($generic),+>)? {
            type GlibType = *mut $ffi_name;
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtr<'a, *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = &'a $crate::boxed::Boxed<$ffi_name, $name $(<$($generic),+>)?>;

            #[inline]
            fn to_glib_none(&'a self) -> $crate::translate::Stash<'a, *const $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtr::to_glib_none(&self.inner);
                $crate::translate::Stash(stash.0, stash.1)
            }

            #[inline]
            fn to_glib_full(&self) -> *const $ffi_name {
                $crate::translate::ToGlibPtr::to_glib_full(&self.inner)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibPtrMut<'a, *mut $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = &'a mut $crate::boxed::Boxed<$ffi_name, $name $(<$($generic),+>)?>;

            #[inline]
            fn to_glib_none_mut(&'a mut self) -> $crate::translate::StashMut<'a, *mut $ffi_name, Self> {
                let stash = $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut self.inner);
                $crate::translate::StashMut(stash.0, stash.1)
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *mut *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (Vec<$crate::translate::Stash<'a, *const $ffi_name, $name $(<$($generic),+>)?>>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();
                let mut v_ptr: Vec<_> = v.iter().map(|s| s.0).collect();
                v_ptr.push(std::ptr::null_mut() as *const $ffi_name);

                (v_ptr.as_ptr() as *mut *const $ffi_name, (v, Some(v_ptr)))
            }

            fn to_glib_container_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*mut *const $ffi_name, Self::Storage) {
                let v: Vec<_> = t.iter().map(|s| $crate::translate::ToGlibPtr::to_glib_none(s)).collect();

                let v_ptr = unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in v.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), s.0);
                    }

                    v_ptr
                };

                (v_ptr, (v, None))
            }

            fn to_glib_full_from_slice(t: &[$name $(<$($generic),+>)?]) -> *mut *const $ffi_name {
                unsafe {
                    let v_ptr = $crate::ffi::g_malloc0(std::mem::size_of::<*const $ffi_name>() * (t.len() + 1)) as *mut *const $ffi_name;

                    for (i, s) in t.iter().enumerate() {
                        std::ptr::write(v_ptr.add(i), $crate::translate::ToGlibPtr::to_glib_full(s));
                    }

                    v_ptr
                }
            }
        }

        #[doc(hidden)]
        impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::translate::ToGlibContainerFromSlice<'a, *const *const $ffi_name> for $name $(<$($generic),+>)? {
            type Storage = (Vec<$crate::translate::Stash<'a, *const $ffi_name, $name $(<$($generic),+>)?>>, Option<Vec<*const $ffi_name>>);

            fn to_glib_none_from_slice(t: &'a [$name $(<$($generic),+>)?]) -> (*const *const $ffi_name, Self::Storage) {
                let (ptr, stash) = $crate::translate::ToGlibContainerFromSlice::<'a, *mut *const $ffi_name>::to_glib_none_from_slice(t);
                (ptr as *const *const $ffi_name, stash)
            }

            fn to_glib_container_from_slice(_: &'a [$name $(<$($generic),+>)?]) -> (*const *const $ffi_name, Self::Storage) {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }

            fn to_glib_full_from_slice(_: &[$name $(<$($generic),+>)?]) -> *const *const $ffi_name {
                // Can't have consumer free a *const pointer
                unimplemented!()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *mut $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_none(ptr),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrNone<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_none(ptr: *const $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_none(ptr),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *mut $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_full(ptr),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrFull<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_full(ptr: *const $ffi_name) -> Self {
                Self {
                    inner: $crate::translate::from_glib_full(ptr),
                    phantom: std::marker::PhantomData,
                }
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*mut $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *mut $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::Borrowed::new(
                    Self {
                        inner: $crate::translate::from_glib_borrow::<_, $crate::boxed::Boxed<_, _>>(ptr).into_inner(),
                        phantom: std::marker::PhantomData,
                    }
                )
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::translate::FromGlibPtrBorrow<*const $ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn from_glib_borrow(ptr: *const $ffi_name) -> $crate::translate::Borrowed<Self> {
                $crate::translate::from_glib_borrow::<_, $name $(<$($generic),+>)?>(ptr as *mut $ffi_name)
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
    };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty) => { };

    (@value_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty, @type_ $get_type_expr:expr) => {
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
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                let ptr = $crate::gobject_ffi::g_value_dup_boxed($crate::translate::ToGlibPtr::to_glib_none(value).0);
                assert!(!ptr.is_null());
                <$name $(<$($generic),+>)? as $crate::translate::FromGlibPtrFull<*mut $ffi_name>>::from_glib_full(ptr as *mut $ffi_name)
            }
        }

        #[doc(hidden)]
        unsafe impl<'a $(, $($generic $(: $bound $(+ $bound2)*)?),+)?> $crate::value::FromValue<'a> for &'a $name $(<$($generic),+>)? {
            type Checker = $crate::value::GenericValueTypeOrNoneChecker<Self>;

            unsafe fn from_value(value: &'a $crate::Value) -> Self {
                assert_eq!(std::mem::size_of::<$name $(<$($generic),+>)?>(), std::mem::size_of::<$crate::ffi::gpointer>());
                let value = &*(value as *const $crate::Value as *const $crate::gobject_ffi::GValue);
                let ptr = &value.data[0].v_pointer as *const $crate::ffi::gpointer as *const *const $ffi_name;
                assert!(!(*ptr).is_null());
                &*(ptr as *const $name $(<$($generic),+>)?)
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValue for $name $(<$($generic),+>)? {
            fn to_value(&self) -> $crate::Value {
                unsafe {
                    let mut value = $crate::Value::from_type(<$name $(<$($generic),+>)? as $crate::StaticType>::static_type());
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_full(self) as *mut _,
                    );
                    value
                }
            }

            fn value_type(&self) -> $crate::Type {
                <$name $(<$($generic),+>)? as $crate::StaticType>::static_type()
            }
        }

        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::value::ToValueOptional for $name $(<$($generic),+>)? {
            fn to_value_optional(s: Option<&Self>) -> $crate::Value {
                let mut value = $crate::Value::for_value_type::<Self>();
                unsafe {
                    $crate::gobject_ffi::g_value_take_boxed(
                        $crate::translate::ToGlibPtrMut::to_glib_none_mut(&mut value).0,
                        $crate::translate::ToGlibPtr::<*const $ffi_name>::to_glib_full(&s) as *mut _,
                    );
                }

                value
            }
        }
    };

    (@memory_manager_impl $name:ident $(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, $ffi_name:ty, @copy $copy_arg:ident $copy_expr:expr, @free $free_arg:ident $free_expr:expr) => {
        #[doc(hidden)]
        impl $(<$($generic $(: $bound $(+ $bound2)*)?),+>)? $crate::boxed::BoxedMemoryManager<$ffi_name> for $name $(<$($generic),+>)? {
            #[inline]
            unsafe fn copy($copy_arg: *const $ffi_name) -> *mut $ffi_name {
                $copy_expr
            }

            #[inline]
            #[allow(clippy::no_effect)]
            unsafe fn free($free_arg: *mut $ffi_name) {
                $free_expr;
            }
        }
    };
}

// The safety docs really belong in the wrapper!() macro for Boxed<T>
/// Memory management functions for a boxed type.
pub trait BoxedMemoryManager<T>: 'static {
    /// Makes a copy.
    unsafe fn copy(ptr: *const T) -> *mut T;
    /// Frees the object.
    unsafe fn free(ptr: *mut T);
}

/// Encapsulates memory management logic for boxed types.
#[repr(transparent)]
pub struct Boxed<T: 'static, MM: BoxedMemoryManager<T>> {
    inner: ptr::NonNull<T>,
    _dummy: PhantomData<*mut MM>,
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtr<'a, *const T> for Boxed<T, MM> {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const T, Self> {
        let ptr = self.inner.as_ptr();
        Stash(ptr, self)
    }

    #[inline]
    fn to_glib_full(&self) -> *const T {
        let ptr = self.inner.as_ptr();
        unsafe { MM::copy(ptr) }
    }
}

impl<'a, T: 'static, MM: BoxedMemoryManager<T>> ToGlibPtrMut<'a, *mut T> for Boxed<T, MM> {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut T, Self> {
        let ptr = self.inner.as_ptr();
        StashMut(ptr, self)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrNone<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrNone<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_none(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        let ptr = MM::copy(ptr);
        from_glib_full(ptr)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrFull<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *mut T) -> Self {
        assert!(!ptr.is_null());
        Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrFull<*const T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_full(ptr: *const T) -> Self {
        assert!(!ptr.is_null());
        Self {
            inner: ptr::NonNull::new_unchecked(ptr as *mut T),
            _dummy: PhantomData,
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> FromGlibPtrBorrow<*mut T> for Boxed<T, MM> {
    #[inline]
    unsafe fn from_glib_borrow(ptr: *mut T) -> Borrowed<Self> {
        assert!(!ptr.is_null());
        Borrowed::new(Self {
            inner: ptr::NonNull::new_unchecked(ptr),
            _dummy: PhantomData,
        })
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Drop for Boxed<T, MM> {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            MM::free(self.inner.as_ptr());
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> fmt::Debug for Boxed<T, MM> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Boxed").field("inner", &self.inner).finish()
    }
}

impl<T, MM: BoxedMemoryManager<T>> PartialOrd for Boxed<T, MM> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.to_glib_none().0.partial_cmp(&other.to_glib_none().0)
    }
}

impl<T, MM: BoxedMemoryManager<T>> Ord for Boxed<T, MM> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.to_glib_none().0.cmp(&other.to_glib_none().0)
    }
}

impl<T, MM: BoxedMemoryManager<T>> PartialEq for Boxed<T, MM> {
    fn eq(&self, other: &Self) -> bool {
        self.to_glib_none().0 == other.to_glib_none().0
    }
}

impl<T, MM: BoxedMemoryManager<T>> Eq for Boxed<T, MM> {}

impl<T, MM: BoxedMemoryManager<T>> Hash for Boxed<T, MM> {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.to_glib_none().0.hash(state)
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Clone for Boxed<T, MM> {
    #[inline]
    fn clone(&self) -> Self {
        unsafe { from_glib_none(self.to_glib_none().0 as *mut T) }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> Deref for Boxed<T, MM> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &*self.to_glib_none().0
        }
    }
}

impl<T: 'static, MM: BoxedMemoryManager<T>> DerefMut for Boxed<T, MM> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe {
            // This is safe because the pointer will remain valid while self is borrowed
            &mut *self.to_glib_none_mut().0
        }
    }
}
