// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ActionMap, SimpleAction};
use glib::{IsA, Variant};

#[doc(alias = "GActionEntry")]
pub struct ActionEntry<O>
where
    O: IsA<ActionMap>,
{
    name: String,
    parameter_type: Option<String>,
    state: Option<String>,
    pub(crate) activate: Option<Box<dyn Fn(&O, &SimpleAction, Option<&Variant>) + 'static>>,
    pub(crate) change_state: Option<Box<dyn Fn(&O, &SimpleAction, Option<&Variant>) + 'static>>,
}

impl<O> ActionEntry<O>
where
    O: IsA<ActionMap>,
{
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn parameter_type(&self) -> Option<&str> {
        self.parameter_type.as_deref()
    }

    pub fn state(&self) -> Option<&str> {
        self.state.as_deref()
    }

    pub fn builder(name: &str) -> ActionEntryBuilder<O> {
        ActionEntryBuilder::new(name)
    }
}

impl<O> std::fmt::Debug for ActionEntry<O>
where
    O: IsA<ActionMap>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionEntry")
            .field("name", &self.name())
            .field("parameter_type", &self.parameter_type())
            .field("state", &self.state())
            .finish()
    }
}

#[derive(Debug)]
pub struct ActionEntryBuilder<O>(ActionEntry<O>)
where
    O: IsA<ActionMap>;

impl<O> ActionEntryBuilder<O>
where
    O: IsA<ActionMap>,
{
    pub fn new(name: &str) -> Self {
        Self(ActionEntry {
            name: name.to_owned(),
            parameter_type: Default::default(),
            state: Default::default(),
            activate: Default::default(),
            change_state: Default::default(),
        })
    }

    pub fn parameter_type(mut self, parameter_type: &str) -> Self {
        self.0.parameter_type = Some(parameter_type.to_owned());
        self
    }

    pub fn state(mut self, state: &str) -> Self {
        self.0.state = Some(state.to_owned());
        self
    }

    pub fn activate<F: Fn(&O, &SimpleAction, Option<&Variant>) + 'static>(
        mut self,
        callback: F,
    ) -> Self {
        self.0.activate = Some(Box::new(callback));
        self
    }

    pub fn change_state<F: Fn(&O, &SimpleAction, Option<&Variant>) + 'static>(
        mut self,
        callback: F,
    ) -> Self {
        self.0.change_state = Some(Box::new(callback));
        self
    }

    pub fn build(self) -> ActionEntry<O> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn action_entry() {
        let app = crate::Application::new(None, Default::default());

        let close = ActionEntry::builder("close")
            .activate(move |_app, _, _| {
                //Do something
            })
            .build();
        app.add_action_entries(vec![close]).unwrap();
        assert!(app.lookup_action("close").is_some());
    }
}
