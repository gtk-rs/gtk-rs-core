// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::Credentials;
use crate::DBusAuthObserver;
use crate::DBusCallFlags;
use crate::DBusCapabilityFlags;
use crate::DBusConnectionFlags;
use crate::DBusMessage;
use crate::DBusSendMessageFlags;
use crate::IOStream;
#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
use crate::UnixFDList;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct DBusConnection(Object<ffi::GDBusConnection>);

    match fn {
        get_type => || ffi::g_dbus_connection_get_type(),
    }
}

impl DBusConnection {
    #[doc(alias = "g_dbus_connection_new_for_address_sync")]
    pub fn new_for_address_sync<P: IsA<Cancellable>>(
        address: &str,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
        cancellable: Option<&P>,
    ) -> Result<DBusConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_new_for_address_sync(
                address.to_glib_none().0,
                flags.into_glib(),
                observer.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_new_sync")]
    pub fn new_sync<P: IsA<IOStream>, Q: IsA<Cancellable>>(
        stream: &P,
        guid: Option<&str>,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
        cancellable: Option<&Q>,
    ) -> Result<DBusConnection, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_new_sync(
                stream.as_ref().to_glib_none().0,
                guid.to_glib_none().0,
                flags.into_glib(),
                observer.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_call")]
    pub fn call<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<glib::Variant, glib::Error>) + Send + 'static,
    >(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn call_trampoline<
            Q: FnOnce(Result<glib::Variant, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_call_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = call_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_connection_call(
                self.to_glib_none().0,
                bus_name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                reply_type.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn call_future(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<glib::Variant, glib::Error>> + 'static>>
    {
        let bus_name = bus_name.map(ToOwned::to_owned);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        let reply_type = reply_type.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.call(
                bus_name.as_ref().map(::std::borrow::Borrow::borrow),
                &object_path,
                &interface_name,
                &method_name,
                parameters.as_ref().map(::std::borrow::Borrow::borrow),
                reply_type.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                timeout_msec,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_connection_call_sync")]
    pub fn call_sync<P: IsA<Cancellable>>(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
    ) -> Result<glib::Variant, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_call_sync(
                self.to_glib_none().0,
                bus_name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                reply_type.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_connection_call_with_unix_fd_list")]
    pub fn call_with_unix_fd_list<
        P: IsA<UnixFDList>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + Send + 'static,
    >(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn call_with_unix_fd_list_trampoline<
            R: FnOnce(Result<(glib::Variant, UnixFDList), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut out_fd_list = ptr::null_mut();
            let ret = ffi::g_dbus_connection_call_with_unix_fd_list_finish(
                _source_object as *mut _,
                &mut out_fd_list,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = call_with_unix_fd_list_trampoline::<R>;
        unsafe {
            ffi::g_dbus_connection_call_with_unix_fd_list(
                self.to_glib_none().0,
                bus_name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                reply_type.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    pub fn call_with_unix_fd_list_future<P: IsA<UnixFDList> + Clone + 'static>(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<(glib::Variant, UnixFDList), glib::Error>>
                + 'static,
        >,
    > {
        let bus_name = bus_name.map(ToOwned::to_owned);
        let object_path = String::from(object_path);
        let interface_name = String::from(interface_name);
        let method_name = String::from(method_name);
        let parameters = parameters.map(ToOwned::to_owned);
        let reply_type = reply_type.map(ToOwned::to_owned);
        let fd_list = fd_list.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.call_with_unix_fd_list(
                bus_name.as_ref().map(::std::borrow::Borrow::borrow),
                &object_path,
                &interface_name,
                &method_name,
                parameters.as_ref().map(::std::borrow::Borrow::borrow),
                reply_type.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                timeout_msec,
                fd_list.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "g_dbus_connection_call_with_unix_fd_list_sync")]
    pub fn call_with_unix_fd_list_sync<P: IsA<UnixFDList>, Q: IsA<Cancellable>>(
        &self,
        bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        method_name: &str,
        parameters: Option<&glib::Variant>,
        reply_type: Option<&glib::VariantTy>,
        flags: DBusCallFlags,
        timeout_msec: i32,
        fd_list: Option<&P>,
        cancellable: Option<&Q>,
    ) -> Result<(glib::Variant, UnixFDList), glib::Error> {
        unsafe {
            let mut out_fd_list = ptr::null_mut();
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_call_with_unix_fd_list_sync(
                self.to_glib_none().0,
                bus_name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                method_name.to_glib_none().0,
                parameters.to_glib_none().0,
                reply_type.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                fd_list.map(|p| p.as_ref()).to_glib_none().0,
                &mut out_fd_list,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(ret), from_glib_full(out_fd_list)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_close")]
    pub fn close<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn close_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_close_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = close_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_connection_close(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn close_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.close(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_connection_close_sync")]
    pub fn close_sync<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_close_sync(
                self.to_glib_none().0,
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

    #[doc(alias = "g_dbus_connection_emit_signal")]
    pub fn emit_signal(
        &self,
        destination_bus_name: Option<&str>,
        object_path: &str,
        interface_name: &str,
        signal_name: &str,
        parameters: Option<&glib::Variant>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_emit_signal(
                self.to_glib_none().0,
                destination_bus_name.to_glib_none().0,
                object_path.to_glib_none().0,
                interface_name.to_glib_none().0,
                signal_name.to_glib_none().0,
                parameters.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_flush")]
    pub fn flush<P: IsA<Cancellable>, Q: FnOnce(Result<(), glib::Error>) + Send + 'static>(
        &self,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn flush_trampoline<
            Q: FnOnce(Result<(), glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_flush_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = flush_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_connection_flush(
                self.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn flush_future(
        &self,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.flush(Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_connection_flush_sync")]
    pub fn flush_sync<P: IsA<Cancellable>>(
        &self,
        cancellable: Option<&P>,
    ) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_flush_sync(
                self.to_glib_none().0,
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

    #[doc(alias = "g_dbus_connection_get_capabilities")]
    pub fn get_capabilities(&self) -> DBusCapabilityFlags {
        unsafe {
            from_glib(ffi::g_dbus_connection_get_capabilities(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_connection_get_exit_on_close")]
    pub fn get_exit_on_close(&self) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_connection_get_exit_on_close(
                self.to_glib_none().0,
            ))
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_dbus_connection_get_flags")]
    pub fn get_flags(&self) -> DBusConnectionFlags {
        unsafe { from_glib(ffi::g_dbus_connection_get_flags(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_connection_get_guid")]
    pub fn get_guid(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::g_dbus_connection_get_guid(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_connection_get_last_serial")]
    pub fn get_last_serial(&self) -> u32 {
        unsafe { ffi::g_dbus_connection_get_last_serial(self.to_glib_none().0) }
    }

    #[doc(alias = "g_dbus_connection_get_peer_credentials")]
    pub fn get_peer_credentials(&self) -> Option<Credentials> {
        unsafe {
            from_glib_none(ffi::g_dbus_connection_get_peer_credentials(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_connection_get_stream")]
    pub fn get_stream(&self) -> IOStream {
        unsafe { from_glib_none(ffi::g_dbus_connection_get_stream(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_dbus_connection_get_unique_name")]
    pub fn get_unique_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_dbus_connection_get_unique_name(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_dbus_connection_is_closed")]
    pub fn is_closed(&self) -> bool {
        unsafe { from_glib(ffi::g_dbus_connection_is_closed(self.to_glib_none().0)) }
    }

    //#[doc(alias = "g_dbus_connection_register_object")]
    //pub fn register_object(&self, object_path: &str, interface_info: &DBusInterfaceInfo, vtable: /*Ignored*/Option<&DBusInterfaceVTable>, user_data: /*Unimplemented*/Option<Fundamental: Pointer>) -> Result<(), glib::Error> {
    //    unsafe { TODO: call ffi:g_dbus_connection_register_object() }
    //}

    #[doc(alias = "g_dbus_connection_send_message")]
    pub fn send_message(
        &self,
        message: &DBusMessage,
        flags: DBusSendMessageFlags,
    ) -> Result<u32, glib::Error> {
        unsafe {
            let mut out_serial = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_connection_send_message(
                self.to_glib_none().0,
                message.to_glib_none().0,
                flags.into_glib(),
                out_serial.as_mut_ptr(),
                &mut error,
            );
            let out_serial = out_serial.assume_init();
            if error.is_null() {
                Ok(out_serial)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_send_message_with_reply")]
    pub fn send_message_with_reply<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<DBusMessage, glib::Error>) + Send + 'static,
    >(
        &self,
        message: &DBusMessage,
        flags: DBusSendMessageFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
        callback: Q,
    ) -> u32 {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn send_message_with_reply_trampoline<
            Q: FnOnce(Result<DBusMessage, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_send_message_with_reply_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = send_message_with_reply_trampoline::<Q>;
        unsafe {
            let mut out_serial = mem::MaybeUninit::uninit();
            ffi::g_dbus_connection_send_message_with_reply(
                self.to_glib_none().0,
                message.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                out_serial.as_mut_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
            let out_serial = out_serial.assume_init();
            out_serial
        }
    }

    pub fn send_message_with_reply_future(
        &self,
        message: &DBusMessage,
        flags: DBusSendMessageFlags,
        timeout_msec: i32,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusMessage, glib::Error>> + 'static>>
    {
        let message = message.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.send_message_with_reply(
                &message,
                flags,
                timeout_msec,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_connection_send_message_with_reply_sync")]
    pub fn send_message_with_reply_sync<P: IsA<Cancellable>>(
        &self,
        message: &DBusMessage,
        flags: DBusSendMessageFlags,
        timeout_msec: i32,
        cancellable: Option<&P>,
    ) -> Result<(DBusMessage, u32), glib::Error> {
        unsafe {
            let mut out_serial = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_send_message_with_reply_sync(
                self.to_glib_none().0,
                message.to_glib_none().0,
                flags.into_glib(),
                timeout_msec,
                out_serial.as_mut_ptr(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            let out_serial = out_serial.assume_init();
            if error.is_null() {
                Ok((from_glib_full(ret), out_serial))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_dbus_connection_set_exit_on_close")]
    pub fn set_exit_on_close(&self, exit_on_close: bool) {
        unsafe {
            ffi::g_dbus_connection_set_exit_on_close(
                self.to_glib_none().0,
                exit_on_close.into_glib(),
            );
        }
    }

    #[doc(alias = "g_dbus_connection_start_message_processing")]
    pub fn start_message_processing(&self) {
        unsafe {
            ffi::g_dbus_connection_start_message_processing(self.to_glib_none().0);
        }
    }

    pub fn get_property_closed(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"closed\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `closed` getter")
                .unwrap()
        }
    }

    pub fn get_property_flags(&self) -> DBusConnectionFlags {
        unsafe {
            let mut value =
                glib::Value::from_type(<DBusConnectionFlags as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.as_ptr() as *mut glib::gobject_ffi::GObject,
                b"flags\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `flags` getter")
                .unwrap()
        }
    }

    #[doc(alias = "g_dbus_connection_new")]
    pub fn new<
        P: IsA<IOStream>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
    >(
        stream: &P,
        guid: Option<&str>,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn new_trampoline<
            R: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_new_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_trampoline::<R>;
        unsafe {
            ffi::g_dbus_connection_new(
                stream.as_ref().to_glib_none().0,
                guid.to_glib_none().0,
                flags.into_glib(),
                observer.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_future<P: IsA<IOStream> + Clone + 'static>(
        stream: &P,
        guid: Option<&str>,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusConnection, glib::Error>> + 'static>>
    {
        let stream = stream.clone();
        let guid = guid.map(ToOwned::to_owned);
        let observer = observer.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
            let cancellable = Cancellable::new();
            Self::new(
                &stream,
                guid.as_ref().map(::std::borrow::Borrow::borrow),
                flags,
                observer.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[doc(alias = "g_dbus_connection_new_for_address")]
    pub fn new_for_address<
        P: IsA<Cancellable>,
        Q: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
    >(
        address: &str,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let user_data: Box_<Q> = Box_::new(callback);
        unsafe extern "C" fn new_for_address_trampoline<
            Q: FnOnce(Result<DBusConnection, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_connection_new_for_address_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<Q> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = new_for_address_trampoline::<Q>;
        unsafe {
            ffi::g_dbus_connection_new_for_address(
                address.to_glib_none().0,
                flags.into_glib(),
                observer.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn new_for_address_future(
        address: &str,
        flags: DBusConnectionFlags,
        observer: Option<&DBusAuthObserver>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<DBusConnection, glib::Error>> + 'static>>
    {
        let address = String::from(address);
        let observer = observer.map(ToOwned::to_owned);
        Box_::pin(crate::GioFuture::new(&(), move |_obj, send| {
            let cancellable = Cancellable::new();
            Self::new_for_address(
                &address,
                flags,
                observer.as_ref().map(::std::borrow::Borrow::borrow),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    pub fn connect_closed<
        F: Fn(&DBusConnection, bool, Option<&glib::Error>) + Send + Sync + 'static,
    >(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<
            F: Fn(&DBusConnection, bool, Option<&glib::Error>) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusConnection,
            remote_peer_vanished: glib::ffi::gboolean,
            error: *mut glib::ffi::GError,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                &from_glib_borrow(this),
                from_glib(remote_peer_vanished),
                Option::<glib::Error>::from_glib_borrow(error)
                    .as_ref()
                    .as_ref(),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_capabilities_notify<F: Fn(&DBusConnection) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_capabilities_trampoline<
            F: Fn(&DBusConnection) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::capabilities\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_capabilities_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_closed_notify<F: Fn(&DBusConnection) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_closed_trampoline<
            F: Fn(&DBusConnection) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::closed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_closed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_exit_on_close_notify<F: Fn(&DBusConnection) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_exit_on_close_trampoline<
            F: Fn(&DBusConnection) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::exit-on-close\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_exit_on_close_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_unique_name_notify<F: Fn(&DBusConnection) + Send + Sync + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_unique_name_trampoline<
            F: Fn(&DBusConnection) + Send + Sync + 'static,
        >(
            this: *mut ffi::GDBusConnection,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::unique-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_unique_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

unsafe impl Send for DBusConnection {}
unsafe impl Sync for DBusConnection {}

impl fmt::Display for DBusConnection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DBusConnection")
    }
}
