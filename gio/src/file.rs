// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Cancellable;
use crate::File;
use crate::FileCreateFlags;
use crate::FileEnumerator;
use crate::FileQueryInfoFlags;
use glib::object::IsA;
use glib::translate::*;
use std::cell::RefCell;
use std::pin::Pin;
use std::ptr;

pub trait FileExtManual: Sized {
    #[doc(alias = "g_file_replace_contents_async")]
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    );

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    >;

    #[doc(alias = "g_file_enumerate_children_async")]
    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>;

    #[doc(alias = "g_file_copy_async")]
    fn copy_async<
        P: FnMut(i64, i64) + Send + 'static,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        destination: &impl IsA<File>,
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: P,
        callback: Q,
    );

    fn copy_async_future(
        &self,
        destination: &(impl IsA<File> + Clone + 'static),
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (i64, i64)> + 'static>>,
    );

    #[doc(alias = "g_file_load_partial_contents_async")]
    fn load_partial_contents_async<
        P: FnMut(&[u8]) -> bool + Send + 'static,
        Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        read_more_callback: P,
        callback: Q,
    );
}

impl<O: IsA<File>> FileExtManual for O {
    fn replace_contents_async<
        B: AsRef<[u8]> + Send + 'static,
        R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        C: IsA<Cancellable>,
    >(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
        cancellable: Option<&C>,
        callback: R,
    ) {
        let etag = etag.to_glib_none();
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let user_data: Box<Option<(R, B)>> = Box::new(Some((callback, contents)));
        // Need to do this after boxing as the contents pointer might change by moving into the box
        let (count, contents_ptr) = {
            let contents = &(*user_data).as_ref().unwrap().1;
            let slice = contents.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe extern "C" fn replace_contents_async_trampoline<
            B: AsRef<[u8]> + Send + 'static,
            R: FnOnce(Result<(B, glib::GString), (B, glib::Error)>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut user_data: Box<Option<(R, B)>> = Box::from_raw(user_data as *mut _);
            let (callback, contents) = user_data.take().unwrap();

            let mut error = ptr::null_mut();
            let mut new_etag = ptr::null_mut();
            let _ = ffi::g_file_replace_contents_finish(
                _source_object as *mut _,
                res,
                &mut new_etag,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((contents, from_glib_full(new_etag)))
            } else {
                Err((contents, from_glib_full(error)))
            };
            callback(result);
        }
        let callback = replace_contents_async_trampoline::<B, R>;
        unsafe {
            ffi::g_file_replace_contents_async(
                self.as_ref().to_glib_none().0,
                mut_override(contents_ptr),
                count,
                etag.0,
                make_backup.into_glib(),
                flags.into_glib(),
                gcancellable.0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn replace_contents_async_future<B: AsRef<[u8]> + Send + 'static>(
        &self,
        contents: B,
        etag: Option<&str>,
        make_backup: bool,
        flags: FileCreateFlags,
    ) -> Pin<
        Box<
            dyn std::future::Future<Output = Result<(B, glib::GString), (B, glib::Error)>>
                + 'static,
        >,
    > {
        let etag = etag.map(glib::GString::from);
        Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.replace_contents_async(
                    contents,
                    etag.as_ref().map(|s| s.as_str()),
                    make_backup,
                    flags,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn enumerate_children_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
    >(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn create_async_trampoline<
            Q: FnOnce(Result<FileEnumerator, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_file_enumerate_children_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = create_async_trampoline::<Q>;
        unsafe {
            ffi::g_file_enumerate_children_async(
                self.as_ref().to_glib_none().0,
                attributes.to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn enumerate_children_async_future(
        &self,
        attributes: &'static str,
        flags: FileQueryInfoFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = Result<FileEnumerator, glib::Error>> + 'static>>
    {
        Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.enumerate_children_async(
                    attributes,
                    flags,
                    io_priority,
                    Some(cancellable),
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ))
    }

    fn copy_async<
        P: FnMut(i64, i64) + Send + 'static,
        Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
    >(
        &self,
        destination: &impl IsA<File>,
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
        cancellable: Option<&impl IsA<Cancellable>>,
        progress_callback: P,
        callback: Q,
    ) {
        let user_data: Box<(Q, RefCell<P>)> = Box::new((callback, RefCell::new(progress_callback)));
        unsafe extern "C" fn copy_async_trampoline<
            P: FnMut(i64, i64) + Send + 'static,
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            ffi::g_file_copy_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(Q, RefCell<P>)> = Box::from_raw(user_data as *mut _);
            callback.0(result);
        }
        unsafe extern "C" fn copy_async_progress_trampoline<
            P: FnMut(i64, i64) + Send + 'static,
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            current_num_bytes: i64,
            total_num_bytes: i64,
            user_data: glib::ffi::gpointer,
        ) {
            let callback: &(Q, RefCell<P>) = &*(user_data as *const _);
            (&mut *callback.1.borrow_mut())(current_num_bytes, total_num_bytes);
        }

        let user_data = Box::into_raw(user_data) as *mut _;

        unsafe {
            ffi::g_file_copy_async(
                self.as_ref().to_glib_none().0,
                destination.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(copy_async_progress_trampoline::<P, Q>),
                user_data,
                Some(copy_async_trampoline::<P, Q>),
                user_data,
            );
        }
    }

    fn copy_async_future(
        &self,
        destination: &(impl IsA<File> + Clone + 'static),
        flags: crate::FileCopyFlags,
        io_priority: glib::Priority,
    ) -> (
        Pin<Box<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>,
        Pin<Box<dyn futures_core::stream::Stream<Item = (i64, i64)> + 'static>>,
    ) {
        let destination = destination.clone();

        let (sender, receiver) = futures_channel::mpsc::unbounded();

        let fut = Box::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.copy_async(
                    &destination,
                    flags,
                    io_priority,
                    Some(cancellable),
                    move |current_num_bytes, total_num_bytes| {
                        let _ = sender.unbounded_send((current_num_bytes, total_num_bytes));
                    },
                    move |res| {
                        send.resolve(res);
                    },
                );
            },
        ));

        (fut, Box::pin(receiver))
    }

    fn load_partial_contents_async<
        P: FnMut(&[u8]) -> bool + Send + 'static,
        Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
    >(
        &self,
        cancellable: Option<&impl IsA<Cancellable>>,
        read_more_callback: P,
        callback: Q,
    ) {
        let user_data: Box<(Q, RefCell<P>)> =
            Box::new((callback, RefCell::new(read_more_callback)));
        unsafe extern "C" fn load_partial_contents_async_trampoline<
            P: FnMut(&[u8]) -> bool + Send + 'static,
            Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut contents = ptr::null_mut();
            let mut length = 0;
            let mut etag_out = ptr::null_mut();
            let mut error = ptr::null_mut();
            ffi::g_file_load_partial_contents_finish(
                _source_object as *mut _,
                res,
                &mut contents,
                &mut length,
                &mut etag_out,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((
                    FromGlibContainer::from_glib_full_num(contents, length as usize),
                    from_glib_full(etag_out),
                ))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<(Q, RefCell<P>)> = Box::from_raw(user_data as *mut _);
            callback.0(result);
        }
        unsafe extern "C" fn load_partial_contents_async_read_more_trampoline<
            P: FnMut(&[u8]) -> bool + Send + 'static,
            Q: FnOnce(Result<(Vec<u8>, Option<glib::GString>), glib::Error>) + Send + 'static,
        >(
            file_contents: *const libc::c_char,
            file_size: i64,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            use std::slice;

            let callback: &(Q, RefCell<P>) = &*(user_data as *const _);
            (&mut *callback.1.borrow_mut())(slice::from_raw_parts(
                file_contents as *const u8,
                file_size as usize,
            ))
            .into_glib()
        }

        let user_data = Box::into_raw(user_data) as *mut _;

        unsafe {
            ffi::g_file_load_partial_contents_async(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(load_partial_contents_async_read_more_trampoline::<P, Q>),
                Some(load_partial_contents_async_trampoline::<P, Q>),
                user_data,
            );
        }
    }
}
