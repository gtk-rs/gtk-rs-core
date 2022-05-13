// Take a look at the license at the top of the repository in the LICENSE file.

use crate::initable::InitableError;
use crate::traits::AsyncInitableExt;
use crate::AsyncInitable;
use crate::Cancellable;
use futures_util::future;
use glib::object::IsA;
use glib::object::IsClass;
use glib::value::ToValue;
use glib::{Cast, Object, StaticType, Type};
use std::boxed::Box as Box_;
use std::pin::Pin;

impl AsyncInitable {
    #[doc(alias = "g_async_initable_new_async")]
    pub fn new_async<
        O: Sized + IsClass + IsA<Object> + IsA<AsyncInitable>,
        P: IsA<Cancellable>,
        Q: FnOnce(Result<O, InitableError>) + 'static,
    >(
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let obj = match Object::new::<O>(properties) {
            Ok(obj) => obj,
            Err(e) => return callback(Err(e.into())),
        };
        unsafe {
            obj.init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| {
                    callback(res.map(|_| obj).map_err(|e| e.into()));
                }),
            );
        }
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn new_future<O: Sized + IsClass + IsA<Object> + IsA<AsyncInitable>>(
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<O, InitableError>> + 'static>> {
        let obj = match Object::new::<O>(properties) {
            Ok(obj) => obj,
            Err(e) => return Box::pin(future::ready(Err(e.into()))),
        };
        Box_::pin(crate::GioFuture::new(
            &obj,
            move |obj, cancellable, send| unsafe {
                obj.init_async(
                    io_priority,
                    Some(cancellable),
                    glib::clone!(@strong obj => move |res| {
                        send.resolve(res.map(|_| obj).map_err(|e| e.into()));
                    }),
                );
            },
        ))
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_type<P: IsA<Cancellable>, Q: FnOnce(Result<Object, InitableError>) + 'static>(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        if !type_.is_a(AsyncInitable::static_type()) {
            return callback(Err(InitableError::NewObjectFailed(glib::bool_error!(
                "Type '{}' is not async initable",
                type_
            ))));
        }
        let obj = match Object::with_type(type_, properties) {
            Ok(obj) => obj,
            Err(e) => return callback(Err(e.into())),
        };
        unsafe {
            obj.unsafe_cast_ref::<Self>().init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| {
                    callback(res.map(|_| obj).map_err(|e| e.into()));
                }),
            )
        };
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_type_future(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Object, InitableError>> + 'static>> {
        if !type_.is_a(AsyncInitable::static_type()) {
            return Box::pin(future::ready(Err(InitableError::NewObjectFailed(
                glib::bool_error!("Type '{}' is not async initable", type_),
            ))));
        }
        let obj = match Object::with_type(type_, properties) {
            Ok(obj) => obj,
            Err(e) => return Box::pin(future::ready(Err(e.into()))),
        };
        Box_::pin(crate::GioFuture::new(
            &obj,
            move |obj, cancellable, send| unsafe {
                obj.unsafe_cast_ref::<Self>().init_async(
                    io_priority,
                    Some(cancellable),
                    glib::clone!(@strong obj => move |res| {
                        send.resolve(res.map(|_| obj).map_err(|e| e.into()));
                    }),
                );
            },
        ))
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_values<P: IsA<Cancellable>, Q: FnOnce(Result<Object, InitableError>) + 'static>(
        type_: Type,
        properties: &[(&str, glib::Value)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        if !type_.is_a(AsyncInitable::static_type()) {
            return callback(Err(InitableError::NewObjectFailed(glib::bool_error!(
                "Type '{}' is not async initable",
                type_
            ))));
        }
        let obj = match Object::with_values(type_, properties) {
            Ok(obj) => obj,
            Err(e) => return callback(Err(e.into())),
        };
        unsafe {
            obj.unsafe_cast_ref::<Self>().init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| {
                    callback(res.map(|_| obj).map_err(|e| e.into()));
                }),
            )
        };
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_values_future(
        type_: Type,
        properties: &[(&str, glib::Value)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Object, InitableError>> + 'static>> {
        if !type_.is_a(AsyncInitable::static_type()) {
            return Box::pin(future::ready(Err(InitableError::NewObjectFailed(
                glib::bool_error!("Type '{}' is not async initable", type_),
            ))));
        }
        let obj = match Object::with_values(type_, properties) {
            Ok(obj) => obj,
            Err(e) => return Box::pin(future::ready(Err(e.into()))),
        };
        Box_::pin(crate::GioFuture::new(
            &obj,
            move |obj, cancellable, send| unsafe {
                obj.unsafe_cast_ref::<Self>().init_async(
                    io_priority,
                    Some(cancellable),
                    glib::clone!(@strong obj => move |res| {
                        send.resolve(res.map(|_| obj).map_err(|e| e.into()));
                    }),
                );
            },
        ))
    }
}
