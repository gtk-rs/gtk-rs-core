// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[cfg(any(feature = "v1_50", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
use crate::FontScale;
#[cfg(any(feature = "v1_46", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
use crate::Overline;
#[cfg(any(feature = "v1_44", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
use crate::ShowFlags;
use crate::{AttrType, Gravity, GravityHint, Stretch, Style, Underline, Variant, Weight};

define_attribute_struct!(
    AttrInt,
    ffi::PangoAttrInt,
    &[
        #[cfg(any(feature = "v1_50", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
        AttrType::AbsoluteLineHeight,
        #[cfg(any(feature = "v1_50", feature = "dox"))]
        #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
        AttrType::FontScale,
        AttrType::AllowBreaks,
        AttrType::BackgroundAlpha,
        AttrType::Fallback,
        AttrType::ForegroundAlpha,
        AttrType::Gravity,
        AttrType::GravityHint,
        AttrType::InsertHyphens,
        AttrType::LetterSpacing,
        AttrType::Overline,
        AttrType::Rise,
        AttrType::Show,
        AttrType::Stretch,
        AttrType::Strikethrough,
        AttrType::Style,
        AttrType::Underline,
        AttrType::Variant,
        AttrType::Weight
    ]
);

impl AttrInt {
    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_attr_line_height_new_absolute")]
    pub fn new_line_height_absolute(height: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_line_height_new_absolute(height)) }
    }

    #[cfg(any(feature = "v1_50", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_50")))]
    #[doc(alias = "pango_attr_font_scale_new")]
    pub fn new_font_scale(scale: FontScale) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_font_scale_new(scale.into_glib())) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_allow_breaks_new")]
    pub fn new_allow_breaks(allow_breaks: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_allow_breaks_new(allow_breaks.into_glib())) }
    }

    #[doc(alias = "pango_attr_background_alpha_new")]
    pub fn new_background_alpha(alpha: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_background_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_fallback_new")]
    pub fn new_fallback(enable_fallback: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_fallback_new(enable_fallback.into_glib())) }
    }

    #[doc(alias = "pango_attr_foreground_alpha_new")]
    pub fn new_foreground_alpha(alpha: u16) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_foreground_alpha_new(alpha)) }
    }

    #[doc(alias = "pango_attr_gravity_hint_new")]
    pub fn new_gravity_hint(hint: GravityHint) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_gravity_hint_new(hint.into_glib())) }
    }

    #[doc(alias = "pango_attr_weight_new")]
    pub fn new_weight(weight: Weight) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_weight_new(weight.into_glib())) }
    }

    #[doc(alias = "pango_attr_gravity_new")]
    pub fn new_gravity(gravity: Gravity) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_gravity_new(gravity.into_glib())) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_insert_hyphens_new")]
    pub fn new_insert_hyphens(insert_hyphens: bool) -> Self {
        unsafe {
            from_glib_full(ffi::pango_attr_insert_hyphens_new(
                insert_hyphens.into_glib(),
            ))
        }
    }

    #[doc(alias = "pango_attr_letter_spacing_new")]
    pub fn new_letter_spacing(letter_spacing: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_letter_spacing_new(letter_spacing)) }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_46")))]
    #[doc(alias = "pango_attr_overline_new")]
    pub fn new_overline(overline: Overline) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_overline_new(overline.into_glib())) }
    }

    #[doc(alias = "pango_attr_rise_new")]
    pub fn new_rise(rise: i32) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_rise_new(rise)) }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v1_44")))]
    #[doc(alias = "pango_attr_show_new")]
    pub fn new_show(flags: ShowFlags) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_show_new(flags.into_glib())) }
    }

    #[doc(alias = "pango_attr_stretch_new")]
    pub fn new_stretch(stretch: Stretch) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_stretch_new(stretch.into_glib())) }
    }

    #[doc(alias = "pango_attr_strikethrough_new")]
    pub fn new_strikethrough(strikethrough: bool) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_strikethrough_new(strikethrough.into_glib())) }
    }

    #[doc(alias = "pango_attr_style_new")]
    pub fn new_style(style: Style) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_style_new(style.into_glib())) }
    }

    #[doc(alias = "pango_attr_underline_new")]
    pub fn new_underline(underline: Underline) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_underline_new(underline.into_glib())) }
    }

    #[doc(alias = "pango_attr_variant_new")]
    pub fn new_variant(variant: Variant) -> Self {
        unsafe { from_glib_full(ffi::pango_attr_variant_new(variant.into_glib())) }
    }

    pub fn value(&self) -> i32 {
        self.0.value
    }
}
