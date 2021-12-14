// Take a look at the license at the top of the repository in the LICENSE file.

use crate::traits::InitableExt;
use crate::Cancellable;
use crate::Initable;
use glib::object::IsA;
use glib::object::IsClass;
use glib::value::ToValue;
use glib::Object;

use std::fmt;

impl Initable {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<O: Sized + IsClass + IsA<Object> + IsA<Initable>, P: IsA<Cancellable>>(
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&P>,
    ) -> Result<O, InitableError> {
        let object = Object::new::<O>(properties)?;
        unsafe { object.init(cancellable)? };
        Ok(object)
    }
}

#[derive(Debug)]
pub enum InitableError {
    NewObjectFailed(glib::error::BoolError),
    InitFailed(glib::Error),
}

impl From<glib::error::BoolError> for InitableError {
    fn from(err: glib::error::BoolError) -> Self {
        Self::NewObjectFailed(err)
    }
}

impl From<glib::Error> for InitableError {
    fn from(err: glib::Error) -> Self {
        Self::InitFailed(err)
    }
}

impl std::error::Error for InitableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NewObjectFailed(e) => Some(e),
            Self::InitFailed(e) => Some(e),
        }
    }
}

impl fmt::Display for InitableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewObjectFailed(e) => write!(f, "Object::new failed with {:?}", e),
            Self::InitFailed(e) => write!(f, "Initable::init failed with {:?}", e),
        }
    }
}
