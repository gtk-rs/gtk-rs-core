// Take a look at the license at the top of the repository in the LICENSE file.

use crate::SimpleAction;
use glib::translate::*;

impl SimpleAction {
    #[doc(alias = "g_simple_action_new_stateful")]
    pub fn new_stateful(
        name: &str,
        parameter_type: Option<&glib::VariantTy>,
        state: glib::Variant,
    ) -> SimpleAction {
        unsafe {
            from_glib_full(ffi::g_simple_action_new_stateful(
                name.to_glib_none().0,
                parameter_type.to_glib_none().0,
                state.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_simple_action_set_state")]
    pub fn set_state(&self, value: glib::Variant) {
        unsafe {
            ffi::g_simple_action_set_state(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[doc(alias = "g_simple_action_set_state_hint")]
    pub fn set_state_hint(&self, state_hint: Option<glib::Variant>) {
        unsafe {
            ffi::g_simple_action_set_state_hint(self.to_glib_none().0, state_hint.to_glib_none().0);
        }
    }
}
