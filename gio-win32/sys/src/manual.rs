pub type GSocketFamily = libc::c_int;
pub type GSocketMsgFlags = libc::c_int;

pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags =
    windows_sys::Win32::Networking::WinSock::MSG_OOB;
pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags =
    windows_sys::Win32::Networking::WinSock::MSG_PEEK;
pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags =
    windows_sys::Win32::Networking::WinSock::MSG_DONTROUTE;

pub type GNetworkMonitorBase = glib_sys::gpointer;
pub type GNetworkMonitorBaseClass = glib_sys::gpointer;
