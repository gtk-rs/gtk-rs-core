// Take a look at the license at the top of the repository in the LICENSE file.

use crate::object::ObjectRef;
use crate::prelude::*;
use crate::translate::*;
use crate::Binding;
use crate::BindingFlags;
use crate::BindingGroup;
use crate::BoolError;
use crate::Object;
use crate::ParamSpec;
use crate::Value;
use std::{fmt, ptr};

impl BindingGroup {
    #[doc(alias = "bind_with_closures")]
    pub fn bind<'a, O: ObjectType>(
        &'a self,
        source_property: &'a str,
        target: &'a O,
        target_property: &'a str,
    ) -> BindingGroupBuilder<'a> {
        BindingGroupBuilder::new(self, source_property, target, target_property)
    }
}

type TransformFn = Option<Box<dyn Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>>;

// rustdoc-stripper-ignore-next
/// Builder for binding group bindings.
#[must_use = "The builder must be built to be used"]
pub struct BindingGroupBuilder<'a> {
    group: &'a BindingGroup,
    source_property: &'a str,
    target: &'a ObjectRef,
    target_property: &'a str,
    flags: BindingFlags,
    transform_to: TransformFn,
    transform_from: TransformFn,
}

impl<'a> fmt::Debug for BindingGroupBuilder<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BindingGroupBuilder")
            .field("group", &self.group)
            .field("source_property", &self.source_property)
            .field("target", &self.target)
            .field("target_property", &self.target_property)
            .field("flags", &self.flags)
            .finish()
    }
}

impl<'a> BindingGroupBuilder<'a> {
    fn new(
        group: &'a BindingGroup,
        source_property: &'a str,
        target: &'a impl ObjectType,
        target_property: &'a str,
    ) -> Self {
        Self {
            group,
            source_property,
            target: target.as_object_ref(),
            target_property,
            flags: BindingFlags::DEFAULT,
            transform_to: None,
            transform_from: None,
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the target object to the source object with the given closure.
    pub fn transform_from<F: Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_from: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Transform changed property values from the source object to the target object with the given closure.
    pub fn transform_to<F: Fn(&Binding, &Value) -> Option<Value> + Send + Sync + 'static>(
        self,
        func: F,
    ) -> Self {
        Self {
            transform_to: Some(Box::new(func)),
            ..self
        }
    }

    // rustdoc-stripper-ignore-next
    /// Bind the properties with the given flags.
    pub fn flags(self, flags: BindingFlags) -> Self {
        Self { flags, ..self }
    }

    // rustdoc-stripper-ignore-next
    /// Establish the property binding.
    ///
    /// This fails if the provided properties do not exist.
    pub fn try_build(self) -> Result<(), BoolError> {
        unsafe extern "C" fn transform_to_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data =
                &*(user_data as *const (TransformFn, TransformFn, ParamSpec, ParamSpec));

            match (transform_data.0.as_ref().unwrap())(
                &from_glib_borrow(binding),
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    assert!(
                        res.type_().is_a(transform_data.3.value_type()),
                        "Target property {} expected type {} but transform_to function returned {}",
                        transform_data.3.name(),
                        transform_data.3.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn transform_from_trampoline(
            binding: *mut gobject_ffi::GBinding,
            from_value: *const gobject_ffi::GValue,
            to_value: *mut gobject_ffi::GValue,
            user_data: ffi::gpointer,
        ) -> ffi::gboolean {
            let transform_data =
                &*(user_data as *const (TransformFn, TransformFn, ParamSpec, ParamSpec));

            match (transform_data.1.as_ref().unwrap())(
                &from_glib_borrow(binding),
                &*(from_value as *const Value),
            ) {
                None => false,
                Some(res) => {
                    assert!(
                        res.type_().is_a(transform_data.2.value_type()),
                        "Source property {} expected type {} but transform_from function returned {}",
                        transform_data.2.name(),
                        transform_data.2.value_type(),
                        res.type_()
                    );
                    *to_value = res.into_raw();
                    true
                }
            }
            .into_glib()
        }

        unsafe extern "C" fn free_transform_data(data: ffi::gpointer) {
            let _ = Box::from_raw(data as *mut (TransformFn, TransformFn, ParamSpec, ParamSpec));
        }

        let source = self
            .group
            .source()
            .ok_or_else(|| bool_error!("Binding group does not have a source set"))?;
        unsafe {
            let target: Object = from_glib_none(self.target.clone().to_glib_none().0);

            let source_property = source.find_property(self.source_property).ok_or_else(|| {
                bool_error!(
                    "Source property {} on type {} not found",
                    self.source_property,
                    source.type_()
                )
            })?;
            let target_property = target.find_property(self.target_property).ok_or_else(|| {
                bool_error!(
                    "Target property {} on type {} not found",
                    self.target_property,
                    target.type_()
                )
            })?;

            let source_property_name = source_property.name().as_ptr();
            let target_property_name = target_property.name().as_ptr();

            let have_transform_to = self.transform_to.is_some();
            let have_transform_from = self.transform_from.is_some();
            let transform_data = if have_transform_to || have_transform_from {
                Box::into_raw(Box::new((
                    self.transform_to,
                    self.transform_from,
                    source_property,
                    target_property,
                )))
            } else {
                ptr::null_mut()
            };

            gobject_ffi::g_binding_group_bind_full(
                self.group.to_glib_none().0,
                source_property_name as *const _,
                target.to_glib_none().0,
                target_property_name as *const _,
                self.flags.into_glib(),
                if have_transform_to {
                    Some(transform_to_trampoline)
                } else {
                    None
                },
                if have_transform_from {
                    Some(transform_from_trampoline)
                } else {
                    None
                },
                transform_data as ffi::gpointer,
                if transform_data.is_null() {
                    None
                } else {
                    Some(free_transform_data)
                },
            );
        }

        Ok(())
    }

    // rustdoc-stripper-ignore-next
    /// Similar to `try_build` but fails instead of panicking.
    pub fn build(self) {
        self.try_build().unwrap()
    }
}
