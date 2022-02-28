// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![allow(unknown_lints)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::non_send_fields_in_send_ty)]
#![doc = include_str!("../README.md")]

pub use ffi;
pub use gobject_ffi;

#[doc(hidden)]
pub use bitflags;

#[doc(hidden)]
pub use once_cell;

pub use glib_macros::{
    clone, closure, closure_local, flags, object_interface, object_subclass, Boxed, Downgrade,
    Enum, ErrorDomain, SharedBoxed, Variant,
};

#[doc(hidden)]
pub use glib_macros::cstr_bytes;

pub use self::byte_array::ByteArray;
pub use self::bytes::Bytes;
pub use self::closure::{Closure, RustClosure};
pub use self::error::{BoolError, Error};
pub use self::file_error::FileError;
pub use self::object::{
    Cast, Class, InitiallyUnowned, Interface, IsA, Object, ObjectExt, ObjectType, SendWeakRef,
    WeakRef,
};
pub use self::signal::{
    signal_handler_block, signal_handler_disconnect, signal_handler_unblock,
    signal_stop_emission_by_name, SignalHandlerId,
};

pub use self::enums::{EnumClass, EnumValue, FlagsBuilder, FlagsClass, FlagsValue, UserDirectory};
pub use self::types::{ILong, Pointer, StaticType, StaticTypeExt, Type, ULong};
pub use self::value::{BoxedValue, SendValue, ToSendValue, ToValue, Value};
pub use self::variant::{
    FixedSizeVariantArray, FixedSizeVariantType, FromVariant, StaticVariantType, ToVariant, Variant,
};
pub use self::variant_dict::VariantDict;
pub use self::variant_iter::{VariantIter, VariantStrIter};
pub use self::variant_type::{VariantTy, VariantType};

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

pub mod collections;
pub use collections::{List, PtrSlice, SList, Slice};

pub use self::auto::functions::*;
pub use self::auto::*;
#[allow(non_upper_case_globals)]
#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
#[allow(unused_imports)]
mod auto;

pub use self::gobject::*;
mod gobject;

mod byte_array;
mod bytes;
pub mod char;
pub use self::char::*;
mod checksum;
pub mod closure;
mod enums;
mod file_error;
mod functions;
pub use self::functions::*;
mod key_file;
pub mod prelude;
pub mod signal;
pub mod source;
pub use self::source::*;
#[macro_use]
pub mod translate;
mod gstring;
pub use self::gstring::{GStr, GString};
mod gstring_builder;
pub use self::gstring_builder::GStringBuilder;
pub mod types;
mod unicollate;
pub use self::unicollate::{CollationKey, FilenameCollationKey};
mod utils;
pub use self::utils::*;
mod main_context;
mod main_context_channel;
pub use self::main_context::MainContextAcquireGuard;
pub use self::main_context_channel::{Receiver, Sender, SyncSender};
mod date;
mod date_time;
mod time_span;
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
mod quark;
pub use self::quark::Quark;
#[macro_use]
mod log;
pub use self::log::log_set_handler;

// #[cfg(any(feature = "v2_50", feature = "dox"))]
// pub use log::log_variant;
pub use self::log::{
    log_default_handler, log_remove_handler, log_set_always_fatal, log_set_default_handler,
    log_set_fatal_mask, log_unset_default_handler, set_print_handler, set_printerr_handler,
    unset_print_handler, unset_printerr_handler, LogHandlerId, LogLevel, LogLevels,
};

#[doc(hidden)]
#[cfg(any(feature = "dox", feature = "log_macros"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "log_macros")))]
pub use rs_log;

#[cfg(any(feature = "log", feature = "dox"))]
#[macro_use]
mod bridged_logging;
#[cfg(any(feature = "log", feature = "dox"))]
pub use self::bridged_logging::{rust_log_handler, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};

pub mod send_unique;
pub use self::send_unique::{SendUnique, SendUniqueCell};

#[macro_use]
pub mod subclass;

mod main_context_futures;
mod source_futures;
pub use self::source_futures::*;

mod thread_pool;
pub use self::thread_pool::ThreadPool;

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
