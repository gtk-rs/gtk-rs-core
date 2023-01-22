// Take a look at the license at the top of the repository in the LICENSE file.

use std::marker::PhantomData;

use glib::{object::IsClass, prelude::*, Object, Type};

use crate::{prelude::*, Cancellable, Initable};

impl Initable {
    // rustdoc-stripper-ignore-next
    /// Create a new instance of an initable object with the given properties.
    ///
    /// Similar to [`Object::new`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[allow(clippy::new_ret_no_self)]
    #[track_caller]
    #[deprecated = "Use Initable::builder() or Initable::new_default() instead"]
    #[allow(deprecated)]
    pub fn new<O: IsClass + IsA<Object> + IsA<Initable>>(
        properties: &[(&str, &dyn ToValue)],
        cancellable: Option<&impl IsA<Cancellable>>,
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
    #[deprecated = "Use Initable::builder() or Initable::new_default_with_type() instead"]
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
    /// Create a new instance of an object with the default property values.
    ///
    /// Similar to [`Object::new_default`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[track_caller]
    pub fn new_default<T: IsA<Object> + IsClass + IsA<Initable>>(
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<T, glib::Error> {
        let object = Self::new_default_with_type(T::static_type(), cancellable)?;
        Ok(unsafe { object.unsafe_cast() })
    }

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object with the default property values.
    ///
    /// Similar to [`Object::new_default_with_type`] but can fail because the object initialization in
    /// `Initable::init` failed.
    #[track_caller]
    pub fn new_default_with_type(
        type_: Type,
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Object, glib::Error> {
        if !type_.is_a(Initable::static_type()) {
            panic!("Type '{type_}' is not initable");
        }

        unsafe {
            let object = Object::new_internal(type_, &mut []);
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
    #[deprecated = "Use Initable::with_mut_values() instead"]
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

    // rustdoc-stripper-ignore-next
    /// Create a new instance of an object of the given type with the given properties as mutable
    /// values.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    pub fn with_mut_values(
        type_: Type,
        properties: &mut [(&str, glib::Value)],
        cancellable: Option<&impl IsA<Cancellable>>,
    ) -> Result<Object, glib::Error> {
        if !type_.is_a(Initable::static_type()) {
            panic!("Type '{type_}' is not initable");
        }

        unsafe {
            let object = Object::new_internal(type_, properties);
            object.unsafe_cast_ref::<Self>().init(cancellable)?;
            Ok(object)
        }
    }

    // rustdoc-stripper-ignore-next
    /// Create a new object builder for a specific type.
    pub fn builder<'a, O: IsA<Object> + IsClass + IsA<Initable>>() -> InitableBuilder<'a, O> {
        InitableBuilder::new(O::static_type())
    }

    // rustdoc-stripper-ignore-next
    /// Create a new object builder for a specific type.
    pub fn builder_with_type<'a>(type_: Type) -> InitableBuilder<'a, Object> {
        if !type_.is_a(Initable::static_type()) {
            panic!("Type '{type_}' is not initable");
        }

        InitableBuilder::new(type_)
    }
}

#[must_use = "builder doesn't do anything unless built"]
pub struct InitableBuilder<'a, O> {
    type_: Type,
    properties: smallvec::SmallVec<[(&'a str, glib::Value); 16]>,
    phantom: PhantomData<O>,
}

impl<'a, O: IsA<Object> + IsClass> InitableBuilder<'a, O> {
    #[inline]
    fn new(type_: Type) -> Self {
        InitableBuilder {
            type_,
            properties: smallvec::SmallVec::new(),
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Gets the type of this builder.
    #[inline]
    pub fn type_(&self) -> Type {
        self.type_
    }

    // rustdoc-stripper-ignore-next
    /// Set property `name` to the given value `value`.
    #[inline]
    pub fn property(self, name: &'a str, value: impl Into<glib::Value>) -> Self {
        let InitableBuilder {
            type_,
            mut properties,
            ..
        } = self;
        properties.push((name, value.into()));

        InitableBuilder {
            type_,
            properties,
            phantom: PhantomData,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the object with the provided properties.
    ///
    /// # Panics
    ///
    /// This panics if the object is not instantiable, doesn't have all the given properties or
    /// property values of the wrong type are provided.
    #[track_caller]
    #[inline]
    pub fn build(mut self, cancellable: Option<&impl IsA<Cancellable>>) -> Result<O, glib::Error> {
        let object = Initable::with_mut_values(self.type_, &mut self.properties, cancellable)?;
        Ok(unsafe { object.unsafe_cast::<O>() })
    }
}
