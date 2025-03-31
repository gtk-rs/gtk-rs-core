// Take a look at the license at the top of the repository in the LICENSE file.

use std::process::Termination;

// rustdoc-stripper-ignore-next
/// Wrapper around an [`i32`] exit code that implements [`std::process::Termination`].
///
/// - a zero exit code means success.
/// - a positive exit code means failure.
/// - a negative exit code is reserved as special.
///
/// Values outside the range of [`u8`] will be mapped to [`u8::MAX`] when converted to [`std::process::ExitCode`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct ExitCode(i32);

impl ExitCode {
    // rustdoc-stripper-ignore-next
    /// The canonical ExitCode for successful termination on this platform.
    pub const SUCCESS: Self = Self(libc::EXIT_SUCCESS);

    // rustdoc-stripper-ignore-next
    /// The canonical ExitCode for unsuccessful termination on this platform.
    pub const FAILURE: Self = Self(libc::EXIT_FAILURE);

    // rustdoc-stripper-ignore-next
    /// A special negative value to indicate that default handling should continue.
    pub const CONTINUE: Self = Self(-1);

    // rustdoc-stripper-ignore-next
    /// Returns the wrapped `i32` value.
    pub fn value(&self) -> i32 {
        self.0
    }
}

impl From<i32> for ExitCode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<ExitCode> for i32 {
    fn from(code: ExitCode) -> Self {
        code.value()
    }
}

impl From<ExitCode> for std::process::ExitCode {
    fn from(code: ExitCode) -> Self {
        Self::from(u8::try_from(code.0).unwrap_or(u8::MAX))
    }
}

impl Termination for ExitCode {
    fn report(self) -> std::process::ExitCode {
        self.into()
    }
}
