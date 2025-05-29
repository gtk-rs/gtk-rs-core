// Take a look at the license at the top of the repository in the LICENSE file.

mod action_group;
mod action_map;
mod app_launch_context;
mod application;
mod async_initable;
mod file_monitor;
mod icon;
mod initable;
mod input_stream;
mod io_stream;
mod list_model;
mod loadable_icon;
mod output_stream;
mod seekable;
mod socket_control_message;

pub use self::application::ArgumentList;

pub mod prelude {
    #[doc(hidden)]
    pub use glib::subclass::prelude::*;

    pub use super::{
        action_group::{ActionGroupImpl, ActionGroupImplExt},
        action_map::{ActionMapImpl, ActionMapImplExt},
        app_launch_context::{AppLaunchContextImpl, AppLaunchContextImplExt},
        application::{ApplicationImpl, ApplicationImplExt},
        async_initable::{AsyncInitableImpl, AsyncInitableImplExt},
        file_monitor::{FileMonitorImpl, FileMonitorImplExt},
        icon::{IconImpl, IconImplExt},
        initable::{InitableImpl, InitableImplExt},
        input_stream::{InputStreamImpl, InputStreamImplExt},
        io_stream::{IOStreamImpl, IOStreamImplExt},
        list_model::{ListModelImpl, ListModelImplExt},
        loadable_icon::{LoadableIconImpl, LoadableIconImplExt},
        output_stream::{OutputStreamImpl, OutputStreamImplExt},
        seekable::{SeekableImpl, SeekableImplExt},
        socket_control_message::{SocketControlMessageImpl, SocketControlMessageImplExt},
    };
}
