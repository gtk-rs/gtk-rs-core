// Take a look at the license at the top of the repository in the LICENSE file.

use crate::traits::InitableExt;
use crate::Cancellable;
use crate::Initable;
use glib::object::IsA;
use glib::object::IsClass;
use glib::value::ToValue;
use glib::{Cast, Object, StaticType, Type};

impl Initable {
    // rustdoc-stripper-ignore-next
    /// Create a new instance of an initable object with the given properties.
    ///
    /// Similar to [`Object::new`] but can fail either because the object
    /// creation failed or because `Initable::init` failed.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<O: Sized + IsClass + IsA<Object> + IsA<Initable>, P: IsA<Cancellable>>(
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&P>,
    ) -> Result<O, InitableError> {
        let object = Object::new::<O>(properties)?;
        unsafe { object.init(cancellable)? };
        Ok(object)
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an initable object of the given type with the given properties.
    ///
    /// Similar to [`Object::with_type`] but can fail either because the object
    /// creation failed or because `Initable::init` failed.
    pub fn with_type(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Object, InitableError> {
        if !type_.is_a(Initable::static_type()) {
            return Err(InitableError::NewObjectFailed(glib::bool_error!(
                "Type '{}' is not initable",
                type_
            )));
        }
        let object = Object::with_type(type_, properties)?;
        unsafe { object.unsafe_cast_ref::<Self>().init(cancellable)? };
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
