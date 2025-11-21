// Take a look at the license at the top of the repository in the LICENSE file.

pub use ffi;

#[allow(unused_imports)]
mod auto;
pub use auto::*;

#[cfg(feature = "v2_58")]
mod desktop_app_info;
mod fd_message;
mod file_descriptor_based;
mod input_stream;
#[cfg(feature = "v2_84")]
mod mount_entry;
mod mount_point;
mod output_stream;

pub mod functions {
    pub use super::auto::functions::*;
}

pub mod prelude {
    pub use super::auto::traits::*;

    #[cfg(feature = "v2_58")]
    pub use crate::desktop_app_info::DesktopAppInfoExtManual;

    pub use crate::fd_message::FDMessageExtManual;
    pub use crate::file_descriptor_based::FileDescriptorBasedExtManual;
    pub use crate::input_stream::InputStreamExtManual;
    pub use crate::output_stream::OutputStreamExtManual;
}
