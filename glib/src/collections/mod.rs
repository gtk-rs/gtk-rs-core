// Take a look at the license at the top of the repository in the LICENSE file.

use std::{iter::FusedIterator, marker::PhantomData, mem, ptr};

use crate::translate::*;

pub mod ptr_slice;
pub use ptr_slice::PtrSlice;

pub mod slice;
pub use slice::Slice;

#[derive(Debug, PartialEq, Eq)]
enum ContainerTransfer {
    Full,
    Container,
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
    /// Create a new `List` around a list.
    #[inline]
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
    #[inline]
    pub unsafe fn from_glib_container_static(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Container,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `List` around a list.
    #[inline]
    pub unsafe fn from_glib_full(list: *mut ffi::GList) -> List<T> {
        List {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a non-destructive iterator over the `List`.
    #[inline]
    pub fn iter(&self) -> ListIter<T> {
        ListIter::new(self)
    }

    // rustdoc-stripper-ignore-next
    /// Check if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ptr.is_none()
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Drop for List<T>
{
    #[inline]
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

    #[inline]
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

                ffi::g_list_free_1(cur.as_ptr());

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

    #[inline]
    unsafe fn from_glib_container_num(ptr: *mut ffi::GList, _num: usize) -> Self {
        Self::from_glib_container(ptr)
    }

    #[inline]
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

    #[inline]
    unsafe fn from_glib_container(ptr: *mut ffi::GList) -> Self {
        Self::from_glib_container(ptr)
    }

    #[inline]
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
    #[inline]
    fn new(list: &'a List<T>) -> ListIter<'a, T> {
        debug_assert_eq!(
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

    #[inline]
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
    /// Create a new `SList` around a list.
    #[inline]
    pub unsafe fn from_glib_container(list: *mut ffi::GSList) -> SList<T> {
        debug_assert_eq!(
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
    #[inline]
    pub unsafe fn from_glib_container_static(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Container,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new `SList` around a list.
    #[inline]
    pub unsafe fn from_glib_full(list: *mut ffi::GSList) -> SList<T> {
        SList {
            ptr: ptr::NonNull::new(list),
            transfer: ContainerTransfer::Full,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a non-destructive iterator over the `SList`.
    #[inline]
    pub fn iter(&self) -> SListIter<T> {
        SListIter::new(self)
    }

    // rustdoc-stripper-ignore-next
    /// Check if the list is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.ptr.is_none()
    }
}

impl<
        T: GlibPtrDefault
            + FromGlibPtrFull<<T as GlibPtrDefault>::GlibType>
            + FromGlibPtrNone<<T as GlibPtrDefault>::GlibType>,
    > Drop for SList<T>
{
    #[inline]
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

    #[inline]
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

                ffi::g_slist_free_1(cur.as_ptr());

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

    #[inline]
    unsafe fn from_glib_container_num(ptr: *mut ffi::GSList, _num: usize) -> Self {
        Self::from_glib_container(ptr)
    }

    #[inline]
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

    #[inline]
    unsafe fn from_glib_container(ptr: *mut ffi::GSList) -> Self {
        Self::from_glib_container(ptr)
    }

    #[inline]
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
    #[inline]
    fn new(list: &'a SList<T>) -> SListIter<'a, T> {
        debug_assert_eq!(
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

    #[inline]
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
    // checker-ignore-item
    fn list() {
        let items = [
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 12.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 13.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 14.0).unwrap(),
            crate::DateTime::from_utc(2021, 11, 20, 23, 41, 15.0).unwrap(),
        ];
        let list = unsafe {
            let mut list = ffi::g_list_append(
                ptr::null_mut(),
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_full(&items[0]) as ffi::gpointer,
            );
            list = ffi::g_list_append(
                list,
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_full(&items[1]) as ffi::gpointer,
            );
            list = ffi::g_list_append(
                list,
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_full(&items[2]) as ffi::gpointer,
            );
            list = ffi::g_list_append(
                list,
                ToGlibPtr::<*mut ffi::GDateTime>::to_glib_full(&items[3]) as ffi::gpointer,
            );
            List::<crate::DateTime>::from_glib_full(list)
        };
        assert!(!list.is_empty());

        let list_items = list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list_items = list.collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list = unsafe { List::<crate::DateTime>::from_glib_full(ptr::null_mut()) };
        assert!(list.is_empty());
    }

    #[test]
    // checker-ignore-item
    fn slist() {
        let items = [
            crate::Object::with_type(crate::Type::OBJECT, &[]),
            crate::Object::with_type(crate::Type::OBJECT, &[]),
            crate::Object::with_type(crate::Type::OBJECT, &[]),
            crate::Object::with_type(crate::Type::OBJECT, &[]),
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
        assert!(!list.is_empty());

        let list_items = list.iter().cloned().collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list_items = list.collect::<Vec<_>>();
        assert_eq!(&items[..], &list_items);

        let list = unsafe { SList::<crate::Object>::from_glib_full(ptr::null_mut()) };
        assert!(list.is_empty());
    }
}
