// Take a look at the license at the top of the repository in the LICENSE file.

use crate::traits::InitableExt;
use crate::Cancellable;
use crate::Initable;
use glib::object::IsA;
use glib::object::IsClass;
use glib::value::ToValue;
use glib::Object;

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

#[derive(thiserror::Error, Debug)]
pub enum InitableError {
    #[error("Object::new failed with {0:?}")]
    NewObjectFailed(#[from] glib::error::BoolError),
    #[error("Initable::init failed with {0:?}")]
    InitFailed(#[from] glib::Error),
}
