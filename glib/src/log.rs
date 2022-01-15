// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::*;
use crate::GString;
use once_cell::sync::Lazy;
use std::boxed::Box as Box_;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct LogHandlerId(u32);

#[doc(hidden)]
impl FromGlib<u32> for LogHandlerId {
    unsafe fn from_glib(value: u32) -> Self {
        Self(value)
    }
}

#[doc(hidden)]
impl IntoGlib for LogHandlerId {
    type GlibType = u32;

    fn into_glib(self) -> u32 {
        self.0
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LogLevel {
    #[doc(alias = "G_LOG_LEVEL_ERROR")]
    Error,
    #[doc(alias = "G_LOG_LEVEL_CRITICAL")]
    Critical,
    #[doc(alias = "G_LOG_LEVEL_WARNING")]
    Warning,
    #[doc(alias = "G_LOG_LEVEL_MESSAGE")]
    Message,
    #[doc(alias = "G_LOG_LEVEL_INFO")]
    Info,
    #[doc(alias = "G_LOG_LEVEL_DEBUG")]
    Debug,
}

#[doc(hidden)]
impl IntoGlib for LogLevel {
    type GlibType = u32;

    fn into_glib(self) -> u32 {
        match self {
            Self::Error => ffi::G_LOG_LEVEL_ERROR,
            Self::Critical => ffi::G_LOG_LEVEL_CRITICAL,
            Self::Warning => ffi::G_LOG_LEVEL_WARNING,
            Self::Message => ffi::G_LOG_LEVEL_MESSAGE,
            Self::Info => ffi::G_LOG_LEVEL_INFO,
            Self::Debug => ffi::G_LOG_LEVEL_DEBUG,
        }
    }
}

#[doc(hidden)]
impl FromGlib<u32> for LogLevel {
    unsafe fn from_glib(value: u32) -> Self {
        if value & ffi::G_LOG_LEVEL_ERROR != 0 {
            Self::Error
        } else if value & ffi::G_LOG_LEVEL_CRITICAL != 0 {
            Self::Critical
        } else if value & ffi::G_LOG_LEVEL_WARNING != 0 {
            Self::Warning
        } else if value & ffi::G_LOG_LEVEL_MESSAGE != 0 {
            Self::Message
        } else if value & ffi::G_LOG_LEVEL_INFO != 0 {
            Self::Info
        } else if value & ffi::G_LOG_LEVEL_DEBUG != 0 {
            Self::Debug
        } else {
            panic!("Unknown log level: {}", value)
        }
    }
}

bitflags::bitflags! {
    #[doc(alias = "GLogLevelFlags")]
    pub struct LogLevels: u32 {
        #[doc(alias = "G_LOG_LEVEL_ERROR")]
        const LEVEL_ERROR = ffi::G_LOG_LEVEL_ERROR;
        #[doc(alias = "G_LOG_LEVEL_CRITICAL")]
        const LEVEL_CRITICAL = ffi::G_LOG_LEVEL_CRITICAL;
        #[doc(alias = "G_LOG_LEVEL_WARNING")]
        const LEVEL_WARNING = ffi::G_LOG_LEVEL_WARNING;
        #[doc(alias = "G_LOG_LEVEL_MESSAGE")]
        const LEVEL_MESSAGE = ffi::G_LOG_LEVEL_MESSAGE;
        #[doc(alias = "G_LOG_LEVEL_INFO")]
        const LEVEL_INFO = ffi::G_LOG_LEVEL_INFO;
        #[doc(alias = "G_LOG_LEVEL_DEBUG")]
        const LEVEL_DEBUG = ffi::G_LOG_LEVEL_DEBUG;
    }
}

#[doc(hidden)]
impl IntoGlib for LogLevels {
    type GlibType = ffi::GLogLevelFlags;

    fn into_glib(self) -> ffi::GLogLevelFlags {
        self.bits()
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GLogLevelFlags> for LogLevels {
    unsafe fn from_glib(value: ffi::GLogLevelFlags) -> Self {
        Self::from_bits_truncate(value)
    }
}

fn to_log_flags(fatal: bool, recursion: bool) -> u32 {
    (if fatal { ffi::G_LOG_FLAG_FATAL } else { 0 })
        | if recursion {
            ffi::G_LOG_FLAG_RECURSION
        } else {
            0
        }
}

#[doc(alias = "g_log_set_handler_full")]
pub fn log_set_handler<P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static>(
    log_domain: Option<&str>,
    log_levels: LogLevels,
    fatal: bool,
    recursion: bool,
    log_func: P,
) -> LogHandlerId {
    let log_func_data: Box_<P> = Box_::new(log_func);
    unsafe extern "C" fn log_func_func<
        P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static,
    >(
        log_domain: *const libc::c_char,
        log_level: ffi::GLogLevelFlags,
        message: *const libc::c_char,
        user_data: ffi::gpointer,
    ) {
        let log_domain: Borrowed<Option<GString>> = from_glib_borrow(log_domain);
        let message: Borrowed<GString> = from_glib_borrow(message);
        let callback: &P = &*(user_data as *mut _);
        (*callback)(
            (*log_domain).as_deref(),
            from_glib(log_level),
            message.as_str(),
        );
    }
    let log_func = Some(log_func_func::<P> as _);
    unsafe extern "C" fn destroy_func<
        P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static,
    >(
        data: ffi::gpointer,
    ) {
        let _callback: Box_<P> = Box_::from_raw(data as *mut _);
    }
    let destroy_call4 = Some(destroy_func::<P> as _);
    let super_callback0: Box_<P> = log_func_data;
    unsafe {
        from_glib(ffi::g_log_set_handler_full(
            log_domain.to_glib_none().0,
            log_levels.into_glib() | to_log_flags(fatal, recursion),
            log_func,
            Box_::into_raw(super_callback0) as *mut _,
            destroy_call4,
        ))
    }
}

#[doc(alias = "g_log_remove_handler")]
pub fn log_remove_handler(log_domain: Option<&str>, handler_id: LogHandlerId) {
    unsafe {
        ffi::g_log_remove_handler(log_domain.to_glib_none().0, handler_id.into_glib());
    }
}

#[doc(alias = "g_log_set_always_fatal")]
pub fn log_set_always_fatal(fatal_levels: LogLevels) -> LogLevels {
    unsafe { from_glib(ffi::g_log_set_always_fatal(fatal_levels.into_glib())) }
}

#[doc(alias = "g_log_set_fatal_mask")]
pub fn log_set_fatal_mask(log_domain: Option<&str>, fatal_levels: LogLevels) -> LogLevels {
    unsafe {
        from_glib(ffi::g_log_set_fatal_mask(
            log_domain.to_glib_none().0,
            fatal_levels.into_glib(),
        ))
    }
}

// #[cfg(any(feature = "v2_50", feature = "dox"))]
// pub fn log_variant(log_domain: Option<&str>, log_level: LogLevel, fields: &Variant) {
//     unsafe {
//         ffi::g_log_variant(
//             log_domain.to_glib_none().0,
//             log_level.into_glib(),
//             fields.to_glib_none().0,
//         );
//     }
// }

type PrintCallback = dyn Fn(&str) + Send + Sync + 'static;

static PRINT_HANDLER: Lazy<Mutex<Option<Arc<PrintCallback>>>> = Lazy::new(|| Mutex::new(None));

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`unset_print_handler`] function.
#[doc(alias = "g_set_print_handler")]
pub fn set_print_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        if let Some(callback) = PRINT_HANDLER
            .lock()
            .expect("Failed to lock PRINT_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let string: Borrowed<GString> = from_glib_borrow(string);
            (*callback)(string.as_str())
        }
    }
    *PRINT_HANDLER
        .lock()
        .expect("Failed to lock PRINT_HANDLER to change callback") = Some(Arc::new(func));
    unsafe { ffi::g_set_print_handler(Some(func_func as _)) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`set_print_handler`] function.
pub fn unset_print_handler() {
    *PRINT_HANDLER
        .lock()
        .expect("Failed to lock PRINT_HANDLER to remove callback") = None;
    unsafe { ffi::g_set_print_handler(None) };
}

static PRINTERR_HANDLER: Lazy<Mutex<Option<Arc<PrintCallback>>>> = Lazy::new(|| Mutex::new(None));

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`unset_printerr_handler`] function.
#[doc(alias = "g_set_printerr_handler")]
pub fn set_printerr_handler<P: Fn(&str) + Send + Sync + 'static>(func: P) {
    unsafe extern "C" fn func_func(string: *const libc::c_char) {
        if let Some(callback) = PRINTERR_HANDLER
            .lock()
            .expect("Failed to lock PRINTERR_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let string: Borrowed<GString> = from_glib_borrow(string);
            (*callback)(string.as_str())
        }
    }
    *PRINTERR_HANDLER
        .lock()
        .expect("Failed to lock PRINTERR_HANDLER to change callback") = Some(Arc::new(func));
    unsafe { ffi::g_set_printerr_handler(Some(func_func as _)) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`set_printerr_handler`] function.
pub fn unset_printerr_handler() {
    *PRINTERR_HANDLER
        .lock()
        .expect("Failed to lock PRINTERR_HANDLER to remove callback") = None;
    unsafe { ffi::g_set_printerr_handler(None) };
}

type LogCallback = dyn Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static;

static DEFAULT_HANDLER: Lazy<Mutex<Option<Arc<LogCallback>>>> = Lazy::new(|| Mutex::new(None));

// rustdoc-stripper-ignore-next
/// To set back the default print handler, use the [`log_unset_default_handler`] function.
#[doc(alias = "g_log_set_default_handler")]
pub fn log_set_default_handler<P: Fn(Option<&str>, LogLevel, &str) + Send + Sync + 'static>(
    log_func: P,
) {
    unsafe extern "C" fn func_func(
        log_domain: *const libc::c_char,
        log_levels: ffi::GLogLevelFlags,
        message: *const libc::c_char,
        _user_data: ffi::gpointer,
    ) {
        if let Some(callback) = DEFAULT_HANDLER
            .lock()
            .expect("Failed to lock DEFAULT_HANDLER")
            .as_ref()
            .map(Arc::clone)
        {
            let log_domain: Borrowed<Option<GString>> = from_glib_borrow(log_domain);
            let message: Borrowed<GString> = from_glib_borrow(message);
            (*callback)(
                (*log_domain).as_deref(),
                from_glib(log_levels),
                message.as_str(),
            );
        }
    }
    *DEFAULT_HANDLER
        .lock()
        .expect("Failed to lock DEFAULT_HANDLER to change callback") = Some(Arc::new(log_func));
    unsafe { ffi::g_log_set_default_handler(Some(func_func as _), std::ptr::null_mut()) };
}

// rustdoc-stripper-ignore-next
/// To set the default print handler, use the [`log_set_default_handler`] function.
#[doc(alias = "g_log_set_default_handler")]
pub fn log_unset_default_handler() {
    *DEFAULT_HANDLER
        .lock()
        .expect("Failed to lock DEFAULT_HANDLER to remove callback") = None;
    unsafe {
        ffi::g_log_set_default_handler(Some(ffi::g_log_default_handler), std::ptr::null_mut())
    };
}

#[doc(alias = "g_log_default_handler")]
pub fn log_default_handler(log_domain: Option<&str>, log_level: LogLevel, message: Option<&str>) {
    unsafe {
        ffi::g_log_default_handler(
            log_domain.to_glib_none().0,
            log_level.into_glib(),
            message.to_glib_none().0,
            std::ptr::null_mut(),
        )
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! g_log_inner {
    ($log_domain:expr, $log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        use $crate::translate::{IntoGlib, ToGlibPtr};
        use $crate::LogLevel;
        use std::fmt::{self, Write};

        fn check_log_args(_log_level: LogLevel, _format: &str) {}

        check_log_args($log_level, $format);

        // Replace literal percentage signs with two so that they are not interpreted as printf
        // format specifiers
        struct GWrite($crate::GStringBuilder);

        impl fmt::Write for GWrite {
            fn write_str(&mut self, mut s: &str) -> ::std::result::Result<(), fmt::Error> {
                while let Some((prefix, suffix)) = s.split_once('%') {
                    self.0.append(prefix);
                    self.0.append("%%");
                    s = suffix;
                }
                self.0.append(s);
                Ok(())
            }

            fn write_char(&mut self, c: char) -> fmt::Result {
                if c == '%' {
                    self.0.append("%%");
                } else {
                    self.0.append_c(c);
                }
                Ok(())
            }
        }

        let mut w = GWrite($crate::GStringBuilder::default());

        // Can't really happen but better safe than sorry
        if !std::write!(&mut w, $format, $($arg),*).is_err() {
            unsafe {
                $crate::ffi::g_log(
                    $log_domain,
                    $log_level.into_glib(),
                    w.0.into_string().to_glib_none().0,
                );
            }
        }
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// Example:
///
/// ```no_run
/// use glib::{LogLevel, g_log};
///
/// g_log!("test", LogLevel::Debug, "test");
/// g_log!("test", LogLevel::Message, "test");
/// // trailing commas work as well:
/// g_log!("test", LogLevel::Message, "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_log!("test", LogLevel::Error, "test: {}", x);
/// g_log!("test", LogLevel::Critical, "test: {}", x);
/// g_log!("test", LogLevel::Warning, "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_log!("test", LogLevel::Warning, "test: {} {}", x, "a",);
/// ```
///
/// To be noted that the log domain is optional:
///
/// ```no_run
/// use glib::{LogLevel, g_log};
///
/// // As you can see: no log domain:
/// g_log!(LogLevel::Message, "test");
/// // For the rest, it's just like when you have the log domain:
/// // trailing commas:
/// g_log!(LogLevel::Message, "test",);
///
/// // formatting:
/// let x = 12;
/// g_log!(LogLevel::Warning, "test: {} {}", x, "a");
/// g_log!(LogLevel::Warning, "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_log {
    ($log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        $crate::g_log_inner!(std::ptr::null(), $log_level, $format, $($arg),*);
    }};
    ($log_domain:expr, $log_level:expr, $format:literal $(,$arg:expr)* $(,)?) => {{
        use $crate::translate::{IntoGlib, ToGlibPtr};

        let log_domain: Option<&str> = $log_domain.into();
        $crate::g_log_inner!(log_domain.to_glib_none().0, $log_level, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Error`].
///
/// Example:
///
/// ```no_run
/// use glib::g_error;
///
/// g_error!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Error, "test");
///
/// // trailing commas work as well:
/// g_error!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_error!("test", "test: {}", x);
/// g_error!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_error!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_error {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Error, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Error, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Critical`].
///
/// Example:
///
/// ```no_run
/// use glib::g_critical;
///
/// g_critical!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Critical, "test");
///
/// // trailing commas work as well:
/// g_critical!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_critical!("test", "test: {}", x);
/// g_critical!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_critical!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_critical {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Critical, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Critical, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Warning`].
///
/// Example:
///
/// ```no_run
/// use glib::g_warning;
///
/// g_warning!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Warning, "test");
///
/// // trailing commas work as well:
/// g_warning!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_warning!("test", "test: {}", x);
/// g_warning!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_warning!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_warning {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Warning, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Warning, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Message`].
///
/// Example:
///
/// ```no_run
/// use glib::g_message;
///
/// g_message!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Message, "test");
///
/// // trailing commas work as well:
/// g_message!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_message!("test", "test: {}", x);
/// g_message!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_message!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_message {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Message, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Message, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Info`].
///
/// Example:
///
/// ```no_run
/// use glib::g_info;
///
/// g_info!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Info, "test");
///
/// // trailing commas work as well:
/// g_info!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_info!("test", "test: {}", x);
/// g_info!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_info!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_info {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Info, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Info, $format);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to log using GLib logging system. It uses [g_log].
///
/// [g_log]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log
///
/// It is the same as calling the [`g_log!`] macro with [`LogLevel::Debug`].
///
/// Example:
///
/// ```no_run
/// use glib::g_debug;
///
/// g_debug!("test", "test");
/// // Equivalent to:
/// use glib::{g_log, LogLevel};
/// g_log!("test", LogLevel::Debug, "test");
///
/// // trailing commas work as well:
/// g_debug!("test", "test",);
///
/// // You can also pass arguments like in format! or println!:
/// let x = 12;
/// g_debug!("test", "test: {}", x);
/// g_debug!("test", "test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_debug!("test", "test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_debug {
    ($log_domain:expr, $format:literal, $($arg:expr),* $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Debug, $format, $($arg),*);
    }};
    ($log_domain:expr, $format:literal $(,)?) => {{
        $crate::g_log!($log_domain, $crate::LogLevel::Debug, $format);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! g_print_inner {
    ($func:ident, $format:expr $(, $arg:expr)* $(,)?) => {{
        use $crate::translate::{IntoGlib, ToGlibPtr};
        use $crate::LogLevel;
        use std::fmt::{self, Write};

        fn check_arg(_format: &str) {}

        check_arg($format);

        // Replace literal percentage signs with two so that they are not interpreted as printf
        // format specifiers
        struct GWrite($crate::GStringBuilder);

        impl fmt::Write for GWrite {
            fn write_str(&mut self, mut s: &str) -> ::std::result::Result<(), fmt::Error> {
                while let Some((prefix, suffix)) = s.split_once('%') {
                    self.0.append(prefix);
                    self.0.append("%%");
                    s = suffix;
                }
                self.0.append(s);
                Ok(())
            }

            fn write_char(&mut self, c: char) -> fmt::Result {
                if c == '%' {
                    self.0.append("%%");
                } else {
                    self.0.append_c(c);
                }
                Ok(())
            }
        }

        let mut w = GWrite($crate::GStringBuilder::default());

        // Can't really happen but better safe than sorry
        if !std::write!(&mut w, $format, $($arg),*).is_err() {
            unsafe {
                $crate::ffi::$func(w.0.into_string().to_glib_none().0);
            }
        }
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to print messages. It uses [g_print].
///
/// [g_print]: https://developer.gnome.org/glib/stable/glib-Warnings-and-Assertions.html#g-print
///
/// Example:
///
/// ```no_run
/// use glib::g_print;
///
/// g_print!("test");
/// // trailing commas work as well:
/// g_print!("test",);
///
/// let x = 12;
/// g_print!("test: {}", x);
/// g_print!("test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_print!("test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_print {
    ($format:expr $(,$arg:expr)* $(,)?) => {{
        $crate::g_print_inner!(g_print, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
/// Macro used to print error messages. It uses [g_printerr].
///
/// [g_printerr]: https://developer.gnome.org/glib/stable/glib-Warnings-and-Assertions.html#g-printerr
///
/// Example:
///
/// ```no_run
/// use glib::g_printerr;
///
/// g_printerr!("test");
/// // trailing commas work as well:
/// g_printerr!("test",);
///
/// let x = 12;
/// g_printerr!("test: {}", x);
/// g_printerr!("test: {} {}", x, "a");
/// // trailing commas work as well:
/// g_printerr!("test: {} {}", x, "a",);
/// ```
#[macro_export]
macro_rules! g_printerr {
    ($format:expr $(, $arg:expr)* $(,)?) => {{
        $crate::g_print_inner!(g_printerr, $format, $($arg),*);
    }};
}

// rustdoc-stripper-ignore-next
// /// Macro used to log using GLib logging system. It uses [g_log_structured][gls].
// ///
// /// [gls]: https://developer.gnome.org/glib/stable/glib-Message-Logging.html#g-log-structured)
// ///
// /// Example:
// ///
// /// ```no_run
// /// use glib::{LogLevel, g_log_structured};
// ///
// /// g_log_structured!("test", LogLevel::Debug, {"MESSAGE" => "tadam!"});
// /// g_log_structured!("test", LogLevel::Debug, {"MESSAGE" => "tadam!", "random" => "yes"});
// /// ```
// #[cfg(any(feature = "v2_50", feature = "dox"))]
// #[macro_export]
// macro_rules! g_log_structured {
//     ($log_domain:expr, $log_level:expr, {$($key:expr => $value:expr),+}) => {{
//         use $crate::translate::{Stash, IntoGlib, ToGlibPtr};
//         use $crate::LogLevel;
//         use std::ffi::CString;

//         fn check_log_args(_log_domain: &str, _log_level: LogLevel) {}
//         fn check_key(key: &str) -> Stash<*const i8, str> { key.to_glib_none() }

//         check_log_args(&$log_domain, $log_level);
//         unsafe {
//             ffi::g_log_structured(
//                 $log_domain.to_glib_none().0,
//                 $log_level.into_glib(),
//                 $(check_key($key).0, check_key(format!("{}", $value).as_str()).0 ),+
//             )
//         }
//     }};
// }
