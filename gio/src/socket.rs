// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(all(not(unix), feature = "dox"))]
use std::os::raw::c_int;
#[cfg(all(not(windows), feature = "dox"))]
use std::os::raw::c_void;
#[cfg(unix)]
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, FromRawSocket, IntoRawSocket, RawSocket};
#[cfg(any(feature = "v2_60", feature = "dox"))]
use std::time::Duration;
use std::{cell::RefCell, marker::PhantomData, mem::transmute, pin::Pin, ptr};

use futures_core::stream::Stream;
use glib::{prelude::*, translate::*, Slice};

#[cfg(any(feature = "v2_60", feature = "dox"))]
use crate::PollableReturn;
use crate::{Cancellable, Socket, SocketAddress, SocketControlMessage};

impl Socket {
    #[cfg(any(unix, feature = "dox"))]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_fd(fd: impl IntoRawFd) -> Result<Socket, glib::Error> {
        let fd = fd.into_raw_fd();
        let mut error = ptr::null_mut();
        let ret = ffi::g_socket_new_from_fd(fd, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
    #[cfg(any(windows, feature = "dox"))]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn from_socket(socket: impl IntoRawSocket) -> Result<Socket, glib::Error> {
        let socket = socket.into_raw_socket();
        let mut error = ptr::null_mut();
        let ret = ffi::g_socket_new_from_fd(socket as i32, &mut error);
        if error.is_null() {
            Ok(from_glib_full(ret))
        } else {
            Err(from_glib_full(error))
        }
    }
}

#[cfg(any(unix, feature = "dox"))]
impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        unsafe { ffi::g_socket_get_fd(self.to_glib_none().0) as _ }
    }
}

#[cfg(any(windows, feature = "dox"))]
impl AsRawSocket for Socket {
    fn as_raw_socket(&self) -> RawSocket {
        unsafe { ffi::g_socket_get_fd(self.to_glib_none().0) as _ }
    }
}

#[doc(alias = "GInputVector")]
#[repr(transparent)]
#[derive(Debug)]
pub struct InputVector<'v> {
    vector: ffi::GInputVector,
    buffer: PhantomData<&'v mut [u8]>,
}

impl<'v> InputVector<'v> {
    #[inline]
    pub fn new(buffer: &'v mut [u8]) -> Self {
        Self {
            vector: ffi::GInputVector {
                buffer: buffer.as_mut_ptr() as *mut _,
                size: buffer.len(),
            },
            buffer: PhantomData,
        }
    }
}

unsafe impl<'v> Send for InputVector<'v> {}
unsafe impl<'v> Sync for InputVector<'v> {}

impl<'v> std::ops::Deref for InputVector<'v> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.vector.buffer as *const _, self.vector.size) }
    }
}

impl<'v> std::ops::DerefMut for InputVector<'v> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::slice::from_raw_parts_mut(self.vector.buffer as *mut _, self.vector.size) }
    }
}

#[derive(Debug)]
pub struct SocketControlMessages {
    ptr: *mut *mut ffi::GSocketControlMessage,
    len: u32,
}

impl SocketControlMessages {
    #[inline]
    pub const fn new() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
        }
    }
}

impl AsRef<[SocketControlMessage]> for SocketControlMessages {
    #[inline]
    fn as_ref(&self) -> &[SocketControlMessage] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const _, self.len as usize) }
    }
}

impl std::ops::Deref for SocketControlMessages {
    type Target = [SocketControlMessage];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl Default for SocketControlMessages {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SocketControlMessages {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            let _: Slice<SocketControlMessage> =
                Slice::from_glib_full_num(self.ptr as *mut _, self.len as usize);
        }
    }
}

#[doc(alias = "GInputMessage")]
#[repr(transparent)]
#[derive(Debug)]
pub struct InputMessage<'m> {
    message: ffi::GInputMessage,
    address: PhantomData<Option<&'m mut Option<SocketAddress>>>,
    vectors: PhantomData<&'m mut [InputVector<'m>]>,
    control_messages: PhantomData<Option<&'m mut SocketControlMessages>>,
}

impl<'m> InputMessage<'m> {
    pub fn new(
        mut address: Option<&'m mut Option<SocketAddress>>,
        vectors: &'m mut [InputVector<'m>],
        control_messages: Option<&'m mut SocketControlMessages>,
    ) -> Self {
        let address = address
            .as_mut()
            .map(|a| {
                assert!(a.is_none());
                *a as *mut _ as *mut _
            })
            .unwrap_or_else(ptr::null_mut);
        let (control_messages, num_control_messages) = control_messages
            .map(|c| (&mut c.ptr as *mut _, &mut c.len as *mut _))
            .unwrap_or_else(|| (ptr::null_mut(), ptr::null_mut()));
        Self {
            message: ffi::GInputMessage {
                address,
                vectors: vectors.as_mut_ptr() as *mut ffi::GInputVector,
                num_vectors: vectors.len().try_into().unwrap(),
                bytes_received: 0,
                flags: 0,
                control_messages,
                num_control_messages,
            },
            address: PhantomData,
            vectors: PhantomData,
            control_messages: PhantomData,
        }
    }
    #[inline]
    pub fn vectors(&mut self) -> &'m mut [InputVector<'m>] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.message.vectors as *mut _,
                self.message.num_vectors as usize,
            )
        }
    }
    #[inline]
    pub const fn flags(&self) -> i32 {
        self.message.flags
    }
    #[inline]
    pub const fn bytes_received(&self) -> usize {
        self.message.bytes_received
    }
}

#[doc(alias = "GOutputVector")]
#[repr(transparent)]
#[derive(Debug)]
pub struct OutputVector<'v> {
    vector: ffi::GOutputVector,
    buffer: PhantomData<&'v [u8]>,
}

impl<'v> OutputVector<'v> {
    #[inline]
    pub const fn new(buffer: &'v [u8]) -> Self {
        Self {
            vector: ffi::GOutputVector {
                buffer: buffer.as_ptr() as *const _,
                size: buffer.len(),
            },
            buffer: PhantomData,
        }
    }
}

unsafe impl<'v> Send for OutputVector<'v> {}
unsafe impl<'v> Sync for OutputVector<'v> {}

impl<'v> std::ops::Deref for OutputVector<'v> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.vector.buffer as *const _, self.vector.size) }
    }
}

#[doc(alias = "GOutputMessage")]
#[repr(transparent)]
#[derive(Debug)]
pub struct OutputMessage<'m> {
    message: ffi::GOutputMessage,
    address: PhantomData<Option<&'m SocketAddress>>,
    vectors: PhantomData<&'m [OutputVector<'m>]>,
    control_messages: PhantomData<&'m [SocketControlMessage]>,
}

impl<'m> OutputMessage<'m> {
    pub fn new<A: IsA<SocketAddress>>(
        address: Option<&'m A>,
        vectors: &'m [OutputVector<'m>],
        control_messages: &'m [SocketControlMessage],
    ) -> Self {
        Self {
            message: ffi::GOutputMessage {
                address: address
                    .map(|a| a.upcast_ref::<SocketAddress>().as_ptr())
                    .unwrap_or_else(ptr::null_mut),
                vectors: mut_override(vectors.as_ptr() as *const ffi::GOutputVector),
                num_vectors: vectors.len().try_into().unwrap(),
                bytes_sent: 0,
                control_messages: control_messages.as_ptr() as *mut _,
                num_control_messages: control_messages.len().try_into().unwrap(),
            },
            address: PhantomData,
            vectors: PhantomData,
            control_messages: PhantomData,
        }
    }
    #[inline]
    pub fn vectors(&self) -> &'m [OutputVector<'m>] {
        unsafe {
            std::slice::from_raw_parts(
                self.message.vectors as *const _,
                self.message.num_vectors as usize,
            )
        }
    }
    #[inline]
    pub fn bytes_sent(&self) -> u32 {
        self.message.bytes_sent
    }
}

pub trait SocketExtManual: Sized {
    #[doc(alias = "g_socket_receive")]
    fn receive<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "g_socket_receive_from")]
    fn receive_from<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, SocketAddress), glib::Error>;
    #[doc(alias = "g_socket_receive_message")]
    fn receive_message<C: IsA<Cancellable>>(
        &self,
        address: Option<&mut Option<SocketAddress>>,
        vectors: &mut [InputVector],
        control_messages: Option<&mut SocketControlMessages>,
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<(usize, i32), glib::Error>;
    #[doc(alias = "g_socket_receive_messages")]
    fn receive_messages<C: IsA<Cancellable>>(
        &self,
        messages: &mut [InputMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "g_socket_receive_with_blocking")]
    fn receive_with_blocking<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;

    #[doc(alias = "g_socket_send")]
    fn send<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "g_socket_send_message")]
    fn send_message<P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        vectors: &[OutputVector],
        messages: &[SocketControlMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_socket_send_message_with_timeout")]
    fn send_message_with_timeout<P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        vectors: &[OutputVector],
        messages: &[SocketControlMessage],
        flags: i32,
        timeout: Option<Duration>,
        cancellable: Option<&C>,
    ) -> Result<(PollableReturn, usize), glib::Error>;
    #[doc(alias = "g_socket_send_messages")]
    fn send_messages<C: IsA<Cancellable>>(
        &self,
        messages: &mut [OutputMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "g_socket_send_to")]
    fn send_to<B: AsRef<[u8]>, P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;
    #[doc(alias = "g_socket_send_with_blocking")]
    fn send_with_blocking<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error>;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    #[doc(alias = "get_fd")]
    #[doc(alias = "g_socket_get_fd")]
    fn fd<T: FromRawFd>(&self) -> T;

    #[cfg(any(windows, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(windows)))]
    #[doc(alias = "get_socket")]
    #[doc(alias = "g_socket_get_fd")]
    fn socket<T: FromRawSocket>(&self) -> T;

    #[doc(alias = "g_socket_create_source")]
    fn create_source<F, C>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static,
        C: IsA<Cancellable>;

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = glib::IOCondition> + 'static>>;

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = glib::IOCondition> + 'static>>;
}

impl<O: IsA<Socket>> SocketExtManual for O {
    fn receive<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_receive(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_from<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        cancellable: Option<&C>,
    ) -> Result<(usize, SocketAddress), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let mut addr_ptr = ptr::null_mut();

            let ret = ffi::g_socket_receive_from(
                self.as_ref().to_glib_none().0,
                &mut addr_ptr,
                buffer_ptr,
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok((ret as usize, from_glib_full(addr_ptr)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_message<C: IsA<Cancellable>>(
        &self,
        mut address: Option<&mut Option<SocketAddress>>,
        vectors: &mut [InputVector],
        control_messages: Option<&mut SocketControlMessages>,
        mut flags: i32,
        cancellable: Option<&C>,
    ) -> Result<(usize, i32), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let address = address
            .as_mut()
            .map(|a| {
                assert!(a.is_none());
                *a as *mut _ as *mut _
            })
            .unwrap_or_else(ptr::null_mut);
        let (control_messages, num_control_messages) = control_messages
            .map(|c| (&mut c.ptr as *mut _, &mut c.len as *mut _ as *mut _))
            .unwrap_or_else(|| (ptr::null_mut(), ptr::null_mut()));
        unsafe {
            let mut error = ptr::null_mut();

            let received = ffi::g_socket_receive_message(
                self.as_ref().to_glib_none().0,
                address,
                vectors.as_mut_ptr() as *mut ffi::GInputVector,
                vectors.len().try_into().unwrap(),
                control_messages,
                num_control_messages,
                &mut flags,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((received as usize, flags))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_messages<C: IsA<Cancellable>>(
        &self,
        messages: &mut [InputMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        unsafe {
            let mut error = ptr::null_mut();

            let count = ffi::g_socket_receive_messages(
                self.as_ref().to_glib_none().0,
                messages.as_mut_ptr() as *mut _,
                messages.len().try_into().unwrap(),
                flags,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(count as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn receive_with_blocking<B: AsMut<[u8]>, C: IsA<Cancellable>>(
        &self,
        mut buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let buffer = buffer.as_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let count = buffer.len();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_receive_with_blocking(
                self.as_ref().to_glib_none().0,
                buffer_ptr,
                count,
                blocking.into_glib(),
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_send(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_message<P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        vectors: &[OutputVector],
        messages: &[SocketControlMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_send_message(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
                vectors.as_ptr() as *mut ffi::GOutputVector,
                vectors.len().try_into().unwrap(),
                messages.as_ptr() as *mut _,
                messages.len().try_into().unwrap(),
                flags,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    fn send_message_with_timeout<P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        vectors: &[OutputVector],
        messages: &[SocketControlMessage],
        flags: i32,
        timeout: Option<Duration>,
        cancellable: Option<&C>,
    ) -> Result<(PollableReturn, usize), glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        unsafe {
            let mut error = ptr::null_mut();
            let mut bytes_written = 0;

            let ret = ffi::g_socket_send_message_with_timeout(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
                vectors.as_ptr() as *mut ffi::GOutputVector,
                vectors.len().try_into().unwrap(),
                messages.as_ptr() as *mut _,
                messages.len().try_into().unwrap(),
                flags,
                timeout
                    .map(|t| t.as_micros().try_into().unwrap())
                    .unwrap_or(-1),
                &mut bytes_written,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib(ret), bytes_written))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_messages<C: IsA<Cancellable>>(
        &self,
        messages: &mut [OutputMessage],
        flags: i32,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        unsafe {
            let mut error = ptr::null_mut();
            let count = ffi::g_socket_send_messages(
                self.as_ref().to_glib_none().0,
                messages.as_mut_ptr() as *mut _,
                messages.len().try_into().unwrap(),
                flags,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(count as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_to<B: AsRef<[u8]>, P: IsA<SocketAddress>, C: IsA<Cancellable>>(
        &self,
        address: Option<&P>,
        buffer: B,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();

            let ret = ffi::g_socket_send_to(
                self.as_ref().to_glib_none().0,
                address.map(|p| p.as_ref()).to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn send_with_blocking<B: AsRef<[u8]>, C: IsA<Cancellable>>(
        &self,
        buffer: B,
        blocking: bool,
        cancellable: Option<&C>,
    ) -> Result<usize, glib::Error> {
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        let (count, buffer_ptr) = {
            let slice = buffer.as_ref();
            (slice.len(), slice.as_ptr())
        };
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_socket_send_with_blocking(
                self.as_ref().to_glib_none().0,
                mut_override(buffer_ptr),
                count,
                blocking.into_glib(),
                gcancellable.0,
                &mut error,
            );
            if error.is_null() {
                Ok(ret as usize)
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(unix)))]
    fn fd<T: FromRawFd>(&self) -> T {
        unsafe { FromRawFd::from_raw_fd(ffi::g_socket_get_fd(self.as_ref().to_glib_none().0)) }
    }

    #[cfg(any(windows, feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(windows)))]
    fn socket<T: FromRawSocket>(&self) -> T {
        unsafe {
            FromRawSocket::from_raw_socket(ffi::g_socket_get_fd(self.as_ref().to_glib_none().0) as _)
        }
    }

    fn create_source<F, C>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        name: Option<&str>,
        priority: glib::Priority,
        func: F,
    ) -> glib::Source
    where
        F: FnMut(&Self, glib::IOCondition) -> glib::Continue + 'static,
        C: IsA<Cancellable>,
    {
        unsafe extern "C" fn trampoline<
            O: IsA<Socket>,
            F: FnMut(&O, glib::IOCondition) -> glib::Continue + 'static,
        >(
            socket: *mut ffi::GSocket,
            condition: glib::ffi::GIOCondition,
            func: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let func: &RefCell<F> = &*(func as *const RefCell<F>);
            let mut func = func.borrow_mut();
            (*func)(
                Socket::from_glib_borrow(socket).unsafe_cast_ref(),
                from_glib(condition),
            )
            .into_glib()
        }
        unsafe extern "C" fn destroy_closure<F>(ptr: glib::ffi::gpointer) {
            let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
        }
        let cancellable = cancellable.map(|c| c.as_ref());
        let gcancellable = cancellable.to_glib_none();
        unsafe {
            let source = ffi::g_socket_create_source(
                self.as_ref().to_glib_none().0,
                condition.into_glib(),
                gcancellable.0,
            );
            let trampoline = trampoline::<O, F> as glib::ffi::gpointer;
            glib::ffi::g_source_set_callback(
                source,
                Some(transmute::<
                    _,
                    unsafe extern "C" fn(glib::ffi::gpointer) -> glib::ffi::gboolean,
                >(trampoline)),
                Box::into_raw(Box::new(RefCell::new(func))) as glib::ffi::gpointer,
                Some(destroy_closure::<F>),
            );
            glib::ffi::g_source_set_priority(source, priority.into_glib());

            if let Some(name) = name {
                glib::ffi::g_source_set_name(source, name.to_glib_none().0);
            }

            from_glib_full(source)
        }
    }

    fn create_source_future<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn std::future::Future<Output = glib::IOCondition> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceFuture::new(move |send| {
            let mut send = Some(send);
            obj.create_source(
                condition,
                cancellable.as_ref(),
                None,
                priority,
                move |_, condition| {
                    let _ = send.take().unwrap().send(condition);
                    glib::Continue(false)
                },
            )
        }))
    }

    fn create_source_stream<C: IsA<Cancellable>>(
        &self,
        condition: glib::IOCondition,
        cancellable: Option<&C>,
        priority: glib::Priority,
    ) -> Pin<Box<dyn Stream<Item = glib::IOCondition> + 'static>> {
        let cancellable: Option<Cancellable> = cancellable.map(|c| c.as_ref()).cloned();

        let obj = self.clone();
        Box::pin(glib::SourceStream::new(move |send| {
            let send = Some(send);
            obj.create_source(
                condition,
                cancellable.as_ref(),
                None,
                priority,
                move |_, condition| {
                    if send.as_ref().unwrap().unbounded_send(condition).is_err() {
                        glib::Continue(false)
                    } else {
                        glib::Continue(true)
                    }
                },
            )
        }))
    }
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait IntoRawFd {
    fn into_raw_fd(self) -> c_int;
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait FromRawFd {
    unsafe fn from_raw_fd(fd: c_int) -> Self;
}

#[cfg(all(not(unix), feature = "dox"))]
pub trait AsRawFd {
    fn as_raw_fd(&self) -> RawFd;
}

#[cfg(all(not(unix), feature = "dox"))]
pub type RawFd = c_int;

#[cfg(all(not(windows), feature = "dox"))]
pub trait IntoRawSocket {
    fn into_raw_socket(self) -> u64;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait FromRawSocket {
    unsafe fn from_raw_socket(sock: u64) -> Self;
}

#[cfg(all(not(windows), feature = "dox"))]
pub trait AsRawSocket {
    fn as_raw_socket(&self) -> RawSocket;
}

#[cfg(all(not(windows), feature = "dox"))]
pub type RawSocket = *mut c_void;

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(unix)]
    fn socket_messages() {
        use std::{io, os::unix::io::AsRawFd};

        use super::Socket;
        use crate::{prelude::*, Cancellable, UnixFDMessage};

        let mut fds = [0 as libc::c_int; 2];
        let (out_sock, in_sock) = unsafe {
            let ret = libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr());
            if ret != 0 {
                panic!("{}", io::Error::last_os_error());
            }
            (
                Socket::from_fd(fds[0]).unwrap(),
                Socket::from_fd(fds[1]).unwrap(),
            )
        };

        let fd_msg = UnixFDMessage::new();
        fd_msg.append_fd(out_sock.as_raw_fd()).unwrap();
        let vs = [super::OutputVector::new(&[0])];
        let ctrl_msgs = [fd_msg.upcast()];
        let mut out_msg = [super::OutputMessage::new(
            crate::SocketAddress::NONE,
            vs.as_slice(),
            ctrl_msgs.as_slice(),
        )];
        let written = super::SocketExtManual::send_messages(
            &out_sock,
            out_msg.as_mut_slice(),
            0,
            Cancellable::NONE,
        )
        .unwrap();
        assert_eq!(written, 1);
        assert_eq!(out_msg[0].bytes_sent(), 1);

        let mut v = [0u8];
        let mut vs = [super::InputVector::new(v.as_mut_slice())];
        let mut ctrl_msgs = super::SocketControlMessages::new();
        let mut in_msg = [super::InputMessage::new(
            None,
            vs.as_mut_slice(),
            Some(&mut ctrl_msgs),
        )];
        let received = super::SocketExtManual::receive_messages(
            &in_sock,
            in_msg.as_mut_slice(),
            0,
            Cancellable::NONE,
        )
        .unwrap();

        assert_eq!(received, 1);
        assert_eq!(in_msg[0].bytes_received(), 1);
        assert_eq!(ctrl_msgs.len(), 1);
        let fds = ctrl_msgs[0]
            .downcast_ref::<UnixFDMessage>()
            .unwrap()
            .fd_list();
        assert_eq!(fds.length(), 1);
    }
    #[test]
    #[cfg(unix)]
    fn dgram_socket_messages() {
        use super::Socket;
        use crate::{prelude::*, Cancellable};

        let addr = crate::InetSocketAddress::from_string("127.0.0.1", 28351).unwrap();

        let out_sock = Socket::new(
            crate::SocketFamily::Ipv4,
            crate::SocketType::Datagram,
            crate::SocketProtocol::Udp,
        )
        .unwrap();
        let in_sock = Socket::new(
            crate::SocketFamily::Ipv4,
            crate::SocketType::Datagram,
            crate::SocketProtocol::Udp,
        )
        .unwrap();
        in_sock.bind(&addr, true).unwrap();

        const DATA: [u8; std::mem::size_of::<u64>()] = 1234u64.to_be_bytes();
        let out_vec = DATA;
        let out_vecs = [super::OutputVector::new(out_vec.as_slice())];
        let mut out_msg = [super::OutputMessage::new(
            Some(&addr),
            out_vecs.as_slice(),
            &[],
        )];
        let written = super::SocketExtManual::send_messages(
            &out_sock,
            out_msg.as_mut_slice(),
            0,
            Cancellable::NONE,
        )
        .unwrap();
        assert_eq!(written, 1);
        assert_eq!(out_msg[0].bytes_sent() as usize, out_vec.len());

        let mut in_addr = None;
        let mut in_vec = [0u8; DATA.len()];
        let mut in_vecs = [super::InputVector::new(in_vec.as_mut_slice())];
        let mut in_msg = [super::InputMessage::new(
            Some(&mut in_addr),
            in_vecs.as_mut_slice(),
            None,
        )];
        let received = super::SocketExtManual::receive_messages(
            &in_sock,
            in_msg.as_mut_slice(),
            0,
            Cancellable::NONE,
        )
        .unwrap();

        assert_eq!(received, 1);
        assert_eq!(in_msg[0].bytes_received(), in_vec.len());
        assert_eq!(in_vec, out_vec);
        let in_addr = in_addr
            .unwrap()
            .downcast::<crate::InetSocketAddress>()
            .unwrap();
        assert_eq!(in_addr.address().to_str(), addr.address().to_str());
    }
}
