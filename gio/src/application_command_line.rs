// Take a look at the license at the top of the repository in the LICENSE file.

use std::{boxed::Box as Box_, mem::transmute, ops::ControlFlow};

use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
    ExitCode, GString,
};

use crate::{ffi, Application, ApplicationCommandLine, ExitCode, File};

pub trait ApplicationCommandLineExtManual: IsA<Application> {
    #[doc(alias = "g_application_command_line_get_exit_status")]
    #[doc(alias = "get_exit_status")]
    fn exit_code(&self) -> ExitCode {
        let status = unsafe {
            ffi::g_application_command_line_get_exit_status(self.as_ref().to_glib_none().0)
        };

        ExitCode::try_from(status).unwrap()
    }

    #[doc(alias = "g_application_command_line_set_exit_status")]
    #[doc(alias = "set_exit_status")]
    fn set_exit_code(&self, exit_code: ExitCode) {
        let status = i32::from(exit_code.get());

        unsafe {
            ffi::g_application_command_line_set_exit_status(self.as_ref().to_glib_none().0, status);
        }
    }
}

impl<O: IsA<Application>> ApplicationExtManual for O {}
