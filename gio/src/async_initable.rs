// Take a look at the license at the top of the repository in the LICENSE file.

use crate::traits::AsyncInitableExt;
use crate::AsyncInitable;
use crate::Cancellable;

use glib::object::IsA;
use glib::object::IsClass;
use glib::value::ToValue;
use glib::{Cast, Object, StaticType, Type};

use futures_util::TryFutureExt;
use std::boxed::Box as Box_;
use std::pin::Pin;

impl AsyncInitable {
    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
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
        Self::with_type(
            O::static_type(),
            properties,
            io_priority,
            cancellable,
            move |res| callback(res.map(|o| unsafe { o.unsafe_cast() })),
        )
    }

    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
    pub fn new_future<O: Sized + IsClass + IsA<Object> + IsA<AsyncInitable>>(
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<O, glib::Error>> + 'static>> {
        Box::pin(
            Self::with_type_future(O::static_type(), properties, io_priority)
                .map_ok(|o| unsafe { o.unsafe_cast() }),
        )
    }

    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
    pub fn with_type<P: IsA<Cancellable>, Q: FnOnce(Result<Object, glib::Error>) + 'static>(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        if !type_.is_a(AsyncInitable::static_type()) {
            panic!("Type '{type_}' is not async initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.to_value()));
        }

        unsafe {
            let obj = Object::new_internal(type_, &mut property_values);
            obj.unsafe_cast_ref::<Self>().init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| {
                    callback(res.map(|_| obj));
                }),
            )
        };
    }

    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
    pub fn with_type_future(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Object, glib::Error>> + 'static>> {
        if !type_.is_a(AsyncInitable::static_type()) {
            panic!("Type '{type_}' is not async initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.to_value()));
        }

        unsafe {
            // FIXME: object construction should ideally happen as part of the future
            let obj = Object::new_internal(type_, &mut property_values);
            Box_::pin(crate::GioFuture::new(
                &obj,
                move |obj, cancellable, send| {
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

    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
    pub fn with_values<P: IsA<Cancellable>, Q: FnOnce(Result<Object, glib::Error>) + 'static>(
        type_: Type,
        properties: &[(&str, glib::Value)],
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        if !type_.is_a(AsyncInitable::static_type()) {
            panic!("Type '{type_}' is not async initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.clone()));
        }

        unsafe {
            let obj = Object::new_internal(type_, &mut property_values);
            obj.unsafe_cast_ref::<Self>().init_async(
                io_priority,
                cancellable,
                glib::clone!(@strong obj => move |res| {
                    callback(res.map(|_| obj));
                }),
            )
        };
    }

    #[doc(alias = "g_async_initable_new_async")]
    #[track_caller]
    pub fn with_values_future(
        type_: Type,
        properties: &[(&str, glib::Value)],
        io_priority: glib::Priority,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<Object, glib::Error>> + 'static>> {
        if !type_.is_a(AsyncInitable::static_type()) {
            panic!("Type '{type_}' is not async initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.clone()));
        }

        unsafe {
            // FIXME: object construction should ideally happen as part of the future
            let obj = Object::new_internal(type_, &mut property_values);
            Box_::pin(crate::GioFuture::new(
                &obj,
                move |obj, cancellable, send| {
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
}
