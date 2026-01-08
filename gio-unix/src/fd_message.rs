// Take a look at the license at the top of the repository in the LICENSE file.

use std::os::unix::io::{AsRawFd, RawFd};
use std::{mem, ptr};

use glib::{prelude::*, translate::*};

use crate::{FDMessage, ffi};

pub trait FDMessageExtManual: IsA<FDMessage> + Sized {
    #[doc(alias = "g_unix_fd_message_append_fd")]
    fn append_fd<T: AsRawFd>(&self, fd: T) -> Result<(), glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            ffi::g_unix_fd_message_append_fd(
                self.as_ref().to_glib_none().0,
                fd.as_raw_fd(),
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
    #[doc(alias = "g_unix_fd_message_steal_fds")]
    fn steal_fds(&self) -> Vec<RawFd> {
        unsafe {
            let mut length = mem::MaybeUninit::uninit();

            FromGlibContainer::from_glib_full_num(
                ffi::g_unix_fd_message_steal_fds(
                    self.as_ref().to_glib_none().0,
                    length.as_mut_ptr(),
                ),
                length.assume_init() as usize,
            )
        }
    }
}

impl<O: IsA<FDMessage>> FDMessageExtManual for O {}

#[cfg(test)]
mod tests {
    use std::{
        io,
        os::unix::io::{AsRawFd, FromRawFd, OwnedFd},
    };

    use crate::prelude::*;
    use gio::Cancellable;
    use gio::Socket;
    use gio::prelude::UnixFDListExt;
    use glib::prelude::Cast;

    #[test]
    fn socket_messages() {
        let mut fds = [0 as libc::c_int; 2];
        let (out_sock, in_sock) = unsafe {
            let ret = libc::socketpair(libc::AF_UNIX, libc::SOCK_STREAM, 0, fds.as_mut_ptr());
            if ret != 0 {
                panic!("{}", io::Error::last_os_error());
            }
            (
                Socket::from_fd(OwnedFd::from_raw_fd(fds[0])).unwrap(),
                Socket::from_fd(OwnedFd::from_raw_fd(fds[1])).unwrap(),
            )
        };

        let fd_msg = crate::FDMessage::new();
        fd_msg.append_fd(out_sock.as_raw_fd()).unwrap();
        let vs = [gio::OutputVector::new(&[0])];
        let ctrl_msgs = [fd_msg.upcast()];
        let mut out_msg = [gio::OutputMessage::new(
            gio::SocketAddress::NONE,
            vs.as_slice(),
            ctrl_msgs.as_slice(),
        )];
        let written = gio::prelude::SocketExtManual::send_messages(
            &out_sock,
            out_msg.as_mut_slice(),
            0,
            Cancellable::NONE,
        )
        .unwrap();
        assert_eq!(written, 1);
        assert_eq!(out_msg[0].bytes_sent(), 1);

        let mut v = [0u8];
        let mut vs = [gio::InputVector::new(v.as_mut_slice())];
        let mut ctrl_msgs = gio::SocketControlMessages::new();
        let mut in_msg = [gio::InputMessage::new(
            None,
            vs.as_mut_slice(),
            Some(&mut ctrl_msgs),
        )];
        let received = gio::prelude::SocketExtManual::receive_messages(
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
            .downcast_ref::<crate::FDMessage>()
            .unwrap()
            .fd_list();
        assert_eq!(fds.length(), 1);
    }
}
