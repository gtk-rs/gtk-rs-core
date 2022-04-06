// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

pub use crate::auto::traits::*;

#[cfg(any(feature = "v2_60", feature = "dox"))]
pub use crate::app_info::AppInfoExtManual;
pub use crate::application::*;
pub use crate::cancellable::*;
pub use crate::converter::*;
pub use crate::data_input_stream::DataInputStreamExtManual;
pub use crate::dbus_proxy::DBusProxyExtManual;
#[cfg(any(feature = "v2_72", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v2_72")))]
pub use crate::debug_controller_dbus::DebugControllerDBusExtManual;
#[cfg(any(feature = "v2_58", feature = "dox"))]
#[cfg(any(all(not(windows), not(target_os = "macos")), feature = "dox"))]
pub use crate::desktop_app_info::DesktopAppInfoExtManual;
pub use crate::file::FileExtManual;
pub use crate::inet_address::InetAddressExtManual;
pub use crate::initable::InitableError;
pub use crate::input_stream::InputStreamExtManual;
pub use crate::io_stream::IOStreamExtManual;
pub use crate::list_model::ListModelExt;
pub use crate::output_stream::OutputStreamExtManual;
pub use crate::pollable_input_stream::PollableInputStreamExtManual;
pub use crate::pollable_output_stream::PollableOutputStreamExtManual;
pub use crate::settings::SettingsExtManual;
pub use crate::simple_proxy_resolver::SimpleProxyResolverExtManual;
pub use crate::socket::*;
pub use crate::tls_connection::TlsConnectionExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_fd_list::UnixFDListExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_input_stream::UnixInputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_output_stream::UnixOutputStreamExtManual;
#[cfg(any(unix, feature = "dox"))]
pub use crate::unix_socket_address::{UnixSocketAddressExtManual, UnixSocketAddressPath};
