// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use std::iter::FusedIterator;
use std::marker::PhantomData;
use std::{fmt, mem, ptr};

#[derive(Debug, PartialEq, Eq)]
enum ContainerTransfer {
    Full,
    Container,
    None,
}

// rustdoc-stripper-ignore-next
/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// This can be used like a `&[T]`.
pub struct PtrSlice<
    T: GlibPtrDefault
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
> {
    ptr: ptr::NonNull<T::GlibType>,
    len: usize,
    transfer: ContainerTransfer,
}

impl<
        T: fmt::Debug
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > fmt::Debug for PtrSlice<T>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

unsafe impl<
        T: Send
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Send for PtrSlice<T>
{
}

unsafe impl<
        T: Sync
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Sync for PtrSlice<T>
{
}

impl<
        T: PartialEq
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > PartialEq for PtrSlice<T>
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<
        T: Eq
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Eq for PtrSlice<T>
{
}

impl<
        T: PartialOrd
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > PartialOrd for PtrSlice<T>
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<
        T: Ord
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Ord for PtrSlice<T>
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<
        T: std::hash::Hash
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > std::hash::Hash for PtrSlice<T>
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<
        T: PartialEq
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > PartialEq<[T]> for PtrSlice<T>
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<
        T: PartialEq
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > PartialEq<PtrSlice<T>> for [T]
{
    fn eq(&self, other: &PtrSlice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > PtrSlice<T>
{
    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a C array where the items are borrowed.
    pub unsafe fn from_glib_container_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        len: usize,
    ) -> Self {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );
        assert!(!ptr.is_null() || len == 0);

        for i in 0..len {
            let p = ptr.add(i);
            let v: T = from_glib_none(*p);
            ptr::write((*p).to(), v);
        }

        PtrSlice {
            ptr: if len == 0 {
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    // rustdoc-stripper-ignore-next
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
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a static `NULL`-terminated C array.
    ///
    /// Must only be called for static allocations that are never invalidated.
    pub unsafe fn from_glib_none_static(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_none_num_static(ptr, len)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array of which the items are static.
    ///
    /// Must only be called for static items that are never invalidated.
    pub unsafe fn from_glib_container_static(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_container_num_static(ptr, len)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array where the items are borrowed.
    pub unsafe fn from_glib_container(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );

        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                let p = ptr.add(len);
                let v: T = from_glib_none(*p);
                ptr::write((*p).to(), v);

                len += 1;
            }
        }

        PtrSlice::from_glib_full_num(ptr, len)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `PtrSlice` around a `NULL`-terminated C array.
    pub unsafe fn from_glib_full(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        let mut len = 0;
        if !ptr.is_null() {
            while !(*ptr.add(len)).is_null() {
                len += 1;
            }
        }

        PtrSlice::from_glib_full_num(ptr, len)
    }

    // rustdoc-stripper-ignore-next
    /// Returns the underlying pointer.
    pub fn as_ptr(&self) -> *const <T as GlibPtrDefault>::GlibType {
        if self.len == 0 {
            ptr::null()
        } else {
            self.ptr.as_ptr()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows this slice as a `&[T]`.
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Drop for PtrSlice<T>
{
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

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > AsRef<[T]> for PtrSlice<T>
{
    fn as_ref(&self) -> &[T] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.ptr.as_ptr() as *const T, self.len)
            }
        }
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > std::ops::Deref for PtrSlice<T>
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_ref()
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibContainer<<T as GlibPtrDefault>::GlibType, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    unsafe fn from_glib_none_num(_ptr: *mut <T as GlibPtrDefault>::GlibType, _num: usize) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container_num(
        ptr: *mut <T as GlibPtrDefault>::GlibType,
        num: usize,
    ) -> Self {
        Self::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut <T as GlibPtrDefault>::GlibType, num: usize) -> Self {
        Self::from_glib_full_num(ptr, num)
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibPtrContainer<<T as GlibPtrDefault>::GlibType, *mut <T as GlibPtrDefault>::GlibType>
    for PtrSlice<T>
{
    unsafe fn from_glib_none(_ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_container(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut <T as GlibPtrDefault>::GlibType) -> Self {
        Self::from_glib_full(ptr)
    }
}

// rustdoc-stripper-ignore-next
/// Slice of elements of type `T` allocated by the GLib allocator.
///
/// This can be used like a `&[T]`.
pub struct Slice<T: 'static> {
    ptr: ptr::NonNull<T>,
    len: usize,
    transfer: ContainerTransfer,
}

unsafe impl<T: Send + 'static> Send for Slice<T> {}

unsafe impl<T: Sync + 'static> Sync for Slice<T> {}

impl<T: fmt::Debug + 'static> fmt::Debug for Slice<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: PartialEq + 'static> PartialEq for Slice<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Eq + 'static> Eq for Slice<T> {}

impl<T: PartialOrd + 'static> PartialOrd for Slice<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord + 'static> Ord for Slice<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: std::hash::Hash + 'static> std::hash::Hash for Slice<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T: PartialEq + 'static> PartialEq<[T]> for Slice<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice() == other
    }
}

impl<T: PartialEq + 'static> PartialEq<Slice<T>> for [T] {
    fn eq(&self, other: &Slice<T>) -> bool {
        self == other.as_slice()
    }
}

impl<T: 'static> Slice<T> {
    // rustdoc-stripper-ignore-next
    /// Borrows a static C array.
    pub unsafe fn from_glib_borrow_num<'a>(ptr: *const T, len: usize) -> &'a [T] {
        assert!(!ptr.is_null() || len == 0);

        if len == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(ptr, len)
        }
    }

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array of which the items are static.
    ///
    /// Must only be called for static items that are never invalidated.
    pub unsafe fn from_glib_container_num_static(ptr: *mut T, len: usize) -> Self {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array where the items are `Copy`.
    pub unsafe fn from_glib_container_num_copy(ptr: *mut T, len: usize) -> Self
    where
        T: Copy,
    {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    // rustdoc-stripper-ignore-next
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
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array where the items are `Copy`.
    pub unsafe fn from_glib_full_num_copy(ptr: *mut T, len: usize) -> Self
    where
        T: Copy,
    {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Container,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `Slice` around a C array.
    pub unsafe fn from_glib_full_num(ptr: *mut T, len: usize) -> Self {
        assert!(!ptr.is_null() || len == 0);

        Slice {
            ptr: if len == 0 {
                ffi::g_free(ptr as ffi::gpointer);
                ptr::NonNull::dangling()
            } else {
                ptr::NonNull::new_unchecked(ptr)
            },
            len,
            transfer: ContainerTransfer::Full,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Borrows this slice as a `&[T]`.
    pub fn as_slice(&self) -> &[T] {
        self.as_ref()
    }
}

impl<T: 'static> Drop for Slice<T> {
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

impl<T: 'static> AsRef<[T]> for Slice<T> {
    fn as_ref(&self) -> &[T] {
        unsafe {
            if self.len == 0 {
                &[]
            } else {
                std::slice::from_raw_parts(self.ptr.as_ptr() as *const T, self.len)
            }
        }
    }
}

impl<T: 'static> std::ops::Deref for Slice<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_ref()
    }
}

// FIXME: Ideally FromGlibPtrNone would not be needed for from_glib_full()
impl<T: FromGlibPtrNone<*mut T> + 'static> FromGlibContainer<T, *mut T> for Slice<T> {
    unsafe fn from_glib_none_num(_ptr: *mut T, _num: usize) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container_num(ptr: *mut T, num: usize) -> Self {
        Self::from_glib_container_num(ptr, num)
    }

    unsafe fn from_glib_full_num(ptr: *mut T, num: usize) -> Self {
        Self::from_glib_full_num(ptr, num)
    }
}

// rustdoc-stripper-ignore-next
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

unsafe impl<
        T: Send
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Send for List<T>
{
}

unsafe impl<
        T: Sync
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Sync for List<T>
{
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > List<T>
{
    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
    /// Create a new `List` around a list.
    pub unsafe fn from_glib_full(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a non-destructive iterator over the `List`.
    pub fn iter(&self) -> ListIter<T> {
        ListIter::new(self)
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

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FusedIterator for List<T>
{
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibContainer<<T as GlibPtrDefault>::GlibType, *mut ffi::GList> for List<T>
{
    unsafe fn from_glib_none_num(_ptr: *mut ffi::GList, _num: usize) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container_num(ptr: *mut ffi::GList, _num: usize) -> Self {
        Self::from_glib_container(ptr)
    }

    unsafe fn from_glib_full_num(ptr: *mut ffi::GList, _num: usize) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibPtrContainer<<T as GlibPtrDefault>::GlibType, *mut ffi::GList> for List<T>
{
    unsafe fn from_glib_none(_ptr: *mut ffi::GList) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container(ptr: *mut ffi::GList) -> Self {
        Self::from_glib_container(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut ffi::GList) -> Self {
        Self::from_glib_full(ptr)
    }
}

// rustdoc-stripper-ignore-next
/// A non-destructive iterator over a [`List`].
pub struct ListIter<
    'a,
    T: GlibPtrDefault
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
> {
    ptr: Option<ptr::NonNull<ffi::GList>>,
    phantom: PhantomData<&'a T>,
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > ListIter<'a, T>
{
    fn new(list: &'a List<T>) -> ListIter<'a, T> {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );

        ListIter {
            ptr: list.ptr,
            phantom: PhantomData,
        }
    }
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Iterator for ListIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.ptr {
            None => None,
            Some(cur) => unsafe {
                self.ptr = ptr::NonNull::new(cur.as_ref().next);

                let item = &*(&cur.as_ref().data as *const ffi::gpointer as *const T);

                Some(item)
            },
        }
    }
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FusedIterator for ListIter<'a, T>
{
}

// rustdoc-stripper-ignore-next
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

unsafe impl<
        T: Send
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Send for SList<T>
{
}

unsafe impl<
        T: Sync
            + GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Sync for SList<T>
{
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > SList<T>
{
    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
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

    // rustdoc-stripper-ignore-next
    /// Create a new `SList` around a list.
    pub unsafe fn from_glib_full(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a non-destructive iterator over the `SList`.
    pub fn iter(&self) -> SListIter<T> {
        SListIter::new(self)
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

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FusedIterator for SList<T>
{
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibContainer<<T as GlibPtrDefault>::GlibType, *mut ffi::GSList> for SList<T>
{
    unsafe fn from_glib_none_num(_ptr: *mut ffi::GSList, _num: usize) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container_num(ptr: *mut ffi::GSList, _num: usize) -> Self {
        Self::from_glib_container(ptr)
    }

    unsafe fn from_glib_full_num(ptr: *mut ffi::GSList, _num: usize) -> Self {
        Self::from_glib_full(ptr)
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FromGlibPtrContainer<<T as GlibPtrDefault>::GlibType, *mut ffi::GSList> for SList<T>
{
    unsafe fn from_glib_none(_ptr: *mut ffi::GSList) -> Self {
        unimplemented!()
    }

    unsafe fn from_glib_container(ptr: *mut ffi::GSList) -> Self {
        Self::from_glib_container(ptr)
    }

    unsafe fn from_glib_full(ptr: *mut ffi::GSList) -> Self {
        Self::from_glib_full(ptr)
    }
}

// rustdoc-stripper-ignore-next
/// A non-destructive iterator over a [`SList`].
pub struct SListIter<
    'a,
    T: GlibPtrDefault
        + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
        + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
> {
    ptr: Option<ptr::NonNull<ffi::GSList>>,
    phantom: PhantomData<&'a T>,
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > SListIter<'a, T>
{
    fn new(list: &'a SList<T>) -> SListIter<'a, T> {
        assert_eq!(
            mem::size_of::<T>(),
            mem::size_of::<<T as GlibPtrDefault>::GlibType>()
        );

        SListIter {
            ptr: list.ptr,
            phantom: PhantomData,
        }
    }
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Iterator for SListIter<'a, T>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        match self.ptr {
            None => None,
            Some(cur) => unsafe {
                self.ptr = ptr::NonNull::new(cur.as_ref().next);

                let item = &*(&cur.as_ref().data as *const ffi::gpointer as *const T);

                Some(item)
            },
        }
    }
}

impl<
        'a,
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > FusedIterator for SListIter<'a, T>
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn slice() {
        let items = [
            crate::Date::from_dmy(20, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(21, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(22, crate::DateMonth::November, 2021).unwrap(),
            crate::Date::from_dmy(23, crate::DateMonth::November, 2021).unwrap(),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<ffi::GDate>() * 4) as *mut ffi::GDate;
            ptr::write(ptr.add(0), *items[0].to_glib_none().0);
            ptr::write(ptr.add(1), *items[1].to_glib_none().0);
            ptr::write(ptr.add(2), *items[2].to_glib_none().0);
            ptr::write(ptr.add(3), *items[3].to_glib_none().0);

            Slice::<crate::Date>::from_glib_full_num(ptr as *mut crate::Date, 4)
        };

        assert_eq!(&items[..], &*slice);
    }

    #[test]
    fn ptr_slice() {
        let items = [
            crate::Error::new(crate::FileError::Failed, "Failed 1"),
            crate::Error::new(crate::FileError::Noent, "Failed 2"),
            crate::Error::new(crate::FileError::Io, "Failed 3"),
            crate::Error::new(crate::FileError::Perm, "Failed 4"),
        ];

        let slice = unsafe {
            let ptr = ffi::g_malloc(mem::size_of::<ffi::GDate>() * 4) as *mut *mut ffi::GError;
            ptr::write(ptr.add(0), items[0].to_glib_full() as *mut _);
            ptr::write(ptr.add(1), items[1].to_glib_full() as *mut _);
            ptr::write(ptr.add(2), items[2].to_glib_full() as *mut _);
            ptr::write(ptr.add(3), items[3].to_glib_full() as *mut _);

            PtrSlice::<crate::Error>::from_glib_full_num(ptr, 4)
        };

        for (a, b) in Iterator::zip(items.iter(), slice.iter()) {
            assert_eq!(a.message(), b.message());
            assert_eq!(
                a.kind::<crate::FileError>().unwrap(),
                b.kind::<crate::FileError>().unwrap()
            );
        }
    }

    #[test]
    // checker-ignore-item
    fn list() {
        let items = [
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 12.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 13.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 14.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 15.0).unwrap(),
        ];
        let list = unsafe {
            let mut list =
                ffi::g_list_append(ptr::null_mut(), items[0].to_glib_full() as ffi::gpointer);
            list = ffi::g_list_append(list, items[1].to_glib_full() as ffi::gpointer);
            list = ffi::g_list_append(list, items[2].to_glib_full() as ffi::gpointer);
            list = ffi::g_list_append(list, items[3].to_glib_full() as ffi::gpointer);
            List::<crate::DateTime>::from_glib_full(list)
        };

        let list_items = list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list_items = list.collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);
    }

    #[test]
    // checker-ignore-item
    fn slist() {
        let items = [
            crate::Object::with_type(crate::Type::OBJECT, &[]).unwrap(),
            crate::Object::with_type(crate::Type::OBJECT, &[]).unwrap(),
            crate::Object::with_type(crate::Type::OBJECT, &[]).unwrap(),
            crate::Object::with_type(crate::Type::OBJECT, &[]).unwrap(),
        ];
        let list = unsafe {
            let mut list = ffi::g_slist_append(
                ptr::null_mut(),
                <crate::Object as ToGlibPtr<*mut gobject_ffi::GObject>>::to_glib_full(&items[0])
                    as ffi::gpointer,
            );
            list = ffi::g_slist_append(
                list,
                <crate::Object as ToGlibPtr<*mut gobject_ffi::GObject>>::to_glib_full(&items[1])
                    as ffi::gpointer,
            );
            list = ffi::g_slist_append(
                list,
                <crate::Object as ToGlibPtr<*mut gobject_ffi::GObject>>::to_glib_full(&items[2])
                    as ffi::gpointer,
            );
            list = ffi::g_slist_append(
                list,
                <crate::Object as ToGlibPtr<*mut gobject_ffi::GObject>>::to_glib_full(&items[3])
                    as ffi::gpointer,
            );
            SList::<crate::Object>::from_glib_full(list)
        };

        let list_items = list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list_items = list.collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);
    }
}
