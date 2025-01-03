// Take a look at the license at the top of the repository in the LICENSE file.

use std::fmt::Debug;

#[cfg(feature = "use_glib")]
use glib::translate::*;

use crate::{ffi, Error};

// Helper macro for our GValue related trait impls
#[cfg(feature = "use_glib")]
macro_rules! gvalue_impl {
    ($name:ty, $get_type:expr) => {
        impl glib::prelude::StaticType for $name {
            #[inline]
            fn static_type() -> glib::Type {
                unsafe { from_glib($get_type()) }
            }
        }

        impl glib::value::ValueType for $name {
            type Type = Self;
        }

        unsafe impl<'a> glib::value::FromValue<'a> for $name {
            type Checker = glib::value::GenericValueTypeChecker<Self>;

            unsafe fn from_value(value: &'a glib::Value) -> Self {
                Self::from(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
            }
        }

        impl glib::value::ToValue for $name {
            fn to_value(&self) -> glib::Value {
                let mut value = glib::Value::for_value_type::<Self>();
                unsafe {
                    glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, (*self).into());
                }
                value
            }

            fn value_type(&self) -> glib::Type {
                <Self as glib::prelude::StaticType>::static_type()
            }
        }

        impl From<$name> for glib::Value {
            #[inline]
            fn from(v: $name) -> Self {
                glib::value::ToValue::to_value(&v)
            }
        }
    };
}

/// Specifies the type of antialiasing to do when rendering text or shapes.
///
/// `cairo 1.12` added a set of antialiasing hints, rather than specifying a specific antialias
/// method. These hints are:
///
/// - [`Antialias::Fast`]: Allow the backend to degrade raster quality for speed.
/// - [`Antialias::Good`]: Balance between speed and quality.
/// - [`Antialias::Best`]: High-fidelity, but potentially slow, raster mode.
///
/// These make no guarantee on how the backend will perform its rasterisation (if it even
/// rasterises!), nor that they have any differing effect other than to enable some form of
/// antialiasing. In the case of glyph rendering, [`Antialias::Fast`] and [`Antialias::Good`] will
/// be mapped to [`Antialias::Gray`], with [`Antialias::Best`] being equivalent to
/// [`Antialias::Subpixel`].
///
/// The interpretation of [`Antialias::Default`] is left entirely up to the backend. Typically,
/// this will be similar to [`Antialias::Good`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_antialias_t")]
pub enum Antialias {
    /// Use the default antialiasing for the subsystem and target device.
    #[doc(alias = "ANTIALIAS_DEFAULT")]
    Default,

    /// Use a bilevel alpha mask.
    #[doc(alias = "ANTIALIAS_NONE")]
    None,

    /// Perform single-color antialiasing (using shades of gray for black text on a white
    /// background, for example).
    #[doc(alias = "ANTIALIAS_GRAY")]
    Gray,

    /// Perform antialiasing by taking advantage of the order of subpixel elements on devices such
    /// as LCD panels.
    #[doc(alias = "ANTIALIAS_SUBPIXEL")]
    Subpixel,

    /* hints */
    /// Hint that the backend should perform some antialiasing, but prefer speed over quality.
    #[doc(alias = "ANTIALIAS_FAST")]
    Fast,

    /// Hint that the backend should balance quality against performance.
    #[doc(alias = "ANTIALIAS_GOOD")]
    Good,

    /// Hint that the backend should render at the highest quality, sacrificing speed if necessary.
    #[doc(alias = "ANTIALIAS_BEST")]
    Best,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Antialias> for ffi::cairo_antialias_t {
    fn from(val: Antialias) -> ffi::cairo_antialias_t {
        match val {
            Antialias::Default => ffi::ANTIALIAS_DEFAULT,
            Antialias::None => ffi::ANTIALIAS_NONE,
            Antialias::Gray => ffi::ANTIALIAS_GRAY,
            Antialias::Subpixel => ffi::ANTIALIAS_SUBPIXEL,
            Antialias::Fast => ffi::ANTIALIAS_FAST,
            Antialias::Good => ffi::ANTIALIAS_GOOD,
            Antialias::Best => ffi::ANTIALIAS_BEST,
            Antialias::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_antialias_t> for Antialias {
    fn from(value: ffi::cairo_antialias_t) -> Self {
        match value {
            ffi::ANTIALIAS_DEFAULT => Self::Default,
            ffi::ANTIALIAS_NONE => Self::None,
            ffi::ANTIALIAS_GRAY => Self::Gray,
            ffi::ANTIALIAS_SUBPIXEL => Self::Subpixel,
            ffi::ANTIALIAS_FAST => Self::Fast,
            ffi::ANTIALIAS_GOOD => Self::Good,
            ffi::ANTIALIAS_BEST => Self::Best,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Antialias, ffi::gobject::cairo_gobject_antialias_get_type);

/// Select how paths are filled.
///
/// For both fill rules, whether or not a point is included in the fill is determined by taking a
/// ray from that point to infinity, and looking at intersections with the path. The ray can be in
/// any direction, as long as it doesn't pass through the end point of a segment, or have a tricky
/// intersection, such as intersecting tangent to the path.
///
/// The default fill rule is [`FillRule::Winding`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_fill_rule_t")]
pub enum FillRule {
    /// Count the number of times the ray crosses the path from left to right.
    ///
    /// When crossing a path from left to right, increment the count by 1. When crossing from right
    /// to left, decrement the count by 1. If the result is non-zero, the point will be filled.
    #[doc(alias = "FILL_RULE_WINDING")]
    Winding,

    /// Count the number of intersections of the ray with the path, with no regard to the
    /// orientation of the crossing.
    ///
    /// If the total number of intersections is odd, the point will be filled.
    #[doc(alias = "FILL_RULE_EVEN_ODD")]
    EvenOdd,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FillRule> for ffi::cairo_fill_rule_t {
    fn from(val: FillRule) -> ffi::cairo_fill_rule_t {
        match val {
            FillRule::Winding => ffi::FILL_RULE_WINDING,
            FillRule::EvenOdd => ffi::FILL_RULE_EVEN_ODD,
            FillRule::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_fill_rule_t> for FillRule {
    fn from(value: ffi::cairo_fill_rule_t) -> Self {
        match value {
            ffi::FILL_RULE_WINDING => Self::Winding,
            ffi::FILL_RULE_EVEN_ODD => Self::EvenOdd,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FillRule, ffi::gobject::cairo_gobject_fill_rule_get_type);

/// Specifies how to render endpoints of paths when stroking.
///
/// The default line cap style is [`LineCap::Butt`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_line_cap_t")]
pub enum LineCap {
    /// Begin and end the line exactly at the start and end points.
    #[doc(alias = "LINE_CAP_BUTT")]
    Butt,

    /// Use a round ending, with the center of the circle at the end point.
    #[doc(alias = "LINE_CAP_ROUND")]
    Round,

    /// Use squared-off ending, with the center at the end point.
    #[doc(alias = "LINE_CAP_SQUARE")]
    Square,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<LineCap> for ffi::cairo_line_cap_t {
    fn from(val: LineCap) -> ffi::cairo_line_cap_t {
        match val {
            LineCap::Butt => ffi::LINE_CAP_BUTT,
            LineCap::Round => ffi::LINE_CAP_ROUND,
            LineCap::Square => ffi::LINE_CAP_SQUARE,
            LineCap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_line_cap_t> for LineCap {
    fn from(value: ffi::cairo_line_cap_t) -> Self {
        match value {
            ffi::LINE_CAP_BUTT => Self::Butt,
            ffi::LINE_CAP_ROUND => Self::Round,
            ffi::LINE_CAP_SQUARE => Self::Square,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineCap, ffi::gobject::cairo_gobject_line_cap_get_type);

/// Specifies how to render the junction of two lines when stroking.
///
/// The default line join style is [`LineJoin::Miter`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_line_join_t")]
pub enum LineJoin {
    /// Use a sharp (angled) corner.
    ///
    /// See [`Context::set_miter_limit`] for more details.
    ///
    /// [`Context::set_miter_limit`]: crate::Context::set_miter_limit
    #[doc(alias = "LINE_JOIN_MITER")]
    Miter,

    /// Use a rounded join, with the center of the circle at the join point.
    #[doc(alias = "LINE_JOIN_ROUND")]
    Round,

    /// Use a cut-off join, with the join cut off at half the line width from the join point.
    #[doc(alias = "LINE_JOIN_BEVEL")]
    Bevel,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<LineJoin> for ffi::cairo_line_join_t {
    fn from(val: LineJoin) -> ffi::cairo_line_join_t {
        match val {
            LineJoin::Miter => ffi::LINE_JOIN_MITER,
            LineJoin::Round => ffi::LINE_JOIN_ROUND,
            LineJoin::Bevel => ffi::LINE_JOIN_BEVEL,
            LineJoin::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_line_join_t> for LineJoin {
    fn from(value: ffi::cairo_line_join_t) -> Self {
        match value {
            ffi::LINE_JOIN_MITER => Self::Miter,
            ffi::LINE_JOIN_ROUND => Self::Round,
            ffi::LINE_JOIN_BEVEL => Self::Bevel,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(LineJoin, ffi::gobject::cairo_gobject_line_join_get_type);

/// Set the compositing operator to be used for all drawing operations.
///
/// Operators marked as **unbounded** will modify their destination, even outside of the mask
/// layer. Their effect can still be limited by clipping.
///
/// For a detailed discussion of the effects of each operator, see the [Cairo operator
/// documentation](https://www.cairographics.org/operators/).
///
/// The default operator is [`Operator::Over`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_operator_t")]
pub enum Operator {
    /// Clear the destination layer.
    #[doc(alias = "OPERATOR_CLEAR")]
    Clear,

    /// Replace the destination layer.
    #[doc(alias = "OPERATOR_SOURCE")]
    Source,

    /// Draw the source layer on top of the destination layer.
    #[doc(alias = "OPERATOR_OVER")]
    Over,

    /// **Unbounded**: Draw the source where there was destination content.
    #[doc(alias = "OPERATOR_IN")]
    In,

    /// **Unbounded**: Draw the source where there was no destination content.
    #[doc(alias = "OPERATOR_OUT")]
    Out,

    /// Draw the source on top of the destination and only there.
    #[doc(alias = "OPERATOR_ATOP")]
    Atop,

    /// Ignore the source, drawing only destination content.
    #[doc(alias = "OPERATOR_DEST")]
    Dest,

    /// Ignore the destination, drawing only source content.
    #[doc(alias = "OPERATOR_DEST_OVER")]
    DestOver,

    /// Leave destination content only where there was source content.
    #[doc(alias = "OPERATOR_DEST_IN")]
    DestIn,

    /// Leave destination content only where there was no source content.
    #[doc(alias = "OPERATOR_DEST_OUT")]
    DestOut,

    /// Leave destination content on top of the source and only there.
    #[doc(alias = "OPERATOR_DEST_ATOP")]
    DestAtop,

    /// Source and destination are shown only where there is no overlap.
    #[doc(alias = "OPERATOR_XOR")]
    Xor,

    /// Source and destination layers are accumulated.
    #[doc(alias = "OPERATOR_ADD")]
    Add,

    /// Like [`Operator::Over`], but assumes the source and destination are disjoint geometries.
    #[doc(alias = "OPERATOR_SATURATE")]
    Saturate,

    /// The source and destination layers are multiplied. This causes the result to be at least as
    /// dark as the darker inputs.
    #[doc(alias = "OPERATOR_MULTIPLY")]
    Multiply,

    /// The source and destination layers are complemented and multiplied. This causes the result
    /// to be at least as light as the lighter inputs.
    #[doc(alias = "OPERATOR_SCREEN")]
    Screen,

    /// Chooses [`Operator::Multiply`] or [`Operator::Screen`], depending on the lightness of the
    /// destination color.
    #[doc(alias = "OPERATOR_OVERLAY")]
    Overlay,

    /// Replaces the destination layer with the source layer if the source layer is darker;
    /// otherwise, keeps the source layer.
    #[doc(alias = "OPERATOR_DARKEN")]
    Darken,

    /// Replaces the destination layer with the source layer if the source layer is lighter;
    /// otherwise, keeps the source layer.
    #[doc(alias = "OPERATOR_LIGHTEN")]
    Lighten,

    /// Brightens the destination color to reflect the source color.
    #[doc(alias = "OPERATOR_COLOR_DODGE")]
    ColorDodge,

    /// Darkens the destination color to reflect the source color.
    #[doc(alias = "OPERATOR_COLOR_BURN")]
    ColorBurn,

    /// Chooses [`Operator::Multiply`] or [`Operator::Screen`], depending on the source color.
    #[doc(alias = "OPERATOR_HARD_LIGHT")]
    HardLight,

    /// Chooses [`Operator::ColorDodge`] or [`Operator::ColorBurn`], depending on the source color.
    #[doc(alias = "OPERATOR_SOFT_LIGHT")]
    SoftLight,

    /// Takes the difference of the source and destination color.
    #[doc(alias = "OPERATOR_DIFFERENCE")]
    Difference,

    /// Like [`Operator::Difference`], but with lower contrast.
    #[doc(alias = "OPERATOR_EXCLUSION")]
    Exclusion,

    /// Creates a color with the hue of the source, and the saturation and luminosity of the
    /// destination.
    #[doc(alias = "OPERATOR_HSL_HUE")]
    HslHue,

    /// Creates a color with the saturation of the source, and the hue and luminosity of the
    /// destination.
    #[doc(alias = "OPERATOR_HSL_SATURATION")]
    HslSaturation,

    /// Creates a color with the hue and saturation of the source, and the luminosity of the
    /// destination.
    #[doc(alias = "OPERATOR_HSL_COLOR")]
    HslColor,

    /// Creates a color with the luminosity of the source, and the hue and saturation of the
    /// destination.
    #[doc(alias = "OPERATOR_HSL_LUMINOSITY")]
    HslLuminosity,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Operator> for ffi::cairo_operator_t {
    fn from(val: Operator) -> ffi::cairo_operator_t {
        match val {
            Operator::Clear => ffi::OPERATOR_CLEAR,
            Operator::Source => ffi::OPERATOR_SOURCE,
            Operator::Over => ffi::OPERATOR_OVER,
            Operator::In => ffi::OPERATOR_IN,
            Operator::Out => ffi::OPERATOR_OUT,
            Operator::Atop => ffi::OPERATOR_ATOP,
            Operator::Dest => ffi::OPERATOR_DEST,
            Operator::DestOver => ffi::OPERATOR_DEST_OVER,
            Operator::DestIn => ffi::OPERATOR_DEST_IN,
            Operator::DestOut => ffi::OPERATOR_DEST_OUT,
            Operator::DestAtop => ffi::OPERATOR_DEST_ATOP,
            Operator::Xor => ffi::OPERATOR_XOR,
            Operator::Add => ffi::OPERATOR_ADD,
            Operator::Saturate => ffi::OPERATOR_SATURATE,
            Operator::Multiply => ffi::OPERATOR_MULTIPLY,
            Operator::Screen => ffi::OPERATOR_SCREEN,
            Operator::Overlay => ffi::OPERATOR_OVERLAY,
            Operator::Darken => ffi::OPERATOR_DARKEN,
            Operator::Lighten => ffi::OPERATOR_LIGHTEN,
            Operator::ColorDodge => ffi::OPERATOR_COLOR_DODGE,
            Operator::ColorBurn => ffi::OPERATOR_COLOR_BURN,
            Operator::HardLight => ffi::OPERATOR_HARD_LIGHT,
            Operator::SoftLight => ffi::OPERATOR_SOFT_LIGHT,
            Operator::Difference => ffi::OPERATOR_DIFFERENCE,
            Operator::Exclusion => ffi::OPERATOR_EXCLUSION,
            Operator::HslHue => ffi::OPERATOR_HSL_HUE,
            Operator::HslSaturation => ffi::OPERATOR_HSL_SATURATION,
            Operator::HslColor => ffi::OPERATOR_HSL_COLOR,
            Operator::HslLuminosity => ffi::OPERATOR_HSL_LUMINOSITY,
            Operator::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_operator_t> for Operator {
    fn from(value: ffi::cairo_operator_t) -> Self {
        match value {
            ffi::OPERATOR_CLEAR => Self::Clear,
            ffi::OPERATOR_SOURCE => Self::Source,
            ffi::OPERATOR_OVER => Self::Over,
            ffi::OPERATOR_IN => Self::In,
            ffi::OPERATOR_OUT => Self::Out,
            ffi::OPERATOR_ATOP => Self::Atop,
            ffi::OPERATOR_DEST => Self::Dest,
            ffi::OPERATOR_DEST_OVER => Self::DestOver,
            ffi::OPERATOR_DEST_IN => Self::DestIn,
            ffi::OPERATOR_DEST_OUT => Self::DestOut,
            ffi::OPERATOR_DEST_ATOP => Self::DestAtop,
            ffi::OPERATOR_XOR => Self::Xor,
            ffi::OPERATOR_ADD => Self::Add,
            ffi::OPERATOR_SATURATE => Self::Saturate,
            ffi::OPERATOR_MULTIPLY => Self::Multiply,
            ffi::OPERATOR_SCREEN => Self::Screen,
            ffi::OPERATOR_OVERLAY => Self::Overlay,
            ffi::OPERATOR_DARKEN => Self::Darken,
            ffi::OPERATOR_LIGHTEN => Self::Lighten,
            ffi::OPERATOR_COLOR_DODGE => Self::ColorDodge,
            ffi::OPERATOR_COLOR_BURN => Self::ColorBurn,
            ffi::OPERATOR_HARD_LIGHT => Self::HardLight,
            ffi::OPERATOR_SOFT_LIGHT => Self::SoftLight,
            ffi::OPERATOR_DIFFERENCE => Self::Difference,
            ffi::OPERATOR_EXCLUSION => Self::Exclusion,
            ffi::OPERATOR_HSL_HUE => Self::HslHue,
            ffi::OPERATOR_HSL_SATURATION => Self::HslSaturation,
            ffi::OPERATOR_HSL_COLOR => Self::HslColor,
            ffi::OPERATOR_HSL_LUMINOSITY => Self::HslLuminosity,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Operator, ffi::gobject::cairo_gobject_operator_get_type);

/// Describes the type of a portion of a [`Path`].
///
/// [`Path`]: crate::Path
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_path_data_type_t")]
pub enum PathDataType {
    #[doc(alias = "PATH_DATA_TYPE_MOVE_TO")]
    MoveTo,
    #[doc(alias = "PATH_DATA_TYPE_LINE_TO")]
    LineTo,
    #[doc(alias = "PATH_DATA_TYPE_CURVE_TO")]
    CurveTo,
    #[doc(alias = "PATH_DATA_TYPE_CLOSE_PATH")]
    ClosePath,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<PathDataType> for ffi::cairo_path_data_type_t {
    fn from(val: PathDataType) -> ffi::cairo_path_data_type_t {
        match val {
            PathDataType::MoveTo => ffi::PATH_DATA_TYPE_MOVE_TO,
            PathDataType::LineTo => ffi::PATH_DATA_TYPE_LINE_TO,
            PathDataType::CurveTo => ffi::PATH_DATA_TYPE_CURVE_TO,
            PathDataType::ClosePath => ffi::PATH_DATA_TYPE_CLOSE_PATH,
            PathDataType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_path_data_type_t> for PathDataType {
    fn from(value: ffi::cairo_path_data_type_t) -> Self {
        match value {
            ffi::PATH_DATA_TYPE_MOVE_TO => Self::MoveTo,
            ffi::PATH_DATA_TYPE_LINE_TO => Self::LineTo,
            ffi::PATH_DATA_TYPE_CURVE_TO => Self::CurveTo,
            ffi::PATH_DATA_TYPE_CLOSE_PATH => Self::ClosePath,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    PathDataType,
    ffi::gobject::cairo_gobject_path_data_type_get_type
);

/// Describes the content a [`Surface`] will contain.
///
/// [`Surface`]: crate::Surface
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_content_t")]
pub enum Content {
    /// The [`Surface`] will hold color content only.
    ///
    /// [`Surface`]: crate::Surface
    #[doc(alias = "CONTENT_COLOR")]
    Color,

    /// The [`Surface`] will hold alpha content only.
    ///
    /// [`Surface`]: crate::Surface
    #[doc(alias = "CONTENT_ALPHA")]
    Alpha,

    /// The [`Surface`] will hold both color and alpha content.
    ///
    /// [`Surface`]: crate::Surface
    #[doc(alias = "CONTENT_COLOR_ALPHA")]
    ColorAlpha,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Content> for ffi::cairo_content_t {
    fn from(val: Content) -> ffi::cairo_content_t {
        match val {
            Content::Color => ffi::CONTENT_COLOR,
            Content::Alpha => ffi::CONTENT_ALPHA,
            Content::ColorAlpha => ffi::CONTENT_COLOR_ALPHA,
            Content::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_content_t> for Content {
    fn from(value: ffi::cairo_content_t) -> Self {
        match value {
            ffi::CONTENT_COLOR => Self::Color,
            ffi::CONTENT_ALPHA => Self::Alpha,
            ffi::CONTENT_COLOR_ALPHA => Self::ColorAlpha,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Content, ffi::gobject::cairo_gobject_content_get_type);

/// Describes how pattern color / alpha is determined for areas "outside" the pattern's natural
/// area, (for example, outside the surface bounds or outside the gradient geometry).
///
/// Mesh patterns are not affected by this setting.
///
/// The default extend mode is [`Extend::None`] for surface patterns and [`Extend::Pad`] for
/// gradient patterns.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_extend_t")]
pub enum Extend {
    /// Pixels outside of the source pattern are fully transparent.
    #[doc(alias = "EXTEND_NONE")]
    None,

    /// The pattern is tiled by repeating.
    #[doc(alias = "EXTEND_REPEAT")]
    Repeat,

    /// The pattern is tiled by reflecting at the edges.
    #[doc(alias = "EXTEND_REFLECT")]
    Reflect,

    /// Pixels outside of the pattern copy the closest pixel from the source.
    #[doc(alias = "EXTEND_PAD")]
    Pad,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Extend> for ffi::cairo_extend_t {
    fn from(val: Extend) -> ffi::cairo_extend_t {
        match val {
            Extend::None => ffi::EXTEND_NONE,
            Extend::Repeat => ffi::EXTEND_REPEAT,
            Extend::Reflect => ffi::EXTEND_REFLECT,
            Extend::Pad => ffi::EXTEND_PAD,
            Extend::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_extend_t> for Extend {
    fn from(value: ffi::cairo_extend_t) -> Self {
        match value {
            ffi::EXTEND_NONE => Self::None,
            ffi::EXTEND_REPEAT => Self::Repeat,
            ffi::EXTEND_REFLECT => Self::Reflect,
            ffi::EXTEND_PAD => Self::Pad,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Extend, ffi::gobject::cairo_gobject_extend_get_type);

/// Indicates the filtering to apply when reading pixel values from [`Pattern`]s.
///
/// [`Pattern`]: crate::Pattern
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_filter_t")]
pub enum Filter {
    /// High-performance filter with quality similar to [`Filter::Nearest`].
    #[doc(alias = "FILTER_FAST")]
    Fast,

    /// Reasonable-performance filter with quality similar to [`Filter::Bilinear`].
    #[doc(alias = "FILTER_GOOD")]
    Good,

    /// Highest-quality filter; performance may not be suitable for interactive use.
    #[doc(alias = "FILTER_BEST")]
    Best,

    /// Nearest-neighbor filtering.
    #[doc(alias = "FILTER_NEAREST")]
    Nearest,

    /// Linear interpolation in two dimensions.
    #[doc(alias = "FILTER_BILINEAR")]
    Bilinear,

    /// Gaussian interpolation in two dimensions.
    #[doc(alias = "FILTER_GAUSSIAN")]
    Gaussian,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Filter> for ffi::cairo_filter_t {
    fn from(val: Filter) -> ffi::cairo_filter_t {
        match val {
            Filter::Fast => ffi::FILTER_FAST,
            Filter::Good => ffi::FILTER_GOOD,
            Filter::Best => ffi::FILTER_BEST,
            Filter::Nearest => ffi::FILTER_NEAREST,
            Filter::Bilinear => ffi::FILTER_BILINEAR,
            Filter::Gaussian => ffi::FILTER_GAUSSIAN,
            Filter::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_filter_t> for Filter {
    fn from(value: ffi::cairo_filter_t) -> Self {
        match value {
            ffi::FILTER_FAST => Self::Fast,
            ffi::FILTER_GOOD => Self::Good,
            ffi::FILTER_BEST => Self::Best,
            ffi::FILTER_NEAREST => Self::Nearest,
            ffi::FILTER_BILINEAR => Self::Bilinear,
            ffi::FILTER_GAUSSIAN => Self::Gaussian,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Filter, ffi::gobject::cairo_gobject_filter_get_type);

/// Describes the type of a [`Pattern`].
///
/// If you are looking to create a specific type of pattern, you should use the appropriate
/// constructor method with one of the `struct`s below:
///
/// - [`SolidPattern`]
/// - [`SurfacePattern`]
/// - [`LinearGradient`]
/// - [`RadialGradient`]
/// - [`Mesh`]
///
/// [`Pattern`]: crate::Pattern
/// [`SolidPattern`]: crate::SolidPattern
/// [`SurfacePattern`]: crate::SurfacePattern
/// [`LinearGradient`]: crate::LinearGradient
/// [`RadialGradient`]: crate::RadialGradient
/// [`Mesh`]: crate::Mesh
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_pattern_type_t")]
pub enum PatternType {
    /// The pattern is a solid opaque / translucent color.
    #[doc(alias = "PATTERN_TYPE_SOLID")]
    Solid,

    /// The pattern is based on a [`Surface`].
    ///
    /// [`Surface`]: crate::Surface
    #[doc(alias = "PATTERN_TYPE_SURFACE")]
    Surface,

    /// The pattern is a linear gradient.
    #[doc(alias = "PATTERN_TYPE_LINEAR_GRADIENT")]
    LinearGradient,

    /// The pattern is a radial gradient.
    #[doc(alias = "PATTERN_TYPE_RADIAL_GRADIENT")]
    RadialGradient,

    /// The pattern is a mesh.
    #[doc(alias = "PATTERN_TYPE_MESH")]
    Mesh,

    /// The pattern is a user providing raster data.
    #[doc(alias = "PATTERN_TYPE_RASTER_SOURCE")]
    RasterSource,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<PatternType> for ffi::cairo_pattern_type_t {
    fn from(val: PatternType) -> ffi::cairo_pattern_type_t {
        match val {
            PatternType::Solid => ffi::PATTERN_TYPE_SOLID,
            PatternType::Surface => ffi::PATTERN_TYPE_SURFACE,
            PatternType::LinearGradient => ffi::PATTERN_TYPE_LINEAR_GRADIENT,
            PatternType::RadialGradient => ffi::PATTERN_TYPE_RADIAL_GRADIENT,
            PatternType::Mesh => ffi::PATTERN_TYPE_MESH,
            PatternType::RasterSource => ffi::PATTERN_TYPE_RASTER_SOURCE,
            PatternType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_pattern_type_t> for PatternType {
    fn from(value: ffi::cairo_pattern_type_t) -> Self {
        match value {
            ffi::PATTERN_TYPE_SOLID => Self::Solid,
            ffi::PATTERN_TYPE_SURFACE => Self::Surface,
            ffi::PATTERN_TYPE_LINEAR_GRADIENT => Self::LinearGradient,
            ffi::PATTERN_TYPE_RADIAL_GRADIENT => Self::RadialGradient,
            ffi::PATTERN_TYPE_MESH => Self::Mesh,
            ffi::PATTERN_TYPE_RASTER_SOURCE => Self::RasterSource,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    PatternType,
    ffi::gobject::cairo_gobject_pattern_type_get_type
);

/// Variants of a font face based on their slant.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_font_slant_t")]
pub enum FontSlant {
    /// Text is rendered upright.
    #[doc(alias = "FONT_SLANT_NORMAL")]
    Normal,

    /// Text is rendered with an italic slant.
    #[doc(alias = "FONT_SLANT_ITALIC")]
    Italic,

    /// Text is rendered with an oblique slant.
    #[doc(alias = "FONT_SLANT_OBLIQUE")]
    Oblique,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FontSlant> for ffi::cairo_font_slant_t {
    fn from(val: FontSlant) -> ffi::cairo_font_slant_t {
        match val {
            FontSlant::Normal => ffi::FONT_SLANT_NORMAL,
            FontSlant::Italic => ffi::FONT_SLANT_ITALIC,
            FontSlant::Oblique => ffi::FONT_SLANT_OBLIQUE,
            FontSlant::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_slant_t> for FontSlant {
    fn from(value: ffi::cairo_font_slant_t) -> Self {
        match value {
            ffi::FONT_SLANT_NORMAL => Self::Normal,
            ffi::FONT_SLANT_ITALIC => Self::Italic,
            ffi::FONT_SLANT_OBLIQUE => Self::Oblique,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontSlant, ffi::gobject::cairo_gobject_font_slant_get_type);

/// Variants of a font face based on their weight.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_font_weight_t")]
pub enum FontWeight {
    /// Normal font weight.
    #[doc(alias = "FONT_WEIGHT_NORMAL")]
    Normal,

    /// Bolded font weight.
    #[doc(alias = "FONT_WEIGHT_BOLD")]
    Bold,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FontWeight> for ffi::cairo_font_weight_t {
    fn from(val: FontWeight) -> ffi::cairo_font_weight_t {
        match val {
            FontWeight::Normal => ffi::FONT_WEIGHT_NORMAL,
            FontWeight::Bold => ffi::FONT_WEIGHT_BOLD,
            FontWeight::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_weight_t> for FontWeight {
    fn from(value: ffi::cairo_font_weight_t) -> Self {
        match value {
            ffi::FONT_WEIGHT_NORMAL => Self::Normal,
            ffi::FONT_WEIGHT_BOLD => Self::Bold,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontWeight, ffi::gobject::cairo_gobject_font_weight_get_type);

/// Specifies properties of a text cluster mapping.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_text_cluster_flags_t")]
pub enum TextClusterFlags {
    /// The clusters in the cluster array map to glyphs in the glyph array from start to end.
    #[doc(alias = "TEXT_CLUSTER_FLAGS_NONE")]
    None,

    /// The clusters in the cluster array map to glyphs in the glyph array from end to start.
    #[doc(alias = "TEXT_CLUSTER_FLAGS_BACKWARD")]
    Backward,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<TextClusterFlags> for ffi::cairo_text_cluster_flags_t {
    fn from(val: TextClusterFlags) -> ffi::cairo_text_cluster_flags_t {
        match val {
            TextClusterFlags::None => ffi::TEXT_CLUSTER_FLAGS_NONE,
            TextClusterFlags::Backward => ffi::TEXT_CLUSTER_FLAGS_BACKWARD,
            TextClusterFlags::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_text_cluster_flags_t> for TextClusterFlags {
    fn from(value: ffi::cairo_text_cluster_flags_t) -> Self {
        match value {
            ffi::TEXT_CLUSTER_FLAGS_NONE => Self::None,
            ffi::TEXT_CLUSTER_FLAGS_BACKWARD => Self::Backward,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    TextClusterFlags,
    ffi::gobject::cairo_gobject_text_cluster_flags_get_type
);

/// Describes the type of a [`FontFace`] or [`ScaledFont`], also known as "font backends" within
/// Cairo.
///
/// [`FontFace`]: crate::FontFace
/// [`ScaledFont`]: crate::ScaledFont
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_font_type_t")]
pub enum FontType {
    /// The font was created using [`FontFace::toy_create`].
    ///
    /// [`FontFace::toy_create`]: crate::FontFace::toy_create
    #[doc(alias = "FONT_TYPE_FONT_TYPE_TOY")]
    FontTypeToy,

    /// The font is of type FreeType.
    #[doc(alias = "FONT_TYPE_FONT_TYPE_FT")]
    FontTypeFt,

    /// The font is of type Win32.
    #[doc(alias = "FONT_TYPE_FONT_TYPE_WIN32")]
    FontTypeWin32,

    /// The font is of type Quartz.
    #[doc(alias = "FONT_TYPE_FONT_TYPE_QUARTZ")]
    FontTypeQuartz,

    /// The font is a [`UserFontFace`].
    ///
    /// [`UserFontFace`]: crate::UserFontFace
    #[doc(alias = "FONT_TYPE_FONT_TYPE_USER")]
    FontTypeUser,

    /// The font is of type Win32 DWrite.
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "FONT_TYPE_FONT_TYPE_DWRITE")]
    FontTypeDwrite,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<FontType> for ffi::cairo_font_type_t {
    fn from(val: FontType) -> ffi::cairo_font_type_t {
        match val {
            FontType::FontTypeToy => ffi::FONT_TYPE_FONT_TYPE_TOY,
            FontType::FontTypeFt => ffi::FONT_TYPE_FONT_TYPE_FT,
            FontType::FontTypeWin32 => ffi::FONT_TYPE_FONT_TYPE_WIN32,
            FontType::FontTypeQuartz => ffi::FONT_TYPE_FONT_TYPE_QUARTZ,
            FontType::FontTypeUser => ffi::FONT_TYPE_FONT_TYPE_USER,
            #[cfg(feature = "v1_18")]
            FontType::FontTypeDwrite => ffi::FONT_TYPE_FONT_TYPE_DWRITE,
            FontType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_font_type_t> for FontType {
    fn from(value: ffi::cairo_font_type_t) -> Self {
        match value {
            ffi::FONT_TYPE_FONT_TYPE_TOY => Self::FontTypeToy,
            ffi::FONT_TYPE_FONT_TYPE_FT => Self::FontTypeFt,
            ffi::FONT_TYPE_FONT_TYPE_WIN32 => Self::FontTypeWin32,
            ffi::FONT_TYPE_FONT_TYPE_QUARTZ => Self::FontTypeQuartz,
            ffi::FONT_TYPE_FONT_TYPE_USER => Self::FontTypeUser,
            #[cfg(feature = "v1_18")]
            ffi::FONT_TYPE_FONT_TYPE_DWRITE => Self::FontTypeDwrite,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(FontType, ffi::gobject::cairo_gobject_font_type_get_type);

/// Specifies the order of color elements within each pixel on the display
/// device, when rendering with [`Antialias::Subpixel`].
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_subpixel_order_t")]
pub enum SubpixelOrder {
    /// Use the default subpixel order for for the target device.
    #[doc(alias = "SUBPIXEL_ORDER_DEFAULT")]
    Default,

    /// Subpixel elements are arranged horizontally with red at the left.
    #[doc(alias = "SUBPIXEL_ORDER_RGB")]
    Rgb,

    /// Subpixel elements are arranged horizontally with blue at the left.
    #[doc(alias = "SUBPIXEL_ORDER_BGR")]
    Bgr,

    /// Subpixel elements are arranged vertically with red at the top.
    #[doc(alias = "SUBPIXEL_ORDER_VRGB")]
    Vrgb,

    /// Subpixel elements are arranged vertically with blue at the top.
    #[doc(alias = "SUBPIXEL_ORDER_VBGR")]
    Vbgr,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<SubpixelOrder> for ffi::cairo_subpixel_order_t {
    fn from(val: SubpixelOrder) -> ffi::cairo_subpixel_order_t {
        match val {
            SubpixelOrder::Default => ffi::SUBPIXEL_ORDER_DEFAULT,
            SubpixelOrder::Rgb => ffi::SUBPIXEL_ORDER_RGB,
            SubpixelOrder::Bgr => ffi::SUBPIXEL_ORDER_BGR,
            SubpixelOrder::Vrgb => ffi::SUBPIXEL_ORDER_VRGB,
            SubpixelOrder::Vbgr => ffi::SUBPIXEL_ORDER_VBGR,
            SubpixelOrder::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_subpixel_order_t> for SubpixelOrder {
    fn from(value: ffi::cairo_subpixel_order_t) -> Self {
        match value {
            ffi::SUBPIXEL_ORDER_DEFAULT => Self::Default,
            ffi::SUBPIXEL_ORDER_RGB => Self::Rgb,
            ffi::SUBPIXEL_ORDER_BGR => Self::Bgr,
            ffi::SUBPIXEL_ORDER_VRGB => Self::Vrgb,
            ffi::SUBPIXEL_ORDER_VBGR => Self::Vbgr,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    SubpixelOrder,
    ffi::gobject::cairo_gobject_subpixel_order_get_type
);

/// Specify the type of hinting to do on font outlines.
///
/// Hinting is the process of fitting outlines to the pixel grid in order to improve the appearance
/// of the result. Since hinting outlines involves distorting them, it also reduces the
/// faithfulness to the original outline shapes.
///
/// Not all of the outline hinting styles are supported by all font backends.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_hint_style_t")]
pub enum HintStyle {
    /// Use the default hint style for the font backend and target device.
    #[doc(alias = "HINT_STYLE_DEFAULT")]
    Default,

    /// Do not hint outlines.
    #[doc(alias = "HINT_STYLE_NONE")]
    None,

    /// Hint outlines slightly to improve contrast while retaining good fidelity to the original.
    #[doc(alias = "HINT_STYLE_SLIGHT")]
    Slight,

    /// Hint outlines with medium strength giving a compromise between fidelity to the original and
    /// contrast.
    #[doc(alias = "HINT_STYLE_MEDIUM")]
    Medium,

    /// Hint outlines to maximize contrast.
    #[doc(alias = "HINT_STYLE_FULL")]
    Full,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<HintStyle> for ffi::cairo_hint_style_t {
    fn from(val: HintStyle) -> ffi::cairo_hint_style_t {
        match val {
            HintStyle::Default => ffi::HINT_STYLE_DEFAULT,
            HintStyle::None => ffi::HINT_STYLE_NONE,
            HintStyle::Slight => ffi::HINT_STYLE_SLIGHT,
            HintStyle::Medium => ffi::HINT_STYLE_MEDIUM,
            HintStyle::Full => ffi::HINT_STYLE_FULL,
            HintStyle::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_hint_style_t> for HintStyle {
    fn from(value: ffi::cairo_hint_style_t) -> Self {
        match value {
            ffi::HINT_STYLE_DEFAULT => Self::Default,
            ffi::HINT_STYLE_NONE => Self::None,
            ffi::HINT_STYLE_SLIGHT => Self::Slight,
            ffi::HINT_STYLE_MEDIUM => Self::Medium,
            ffi::HINT_STYLE_FULL => Self::Full,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(HintStyle, ffi::gobject::cairo_gobject_hint_style_get_type);

/// Whether to hint font metrics.
///
/// Hinting font metrics means quantizing them so that they are integer values in device space.
/// Doing this improves the consistency of letter and line spacing, however it also means that text
/// will be laid out differently at different zoom factors.
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_hint_metrics_t")]
pub enum HintMetrics {
    /// Hint metrics in the default manner for the font backend and target device.
    #[doc(alias = "HINT_METRICS_DEFAULT")]
    Default,

    /// Do not hint font metrics.
    #[doc(alias = "HINT_METRICS_OFF")]
    Off,

    /// Do hint font metrics.
    #[doc(alias = "HINT_METRICS_ON")]
    On,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<HintMetrics> for ffi::cairo_hint_metrics_t {
    fn from(val: HintMetrics) -> ffi::cairo_hint_metrics_t {
        match val {
            HintMetrics::Default => ffi::HINT_METRICS_DEFAULT,
            HintMetrics::Off => ffi::HINT_METRICS_OFF,
            HintMetrics::On => ffi::HINT_METRICS_ON,
            HintMetrics::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_hint_metrics_t> for HintMetrics {
    fn from(value: ffi::cairo_hint_metrics_t) -> Self {
        match value {
            ffi::HINT_METRICS_DEFAULT => Self::Default,
            ffi::HINT_METRICS_OFF => Self::Off,
            ffi::HINT_METRICS_ON => Self::On,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    HintMetrics,
    ffi::gobject::cairo_gobject_hint_metrics_get_type
);


/// Describes the type of a [`Surface`], also known as "surface backends" within Cairo.
///
/// If you are looking to create a specific type of surface, you should use the appropriate
/// constructor method with one of the `struct`s below:
///
/// - [`ImageSurface`]
/// - [`PdfSurface`]
/// - [`PsSurface`]
/// - [`SvgSurface`]
/// - [`XCBSurface`]
///
/// Note that each surface, with the exception of [`ImageSurface`], has a corresponding feature
/// flag that must be enabled in order to use it.
///
/// [`Surface`]: crate::Surface
/// [`ImageSurface`]: crate::ImageSurface
/// [`PdfSurface`]: crate::PdfSurface
/// [`PsSurface`]: crate::PsSurface
/// [`SvgSurface`]: crate::SvgSurface
/// [`XCBSurface`]: crate::XCBSurface
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_surface_type_t")]
pub enum SurfaceType {
    /// Type image.
    #[doc(alias = "SURFACE_TYPE_IMAGE")]
    Image,

    /// Type `pdf`.
    #[doc(alias = "SURFACE_TYPE_PDF")]
    Pdf,

    /// Type `ps`.
    #[doc(alias = "SURFACE_TYPE_PS")]
    Ps,

    /// Type `xlib`.
    #[doc(alias = "SURFACE_TYPE_XLIB")]
    Xlib,

    /// Type `xcb`.
    #[doc(alias = "SURFACE_TYPE_XCB")]
    Xcb,

    /// Type `glitz`.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_GLITZ")]
    Glitz,

    /// Type `quartz`.
    #[doc(alias = "SURFACE_TYPE_QUARTZ")]
    Quartz,

    /// Type `win32`.
    #[doc(alias = "SURFACE_TYPE_WIN32")]
    Win32,

    /// Type `beos`.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_BE_OS")]
    BeOs,

    /// Type `directfb`.
    #[doc(alias = "SURFACE_TYPE_DIRECT_FB")]
    DirectFb,

    /// Type `svg`.
    #[doc(alias = "SURFACE_TYPE_SVG")]
    Svg,

    /// Type `os2`.
    #[doc(alias = "SURFACE_TYPE_OS2")]
    Os2,

    /// Win32 printing surface.
    #[doc(alias = "SURFACE_TYPE_WIN32_PRINTING")]
    Win32Printing,

    /// Type `quartz_image`.
    #[doc(alias = "SURFACE_TYPE_QUARTZ_IMAGE")]
    QuartzImage,

    /// Type `script`.
    #[doc(alias = "SURFACE_TYPE_SCRIPT")]
    Script,

    /// Type `qt`.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_QT")]
    Qt,

    /// Type `recording`.
    #[doc(alias = "SURFACE_TYPE_RECORDING")]
    Recording,

    /// OpenVG drawing surface.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_VG")]
    Vg,

    /// OpenGL surface.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_GL")]
    Gl,

    /// Direct Render Manager surface.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_DRM")]
    Drm,

    /// Type `tee`.
    #[doc(alias = "SURFACE_TYPE_TEE")]
    Tee,

    /// Type `xml`.
    #[doc(alias = "SURFACE_TYPE_XML")]
    Xml,

    /// Type `skia`.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_SKIA")]
    Skia,

    /// Subsurface created with [`Surface::create_for_rectangle`].
    ///
    /// [`Surface::create_for_rectangle`]: crate::Surface::create_for_rectangle
    #[doc(alias = "SURFACE_TYPE_SUBSURFACE")]
    Subsurface,

    /// Type `cogl`.
    ///
    /// This surface type is deprecated and will never be set by Cairo.
    #[doc(alias = "SURFACE_TYPE_COGL")]
    Cogl,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<SurfaceType> for ffi::cairo_surface_type_t {
    fn from(val: SurfaceType) -> ffi::cairo_surface_type_t {
        match val {
            SurfaceType::Image => ffi::SURFACE_TYPE_IMAGE,
            SurfaceType::Pdf => ffi::SURFACE_TYPE_PDF,
            SurfaceType::Ps => ffi::SURFACE_TYPE_PS,
            SurfaceType::Xlib => ffi::SURFACE_TYPE_XLIB,
            SurfaceType::Xcb => ffi::SURFACE_TYPE_XCB,
            SurfaceType::Glitz => ffi::SURFACE_TYPE_GLITZ,
            SurfaceType::Quartz => ffi::SURFACE_TYPE_QUARTZ,
            SurfaceType::Win32 => ffi::SURFACE_TYPE_WIN32,
            SurfaceType::BeOs => ffi::SURFACE_TYPE_BE_OS,
            SurfaceType::DirectFb => ffi::SURFACE_TYPE_DIRECT_FB,
            SurfaceType::Svg => ffi::SURFACE_TYPE_SVG,
            SurfaceType::Os2 => ffi::SURFACE_TYPE_OS2,
            SurfaceType::Win32Printing => ffi::SURFACE_TYPE_WIN32_PRINTING,
            SurfaceType::QuartzImage => ffi::SURFACE_TYPE_QUARTZ_IMAGE,
            SurfaceType::Script => ffi::SURFACE_TYPE_SCRIPT,
            SurfaceType::Qt => ffi::SURFACE_TYPE_QT,
            SurfaceType::Recording => ffi::SURFACE_TYPE_RECORDING,
            SurfaceType::Vg => ffi::SURFACE_TYPE_VG,
            SurfaceType::Gl => ffi::SURFACE_TYPE_GL,
            SurfaceType::Drm => ffi::SURFACE_TYPE_DRM,
            SurfaceType::Tee => ffi::SURFACE_TYPE_TEE,
            SurfaceType::Xml => ffi::SURFACE_TYPE_XML,
            SurfaceType::Skia => ffi::SURFACE_TYPE_SKIA,
            SurfaceType::Subsurface => ffi::SURFACE_TYPE_SUBSURFACE,
            SurfaceType::Cogl => ffi::SURFACE_TYPE_COGL,
            SurfaceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_surface_type_t> for SurfaceType {
    fn from(value: ffi::cairo_surface_type_t) -> Self {
        match value {
            ffi::SURFACE_TYPE_IMAGE => Self::Image,
            ffi::SURFACE_TYPE_PDF => Self::Pdf,
            ffi::SURFACE_TYPE_PS => Self::Ps,
            ffi::SURFACE_TYPE_XLIB => Self::Xlib,
            ffi::SURFACE_TYPE_XCB => Self::Xcb,
            ffi::SURFACE_TYPE_GLITZ => Self::Glitz,
            ffi::SURFACE_TYPE_QUARTZ => Self::Quartz,
            ffi::SURFACE_TYPE_WIN32 => Self::Win32,
            ffi::SURFACE_TYPE_BE_OS => Self::BeOs,
            ffi::SURFACE_TYPE_DIRECT_FB => Self::DirectFb,
            ffi::SURFACE_TYPE_SVG => Self::Svg,
            ffi::SURFACE_TYPE_OS2 => Self::Os2,
            ffi::SURFACE_TYPE_WIN32_PRINTING => Self::Win32Printing,
            ffi::SURFACE_TYPE_QUARTZ_IMAGE => Self::QuartzImage,
            ffi::SURFACE_TYPE_SCRIPT => Self::Script,
            ffi::SURFACE_TYPE_QT => Self::Qt,
            ffi::SURFACE_TYPE_RECORDING => Self::Recording,
            ffi::SURFACE_TYPE_VG => Self::Vg,
            ffi::SURFACE_TYPE_GL => Self::Gl,
            ffi::SURFACE_TYPE_DRM => Self::Drm,
            ffi::SURFACE_TYPE_TEE => Self::Tee,
            ffi::SURFACE_TYPE_XML => Self::Xml,
            ffi::SURFACE_TYPE_SKIA => Self::Skia,
            ffi::SURFACE_TYPE_SUBSURFACE => Self::Subsurface,
            ffi::SURFACE_TYPE_COGL => Self::Cogl,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    SurfaceType,
    ffi::gobject::cairo_gobject_surface_type_get_type
);

/// Units of measurement that can be used for various coordinates and lengths in the SVG
/// specification, and when drawing to an [`SvgSurface`].
///
/// [`SvgSurface`]: crate::SvgSurface
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg(all(feature = "svg", feature = "v1_16"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "svg", feature = "v1_16"))))]
#[non_exhaustive]
#[doc(alias = "cairo_svg_unit_t")]
pub enum SvgUnit {
    /// User-specified unit; a value in the current coordinate system.
    ///
    /// If used in the root element for the initial coordinate system, it corresponds to
    /// [`SvgUnit::Px`].
    #[doc(alias = "SVG_UNIT_USER")]
    User,

    /// The size of the element's font.
    #[doc(alias = "SVG_UNIT_EM")]
    Em,

    /// The x-height of the element's font.
    #[doc(alias = "SVG_UNIT_EX")]
    Ex,

    /// Pixels. `1px` is equal to `1/96th` of an inch.
    #[doc(alias = "SVG_UNIT_PX")]
    Px,

    /// Inches. `1in` is equal to `2.54cm`.
    #[doc(alias = "SVG_UNIT_IN")]
    In,

    /// Centimeters. `1cm` is equal to `96px/2.54`.
    #[doc(alias = "SVG_UNIT_CM")]
    Cm,

    /// Millimeters. `1mm` is equal to `1/10th of 1cm`.
    #[doc(alias = "SVG_UNIT_MM")]
    Mm,

    /// Points. `1pt` is equal to `1/72nd of 1in`.
    #[doc(alias = "SVG_UNIT_PT")]
    Pt,

    /// Picas. `1pc` is equal to `12pt`.
    #[doc(alias = "SVG_UNIT_PC")]
    Pc,

    /// Some fraction of another reference value.
    #[doc(alias = "SVG_UNIT_PERCENT")]
    Percent,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
#[cfg(all(feature = "svg", feature = "v1_16"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "svg", feature = "v1_16"))))]
impl From<SvgUnit> for ffi::cairo_svg_unit_t {
    fn from(val: SvgUnit) -> ffi::cairo_svg_unit_t {
        match val {
            SvgUnit::User => ffi::SVG_UNIT_USER,
            SvgUnit::Em => ffi::SVG_UNIT_EM,
            SvgUnit::Ex => ffi::SVG_UNIT_EX,
            SvgUnit::Px => ffi::SVG_UNIT_PX,
            SvgUnit::In => ffi::SVG_UNIT_IN,
            SvgUnit::Cm => ffi::SVG_UNIT_CM,
            SvgUnit::Mm => ffi::SVG_UNIT_MM,
            SvgUnit::Pt => ffi::SVG_UNIT_PT,
            SvgUnit::Pc => ffi::SVG_UNIT_PC,
            SvgUnit::Percent => ffi::SVG_UNIT_PERCENT,
            SvgUnit::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
#[cfg(all(feature = "svg", feature = "v1_16"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "svg", feature = "v1_16"))))]
impl From<ffi::cairo_svg_unit_t> for SvgUnit {
    fn from(value: ffi::cairo_svg_unit_t) -> Self {
        match value {
            ffi::SVG_UNIT_USER => Self::User,
            ffi::SVG_UNIT_EM => Self::Em,
            ffi::SVG_UNIT_EX => Self::Ex,
            ffi::SVG_UNIT_PX => Self::Px,
            ffi::SVG_UNIT_IN => Self::In,
            ffi::SVG_UNIT_CM => Self::Cm,
            ffi::SVG_UNIT_MM => Self::Mm,
            ffi::SVG_UNIT_PT => Self::Pt,
            ffi::SVG_UNIT_PC => Self::Pc,
            ffi::SVG_UNIT_PERCENT => Self::Percent,
            value => Self::__Unknown(value),
        }
    }
}

/// Identifies the memory format of image data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_format_t")]
pub enum Format {
    /// No such format exists or is supported.
    #[doc(alias = "FORMAT_INVALID")]
    Invalid,

    /// Each pixel is a 32-bit quantity stored native-endian, with alpha in the upper 8 bits, then
    /// red, then green, then blue.
    ///
    /// Pre-multiplied alpha is used (i.e. 50% transparent red is `0x80800000`, not `0x80ff0000`).
    #[doc(alias = "FORMAT_A_RGB32")]
    ARgb32,

    /// Each pixel is a 32-bit quantity stored native-endian, with the upper 8 bits unused, and the
    /// remaining 24 bits containing red, green, and blue, in that order.
    #[doc(alias = "FORMAT_RGB24")]
    Rgb24,

    /// Each pixel is a 8-bit quantity holding an alpha value.
    #[doc(alias = "FORMAT_A8")]
    A8,

    /// Each pixel is a 1-bit quantity holding an alpha value.
    ///
    /// Pixels are packed together into 32-bit quantities. The ordering of the bits matches the
    /// endianness of the platform. On a big-endian machine, the first pixel is in the uppermost
    /// bit, and on a little-endian machine, the first pixel is in the least-significant bit.
    #[doc(alias = "FORMAT_A1")]
    A1,

    /// Each pixel is a 16-bit quantity with red in the upper 5 bits, green in the middle 6
    /// bits, and blue in the lower 5 bits.
    #[doc(alias = "FORMAT_RGB16_565")]
    Rgb16_565,

    /// Each pixel is a 30-bit quantity stored native-endian, with the 30 bits split evenly between
    /// red, green, and blue, in that order.
    #[doc(alias = "FORMAT_RGB30")]
    Rgb30,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<Format> for ffi::cairo_format_t {
    fn from(val: Format) -> ffi::cairo_format_t {
        match val {
            Format::Invalid => ffi::FORMAT_INVALID,
            Format::ARgb32 => ffi::FORMAT_A_RGB32,
            Format::Rgb24 => ffi::FORMAT_RGB24,
            Format::A8 => ffi::FORMAT_A8,
            Format::A1 => ffi::FORMAT_A1,
            Format::Rgb16_565 => ffi::FORMAT_RGB16_565,
            Format::Rgb30 => ffi::FORMAT_RGB30,
            Format::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_format_t> for Format {
    fn from(value: ffi::cairo_format_t) -> Self {
        match value {
            ffi::FORMAT_INVALID => Self::Invalid,
            ffi::FORMAT_A_RGB32 => Self::ARgb32,
            ffi::FORMAT_RGB24 => Self::Rgb24,
            ffi::FORMAT_A8 => Self::A8,
            ffi::FORMAT_A1 => Self::A1,
            ffi::FORMAT_RGB16_565 => Self::Rgb16_565,
            ffi::FORMAT_RGB30 => Self::Rgb30,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(Format, ffi::gobject::cairo_gobject_format_get_type);

impl Format {
    /// Provides a stride value that will respect all alignment requirements of the accelerated
    /// image-rendering code within Cairo.
    #[doc(alias = "cairo_format_stride_for_width")]
    pub fn stride_for_width(self, width: u32) -> Result<i32, Error> {
        assert!(width <= i32::MAX as u32);
        let width = width as i32;

        let stride = unsafe { ffi::cairo_format_stride_for_width(self.into(), width) };
        if stride == -1 {
            Err(Error::InvalidFormat)
        } else {
            Ok(stride)
        }
    }
}

/// The result of [`Region::contains_rectangle`].
///
/// [`Region::contains_rectangle`]: crate::Region::contains_rectangle
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_region_overlap_t")]
pub enum RegionOverlap {
    /// The contents are entirely inside the region.
    #[doc(alias = "REGION_OVERLAP_IN")]
    In,

    /// The contents are entirely outside the region.
    #[doc(alias = "REGION_OVERLAP_OUT")]
    Out,

    /// Some contents are inside and some are outside the region.
    #[doc(alias = "REGION_OVERLAP_PART")]
    Part,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<RegionOverlap> for ffi::cairo_region_overlap_t {
    fn from(val: RegionOverlap) -> ffi::cairo_region_overlap_t {
        match val {
            RegionOverlap::In => ffi::REGION_OVERLAP_IN,
            RegionOverlap::Out => ffi::REGION_OVERLAP_OUT,
            RegionOverlap::Part => ffi::REGION_OVERLAP_PART,
            RegionOverlap::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_region_overlap_t> for RegionOverlap {
    fn from(value: ffi::cairo_region_overlap_t) -> Self {
        match value {
            ffi::REGION_OVERLAP_IN => Self::In,
            ffi::REGION_OVERLAP_OUT => Self::Out,
            ffi::REGION_OVERLAP_PART => Self::Part,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(
    RegionOverlap,
    ffi::gobject::cairo_gobject_region_overlap_get_type
);

bitflags::bitflags! {
    /// Attributes of an outline item.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct PdfOutline: i32 {
        /// The outline item defaults to being open in the PDF viewer.
        #[doc(alias = "PDF_OUTLINE_FLAG_OPEN")]
        const OPEN = ffi::PDF_OUTLINE_FLAG_OPEN;

        /// The outline item is displayed in bold text.
        #[doc(alias = "PDF_OUTLINE_FLAG_BOLD")]
        const BOLD = ffi::PDF_OUTLINE_FLAG_BOLD;

        /// The outline item is displayed in italic text.
        #[doc(alias = "PDF_OUTLINE_FLAG_ITALIC")]
        const ITALIC = ffi::PDF_OUTLINE_FLAG_ITALIC;
    }
}

/// Specify which part of a PDF document's metadata to set.
#[cfg(feature = "pdf")]
#[cfg_attr(docsrs, doc(cfg(feature = "pdf")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_pdf_metadata_t")]
pub enum PdfMetadata {
    /// The document's title.
    #[doc(alias = "PDF_METADATA_TITLE")]
    Title,

    /// The document's author.
    #[doc(alias = "PDF_METADATA_AUTHOR")]
    Author,

    /// The document's subject.
    #[doc(alias = "PDF_METADATA_SUBJECT")]
    Subject,

    /// The document's keywords.
    #[doc(alias = "PDF_METADATA_KEYWORDS")]
    Keywords,

    /// The document's creator.
    #[doc(alias = "PDF_METADATA_CREATOR")]
    Creator,

    /// The document's creation date.
    #[doc(alias = "PDF_METADATA_CREATE_DATE")]
    CreateDate,

    /// The document's modification date.
    #[doc(alias = "PDF_METADATA_MOD_DATE")]
    ModDate,

    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(all(feature = "pdf", feature = "v1_16"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "pdf", feature = "v1_16"))))]
#[doc(hidden)]
impl From<PdfMetadata> for ffi::cairo_pdf_metadata_t {
    fn from(val: PdfMetadata) -> ffi::cairo_pdf_metadata_t {
        match val {
            PdfMetadata::Title => ffi::PDF_METADATA_TITLE,
            PdfMetadata::Author => ffi::PDF_METADATA_AUTHOR,
            PdfMetadata::Subject => ffi::PDF_METADATA_SUBJECT,
            PdfMetadata::Keywords => ffi::PDF_METADATA_KEYWORDS,
            PdfMetadata::Creator => ffi::PDF_METADATA_CREATOR,
            PdfMetadata::CreateDate => ffi::PDF_METADATA_CREATE_DATE,
            PdfMetadata::ModDate => ffi::PDF_METADATA_MOD_DATE,
            PdfMetadata::__Unknown(value) => value,
        }
    }
}

#[cfg(all(feature = "pdf", feature = "v1_16"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "pdf", feature = "v1_16"))))]
#[doc(hidden)]
impl From<ffi::cairo_pdf_metadata_t> for PdfMetadata {
    fn from(value: ffi::cairo_pdf_metadata_t) -> Self {
        match value {
            ffi::PDF_METADATA_TITLE => Self::Title,
            ffi::PDF_METADATA_AUTHOR => Self::Author,
            ffi::PDF_METADATA_SUBJECT => Self::Subject,
            ffi::PDF_METADATA_KEYWORDS => Self::Keywords,
            ffi::PDF_METADATA_CREATOR => Self::Creator,
            ffi::PDF_METADATA_CREATE_DATE => Self::CreateDate,
            ffi::PDF_METADATA_MOD_DATE => Self::ModDate,
            value => Self::__Unknown(value),
        }
    }
}

/// Specify the version of the PDF specification to use when generating a PDF document.
#[cfg(feature = "pdf")]
#[cfg_attr(docsrs, doc(cfg(feature = "pdf")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_pdf_version_t")]
pub enum PdfVersion {
    /// Version 1.4 of the PDF specification.
    #[doc(alias = "PDF_VERSION__1_4")]
    _1_4,

    /// Version 1.5 of the PDF specification.
    #[doc(alias = "PDF_VERSION__1_5")]
    _1_5,

    /// Version 1.6 of the PDF specification.
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "PDF_VERSION__1_6")]
    _1_6,

    /// Version 1.7 of the PDF specification.
    #[cfg(feature = "v1_18")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v1_18")))]
    #[doc(alias = "PDF_VERSION__1_7")]
    _1_7,

    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "pdf")]
#[cfg_attr(docsrs, doc(cfg(feature = "pdf")))]
#[doc(hidden)]
impl From<PdfVersion> for ffi::cairo_pdf_version_t {
    fn from(val: PdfVersion) -> ffi::cairo_pdf_version_t {
        match val {
            PdfVersion::_1_4 => ffi::PDF_VERSION__1_4,
            PdfVersion::_1_5 => ffi::PDF_VERSION__1_5,
            #[cfg(feature = "v1_18")]
            PdfVersion::_1_6 => ffi::PDF_VERSION__1_6,
            #[cfg(feature = "v1_18")]
            PdfVersion::_1_7 => ffi::PDF_VERSION__1_7,
            PdfVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "pdf")]
#[cfg_attr(docsrs, doc(cfg(feature = "pdf")))]
#[doc(hidden)]
impl From<ffi::cairo_pdf_version_t> for PdfVersion {
    fn from(value: ffi::cairo_pdf_version_t) -> Self {
        match value {
            ffi::PDF_VERSION__1_4 => Self::_1_4,
            ffi::PDF_VERSION__1_5 => Self::_1_5,
            #[cfg(feature = "v1_18")]
            ffi::PDF_VERSION__1_6 => Self::_1_6,
            #[cfg(feature = "v1_18")]
            ffi::PDF_VERSION__1_7 => Self::_1_7,
            value => Self::__Unknown(value),
        }
    }
}

/// Specify the version of the SVG specification to use when generating an SVG document.
#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_svg_version_t")]
pub enum SvgVersion {
    /// Version 1.1 of the SVG specification.
    #[doc(alias = "SVG_VERSION__1_1")]
    _1_1,

    /// Version 1.2 of the SVG specification.
    #[doc(alias = "SVG_VERSION__1_2")]
    _1_2,

    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
#[doc(hidden)]
impl From<SvgVersion> for ffi::cairo_svg_version_t {
    fn from(val: SvgVersion) -> ffi::cairo_svg_version_t {
        match val {
            SvgVersion::_1_1 => ffi::SVG_VERSION__1_1,
            SvgVersion::_1_2 => ffi::SVG_VERSION__1_2,
            SvgVersion::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
#[doc(hidden)]
impl From<ffi::cairo_svg_version_t> for SvgVersion {
    fn from(value: ffi::cairo_svg_version_t) -> Self {
        match value {
            ffi::SVG_VERSION__1_1 => Self::_1_1,
            ffi::SVG_VERSION__1_2 => Self::_1_2,
            value => Self::__Unknown(value),
        }
    }
}

/// Specify the language level of the PostScript Language Reference to use when generating a
/// PostScript document.
#[cfg(feature = "ps")]
#[cfg_attr(docsrs, doc(cfg(feature = "ps")))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "cairo_ps_level_t")]
pub enum PsLevel {
    /// Language level 2 of the PostScript specification.
    #[doc(alias = "PS_LEVEL__2")]
    _2,

    /// Language level 3 of the PostScript specification.
    #[doc(alias = "PS_LEVEL__3")]
    _3,

    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "ps")]
#[cfg_attr(docsrs, doc(cfg(feature = "ps")))]
#[doc(hidden)]
impl From<PsLevel> for ffi::cairo_ps_level_t {
    fn from(val: PsLevel) -> ffi::cairo_ps_level_t {
        match val {
            PsLevel::_2 => ffi::PS_LEVEL__2,
            PsLevel::_3 => ffi::PS_LEVEL__3,
            PsLevel::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "ps")]
#[cfg_attr(docsrs, doc(cfg(feature = "ps")))]
#[doc(hidden)]
impl From<ffi::cairo_ps_level_t> for PsLevel {
    fn from(value: ffi::cairo_ps_level_t) -> Self {
        match value {
            ffi::PS_LEVEL__2 => Self::_2,
            ffi::PS_LEVEL__3 => Self::_3,
            value => Self::__Unknown(value),
        }
    }
}

/// Specify which control point of a [`Mesh`] to set or get.
///
/// [`Mesh`]: crate::Mesh
#[derive(Clone, PartialEq, Eq, PartialOrd, Copy, Debug)]
#[non_exhaustive]
#[doc(alias = "cairo_mesh_corner_t")]
pub enum MeshCorner {
    /// Mesh corner 0 (the first control point).
    #[doc(alias = "MESH_CORNER_MESH_CORNER0")]
    MeshCorner0,

    /// Mesh corner 1.
    #[doc(alias = "MESH_CORNER_MESH_CORNER1")]
    MeshCorner1,

    /// Mesh corner 2.
    #[doc(alias = "MESH_CORNER_MESH_CORNER2")]
    MeshCorner2,

    /// Mesh corner 3 (the last control point).
    #[doc(alias = "MESH_CORNER_MESH_CORNER3")]
    MeshCorner3,

    #[doc(hidden)]
    __Unknown(u32),
}

#[doc(hidden)]
impl From<MeshCorner> for ffi::cairo_mesh_corner_t {
    fn from(val: MeshCorner) -> ffi::cairo_mesh_corner_t {
        match val {
            MeshCorner::MeshCorner0 => ffi::MESH_CORNER_MESH_CORNER0,
            MeshCorner::MeshCorner1 => ffi::MESH_CORNER_MESH_CORNER1,
            MeshCorner::MeshCorner2 => ffi::MESH_CORNER_MESH_CORNER2,
            MeshCorner::MeshCorner3 => ffi::MESH_CORNER_MESH_CORNER3,
            MeshCorner::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_mesh_corner_t> for MeshCorner {
    fn from(value: ffi::cairo_mesh_corner_t) -> Self {
        match value {
            ffi::MESH_CORNER_MESH_CORNER0 => Self::MeshCorner0,
            ffi::MESH_CORNER_MESH_CORNER1 => Self::MeshCorner1,
            ffi::MESH_CORNER_MESH_CORNER2 => Self::MeshCorner2,
            ffi::MESH_CORNER_MESH_CORNER3 => Self::MeshCorner3,
            value => Self::__Unknown(value),
        }
    }
}

/// Flags to control how FreeType renders the glyphs for a particular [`FontFace`].
///
/// FreeType provides the ability to synthesize different glyphs from a base font, which is useful
/// if you lack those glyphs from a true bold or oblique font.
///
/// Note that when synthesizing glyphs, any generated [`FontExtents`] will only be estimates.
///
/// [`FontFace`]: crate::FontFace
/// [`FontExtents`]: crate::FontExtents
#[cfg(feature = "freetype")]
#[cfg_attr(docsrs, doc(cfg(feature = "freetype")))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_ft_synthesize_t")]
pub enum FtSynthesize {
    /// Embolden the glyphs (redraw them with a pixel offset).
    #[doc(alias = "CAIRO_FT_SYNTHESIZE_BOLD")]
    Bold,

    /// Slant the glyphs 12 degrees to the right.
    #[doc(alias = "CAIRO_FT_SYNTHESIZE_OBLIQUE")]
    Oblique,

    #[doc(hidden)]
    __Unknown(u32),
}

#[cfg(feature = "freetype")]
#[cfg_attr(docsrs, doc(cfg(feature = "freetype")))]
#[doc(hidden)]
impl From<FtSynthesize> for ffi::cairo_ft_synthesize_t {
    fn from(val: FtSynthesize) -> ffi::cairo_ft_synthesize_t {
        match val {
            FtSynthesize::Bold => ffi::CAIRO_FT_SYNTHESIZE_BOLD,
            FtSynthesize::Oblique => ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE,
            FtSynthesize::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "freetype")]
#[cfg_attr(docsrs, doc(cfg(feature = "freetype")))]
#[doc(hidden)]
impl From<ffi::cairo_ft_synthesize_t> for FtSynthesize {
    fn from(value: ffi::cairo_ft_synthesize_t) -> Self {
        match value {
            ffi::CAIRO_FT_SYNTHESIZE_BOLD => Self::Bold,
            ffi::CAIRO_FT_SYNTHESIZE_OBLIQUE => Self::Oblique,
            value => Self::__Unknown(value),
        }
    }
}

/// Possible output variants when drawing to a script surface.
#[cfg(feature = "script")]
#[cfg_attr(docsrs, doc(cfg(feature = "script")))]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_script_mode_t")]
pub enum ScriptMode {
    /// The output will be in readable text.
    #[doc(alias = "CAIRO_SCRIPT_MODE_ASCII")]
    Ascii,

    /// The output will use byte codes.
    #[doc(alias = "CAIRO_SCRIPT_MODE_BINARY")]
    Binary,

    #[doc(hidden)]
    __Unknown(i32),
}

#[cfg(feature = "script")]
#[cfg_attr(docsrs, doc(cfg(feature = "script")))]
#[doc(hidden)]
impl From<ScriptMode> for ffi::cairo_script_mode_t {
    fn from(val: ScriptMode) -> ffi::cairo_script_mode_t {
        match val {
            ScriptMode::Ascii => ffi::CAIRO_SCRIPT_MODE_ASCII,
            ScriptMode::Binary => ffi::CAIRO_SCRIPT_MODE_BINARY,
            ScriptMode::__Unknown(value) => value,
        }
    }
}

#[cfg(feature = "script")]
#[cfg_attr(docsrs, doc(cfg(feature = "script")))]
#[doc(hidden)]
impl From<ffi::cairo_script_mode_t> for ScriptMode {
    fn from(value: ffi::cairo_script_mode_t) -> Self {
        match value {
            ffi::CAIRO_SCRIPT_MODE_ASCII => Self::Ascii,
            ffi::CAIRO_SCRIPT_MODE_BINARY => Self::Binary,
            value => Self::__Unknown(value),
        }
    }
}

/// Specifies the type of a given [`Device`], also known as "backends" within Cairo.
///
/// [`Device`]: crate::Device
#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
#[non_exhaustive]
#[doc(alias = "cairo_device_type_t")]
pub enum DeviceType {
    /// Type Direct Render Manager.
    #[doc(alias = "CAIRO_DEVICE_TYPE_DRM")]
    Drm,

    /// Type OpenGL.
    #[doc(alias = "CAIRO_DEVICE_TYPE_GL")]
    Gl,

    /// Type script.
    #[doc(alias = "CAIRO_DEVICE_TYPE_SCRIPT")]
    Script,

    /// Type XCB.
    #[doc(alias = "CAIRO_DEVICE_TYPE_XCB")]
    Xcb,

    /// Type Xlib.
    #[doc(alias = "CAIRO_DEVICE_TYPE_XLIB")]
    Xlib,

    /// Type XML.
    #[doc(alias = "CAIRO_DEVICE_TYPE_XML")]
    Xml,

    /// Type Cogl.
    #[doc(alias = "CAIRO_DEVICE_TYPE_COGL")]
    Cogl,

    /// Type Win32.
    #[doc(alias = "CAIRO_DEVICE_TYPE_WIN32")]
    Win32,

    /// Invalid type.
    #[doc(alias = "CAIRO_DEVICE_TYPE_INVALID")]
    Invalid,

    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl From<DeviceType> for ffi::cairo_device_type_t {
    fn from(val: DeviceType) -> ffi::cairo_device_type_t {
        match val {
            DeviceType::Drm => ffi::CAIRO_DEVICE_TYPE_DRM,
            DeviceType::Gl => ffi::CAIRO_DEVICE_TYPE_GL,
            DeviceType::Script => ffi::CAIRO_DEVICE_TYPE_SCRIPT,
            DeviceType::Xcb => ffi::CAIRO_DEVICE_TYPE_XCB,
            DeviceType::Xlib => ffi::CAIRO_DEVICE_TYPE_XLIB,
            DeviceType::Xml => ffi::CAIRO_DEVICE_TYPE_XML,
            DeviceType::Cogl => ffi::CAIRO_DEVICE_TYPE_COGL,
            DeviceType::Win32 => ffi::CAIRO_DEVICE_TYPE_WIN32,
            DeviceType::Invalid => ffi::CAIRO_DEVICE_TYPE_INVALID,
            DeviceType::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl From<ffi::cairo_device_type_t> for DeviceType {
    fn from(value: ffi::cairo_device_type_t) -> Self {
        match value {
            ffi::CAIRO_DEVICE_TYPE_DRM => Self::Drm,
            ffi::CAIRO_DEVICE_TYPE_GL => Self::Gl,
            ffi::CAIRO_DEVICE_TYPE_SCRIPT => Self::Script,
            ffi::CAIRO_DEVICE_TYPE_XCB => Self::Xcb,
            ffi::CAIRO_DEVICE_TYPE_XLIB => Self::Xlib,
            ffi::CAIRO_DEVICE_TYPE_XML => Self::Xml,
            ffi::CAIRO_DEVICE_TYPE_COGL => Self::Cogl,
            ffi::CAIRO_DEVICE_TYPE_WIN32 => Self::Win32,
            ffi::CAIRO_DEVICE_TYPE_INVALID => Self::Invalid,
            value => Self::__Unknown(value),
        }
    }
}

#[cfg(feature = "use_glib")]
gvalue_impl!(DeviceType, ffi::gobject::cairo_gobject_device_type_get_type);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn stride_panics_on_bad_value() {
        let _ = Format::Rgb24.stride_for_width(u32::MAX);
    }

    #[test]
    fn stride_errors_on_large_width() {
        assert!(Format::Rgb24.stride_for_width(i32::MAX as u32).is_err());
    }

    #[test]
    fn stride_works() {
        assert_eq!(Format::Rgb24.stride_for_width(1).unwrap(), 4);
    }
}
