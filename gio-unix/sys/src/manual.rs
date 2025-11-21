pub type GSocketFamily = libc::c_int;
pub type GSocketMsgFlags = libc::c_int;

pub const G_SOCKET_MSG_NONE: super::GSocketMsgFlags = 0;
pub const G_SOCKET_MSG_OOB: super::GSocketMsgFlags = libc::MSG_OOB;
pub const G_SOCKET_MSG_PEEK: super::GSocketMsgFlags = libc::MSG_PEEK;
pub const G_SOCKET_MSG_DONTROUTE: super::GSocketMsgFlags = libc::MSG_DONTROUTE;

#[cfg(not(feature = "v2_84"))]
pub mod g_unix_fallback {
    use crate::GUnixMountEntry;
    use glib_sys::gboolean;
    use libc::{c_char, c_int};

    pub unsafe fn g_unix_mount_entry_compare(
        mount1: *mut GUnixMountEntry,
        mount2: *mut GUnixMountEntry,
    ) -> c_int {
        crate::g_unix_mount_compare(mount1, mount2)
    }
    pub unsafe fn g_unix_mount_entry_copy(
        mount_entry: *mut GUnixMountEntry,
    ) -> *mut GUnixMountEntry {
        crate::g_unix_mount_copy(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_free(mount_entry: *mut GUnixMountEntry) {
        crate::g_unix_mount_free(mount_entry);
    }
    pub unsafe fn g_unix_mount_entry_get_device_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        crate::g_unix_mount_get_device_path(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_get_fs_type(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        crate::g_unix_mount_get_fs_type(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_get_mount_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        crate::g_unix_mount_get_mount_path(mount_entry)
    }
    #[cfg(feature = "v2_58")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_58")))]
    pub unsafe fn g_unix_mount_entry_get_options(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        crate::g_unix_mount_get_options(mount_entry)
    }
    #[cfg(feature = "v2_60")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_60")))]
    pub unsafe fn g_unix_mount_entry_get_root_path(
        mount_entry: *mut GUnixMountEntry,
    ) -> *const c_char {
        crate::g_unix_mount_get_root_path(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_can_eject(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        crate::g_unix_mount_guess_can_eject(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_icon(
        mount_entry: *mut GUnixMountEntry,
    ) -> *mut gio_sys::GIcon {
        crate::g_unix_mount_guess_icon(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_name(mount_entry: *mut GUnixMountEntry) -> *mut c_char {
        crate::g_unix_mount_guess_name(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_should_display(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        crate::g_unix_mount_guess_should_display(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_guess_symbolic_icon(
        mount_entry: *mut GUnixMountEntry,
    ) -> *mut gio_sys::GIcon {
        crate::g_unix_mount_guess_symbolic_icon(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_is_readonly(mount_entry: *mut GUnixMountEntry) -> gboolean {
        crate::g_unix_mount_is_readonly(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_is_system_internal(
        mount_entry: *mut GUnixMountEntry,
    ) -> gboolean {
        crate::g_unix_mount_is_system_internal(mount_entry)
    }
    pub unsafe fn g_unix_mount_entry_at(
        mount_path: *const c_char,
        time_read: *mut u64,
    ) -> *mut GUnixMountEntry {
        crate::g_unix_mount_at(mount_path, time_read)
    }
    pub unsafe fn g_unix_mount_entry_for(
        file_path: *const c_char,
        time_read: *mut u64,
    ) -> *mut GUnixMountEntry {
        crate::g_unix_mount_for(file_path, time_read)
    }

    #[cfg(feature = "v2_82")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_82")))]
    pub unsafe fn g_unix_mount_entries_get_from_file(
        table_path: *const c_char,
        time_read_out: *mut u64,
        n_entries_out: *mut size_t,
    ) -> *mut *mut GUnixMountEntry {
        crate::g_unix_mounts_get_from_file(table_path, time_read_out, n_entries_out)
    }

    pub unsafe fn g_unix_mount_entries_get(time_read: *mut u64) -> *mut glib_sys::GList {
        crate::g_unix_mounts_get(time_read)
    }

    pub unsafe fn g_unix_mount_entries_changed_since(time: u64) -> gboolean {
        crate::g_unix_mounts_changed_since(time)
    }
}

#[cfg(not(feature = "v2_84"))]
pub use g_unix_fallback::*;

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
