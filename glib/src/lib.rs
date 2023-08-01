// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::missing_safety_doc)]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use bitflags;
pub use ffi;
#[doc(hidden)]
pub use glib_macros::cstr_bytes;
pub use glib_macros::{
    clone, closure, closure_local, derived_properties, flags, object_interface, object_subclass,
    Boxed, Downgrade, Enum, ErrorDomain, Properties, SharedBoxed, ValueDelegate, Variant,
};
pub use gobject_ffi;
#[doc(hidden)]
pub use once_cell;

pub use self::{
    byte_array::ByteArray,
    bytes::Bytes,
    closure::{Closure, RustClosure},
    enums::{EnumClass, EnumValue, FlagsBuilder, FlagsClass, FlagsValue, UserDirectory},
    error::{BoolError, Error},
    object::{
        BorrowedObject, Cast, CastNone, Class, InitiallyUnowned, Interface, IsA, Object, ObjectExt,
        ObjectType, SendWeakRef, WeakRef,
    },
    signal::{
        signal_handler_block, signal_handler_disconnect, signal_handler_unblock,
        signal_stop_emission_by_name, Propagation, SignalHandlerId,
    },
    types::{ILong, Pointer, StaticType, StaticTypeExt, Type, ULong},
    value::{BoxedValue, SendValue, ToSendValue, ToValue, Value},
    variant::{
        FixedSizeVariantArray, FixedSizeVariantType, FromVariant, StaticVariantType, ToVariant,
        Variant,
    },
    variant_dict::VariantDict,
    variant_iter::{VariantIter, VariantStrIter},
    variant_type::{VariantTy, VariantTyIterator, VariantType},
    FileError,
};

// Hack for the time being to retrieve the current function's name as a string.
// Based on the stdext cratelicensed under the MIT license.
//
// Copyright (c) 2020 Igor Aleksanov
//
// Previous attempts to get such a macro into std:
// * https://github.com/rust-lang/rfcs/pull/466
// * https://github.com/rust-lang/rfcs/pull/1719
// * https://github.com/rust-lang/rfcs/issues/1743
// * https://github.com/rust-lang/rfcs/pull/2818
// * ...
// rustdoc-stripper-ignore-next
/// This macro returns the name of the enclosing function.
/// As the internal implementation is based on the [`std::any::type_name`], this macro derives
/// all the limitations of this function.
///
/// ## Examples
///
/// ```rust
/// mod bar {
///     pub fn sample_function() {
///         assert!(glib::function_name!().ends_with("bar::sample_function"));
///     }
/// }
///
/// bar::sample_function();
/// ```
///
/// [`std::any::type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html
#[macro_export]
macro_rules! function_name {
    () => {{
        // Okay, this is ugly, I get it. However, this is the best we can get on a stable rust.
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        // `3` is the length of the `::f`.
        &name[..name.len() - 3]
    }};
}

pub mod clone;
#[macro_use]
pub mod wrapper;
#[macro_use]
pub mod boxed;
#[macro_use]
pub mod boxed_inline;
#[macro_use]
pub mod shared;
#[macro_use]
pub mod error;
#[macro_use]
pub mod object;

mod boxed_any_object;
pub use boxed_any_object::BoxedAnyObject;
mod exit_code;
pub use exit_code::ExitCode;

pub mod collections;
pub use collections::{
    ptr_slice::IntoPtrSlice, strv::IntoStrV, List, PtrSlice, SList, Slice, StrV,
};

pub use self::auto::{functions::*, *};
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
#[allow(unused_imports)]
#[allow(non_upper_case_globals)]
mod auto;

pub use self::gobject::*;
mod gobject;

mod byte_array;
mod bytes;
mod control_flow;
pub use self::control_flow::ControlFlow;
pub mod char;
pub use self::char::*;
mod checksum;
pub mod closure;
mod convert;
pub use self::convert::*;
mod enums;
mod manual_functions;
pub use self::manual_functions::*;
mod key_file;
pub mod prelude;
pub mod signal;
pub mod source;
pub use self::source::*;
#[macro_use]
pub mod translate;
mod gstring;
pub use self::gstring::*;
mod gstring_builder;
pub use self::gstring_builder::GStringBuilder;
pub mod types;
mod unicollate;
pub use self::unicollate::{CollationKey, FilenameCollationKey};
mod utils;
pub use self::utils::*;
mod main_context;
mod main_context_channel;
pub use self::{
    main_context::MainContextAcquireGuard,
    main_context_channel::{Receiver, Sender, SyncSender},
};
mod date;
mod date_time;
mod time_span;
mod time_zone;
pub use self::time_span::*;
pub mod value;
pub mod variant;
mod variant_dict;
mod variant_iter;
mod variant_type;
pub use self::date::Date;
mod value_array;
pub use self::value_array::ValueArray;
mod param_spec;
pub use self::param_spec::*;
mod property;
pub use self::property::*;
mod quark;
pub use self::quark::Quark;
#[macro_use]
mod log;
#[doc(hidden)]
#[cfg(feature = "log_macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "log_macros")))]
pub use rs_log;

pub use self::log::{
    log_default_handler, log_remove_handler, log_set_always_fatal, log_set_default_handler,
    log_set_fatal_mask, log_set_handler, log_set_writer_func, log_structured_array,
    log_unset_default_handler, log_variant, log_writer_default, log_writer_format_fields,
    log_writer_journald, log_writer_standard_streams, set_print_handler, set_printerr_handler,
    unset_print_handler, unset_printerr_handler, LogField, LogHandlerId, LogLevel, LogLevels,
};
#[cfg(feature = "v2_68")]
pub use self::log::{log_writer_default_set_use_stderr, log_writer_default_would_drop};
#[cfg(unix)]
pub use self::log::{log_writer_is_journald, log_writer_supports_color};

#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
#[macro_use]
mod bridged_logging;
#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
pub use self::bridged_logging::{rust_log_handler, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};

#[macro_use]
pub mod subclass;

mod main_context_futures;
pub use main_context_futures::{JoinError, JoinHandle, SpawnWithinJoinHandle};
mod source_futures;
pub use self::source_futures::*;

mod future_with_timeout;
pub use self::future_with_timeout::*;

mod thread_pool;
pub use self::thread_pool::{ThreadHandle, ThreadPool};

pub mod thread_guard;

// rustdoc-stripper-ignore-next
/// This is the log domain used by the [`clone!`][crate::clone!] macro. If you want to use a custom
/// logger (it prints to stdout by default), you can set your own logger using the corresponding
/// `log` functions.
pub const CLONE_MACRO_LOG_DOMAIN: &str = "glib-rs-clone";

#[cfg(target_family = "windows")]
mod win32;

#[cfg(target_family = "windows")]
pub use self::win32::*;
