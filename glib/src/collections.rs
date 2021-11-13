// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::marker::PhantomData;
use std::{fmt, mem, ptr};

#[derive(Debug, PartialEq, Eq)]
enum ContainerTransfer {
    Full,
    Container,
    None,
}

/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// This can be used like a `&[T]`.
pub struct PtrSlice<T: GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> {
    ptr: ptr::NonNull<T::GlibType>,
    len: usize,
    transfer: ContainerTransfer,
}

impl<T: fmt::Debug + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> fmt::Debug
    for PtrSlice<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

unsafe impl<T: Send + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> Send
    for PtrSlice<T>
{
}

unsafe impl<T: Sync + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> Sync
    for PtrSlice<T>
{
}

impl<T: PartialEq + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> PartialEq
    for PtrSlice<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> Eq for PtrSlice<T> {}

impl<T: PartialOrd + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> PartialOrd
    for PtrSlice<T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> Ord
    for PtrSlice<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>>
    std::hash::Hash for PtrSlice<T>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>>
    PartialEq<[T]> for PtrSlice<T>
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq + GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>>
    PartialEq<PtrSlice<T>> for [T]
{
    fn eq(&self, other: &PtrSlice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> PtrSlice<T> {
    /// Borrows a static C array.
    pub unsafe fn from_glib_borrow_num<'a>(
        ptr: *const <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> &'a [T] {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr as *const T, len)
        }
    }

    /// Create a new `PtrSlice` around a static C array.
    ///
    /// Must only be called for static allocations that are never invalidated.
    pub unsafe fn from_glib_none_num_static(
        ptr: *const <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> Self {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        PtrSlice {
            ptr: if len == 0 {
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr as *mut _)
            },
            len,
            transfer: ContainerTransfer::None,
        }
    }

    /// Create a new `PtrSlice` around a C array of which the items are static.
    ///
    /// Must only be called for static items that are never invalidated.
    pub unsafe fn from_glib_container_num_static(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> Self {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        PtrSlice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    /// Create a new `PtrSlice` around a C array where the items are borrowed.
    pub unsafe fn from_glib_container_num<'a>(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> Self
    where
        T: ToGlibPtr<'a, <T as GlibPtrDefault>::GlibType>,
    {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        for i in 0..len {
            let p = ptr.add(i);
            let v = &*(p as *const T);
            *p = v.to_glib_full();
        }

        PtrSlice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    /// Create a new `PtrSlice` around a C array.
    pub unsafe fn from_glib_full_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> Self {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        PtrSlice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    /// Returns the underlying pointer.
    pub fn as_ptr(&self) -> *const <T as GlibPtrDefault>::GlibType {
        if self.len == 0 {
            ptr::null()
        } else {
            self.ptr.as_ptr()
        }
    }

    /// Borrows this slice as a `&[T]`.
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T: GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> Drop for PtrSlice<T> {
    fn drop(&mut self) {
        unsafe {
            if self.transfer == ContainerTransfer::Full {
                for i in 0..self.len {
                    let _: T = from_glib_full(*self.ptr.as_ptr().add(i));
                }
            }

            if self.transfer != ContainerTransfer::None && self.ptr != ptr::NonNull::dangling() {
                ffi::g_free(self.ptr.as_ptr() as ffi::gpointer);
            }
        }
    }
}

impl<T: GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> AsRef<[T]>
    for PtrSlice<T>
{
    fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr() as *const T, self.len) }
    }
}

impl<T: GlibPtrDefault + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>> std::ops::Deref
    for PtrSlice<T>
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_ref()
    }
}

/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// This can be used like a `&[T]`.
pub struct Slice<T> {
    ptr: ptr::NonNull<T>,
    len: usize,
    transfer: ContainerTransfer,
}

unsafe impl<T: Send> Send for Slice<T> {}

unsafe impl<T: Sync> Sync for Slice<T> {}

impl<T: fmt::Debug> fmt::Debug for Slice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: PartialEq> PartialEq for Slice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq> Eq for Slice<T> {}

impl<T: PartialOrd> PartialOrd for Slice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord> Ord for Slice<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Slice<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq> PartialEq<[T]> for Slice<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq> PartialEq<Slice<T>> for [T] {
    fn eq(&self, other: &Slice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T> Slice<T> {
    /// Borrows a static C array.
    pub unsafe fn from_glib_borrow_num<'a>(ptr: *const T, len: usize) -> &'a [T] {
        assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr, len)
        }
    }

    /// Create a new `Slice` around a static C array.
    ///
    /// Must only be called for static allocations that are never invalidated.
    pub unsafe fn from_glib_none_num_static(ptr: *const T, len: usize) -> Self {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr as *mut _)
            },
            len,
            transfer: ContainerTransfer::None,
        }
    }

    /// Create a new `Slice` around a C array of which the items are static.
    ///
    /// Must only be called for static items that are never invalidated.
    pub unsafe fn from_glib_container_num_static(ptr: *mut T, len: usize) -> Self {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    /// Create a new `Slice` around a C array where the items are `Copy`.
    pub unsafe fn from_glib_container_num_copy(ptr: *mut T, len: usize) -> Self
    where
        T: Copy,
    {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    /// Create a new `Slice` around a C array where the items are borrowed.
    pub unsafe fn from_glib_container_num<P: Ptr>(ptr: *mut T, len: usize) -> Self
    where
        T: FromGlibPtrNone<P>,
    {
        assert!(!ptr.is_null() || len == 0);

        for i in 0..len {
            let p = ptr.add(i);
            let v = from_glib_none(Ptr::from(p));
            ptr::write(p, v);
        }

        Slice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    /// Create a new `Slice` around a C array where the items are `Copy`.
    pub unsafe fn from_glib_full_num_copy(ptr: *mut T, len: usize) -> Self
    where
        T: Copy,
    {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    /// Create a new `Slice` around a C array.
    pub unsafe fn from_glib_full_num(ptr: *mut T, len: usize) -> Self {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                if !ptr.is_null() {
                    ffi::g_free(ptr as ffi::gpointer);
                }
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    /// Borrows this slice as a `&[T]`.
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T> Drop for Slice<T> {
    fn drop(&mut self) {
        unsafe {
            if self.transfer == ContainerTransfer::Full {
                for i in 0..self.len {
                    let _ = ptr::read(self.ptr.as_ptr().add(i));
                }
            }

            if self.transfer != ContainerTransfer::None && self.ptr != ptr::NonNull::dangling() {
                ffi::g_free(self.ptr.as_ptr() as ffi::gpointer);
            }
        }
    }
}

impl<T> AsRef<[T]> for Slice<T> {
    fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> std::ops::Deref for Slice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_ref()
    }
}

/// A list of items of type `T`.
///
/// Behaves like an `Iterator<Item = T>`.
pub struct List<
    T: GlibPtrDefault
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
> {
    ptr: Option<ptr::NonNull<ffi::GList>>,
    transfer: ContainerTransfer,
    phantom: PhantomData<T>,
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > List<T>
{
    /// Create a new `List` around a static list of static items.
    ///
    /// Must only be called for a static list of static allocations that are never invalidated.
    pub unsafe fn from_glib_none_static(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::None,
            phantom: PhantomData,
        }
    }

    /// Create a new `List` around a list.
    pub unsafe fn from_glib_container(list: *mut ffi::GList) -> List<T> {
        // Need to copy all items as we only own the container
        let mut l = list;
        while !l.is_null() {
            let item: T = from_glib_none(Ptr::from((*l).data));
            ptr::write((*l).data as *mut T, item);
            l = (*l).next;
        }

        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    /// Create a new `List` around a list of static items.
    ///
    /// Must only be called for static allocations that are never invalidated.
    pub unsafe fn from_glib_container_static(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Container,
            phantom: PhantomData,
        }
    }

    /// Create a new `List` around a list.
    pub unsafe fn from_glib_full(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Drop for List<T>
{
    fn drop(&mut self) {
        // Also cleans up the list itself as needed
        for item in self {
            drop(item);
        }
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Iterator for List<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.ptr {
            None => None,
            Some(cur) => unsafe {
                self.ptr = ptr::NonNull::new(cur.as_ref().next);
                if let Some(mut next) = self.ptr {
                    next.as_mut().prev = ptr::null_mut();
                }

                let item = if self.transfer == ContainerTransfer::Full {
                    from_glib_full(Ptr::from(cur.as_ref().data))
                } else {
                    from_glib_none(Ptr::from(cur.as_ref().data))
                };

                if self.transfer != ContainerTransfer::None {
                    ffi::g_list_free_1(cur.as_ptr());
                }

                Some(item)
            },
        }
    }
}

/// A list of items of type `T`.
///
/// Behaves like an `Iterator<Item = T>`.
pub struct SList<
    T: GlibPtrDefault
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
> {
    ptr: Option<ptr::NonNull<ffi::GSList>>,
    transfer: ContainerTransfer,
    phantom: PhantomData<T>,
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > SList<T>
{
    /// Create a new `SList` around a static list of static items.
    ///
    /// Must only be called for a static list of static allocations that are never invalidated.
    pub unsafe fn from_glib_none_static(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::None,
            phantom: PhantomData,
        }
    }

    /// Create a new `SList` around a list.
    pub unsafe fn from_glib_container(list: *mut ffi::GSList) -> SList<T> {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );

        // Need to copy all items as we only own the container
        let mut l = list;
        while !l.is_null() {
            let item: T = from_glib_none(Ptr::from((*l).data));
            ptr::write((*l).data as *mut T, item);
            l = (*l).next;
        }

        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    /// Create a new `SList` around a list of static items.
    ///
    /// Must only be called for static allocations that are never invalidated.
    pub unsafe fn from_glib_container_static(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Container,
            phantom: PhantomData,
        }
    }

    /// Create a new `SList` around a list.
    pub unsafe fn from_glib_full(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Drop for SList<T>
{
    fn drop(&mut self) {
        // Also cleans up the list itself as needed
        for item in self {
            drop(item);
        }
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Iterator for SList<T>
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self.ptr {
            None => None,
            Some(cur) => unsafe {
                self.ptr = ptr::NonNull::new(cur.as_ref().next);

                let item = if self.transfer == ContainerTransfer::Full {
                    from_glib_full(Ptr::from(cur.as_ref().data))
                } else {
                    from_glib_none(Ptr::from(cur.as_ref().data))
                };

                if self.transfer != ContainerTransfer::None {
                    ffi::g_slist_free_1(cur.as_ptr());
                }

                Some(item)
            },
        }
    }
}
