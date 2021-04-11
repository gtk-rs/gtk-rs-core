// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::InputStream;
use crate::OutputStreamSpliceFlags;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v2_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_44")))]
use std::mem;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct OutputStream(Object<ffi::GOutputStream, ffi::GOutputStreamClass>);

    match fn {
        get_type => || ffi::g_output_stream_get_type(),
    }
}

pub const NONE_OUTPUT_STREAM: Option<&OutputStream> = None;

pub trait OutputStreamExt: 'static {
    #[doc(alias = "g_output_stream_clear_pending")]
    fn clear_pending(&self);

    #[doc(alias = "g_output_stream_close")]
    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_close_async")]
    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_output_stream_flush")]
    fn flush<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_flush_async")]
    fn flush_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn flush_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>>;

    #[doc(alias = "g_output_stream_has_pending")]
    fn has_pending(&self) -> bool;

    #[doc(alias = "g_output_stream_is_closed")]
    fn is_closed(&self) -> bool;

    #[doc(alias = "g_output_stream_is_closing")]
    fn is_closing(&self) -> bool;

    //#[doc(alias = "g_output_stream_printf")]
    //fn printf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut glib::Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize>;

    #[doc(alias = "g_output_stream_set_pending")]
    fn set_pending(&self) -> Result<(), glib::Error>;

    #[doc(alias = "g_output_stream_splice")]
    fn splice<P: IsA<InputStream>, Q: IsA<Cancellable>>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Q>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_splice_async")]
    fn splice_async<
        P: IsA<InputStream>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<isize, glib::Error>) + Send + 'static,
    >(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn splice_async_future<P: IsA<InputStream> + Clone + 'static>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;

    //#[doc(alias = "g_output_stream_vprintf")]
    //fn vprintf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut glib::Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize>;

    #[doc(alias = "g_output_stream_write")]
    fn write<P: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_write_bytes")]
    fn write_bytes<P: IsA<Cancellable>>(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error>;

    #[doc(alias = "g_output_stream_write_bytes_async")]
    fn write_bytes_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<isize, glib::Error>) + Send + 'static,
    >(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    );

    fn write_bytes_async_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev")]
    //fn writev<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, glib::Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_all")]
    //fn writev_all<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, glib::Error>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_all_async")]
    //fn writev_all_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, glib::Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>>;

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //#[doc(alias = "g_output_stream_writev_async")]
    //fn writev_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, glib::Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q);

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>>;
}

impl<O: IsA<OutputStream>> OutputStreamExt for O {
    fn clear_pending(&self) {
        unsafe {
            ffi::g_output_stream_clear_pending(self.as_ref().to_glib_none().0);
        }
    }

    fn close<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_close(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn close_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn close_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_async_trampoline::<Q>;
        unsafe {
            ffi::g_output_stream_close_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn close_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.close_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn flush<P: IsA<Cancellable>>(&self, cancellable: Option<&P>) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_flush(
                self.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn flush_async<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn flush_async_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_flush_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = flush_async_trampoline::<Q>;
        unsafe {
            ffi::g_output_stream_flush_async(
                self.as_ref().to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn flush_async_future(
        &self,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.flush_async(io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn has_pending(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_has_pending(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closed(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn is_closing(&self) -> bool {
        unsafe {
            from_glib(ffi::g_output_stream_is_closing(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn printf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut glib::Error, format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<usize> {
    //    unsafe { TODO: call ffi:g_output_stream_printf() }
    //}

    fn set_pending(&self) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_output_stream_set_pending(self.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice<P: IsA<InputStream>, Q: IsA<Cancellable>>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        cancellable: Option<&Q>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_splice(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn splice_async<
        P: IsA<InputStream>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<isize, glib::Error>) + Send + 'static,
    >(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn splice_async_trampoline<
            R: FnOnce(Result<isize, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_splice_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = splice_async_trampoline::<R>;
        unsafe {
            ffi::g_output_stream_splice_async(
                self.as_ref().to_glib_none().0,
                source.as_ref().to_glib_none().0,
                flags.into_glib(),
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn splice_async_future<P: IsA<InputStream> + Clone + 'static>(
        &self,
        source: &P,
        flags: OutputStreamSpliceFlags,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        let source = source.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.splice_async(
                &source,
                flags,
                io_priority,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    //fn vprintf<P: IsA<Cancellable>>(&self, cancellable: Option<&P>, error: &mut glib::Error, format: &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<usize> {
    //    unsafe { TODO: call ffi:g_output_stream_vprintf() }
    //}

    fn write<P: IsA<Cancellable>>(
        &self,
        buffer: &[u8],
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error> {
        let count = buffer.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write(
                self.as_ref().to_glib_none().0,
                buffer.to_glib_none().0,
                count,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes<P: IsA<Cancellable>>(
        &self,
        bytes: &glib::Bytes,
        cancellable: Option<&P>,
    ) -> Result<isize, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_output_stream_write_bytes(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn write_bytes_async<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<isize, glib::Error>) + Send + 'static,
    >(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn write_bytes_async_trampoline<
            Q: FnOnce(Result<isize, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_output_stream_write_bytes_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(ret)
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = write_bytes_async_trampoline::<Q>;
        unsafe {
            ffi::g_output_stream_write_bytes_async(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                io_priority.into_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn write_bytes_async_future(
        &self,
        bytes: &glib::Bytes,
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<isize, glib::Error>> + 'static>> {
        let bytes = bytes.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.write_bytes_async(&bytes, io_priority, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, glib::Error> {
    //    unsafe { TODO: call ffi:g_output_stream_writev() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all<P: IsA<Cancellable>>(&self, vectors: /*Ignored*/&[&OutputVector], cancellable: Option<&P>) -> Result<usize, glib::Error> {
    //    unsafe { TODO: call ffi:g_output_stream_writev_all() }
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, glib::Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ffi:g_output_stream_writev_all_async() }
    //}

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_all_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>> {

    //let vectors = vectors.clone();
    //Box_::pin(crate::GioFuture::new(self, move |obj, send| {
    //    let cancellable = Cancellable::new();
    //    obj.writev_all_async(
    //        &vectors,
    //        io_priority,
    //        Some(&cancellable),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );

    //    cancellable
    //}))
    //}

    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_async<P: IsA<Cancellable>, Q: FnOnce(Result<usize, glib::Error>) + Send + 'static>(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority, cancellable: Option<&P>, callback: Q) {
    //    unsafe { TODO: call ffi:g_output_stream_writev_async() }
    //}

    //
    //#[cfg(any(feature = "v2_60", feature = "dox"))]
    //#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    //fn writev_async_future(&self, vectors: /*Ignored*/&[&OutputVector], io_priority: glib::Priority) -> Pin<Box_<dyn std::future::Future<Output = Result<usize, glib::Error>> + 'static>> {

    //let vectors = vectors.clone();
    //Box_::pin(crate::GioFuture::new(self, move |obj, send| {
    //    let cancellable = Cancellable::new();
    //    obj.writev_async(
    //        &vectors,
    //        io_priority,
    //        Some(&cancellable),
    //        move |res| {
    //            send.resolve(res);
    //        },
    //    );

    //    cancellable
    //}))
    //}
}

impl fmt::Display for OutputStream {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("OutputStream")
    }
}
