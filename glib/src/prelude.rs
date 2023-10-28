// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

pub use crate::{
    gobject::traits::{DynamicObjectRegisterExt, TypeModuleExt, TypePluginExt},
    param_spec::ParamSpecBuilderExt,
    Cast, CastNone, IsA, ObjectClassExt, ObjectExt, ObjectType, ParamSpecType, StaticType,
    StaticTypeExt, StaticVariantType, ToSendValue, ToValue, ToVariant,
};
