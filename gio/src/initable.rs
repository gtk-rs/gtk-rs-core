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
    /// Similar to [`Object::new`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[allow(clippy::new_ret_no_self)]
    #[track_caller]
    pub fn new<O: Sized + IsClass + IsA<Object> + IsA<Initable>, P: IsA<Cancellable>>(
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&P>,
    ) -> Result<O, glib::Error> {
        Self::with_type(O::static_type(), properties, cancellable)
            .map(|o| unsafe { o.unsafe_cast() })
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an initable object of the given type with the given properties.
    ///
    /// Similar to [`Object::with_type`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[track_caller]
    pub fn with_type(
        type_: Type,
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Object, glib::Error> {
        if !type_.is_a(Initable::static_type()) {
            panic!("Type '{type_}' is not initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.to_value()));
        }

        unsafe {
            let object = Object::new_internal(type_, &mut property_values);
            object.unsafe_cast_ref::<Self>().init(cancellable)?;
            Ok(object)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an initable object of the given type with the given properties.
    ///
    /// Similar to [`Object::with_values`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[track_caller]
    pub fn with_values(
        type_: Type,
        properties: &[(&str, glib::Value)],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Object, glib::Error> {
        if !type_.is_a(Initable::static_type()) {
            panic!("Type '{type_}' is not initable");
        }

        let mut property_values = smallvec::SmallVec::<[_; 16]>::with_capacity(properties.len());
        for (name, value) in properties {
            property_values.push((*name, value.clone()));
        }

        unsafe {
            let object = Object::new_internal(type_, &mut property_values);
            object.unsafe_cast_ref::<Self>().init(cancellable)?;
            Ok(object)
        }
    }
}
