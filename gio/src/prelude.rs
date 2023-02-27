// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

#[cfg(any(feature = "v2_60", feature = "dox"))]
pub use crate::app_info::AppInfoExtManual;
#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
pub use crate::debug_controller_dbus::DebugControllerDBusExtManual;
#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
pub use crate::desktop_app_info::DesktopAppInfoExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::file_descriptor_based::FileDescriptorBasedExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_fd_list::UnixFDListExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_fd_message::UnixFDMessageExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_input_stream::UnixInputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_output_stream::UnixOutputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};
pub use crate::{
    action_map::ActionMapExtManual, application::*, auto::traits::*, cancellable::*, converter::*,
    data_input_stream::DataInputStreamExtManual, datagram_based::*, dbus_proxy::DBusProxyExtManual,
    file::FileExtManual, file_enumerator::FileEnumeratorExtManual,
    inet_address::InetAddressExtManual, input_stream::InputStreamExtManual,
    io_stream::IOStreamExtManual, list_model::ListModelExtManual,
    output_stream::OutputStreamExtManual, pollable_input_stream::PollableInputStreamExtManual,
    pollable_output_stream::PollableOutputStreamExtManual, settings::SettingsExtManual,
    simple_proxy_resolver::SimpleProxyResolverExtManual, socket::SocketExtManual,
    socket_control_message::*, tls_connection::TlsConnectionExtManual,
};
