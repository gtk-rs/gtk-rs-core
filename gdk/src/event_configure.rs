// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventConfigure(crate::Event);

event_wrapper!(EventConfigure, GdkEventConfigure);
event_subtype!(EventConfigure, ffi::GDK_CONFIGURE);

impl EventConfigure {
    pub fn get_position(&self) -> (i32, i32) {
        (self.as_ref().x, self.as_ref().y)
    }

    pub fn get_size(&self) -> (u32, u32) {
        let width = self.as_ref().width;
        let height = self.as_ref().height;
        assert!(width >= 0 && height >= 0, "Unexpected negative value");
        (width as u32, height as u32)
    }
}
