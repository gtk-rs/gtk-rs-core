// Take a look at the license at the top of the repository in the LICENSE file.

//! Traits and essential types intended for blanket imports.

#[doc(hidden)]
pub use glib::prelude::*;

pub use crate::auto::traits::*;

pub use crate::accel_group::AccelGroupExtManual;
pub use crate::app_chooser::AppChooserExt;
pub use crate::buildable::BuildableExtManual;
pub use crate::builder::BuilderExtManual;
pub use crate::cell_renderer_pixbuf::CellRendererPixbufExtManual;
pub use crate::color_button::ColorButtonExtManual;
pub use crate::color_chooser::ColorChooserExtManual;
pub use crate::combo_box::ComboBoxExtManual;
pub use crate::dialog::DialogExtManual;
pub use crate::drag_context::DragContextExtManual;
pub use crate::entry::EntryExtManual;
pub use crate::entry_completion::EntryCompletionExtManual;
pub use crate::fixed::FixedExtManual;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use crate::flow_box::FlowBoxExtManual;
#[cfg(any(feature = "v3_24", feature = "dox"))]
pub use crate::gesture_stylus::GestureStylusExtManual;
pub use crate::im_context_simple::IMContextSimpleExtManual;
pub use crate::invisible::InvisibleExtManual;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use crate::list_box::ListBoxExtManual;
pub use crate::list_store::GtkListStoreExtManual;
pub use crate::menu::GtkMenuExtManual;
pub use crate::notebook::NotebookExtManual;
pub use crate::style_context::StyleContextExtManual;
pub use crate::switch::SwitchExtManual;
pub use crate::text_buffer::TextBufferExtManual;
pub use crate::tree_sortable::TreeSortableExtManual;
pub use crate::tree_store::TreeStoreExtManual;
pub use crate::widget::WidgetExtManual;
pub use crate::window::GtkWindowExtManual;

pub use crate::signal::*;
