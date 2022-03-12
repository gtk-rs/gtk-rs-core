// Take a look at the license at the top of the repository in the LICENSE file.

use crate::traits::AsyncInitableExt;
use crate::AsyncInitable;
use crate::Cancellable;
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
        Q: FnOnce(Result<O, glib::Error>) + 'static,
    >(
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let obj = Object::new::<O>(properties).unwrap();
        unsafe {
            obj.init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| callback(res.map(|_| obj))),
            );
        }
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn new_future<O: Sized + IsClass + IsA<Object> + IsA<AsyncInitable>>(
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<O, glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(
            &Object::new::<O>(properties).unwrap(),
            move |obj, cancellable, send| unsafe {
                obj.init_async(
                    io_priority,
                    Some(cancellable),
                    glib::clone!(@strong obj => move |res| {
                        send.resolve(res.map(|_| obj));
                    }),
                );
            },
        ))
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_type<P: IsA<Cancellable>, Q: FnOnce(Result<Object, glib::Error>) + 'static>(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        assert!(type_.is_a(AsyncInitable::static_type()));
        let obj = Object::with_type(type_, properties).unwrap();
        unsafe {
            obj.unsafe_cast_ref::<Self>().init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| callback(res.map(|_| obj))),
            )
        };
    }

    #[doc(alias = "g_async_initable_new_async")]
    pub fn with_type_future(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Object, glib::Error>> + 'static>> {
        assert!(type_.is_a(AsyncInitable::static_type()));
        Box_::pin(crate::GioFuture::new(
            &Object::with_type(type_, properties).unwrap(),
            move |obj, cancellable, send| unsafe {
                obj.unsafe_cast_ref::<Self>().init_async(
                    io_priority,
                    Some(cancellable),
                    glib::clone!(@strong obj => move |res| {
                        send.resolve(res.map(|_| obj));
                    }),
                );
            },
        ))
    }
}
