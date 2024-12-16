// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

#[cfg(feature = "v2_60")]
pub use crate::app_info::AppInfoExtManual;
#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
pub use crate::debug_controller_dbus::DebugControllerDBusExtManual;
#[cfg(feature = "v2_58")]
#[cfg(all(not(windows), not(target_os = "macos")))]
#[cfg_attr(docsrs, doc(cfg(all(not(windows), not(target_os = "macos")))))]
pub use crate::desktop_app_info::DesktopAppInfoExtManual;
#[cfg(unix)]
pub use crate::file_descriptor_based::FileDescriptorBasedExtManual;
#[cfg(unix)]
pub use crate::unix_fd_list::UnixFDListExtManual;
#[cfg(unix)]
pub use crate::unix_fd_message::UnixFDMessageExtManual;
#[cfg(unix)]
pub use crate::unix_input_stream::UnixInputStreamExtManual;
#[cfg(unix)]
pub use crate::unix_output_stream::UnixOutputStreamExtManual;
#[cfg(unix)]
pub use crate::unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};
#[cfg(windows)]
pub use crate::win32_input_stream::Win32InputStreamExt;
#[cfg(windows)]
pub use crate::win32_output_stream::Win32OutputStreamExt;
pub use crate::{
    action_map::ActionMapExtManual, application::ApplicationExtManual, auto::traits::*,
    cancellable::CancellableExtManual, converter::ConverterExtManual,
    data_input_stream::DataInputStreamExtManual, datagram_based::DatagramBasedExtManual,
    dbus_connection::DBusMethodCall, dbus_proxy::DBusProxyExtManual, file::FileExtManual,
    file_enumerator::FileEnumeratorExtManual, inet_address::InetAddressExtManual,
    input_stream::InputStreamExtManual, io_stream::IOStreamExtManual,
    list_model::ListModelExtManual, output_stream::OutputStreamExtManual,
    pollable_input_stream::PollableInputStreamExtManual,
    pollable_output_stream::PollableOutputStreamExtManual, settings::SettingsExtManual,
    simple_proxy_resolver::SimpleProxyResolverExtManual, socket::SocketExtManual,
    socket_control_message::SocketControlMessageExtManual,
    socket_listener::SocketListenerExtManual, tls_connection::TlsConnectionExtManual,
};
