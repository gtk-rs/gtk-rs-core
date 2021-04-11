// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Border;
use crate::CssSection;
use crate::JunctionSides;
use crate::StateFlags;
#[cfg(any(feature = "v3_20", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
use crate::StyleContextPrintFlags;
use crate::StyleProvider;
use crate::TextDirection;
use crate::WidgetPath;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct StyleContext(Object<ffi::GtkStyleContext, ffi::GtkStyleContextClass>);

    match fn {
        get_type => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    #[doc(alias = "gtk_style_context_new")]
    pub fn new() -> StyleContext {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_style_context_new()) }
    }

    #[doc(alias = "gtk_style_context_add_provider_for_screen")]
    pub fn add_provider_for_screen<P: IsA<StyleProvider>>(
        screen: &gdk::Screen,
        provider: &P,
        priority: u32,
    ) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_screen(
                screen.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    #[doc(alias = "gtk_style_context_remove_provider_for_screen")]
    pub fn remove_provider_for_screen<P: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &P) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_remove_provider_for_screen(
                screen.to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_style_context_reset_widgets")]
    pub fn reset_widgets(screen: &gdk::Screen) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_style_context_reset_widgets(screen.to_glib_none().0);
        }
    }
}

impl Default for StyleContext {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
pub struct StyleContextBuilder {
    direction: Option<TextDirection>,
    paint_clock: Option<gdk::FrameClock>,
    parent: Option<StyleContext>,
    screen: Option<gdk::Screen>,
}

impl StyleContextBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> StyleContext {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref direction) = self.direction {
            properties.push(("direction", direction));
        }
        if let Some(ref paint_clock) = self.paint_clock {
            properties.push(("paint-clock", paint_clock));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref screen) = self.screen {
            properties.push(("screen", screen));
        }
        let ret = glib::Object::new::<StyleContext>(&properties).expect("object new");
        ret
    }

    pub fn direction(mut self, direction: TextDirection) -> Self {
        self.direction = Some(direction);
        self
    }

    pub fn paint_clock(mut self, paint_clock: &gdk::FrameClock) -> Self {
        self.paint_clock = Some(paint_clock.clone());
        self
    }

    pub fn parent<P: IsA<StyleContext>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn screen(mut self, screen: &gdk::Screen) -> Self {
        self.screen = Some(screen.clone());
        self
    }
}

pub const NONE_STYLE_CONTEXT: Option<&StyleContext> = None;

pub trait StyleContextExt: 'static {
    #[doc(alias = "gtk_style_context_add_class")]
    fn add_class(&self, class_name: &str);

    #[doc(alias = "gtk_style_context_add_provider")]
    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32);

    //#[doc(alias = "gtk_style_context_get")]
    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[cfg_attr(feature = "v3_16", deprecated)]
    #[doc(alias = "gtk_style_context_get_background_color")]
    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA;

    #[doc(alias = "gtk_style_context_get_border")]
    fn get_border(&self, state: StateFlags) -> Border;

    #[cfg_attr(feature = "v3_16", deprecated)]
    #[doc(alias = "gtk_style_context_get_border_color")]
    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA;

    #[doc(alias = "gtk_style_context_get_color")]
    fn get_color(&self, state: StateFlags) -> gdk::RGBA;

    #[doc(alias = "gtk_style_context_get_frame_clock")]
    fn get_frame_clock(&self) -> Option<gdk::FrameClock>;

    #[doc(alias = "gtk_style_context_get_junction_sides")]
    fn get_junction_sides(&self) -> JunctionSides;

    #[doc(alias = "gtk_style_context_get_margin")]
    fn get_margin(&self, state: StateFlags) -> Border;

    #[doc(alias = "gtk_style_context_get_padding")]
    fn get_padding(&self, state: StateFlags) -> Border;

    #[doc(alias = "gtk_style_context_get_parent")]
    fn get_parent(&self) -> Option<StyleContext>;

    #[doc(alias = "gtk_style_context_get_path")]
    fn get_path(&self) -> Option<WidgetPath>;

    #[doc(alias = "gtk_style_context_get_property")]
    fn get_property(&self, property: &str, state: StateFlags) -> glib::Value;

    #[doc(alias = "gtk_style_context_get_scale")]
    fn get_scale(&self) -> i32;

    #[doc(alias = "gtk_style_context_get_screen")]
    fn get_screen(&self) -> Option<gdk::Screen>;

    #[doc(alias = "gtk_style_context_get_section")]
    fn get_section(&self, property: &str) -> Option<CssSection>;

    #[doc(alias = "gtk_style_context_get_state")]
    fn get_state(&self) -> StateFlags;

    //#[doc(alias = "gtk_style_context_get_style")]
    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs);

    #[doc(alias = "gtk_style_context_get_style_property")]
    fn get_style_property(&self, property_name: &str) -> glib::Value;

    //#[doc(alias = "gtk_style_context_get_style_valist")]
    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    //#[doc(alias = "gtk_style_context_get_valist")]
    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported);

    #[doc(alias = "gtk_style_context_has_class")]
    fn has_class(&self, class_name: &str) -> bool;

    #[doc(alias = "gtk_style_context_list_classes")]
    fn list_classes(&self) -> Vec<glib::GString>;

    #[doc(alias = "gtk_style_context_lookup_color")]
    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA>;

    #[doc(alias = "gtk_style_context_remove_class")]
    fn remove_class(&self, class_name: &str);

    #[doc(alias = "gtk_style_context_remove_provider")]
    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P);

    #[doc(alias = "gtk_style_context_restore")]
    fn restore(&self);

    #[doc(alias = "gtk_style_context_save")]
    fn save(&self);

    #[cfg_attr(feature = "v3_18", deprecated)]
    #[doc(alias = "gtk_style_context_set_background")]
    fn set_background(&self, window: &gdk::Window);

    #[doc(alias = "gtk_style_context_set_frame_clock")]
    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock);

    #[doc(alias = "gtk_style_context_set_junction_sides")]
    fn set_junction_sides(&self, sides: JunctionSides);

    #[doc(alias = "gtk_style_context_set_parent")]
    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>);

    #[doc(alias = "gtk_style_context_set_path")]
    fn set_path(&self, path: &WidgetPath);

    #[doc(alias = "gtk_style_context_set_scale")]
    fn set_scale(&self, scale: i32);

    #[doc(alias = "gtk_style_context_set_screen")]
    fn set_screen(&self, screen: &gdk::Screen);

    #[doc(alias = "gtk_style_context_set_state")]
    fn set_state(&self, flags: StateFlags);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    #[doc(alias = "gtk_style_context_to_string")]
    fn to_string(&self, flags: StyleContextPrintFlags) -> Option<glib::GString>;

    fn get_property_direction(&self) -> TextDirection;

    fn set_property_direction(&self, direction: TextDirection);

    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock>;

    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>);

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<StyleContext>> StyleContextExt for O {
    fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn add_provider<P: IsA<StyleProvider>>(&self, provider: &P, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
                priority,
            );
        }
    }

    //fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_style_context_get() }
    //}

    fn get_background_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_background_color(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_border(&self, state: StateFlags) -> Border {
        unsafe {
            let mut border = Border::uninitialized();
            ffi::gtk_style_context_get_border(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                border.to_glib_none_mut().0,
            );
            border
        }
    }

    fn get_border_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_border_color(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_color(&self, state: StateFlags) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_style_context_get_color(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    fn get_frame_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_frame_clock(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_junction_sides(&self) -> JunctionSides {
        unsafe {
            from_glib(ffi::gtk_style_context_get_junction_sides(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_margin(&self, state: StateFlags) -> Border {
        unsafe {
            let mut margin = Border::uninitialized();
            ffi::gtk_style_context_get_margin(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                margin.to_glib_none_mut().0,
            );
            margin
        }
    }

    fn get_padding(&self, state: StateFlags) -> Border {
        unsafe {
            let mut padding = Border::uninitialized();
            ffi::gtk_style_context_get_padding(
                self.as_ref().to_glib_none().0,
                state.into_glib(),
                padding.to_glib_none_mut().0,
            );
            padding
        }
    }

    fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_path(&self) -> Option<WidgetPath> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_property(&self, property: &str, state: StateFlags) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_style_context_get_property(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
                state.into_glib(),
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    fn get_scale(&self) -> i32 {
        unsafe { ffi::gtk_style_context_get_scale(self.as_ref().to_glib_none().0) }
    }

    fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_screen(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_section(&self, property: &str) -> Option<CssSection> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_section(
                self.as_ref().to_glib_none().0,
                property.to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_style_context_get_state(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi:gtk_style_context_get_style() }
    //}

    fn get_style_property(&self, property_name: &str) -> glib::Value {
        unsafe {
            let mut value = glib::Value::uninitialized();
            ffi::gtk_style_context_get_style_property(
                self.as_ref().to_glib_none().0,
                property_name.to_glib_none().0,
                value.to_glib_none_mut().0,
            );
            value
        }
    }

    //fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_style_context_get_style_valist() }
    //}

    //fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi:gtk_style_context_get_valist() }
    //}

    fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            ))
        }
    }

    fn list_classes(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_style_context_list_classes(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn lookup_color(&self, color_name: &str) -> Option<gdk::RGBA> {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_lookup_color(
                self.as_ref().to_glib_none().0,
                color_name.to_glib_none().0,
                color.to_glib_none_mut().0,
            ));
            if ret {
                Some(color)
            } else {
                None
            }
        }
    }

    fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(
                self.as_ref().to_glib_none().0,
                class_name.to_glib_none().0,
            );
        }
    }

    fn remove_provider<P: IsA<StyleProvider>>(&self, provider: &P) {
        unsafe {
            ffi::gtk_style_context_remove_provider(
                self.as_ref().to_glib_none().0,
                provider.as_ref().to_glib_none().0,
            );
        }
    }

    fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.as_ref().to_glib_none().0);
        }
    }

    fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.as_ref().to_glib_none().0);
        }
    }

    fn set_background(&self, window: &gdk::Window) {
        unsafe {
            ffi::gtk_style_context_set_background(
                self.as_ref().to_glib_none().0,
                window.to_glib_none().0,
            );
        }
    }

    fn set_frame_clock(&self, frame_clock: &gdk::FrameClock) {
        unsafe {
            ffi::gtk_style_context_set_frame_clock(
                self.as_ref().to_glib_none().0,
                frame_clock.to_glib_none().0,
            );
        }
    }

    fn set_junction_sides(&self, sides: JunctionSides) {
        unsafe {
            ffi::gtk_style_context_set_junction_sides(
                self.as_ref().to_glib_none().0,
                sides.into_glib(),
            );
        }
    }

    fn set_parent<P: IsA<StyleContext>>(&self, parent: Option<&P>) {
        unsafe {
            ffi::gtk_style_context_set_parent(
                self.as_ref().to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_path(&self, path: &WidgetPath) {
        unsafe {
            ffi::gtk_style_context_set_path(self.as_ref().to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.as_ref().to_glib_none().0, scale);
        }
    }

    fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_style_context_set_screen(
                self.as_ref().to_glib_none().0,
                screen.to_glib_none().0,
            );
        }
    }

    fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.as_ref().to_glib_none().0, flags.into_glib());
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    fn to_string(&self, flags: StyleContextPrintFlags) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::gtk_style_context_to_string(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            ))
        }
    }

    fn get_property_direction(&self) -> TextDirection {
        unsafe {
            let mut value = glib::Value::from_type(<TextDirection as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"direction\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `direction` getter")
                .unwrap()
        }
    }

    fn set_property_direction(&self, direction: TextDirection) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"direction\0".as_ptr() as *const _,
                glib::Value::from(&direction).to_glib_none().0,
            );
        }
    }

    fn get_property_paint_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            let mut value = glib::Value::from_type(<gdk::FrameClock as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"paint-clock\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `paint-clock` getter")
        }
    }

    fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"paint-clock\0".as_ptr() as *const _,
                glib::Value::from(paint_clock).to_glib_none().0,
            );
        }
    }

    fn connect_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleContext,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_direction_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_direction_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::direction\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_direction_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_paint_clock_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_paint_clock_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::paint-clock\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_paint_clock_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parent_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_parent_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_screen_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_screen_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkStyleContext,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<StyleContext>,
        {
            let f: &F = &*(f as *const F);
            f(&StyleContext::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::screen\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_screen_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for StyleContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StyleContext")
    }
}
