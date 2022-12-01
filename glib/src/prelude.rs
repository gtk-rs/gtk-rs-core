// Take a look at the license at the top of the repository in the LICENSE file.

// rustdoc-stripper-ignore-next
//! Traits and essential types intended for blanket imports.

pub use crate::param_spec::ParamSpecBuilderExt;
pub use crate::{
    Cast, CastNone, Continue, IsA, ObjectExt, ObjectType, ParamSpecType, StaticType, StaticTypeExt,
    StaticVariantType, ToSendValue, ToValue, ToVariant,
};
