// Take a look at the license at the top of the repository in the LICENSE file.

use gio::prelude::ObjectType;
use gio::{IOErrorEnum, IOExtensionPoint, VFS_EXTENSION_POINT_NAME};
use glib::{g_debug, types::StaticType};

mod file;
mod file_enumerator;
mod file_monitor;
mod vfs;

#[unsafe(no_mangle)]
pub extern "C" fn g_io_module_load(module_ptr: *mut gio::ffi::GIOModule) {
    let io_module = unsafe { gio::IOModule::from_glib_ptr_borrow(&module_ptr) };
    let res = register(io_module);
    assert!(res.is_ok(), "{}", res.err().unwrap());
}

#[unsafe(no_mangle)]
pub extern "C" fn g_io_module_unload(module_ptr: *mut gio::ffi::GIOModule) {
    let io_module = unsafe { gio::IOModule::from_glib_ptr_borrow(&module_ptr) };
    let res = unregister(io_module);
    debug_assert!(res.is_ok(), "{}", res.err().unwrap());
}

pub fn register(io_module: &gio::IOModule) -> Result<(), glib::Error> {
    // register module types
    if !vfs::imp::MyVfs::on_implementation_load(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyVfs",
        ))?;
    }
    if !file::imp::MyFile::on_implementation_load(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyFile",
        ))?;
    }
    if !file_enumerator::imp::MyFileEnumerator::on_implementation_load(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyFileEnumerator",
        ))?;
    }
    if !file_monitor::imp::MyFileMonitor::on_implementation_load(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyFileMonitor",
        ))?;
    }

    if IOExtensionPoint::lookup(VFS_EXTENSION_POINT_NAME).is_none() {
        let _ = IOExtensionPoint::builder(VFS_EXTENSION_POINT_NAME).build();
    }
    IOExtensionPoint::implement(
        VFS_EXTENSION_POINT_NAME,
        vfs::MyVfs::static_type(),
        SCHEME,
        20,
    )
    .ok_or(glib::Error::new(
        IOErrorEnum::Failed,
        "failed to register vfs extension point",
    ))?;

    g_debug!("MyVfs", "myvfs registered !!!");
    Ok(())
}

pub fn unregister(io_module: &gio::IOModule) -> Result<(), glib::Error> {
    // unregister module types
    if !file_monitor::imp::MyFileMonitor::on_implementation_unload(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to unregister module type MyFileMonitor",
        ))?;
    }
    if !file_enumerator::imp::MyFileEnumerator::on_implementation_unload(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyFileEnumerator",
        ))?;
    }
    if !file::imp::MyFile::on_implementation_unload(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyFile",
        ))?;
    }
    if !vfs::imp::MyVfs::on_implementation_unload(io_module.as_ref()) {
        Err(glib::Error::new(
            IOErrorEnum::Failed,
            "failed to register module type MyVfs",
        ))?;
    }

    g_debug!("MyVfs", "myvfs unregistered !!!");
    Ok(())
}

pub const SCHEME: &str = "myvfs";

pub const MYVFS_ROOT: &str = "MYVFS_ROOT";

pub const DEFAULT_MYVFS_ROOT: &str = "/tmp/myvfs";

pub fn resolve_virtual_path<T: AsRef<str>>(local_path: T) -> String {
    let local_root = glib::getenv(MYVFS_ROOT)
        .and_then(|os| os.into_string().ok())
        .unwrap_or(DEFAULT_MYVFS_ROOT.to_string());
    g_debug!(
        "MyVfs",
        "resolve_virtual_path({},{})",
        local_root,
        local_path.as_ref()
    );
    local_path
        .as_ref()
        .strip_prefix(&local_root)
        .unwrap_or(local_path.as_ref())
        .to_string()
}

pub fn resolve_local_path<T: AsRef<str>>(virtual_path: T) -> String {
    let local_root = glib::getenv(MYVFS_ROOT)
        .and_then(|os| os.into_string().ok())
        .unwrap_or(DEFAULT_MYVFS_ROOT.to_string());
    g_debug!(
        "MyVfs",
        "resolve_local_path({},{})",
        local_root,
        virtual_path.as_ref()
    );
    format!("{}/{}", local_root, virtual_path.as_ref()).replace("//", "/")
}

pub fn update_file_info(info: &gio::FileInfo) {
    if let Some(v) = info.attribute_as_string(gio::FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET) {
        info.set_attribute_string(
            gio::FILE_ATTRIBUTE_STANDARD_SYMLINK_TARGET,
            &format!("{}://{}", SCHEME, resolve_virtual_path(v)),
        );
    }
}
