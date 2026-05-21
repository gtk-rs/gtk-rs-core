// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(feature = "v2_66")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
use crate::DateTime;
use crate::{ffi, translate::*};

#[cfg(feature = "v2_76")]
crate::wrapper! {
    #[doc(alias = "GBookmarkFile")]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct BookmarkFile(Boxed<ffi::GBookmarkFile>);

    match fn {
        copy => |ptr| ffi::g_bookmark_file_copy(mut_override(ptr)),
        free => |ptr| ffi::g_bookmark_file_free(ptr),
        type_ => || ffi::g_bookmark_file_get_type(),
    }
}

#[cfg(not(feature = "v2_76"))]
pub use non_boxed::BookmarkFile;
#[cfg(not(feature = "v2_76"))]
mod non_boxed {
    use std::{
        marker::PhantomData,
        ptr::{self, NonNull},
    };

    use super::*;

    #[doc(alias = "GBookmarkFile")]
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    #[repr(transparent)]
    pub struct BookmarkFile {
        inner: ptr::NonNull<ffi::GBookmarkFile>,
    }

    impl Drop for BookmarkFile {
        fn drop(&mut self) {
            unsafe {
                ffi::g_bookmark_file_free(self.as_ptr());
            }
        }
    }

    impl BookmarkFile {
        // rustdoc-stripper-ignore-next
        /// Return the inner pointer to the underlying C value.
        #[inline]
        pub fn as_ptr(&self) -> *mut ffi::GBookmarkFile {
            unsafe {
                *(self as *const Self as *const *const ffi::GBookmarkFile)
                    as *mut ffi::GBookmarkFile
            }
        }

        // rustdoc-stripper-ignore-next
        /// Borrows the underlying C value.
        #[inline]
        pub unsafe fn from_glib_ptr_borrow(ptr: &*mut ffi::GBookmarkFile) -> &Self {
            unsafe {
                debug_assert_eq!(
                    std::mem::size_of::<Self>(),
                    std::mem::size_of::<crate::ffi::gpointer>()
                );
                debug_assert!(!ptr.is_null());
                &*(ptr as *const *mut ffi::GBookmarkFile as *const Self)
            }
        }

        // rustdoc-stripper-ignore-next
        /// Borrows the underlying C value mutably.
        #[inline]
        pub unsafe fn from_glib_ptr_borrow_mut(ptr: &mut *mut ffi::GBookmarkFile) -> &mut Self {
            unsafe {
                debug_assert_eq!(
                    std::mem::size_of::<Self>(),
                    std::mem::size_of::<crate::ffi::gpointer>()
                );
                debug_assert!(!ptr.is_null());
                &mut *(ptr as *mut *mut ffi::GBookmarkFile as *mut Self)
            }
        }
    }

    #[doc(hidden)]
    impl crate::translate::GlibPtrDefault for BookmarkFile {
        type GlibType = *mut ffi::GBookmarkFile;
    }

    #[doc(hidden)]
    impl<'a> crate::translate::ToGlibPtr<'a, *const ffi::GBookmarkFile> for BookmarkFile {
        type Storage = PhantomData<&'a Self>;

        #[inline]
        fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GBookmarkFile, Self> {
            Stash(self.inner.as_ptr(), PhantomData)
        }
    }

    #[doc(hidden)]
    impl<'a> crate::translate::ToGlibPtr<'a, *mut ffi::GBookmarkFile> for BookmarkFile {
        type Storage = PhantomData<&'a Self>;

        #[inline]
        fn to_glib_none(&'a self) -> Stash<'a, *mut ffi::GBookmarkFile, Self> {
            Stash(self.inner.as_ptr(), PhantomData)
        }
    }

    #[doc(hidden)]
    impl<'a> crate::translate::ToGlibPtrMut<'a, *mut ffi::GBookmarkFile> for BookmarkFile {
        type Storage = PhantomData<&'a mut Self>;
        #[inline]
        fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GBookmarkFile, Self> {
            StashMut(self.inner.as_ptr(), PhantomData)
        }
    }

    #[doc(hidden)]
    impl<'a> crate::translate::ToGlibContainerFromSlice<'a, *mut *const ffi::GBookmarkFile>
        for BookmarkFile
    {
        type Storage = (
            PhantomData<&'a [Self]>,
            Option<Vec<*const ffi::GBookmarkFile>>,
        );

        fn to_glib_none_from_slice(
            t: &'a [Self],
        ) -> (*mut *const ffi::GBookmarkFile, Self::Storage) {
            let mut v_ptr = Vec::with_capacity(t.len() + 1);
            unsafe {
                let ptr = v_ptr.as_mut_ptr();
                std::ptr::copy_nonoverlapping(
                    t.as_ptr() as *mut *const ffi::GBookmarkFile,
                    ptr,
                    t.len(),
                );
                std::ptr::write(ptr.add(t.len()), std::ptr::null_mut());
                v_ptr.set_len(t.len() + 1);
            }
            (
                v_ptr.as_ptr() as *mut *const ffi::GBookmarkFile,
                (PhantomData, Some(v_ptr)),
            )
        }

        fn to_glib_container_from_slice(
            t: &'a [Self],
        ) -> (*mut *const ffi::GBookmarkFile, Self::Storage) {
            let v_ptr = unsafe {
                let v_ptr = crate::ffi::g_malloc(
                    std::mem::size_of::<*const ffi::GBookmarkFile>() * (t.len() + 1),
                ) as *mut *const ffi::GBookmarkFile;
                std::ptr::copy_nonoverlapping(
                    t.as_ptr() as *mut *const ffi::GBookmarkFile,
                    v_ptr,
                    t.len(),
                );
                std::ptr::write(v_ptr.add(t.len()), std::ptr::null_mut());
                v_ptr
            };
            (v_ptr, (PhantomData, None))
        }

        fn to_glib_full_from_slice(_: &[Self]) -> *mut *const ffi::GBookmarkFile {
            // `g_bookmark_file_copy` is missing
            unimplemented!()
        }
    }

    #[doc(hidden)]
    impl<'a> crate::translate::ToGlibContainerFromSlice<'a, *const *const ffi::GBookmarkFile>
        for BookmarkFile
    {
        type Storage = (
            PhantomData<&'a [Self]>,
            Option<Vec<*const ffi::GBookmarkFile>>,
        );
        fn to_glib_none_from_slice(
            t: &'a [Self],
        ) -> (*const *const ffi::GBookmarkFile, Self::Storage) {
            let (ptr, stash) = crate::translate::ToGlibContainerFromSlice::<
                'a,
                *mut *const ffi::GBookmarkFile,
            >::to_glib_none_from_slice(t);
            (ptr as *const *const ffi::GBookmarkFile, stash)
        }

        fn to_glib_container_from_slice(
            _: &'a [Self],
        ) -> (*const *const ffi::GBookmarkFile, Self::Storage) {
            unimplemented!()
        }

        fn to_glib_full_from_slice(_: &[Self]) -> *const *const ffi::GBookmarkFile {
            unimplemented!()
        }
    }

    #[doc(hidden)]
    impl crate::translate::FromGlibPtrFull<*mut ffi::GBookmarkFile> for BookmarkFile {
        unsafe fn from_glib_full(ptr: *mut ffi::GBookmarkFile) -> Self {
            unsafe {
                Self {
                    inner: NonNull::new_unchecked(ptr),
                }
            }
        }
    }

    #[doc(hidden)]
    impl crate::translate::FromGlibPtrFull<*const ffi::GBookmarkFile> for BookmarkFile {
        unsafe fn from_glib_full(ptr: *const ffi::GBookmarkFile) -> Self {
            unsafe {
                Self {
                    inner: NonNull::new_unchecked(ptr as *mut _),
                }
            }
        }
    }

    #[doc(hidden)]
    impl crate::translate::FromGlibPtrBorrow<*mut ffi::GBookmarkFile> for BookmarkFile {
        #[inline]
        unsafe fn from_glib_borrow(
            ptr: *mut ffi::GBookmarkFile,
        ) -> crate::translate::Borrowed<Self> {
            unsafe {
                crate::translate::Borrowed::new(Self {
                    inner: NonNull::new_unchecked(ptr),
                })
            }
        }
    }

    #[doc(hidden)]
    impl crate::translate::FromGlibPtrBorrow<*const ffi::GBookmarkFile> for BookmarkFile {
        #[inline]
        unsafe fn from_glib_borrow(
            ptr: *const ffi::GBookmarkFile,
        ) -> crate::translate::Borrowed<Self> {
            unsafe { crate::translate::from_glib_borrow::<_, Self>(ptr as *mut ffi::GBookmarkFile) }
        }
    }

    impl
        crate::translate::FromGlibContainerAsVec<
            *mut ffi::GBookmarkFile,
            *mut *mut ffi::GBookmarkFile,
        > for BookmarkFile
    {
        unsafe fn from_glib_none_num_as_vec(
            _ptr: *mut *mut ffi::GBookmarkFile,
            _num: usize,
        ) -> Vec<Self> {
            // `g_bookmark_file_copy` is missing
            unimplemented!()
        }

        unsafe fn from_glib_container_num_as_vec(
            _ptr: *mut *mut ffi::GBookmarkFile,
            _num: usize,
        ) -> Vec<Self> {
            // `g_bookmark_file_copy` is missing
            unimplemented!()
        }

        unsafe fn from_glib_full_num_as_vec(
            ptr: *mut *mut ffi::GBookmarkFile,
            num: usize,
        ) -> Vec<Self> {
            unsafe {
                if num == 0 || ptr.is_null() {
                    crate::ffi::g_free(ptr as *mut _);
                    return Vec::new();
                }
                let mut res = Vec::with_capacity(num);
                let res_ptr = res.as_mut_ptr();
                ::std::ptr::copy_nonoverlapping(ptr as *mut Self, res_ptr, num);
                res.set_len(num);
                crate::ffi::g_free(ptr as *mut _);
                res
            }
        }
    }

    #[doc(hidden)]
    impl
        crate::translate::FromGlibPtrArrayContainerAsVec<
            *mut ffi::GBookmarkFile,
            *mut *mut ffi::GBookmarkFile,
        > for BookmarkFile
    {
        unsafe fn from_glib_none_as_vec(_ptr: *mut *mut ffi::GBookmarkFile) -> Vec<Self> {
            // `g_bookmark_file_copy` is missing
            unimplemented!()
        }

        unsafe fn from_glib_container_as_vec(_ptr: *mut *mut ffi::GBookmarkFile) -> Vec<Self> {
            // `g_bookmark_file_copy` is missing
            unimplemented!()
        }

        unsafe fn from_glib_full_as_vec(ptr: *mut *mut ffi::GBookmarkFile) -> Vec<Self> {
            unsafe {
                crate::translate::FromGlibContainerAsVec::from_glib_full_num_as_vec(
                    ptr,
                    crate::translate::c_ptr_array_len(ptr),
                )
            }
        }
    }

    #[doc(hidden)]
    impl crate::translate::IntoGlibPtr<*mut ffi::GBookmarkFile> for BookmarkFile {
        #[inline]
        fn into_glib_ptr(self) -> *mut ffi::GBookmarkFile {
            std::mem::ManuallyDrop::new(self).as_ptr()
        }
    }

    #[doc(hidden)]
    impl crate::translate::IntoGlibPtr<*const ffi::GBookmarkFile> for BookmarkFile {
        #[inline]
        fn into_glib_ptr(self) -> *const ffi::GBookmarkFile {
            std::mem::ManuallyDrop::new(self).as_ptr()
        }
    }
}

impl BookmarkFile {
    #[doc(alias = "g_bookmark_file_new")]
    pub fn new() -> BookmarkFile {
        unsafe { from_glib_full(ffi::g_bookmark_file_new()) }
    }

    #[doc(alias = "g_bookmark_file_add_application")]
    pub fn add_application(&mut self, uri: &str, name: Option<&str>, exec: Option<&str>) {
        unsafe {
            ffi::g_bookmark_file_add_application(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                name.to_glib_none().0,
                exec.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_add_group")]
    pub fn add_group(&mut self, uri: &str, group: &str) {
        unsafe {
            ffi::g_bookmark_file_add_group(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                group.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_get_added")]
    #[doc(alias = "get_added")]
    pub fn added(&self, uri: &str) -> Result<libc::time_t, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_added(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_get_added_date_time")]
    #[doc(alias = "get_added_date_time")]
    pub fn added_date_time(&self, uri: &str) -> Result<DateTime, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_added_date_time(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_get_app_info")]
    #[doc(alias = "get_app_info")]
    pub fn app_info(
        &self,
        uri: &str,
        name: &str,
    ) -> Result<(crate::GString, u32, libc::time_t), crate::Error> {
        unsafe {
            let mut exec = std::ptr::null_mut();
            let mut count = std::mem::MaybeUninit::uninit();
            let mut stamp = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_get_app_info(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                name.to_glib_none().0,
                &mut exec,
                count.as_mut_ptr(),
                stamp.as_mut_ptr(),
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib_full(exec),
                    count.assume_init(),
                    stamp.assume_init(),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_get_application_info")]
    #[doc(alias = "get_application_info")]
    pub fn application_info(
        &self,
        uri: &str,
        name: &str,
    ) -> Result<(crate::GString, u32, DateTime), crate::Error> {
        unsafe {
            let mut exec = std::ptr::null_mut();
            let mut count = std::mem::MaybeUninit::uninit();
            let mut stamp = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_get_application_info(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                name.to_glib_none().0,
                &mut exec,
                count.as_mut_ptr(),
                &mut stamp,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((
                    from_glib_full(exec),
                    count.assume_init(),
                    from_glib_none(stamp),
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_applications")]
    #[doc(alias = "get_applications")]
    pub fn applications(&self, uri: &str) -> Result<Vec<crate::GString>, crate::Error> {
        unsafe {
            let mut length = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_applications(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_description")]
    #[doc(alias = "get_description")]
    pub fn description(&self, uri: &str) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_description(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_groups")]
    #[doc(alias = "get_groups")]
    pub fn groups(&self, uri: &str) -> Result<Vec<crate::GString>, crate::Error> {
        unsafe {
            let mut length = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_groups(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_icon")]
    #[doc(alias = "get_icon")]
    pub fn icon(&self, uri: &str) -> Result<(crate::GString, crate::GString), crate::Error> {
        unsafe {
            let mut href = std::ptr::null_mut();
            let mut mime_type = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_get_icon(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut href,
                &mut mime_type,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok((from_glib_full(href), from_glib_full(mime_type)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_is_private")]
    #[doc(alias = "get_is_private")]
    pub fn is_private(&self, uri: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_get_is_private(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_mime_type")]
    #[doc(alias = "get_mime_type")]
    pub fn mime_type(&self, uri: &str) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_mime_type(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_get_modified")]
    #[doc(alias = "get_modified")]
    pub fn modified(&self, uri: &str) -> Result<libc::time_t, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_modified(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_get_modified_date_time")]
    #[doc(alias = "get_modified_date_time")]
    pub fn modified_date_time(&self, uri: &str) -> Result<DateTime, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_modified_date_time(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_size")]
    #[doc(alias = "get_size")]
    pub fn size(&self) -> i32 {
        unsafe { ffi::g_bookmark_file_get_size(mut_override(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_bookmark_file_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self, uri: Option<&str>) -> Result<crate::GString, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_title(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_get_uris")]
    #[doc(alias = "get_uris")]
    pub fn uris(&self) -> Vec<crate::GString> {
        unsafe {
            let mut length = std::mem::MaybeUninit::uninit();

            FromGlibContainer::from_glib_full_num(
                ffi::g_bookmark_file_get_uris(
                    mut_override(self.to_glib_none().0),
                    length.as_mut_ptr(),
                ),
                length.assume_init() as _,
            )
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_get_visited")]
    #[doc(alias = "get_visited")]
    pub fn visited(&self, uri: &str) -> Result<libc::time_t, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_visited(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_get_visited_date_time")]
    #[doc(alias = "get_visited_date_time")]
    pub fn visited_date_time(&self, uri: &str) -> Result<DateTime, crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_get_visited_date_time(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_none(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_has_application")]
    pub fn has_application(&self, uri: &str, name: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_has_application(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                name.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_has_group")]
    pub fn has_group(&self, uri: &str, group: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_has_group(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
                group.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_has_item")]
    pub fn has_item(&self, uri: &str) -> bool {
        unsafe {
            from_glib(ffi::g_bookmark_file_has_item(
                mut_override(self.to_glib_none().0),
                uri.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_bookmark_file_load_from_data")]
    pub fn load_from_data(&mut self, data: &[u8]) -> Result<(), crate::Error> {
        let length = data.len() as _;
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_load_from_data(
                self.to_glib_none_mut().0,
                data.to_glib_none().0,
                length,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_load_from_data_dirs")]
    pub fn load_from_data_dirs(
        &mut self,
        file: impl AsRef<std::path::Path>,
    ) -> Result<std::path::PathBuf, crate::Error> {
        unsafe {
            let mut full_path = std::ptr::null_mut();
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_load_from_data_dirs(
                self.to_glib_none_mut().0,
                file.as_ref().to_glib_none().0,
                &mut full_path,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(from_glib_full(full_path))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_load_from_file")]
    pub fn load_from_file(
        &mut self,
        filename: impl AsRef<std::path::Path>,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_load_from_file(
                self.to_glib_none_mut().0,
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_move_item")]
    pub fn move_item(&mut self, old_uri: &str, new_uri: Option<&str>) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_move_item(
                self.to_glib_none_mut().0,
                old_uri.to_glib_none().0,
                new_uri.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_remove_application")]
    pub fn remove_application(&mut self, uri: &str, name: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_remove_application(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                name.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_remove_group")]
    pub fn remove_group(&mut self, uri: &str, group: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_remove_group(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                group.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_remove_item")]
    pub fn remove_item(&mut self, uri: &str) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_remove_item(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_set_added")]
    pub fn set_added(&mut self, uri: &str, added: libc::time_t) {
        unsafe {
            ffi::g_bookmark_file_set_added(self.to_glib_none_mut().0, uri.to_glib_none().0, added);
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_set_added_date_time")]
    pub fn set_added_date_time(&mut self, uri: &str, added: &DateTime) {
        unsafe {
            ffi::g_bookmark_file_set_added_date_time(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                added.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_set_app_info")]
    pub fn set_app_info(
        &mut self,
        uri: &str,
        name: &str,
        exec: &str,
        count: i32,
        stamp: libc::time_t,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_set_app_info(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                name.to_glib_none().0,
                exec.to_glib_none().0,
                count,
                stamp,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_set_application_info")]
    pub fn set_application_info(
        &mut self,
        uri: &str,
        name: &str,
        exec: &str,
        count: i32,
        stamp: Option<&DateTime>,
    ) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_set_application_info(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                name.to_glib_none().0,
                exec.to_glib_none().0,
                count,
                stamp.to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_set_description")]
    pub fn set_description(&mut self, uri: Option<&str>, description: &str) {
        unsafe {
            ffi::g_bookmark_file_set_description(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                description.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_set_groups")]
    pub fn set_groups(&mut self, uri: &str, groups: &[&str]) {
        let length = groups.len() as _;
        unsafe {
            ffi::g_bookmark_file_set_groups(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                groups.to_glib_none().0,
                length,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_set_icon")]
    pub fn set_icon(&mut self, uri: &str, href: Option<&str>, mime_type: &str) {
        unsafe {
            ffi::g_bookmark_file_set_icon(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                href.to_glib_none().0,
                mime_type.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_set_is_private")]
    pub fn set_is_private(&mut self, uri: &str, is_private: bool) {
        unsafe {
            ffi::g_bookmark_file_set_is_private(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                is_private.into_glib(),
            );
        }
    }

    #[doc(alias = "g_bookmark_file_set_mime_type")]
    pub fn set_mime_type(&mut self, uri: &str, mime_type: &str) {
        unsafe {
            ffi::g_bookmark_file_set_mime_type(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                mime_type.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_set_modified")]
    pub fn set_modified(&mut self, uri: &str, modified: libc::time_t) {
        unsafe {
            ffi::g_bookmark_file_set_modified(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                modified,
            );
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_set_modified_date_time")]
    pub fn set_modified_date_time(&mut self, uri: &str, modified: &DateTime) {
        unsafe {
            ffi::g_bookmark_file_set_modified_date_time(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                modified.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_set_title")]
    pub fn set_title(&mut self, uri: Option<&str>, title: &str) {
        unsafe {
            ffi::g_bookmark_file_set_title(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                title.to_glib_none().0,
            );
        }
    }

    #[cfg_attr(feature = "v2_66", deprecated = "Since 2.66")]
    #[allow(deprecated)]
    #[doc(alias = "g_bookmark_file_set_visited")]
    pub fn set_visited(&mut self, uri: &str, visited: libc::time_t) {
        unsafe {
            ffi::g_bookmark_file_set_visited(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                visited,
            );
        }
    }

    #[cfg(feature = "v2_66")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_66")))]
    #[doc(alias = "g_bookmark_file_set_visited_date_time")]
    pub fn set_visited_date_time(&mut self, uri: &str, visited: &DateTime) {
        unsafe {
            ffi::g_bookmark_file_set_visited_date_time(
                self.to_glib_none_mut().0,
                uri.to_glib_none().0,
                visited.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "g_bookmark_file_to_data")]
    pub fn to_data(&self) -> Result<Vec<u8>, crate::Error> {
        unsafe {
            let mut length = std::mem::MaybeUninit::uninit();
            let mut error = std::ptr::null_mut();
            let ret = ffi::g_bookmark_file_to_data(
                mut_override(self.to_glib_none().0),
                length.as_mut_ptr(),
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    ret,
                    length.assume_init() as _,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_bookmark_file_to_file")]
    pub fn to_file(&self, filename: impl AsRef<std::path::Path>) -> Result<(), crate::Error> {
        unsafe {
            let mut error = std::ptr::null_mut();
            let is_ok = ffi::g_bookmark_file_to_file(
                mut_override(self.to_glib_none().0),
                filename.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == crate::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl Default for BookmarkFile {
    fn default() -> Self {
        Self::new()
    }
}
