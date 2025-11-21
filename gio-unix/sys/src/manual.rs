pub type GSocketFamily = libc::c_int;
pub type GSocketMsgFlags = libc::c_int;

pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags = libc::MSG_OOB;
pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags = libc::MSG_PEEK;
pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags = libc::MSG_DONTROUTE;

extern "C" {

    #[cfg(feature = "v2_84")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_84")))]
    pub fn g_unix_mount_entry_copy(
        mount_entry: *const crate::GUnixMountEntry,
    ) -> *mut crate::GUnixMountEntry;

    pub fn g_unix_mount_point_copy(
        mount_point: *const crate::GUnixMountPoint,
    ) -> *mut crate::GUnixMountPoint;
}
