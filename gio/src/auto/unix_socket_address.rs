// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{SocketAddress, SocketConnectable, UnixSocketAddressType};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUnixSocketAddress")]
    pub struct UnixSocketAddress(Object<ffi::GUnixSocketAddress, ffi::GUnixSocketAddressClass>) @extends SocketAddress, @implements SocketConnectable;

    match fn {
        type_ => || ffi::g_unix_socket_address_get_type(),
    }
}

impl UnixSocketAddress {
    pub const NONE: Option<&'static UnixSocketAddress> = None;

    //#[doc(alias = "g_unix_socket_address_new_abstract")]
    //pub fn new_abstract(path: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 10 }) -> UnixSocketAddress {
    //    unsafe { TODO: call ffi:g_unix_socket_address_new_abstract() }
    //}

    //#[doc(alias = "g_unix_socket_address_new_with_type")]
    //#[doc(alias = "new_with_type")]
    //pub fn with_type(path: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 10 }, type_: UnixSocketAddressType) -> UnixSocketAddress {
    //    unsafe { TODO: call ffi:g_unix_socket_address_new_with_type() }
    //}

    #[doc(alias = "g_unix_socket_address_abstract_names_supported")]
    pub fn abstract_names_supported() -> bool {
        unsafe { from_glib(ffi::g_unix_socket_address_abstract_names_supported()) }
    }
}

unsafe impl Send for UnixSocketAddress {}
unsafe impl Sync for UnixSocketAddress {}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::UnixSocketAddress>> Sealed for T {}
}

pub trait UnixSocketAddressExt: IsA<UnixSocketAddress> + sealed::Sealed + 'static {
    #[doc(alias = "g_unix_socket_address_get_address_type")]
    #[doc(alias = "get_address_type")]
    fn address_type(&self) -> UnixSocketAddressType {
        unsafe {
            from_glib(ffi::g_unix_socket_address_get_address_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_unix_socket_address_get_is_abstract")]
    #[doc(alias = "get_is_abstract")]
    fn is_abstract(&self) -> bool {
        unsafe {
            from_glib(ffi::g_unix_socket_address_get_is_abstract(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_unix_socket_address_get_path_len")]
    #[doc(alias = "get_path_len")]
    fn path_len(&self) -> usize {
        unsafe { ffi::g_unix_socket_address_get_path_len(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "path-as-array")]
    fn path_as_array(&self) -> Option<glib::ByteArray> {
        ObjectExt::property(self.as_ref(), "path-as-array")
    }
}

impl<O: IsA<UnixSocketAddress>> UnixSocketAddressExt for O {}

impl fmt::Display for UnixSocketAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("UnixSocketAddress")
    }
}
