// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{prelude::*, ActionEntry, ActionMap, SimpleAction};
use glib::{clone, Cast, IsA};

pub trait ActionMapExtManual {
    #[doc(alias = "g_action_map_add_action_entries")]
    fn add_action_entries(
        &self,
        entries: impl IntoIterator<Item = ActionEntry<Self>>,
    ) -> Result<(), glib::BoolError>
    where
        Self: IsA<ActionMap>;
}

impl<O: IsA<ActionMap>> ActionMapExtManual for O {
    fn add_action_entries(
        &self,
        entries: impl IntoIterator<Item = ActionEntry<Self>>,
    ) -> Result<(), glib::BoolError> {
        for entry in entries.into_iter() {
            let parameter_type = if let Some(param_type) = entry.parameter_type() {
                Some(glib::VariantType::new(param_type)?)
            } else {
                None
            };
            let action = if let Some(state) = entry.state() {
                let state = glib::Variant::parse(None, state).map_err(|e| {
                    glib::bool_error!(
                        "Invalid state passed to gio::ActionEntry {} {}",
                        entry.name(),
                        e
                    )
                })?;
                SimpleAction::new_stateful(entry.name(), parameter_type.as_deref(), &state)
            } else {
                SimpleAction::new(entry.name(), parameter_type.as_deref())
            };
            let action_map = self.as_ref();
            if let Some(callback) = entry.activate {
                action.connect_activate(clone!(@strong action_map =>  move |action, state| {
                    // safe to unwrap as O: IsA<ActionMap>
                    callback(action_map.downcast_ref::<O>().unwrap(), action, state);
                }));
            }
            if let Some(callback) = entry.change_state {
                action.connect_change_state(clone!(@strong action_map => move |action, state| {
                    // safe to unwrap as O: IsA<ActionMap>
                    callback(action_map.downcast_ref::<O>().unwrap(), action, state);
                }));
            }
            self.as_ref().add_action(&action);
        }
        Ok(())
    }
}
