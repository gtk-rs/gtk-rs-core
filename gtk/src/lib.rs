// Take a look at the license at the top of the repository in the LICENSE file.

#![allow(clippy::needless_doctest_main)]

//! # GTK+ 3 bindings
//!
//! This library contains safe Rust bindings for [GTK+ 3](http://www.gtk.org), a
//! multi-platform GUI toolkit. It's a part of [Gtk-rs](http://gtk-rs.org/).
//!
//! The library is a work in progress: expect missing bindings and breaking
//! changes. A steadily increasing share of the code is machine-generated from
//! `GObject` introspection metadata. The API docs were converted from the
//! upstream ones so until they've all been reviewed there will be incongruities
//! with actual Rust APIs.
//!
//! See also:
//!
//! - [Gtk-rs documentation overview](https://gtk-rs.org/docs-src/)
//!
//! - [General `GLib` family types and object system overview](../glib/index.html)
//!
//! - [GTK+ documentation](https://www.gtk.org/docs/)
//!
//! # Hello World
//!
//! ```no_run
//! use gtk::prelude::*;
//! use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
//!
//! fn main() {
//!     if gtk::init().is_err() {
//!         println!("Failed to initialize GTK.");
//!         return;
//!     }
//!     MessageDialog::new(None::<&Window>,
//!                        DialogFlags::empty(),
//!                        MessageType::Info,
//!                        ButtonsType::Ok,
//!                        "Hello World").run();
//! }
//! ```
//!
//! # Initialization
//!
//! GTK+ needs to be initialized before use by calling [`init`](fn.init.html) or
//! [`Application::new`](struct.Application.html#method.new). You only need to
//! do it once and there is no 'finalize'.
//!
//! # The main loop
//!
//! In a typical GTK+ application you set up the UI, assign signal handlers
//! and run the main event loop:
//!
//! ```no_run
//! // To import all needed traits.
//! use gtk::prelude::*;
//! use gio::prelude::*;
//!
//! use std::env;
//!
//! fn main() {
//!     let uiapp = gtk::Application::new(Some("org.gtkrsnotes.demo"),
//!                                       gio::ApplicationFlags::FLAGS_NONE)
//!                                  .expect("Application::new failed");
//!     uiapp.connect_activate(|app| {
//!         // We create the main window.
//!         let win = gtk::ApplicationWindow::new(app);
//!
//!         // Then we set its size and a title.
//!         win.set_default_size(320, 200);
//!         win.set_title("Basic example");
//!
//!         // Don't forget to make all widgets visible.
//!         win.show_all();
//!     });
//!     uiapp.run(&env::args().collect::<Vec<_>>());
//! }
//! ```
//!
//! # Threads
//!
//! GTK+ is not thread-safe. Accordingly, none of this crate's structs implement
//! `Send` or `Sync`.
//!
//! The thread where `init` was called is considered the main thread. OS X has
//! its own notion of the main thread and `init` must be called on that thread.
//! After successful initialization, calling any `gtk` or `gdk` functions
//! (including `init`) from other threads will `panic`.
//!
//! Any thread can schedule a closure to be run by the main loop on the main
//! thread via [`glib::idle_add`](../glib/source/fn.idle_add.html) or
//! [`glib::timeout_add`](../glib/source/fn.timeout_add.html). This crate has
//! versions of those functions without the `Send` bound, which may only be
//! called from the main thread: [`idle_add`](fn.idle_add.html),
//! [`timeout_add`](fn.timeout_add.html).
//!
//! # Panics
//!
//! This and the `gdk` crate have some run-time safety and contract checks:
//!
//! - Any constructor or free function will panic if called before `init` or on
//! a non-main thread.
//!
//! - Any `&str` or `&Path` parameter with an interior null (`\0`) character will
//! cause a panic.
//!
//! - Some functions will panic if supplied out-of-range integer parameters. All
//! such cases will be documented individually but they're not yet.
//!
//! **A panic in a closure will abort the process.**
//!
//! # Crate features
//!
//! ## Library versions
//!
//! By default this crate provides only GTK+ 3.14 APIs. You can access more
//! modern APIs by selecting one of the following features: `v3_14`, `v3_16`, etc.
//!
//! `Cargo.toml` example:
//!
//! ```toml
//! [dependencies.gtk]
//! version = "0.x.y"
//! features = ["v3_16"]
//! ```
//!
//! **Take care when choosing the version to target: some of your users might
//! not have easy access to the latest ones.** The higher the version, the fewer
//! users will have it installed.
//!
//! ## Lgpl-docs
//!
//! The Gtk-rs crates come with API docs missing because of licensing
//! incompatibilty. You can embed those docs locally via the `embed-lgpl-docs`
//! feature, e.g.
//!
//! ```shell
//! > cargo doc --features embed-lgpl-docs
//! ```
//!
//! Its counterpart `purge-lgpl-docs` removes those docs regardless of edits.
//!
//! These features **rewrite the crate sources** so it's sufficient to enable
//! them once. **Omitting them in the following cargo invocations will not undo
//! their effects!**

#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::derive_hash_xor_eq)]
#![allow(clippy::too_many_arguments)]
#![allow(deprecated)]
#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;

#[doc(hidden)]
pub use field_offset::*;
#[doc(hidden)]
pub use gtk3_macros::*;

pub mod xlib;

pub const STYLE_PROVIDER_PRIORITY_FALLBACK: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_FALLBACK as u32;
pub const STYLE_PROVIDER_PRIORITY_THEME: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_THEME as u32;
pub const STYLE_PROVIDER_PRIORITY_SETTINGS: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_SETTINGS as u32;
pub const STYLE_PROVIDER_PRIORITY_APPLICATION: u32 =
    ffi::GTK_STYLE_PROVIDER_PRIORITY_APPLICATION as u32;
pub const STYLE_PROVIDER_PRIORITY_USER: u32 = ffi::GTK_STYLE_PROVIDER_PRIORITY_USER as u32;

#[macro_use]
mod rt;

#[allow(clippy::match_same_arms)]
#[allow(clippy::let_and_return)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::wrong_self_convention)]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::many_single_char_names)]
#[allow(clippy::cast_ptr_alignment)]
#[allow(unused_doc_comments)]
#[allow(unused_imports)]
mod auto;

mod accel_group;
mod app_chooser;
mod application;
mod application_window;
mod border;
mod buildable;
mod builder;
mod cell_renderer_pixbuf;
mod clipboard;
mod color_button;
mod color_chooser;
mod combo_box;
mod dialog;
mod drag_context;
mod entry;
mod entry_buffer;
mod entry_completion;
mod enums;
mod file_chooser_dialog;
mod fixed;
#[cfg(any(feature = "v3_18", feature = "dox"))]
mod flow_box;
#[cfg(any(feature = "v3_24", feature = "dox"))]
mod gesture_stylus;
mod im_context_simple;
mod invisible;
#[cfg(any(feature = "v3_16", feature = "dox"))]
mod list_box;
mod list_store;
mod menu;
mod message_dialog;
mod notebook;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_action_entry;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod pad_controller;
mod page_range;
mod print_settings;
mod radio_button;
mod radio_menu_item;
mod radio_tool_button;
mod recent_chooser_dialog;
mod recent_data;
mod requisition;
mod response_type;
mod selection_data;
mod signal;
mod style_context;
mod switch;
mod target_entry;
mod target_list;
mod text_buffer;
mod text_iter;
mod tree_model_filter;
mod tree_path;
mod tree_row_reference;
mod tree_sortable;
mod tree_store;
mod widget;
mod window;

#[macro_use]
pub mod subclass;

pub mod prelude;

pub use crate::auto::functions::*;
pub use crate::auto::*;
pub use crate::rt::*;
pub use crate::signal::*;

pub use gdk::Rectangle as Allocation;
pub use gdk::Rectangle;

pub use crate::app_chooser::AppChooser;
pub use crate::border::Border;
pub use crate::entry_buffer::EntryBuffer;
pub use crate::page_range::PageRange;
pub use crate::recent_data::RecentData;
pub use crate::requisition::Requisition;
pub use crate::response_type::ResponseType;
pub use crate::target_entry::TargetEntry;
pub use crate::tree_sortable::SortColumn;
pub use crate::widget::TickCallbackId;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use pad_action_entry::PadActionEntry;
