// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![doc = include_str!("../README.md")]
#![allow(clippy::missing_safety_doc)]

pub use ffi;
pub use glib;

#[allow(clippy::too_many_arguments)]
#[allow(clippy::should_implement_trait)]
#[allow(clippy::derive_hash_xor_eq)]
#[allow(clippy::let_and_return)]
#[allow(unused_imports)]
mod auto;
pub use crate::auto::functions::*;
pub use crate::auto::*;
pub use crate::functions::*;

pub use ffi::PANGO_SCALE as SCALE;

/// The scale factor for three shrinking steps (1 / (1.2 * 1.2 * 1.2)).
pub const SCALE_XX_SMALL: f64 = 0.5787037037037;

/// The scale factor for two shrinking steps (1 / (1.2 * 1.2)).
pub const SCALE_X_SMALL: f64 = 0.6944444444444;

/// The scale factor for one shrinking step (1 / 1.2).
pub const SCALE_SMALL: f64 = 0.8333333333333;

/// The scale factor for normal size (1.0).
pub const SCALE_MEDIUM: f64 = 1.0;

/// The scale factor for one magnification step (1.2).
pub const SCALE_LARGE: f64 = 1.2;

/// The scale factor for two magnification steps (1.2 * 1.2).
pub const SCALE_X_LARGE: f64 = 1.44;

/// The scale factor for three magnification steps (1.2 * 1.2 * 1.2).
pub const SCALE_XX_LARGE: f64 = 1.728;

pub mod prelude;

#[macro_use]
mod attribute;

pub mod analysis;
pub use crate::analysis::Analysis;
pub mod attr_class;
pub use crate::attr_class::AttrClass;
mod attr_color;
pub use attr_color::AttrColor;
mod attr_float;
pub use attr_float::AttrFloat;
mod attr_font_desc;
pub use attr_font_desc::AttrFontDesc;
mod attr_font_features;
pub use attr_font_features::AttrFontFeatures;
mod attr_int;
pub use attr_int::AttrInt;
pub mod attr_iterator;
mod attr_language;
pub use attr_language::AttrLanguage;
pub mod attr_list;
mod attr_shape;
pub use attr_shape::AttrShape;
mod attr_size;
pub use attr_size::AttrSize;
mod attr_string;
pub use crate::attribute::IsAttribute;
pub use attr_string::AttrString;
pub mod color;
mod functions;
pub mod item;
pub mod language;
pub use crate::language::Language;
pub mod layout;
pub use crate::layout::HitPosition;
pub mod rectangle;
pub use crate::rectangle::Rectangle;
mod glyph_geometry;
pub use glyph_geometry::GlyphGeometry;
mod glyph_info;
pub use glyph_info::GlyphInfo;
mod glyph_item;
mod glyph_item_iter;
mod glyph_string;

mod coverage;
pub use crate::coverage::*;
