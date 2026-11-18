mod imp {
	use glib::{subclass::prelude::*, prelude::*};
	use std::cell::RefCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::CustomObject)]
    pub(super) struct CustomObject {
    	#[property(pub(self) get, set)]
    	value: RefCell<bool>,
    }

    #[glib::object_subclass]
	impl ObjectSubclass for CustomObject {
		const NAME: &'static str = "CustomObject";
		type Type = super::CustomObject;
	}

	#[glib::derived_properties]
	impl ObjectImpl for CustomObject {
		fn constructed(&self) {
			self.parent_constructed();

			// Getter is visible in this module
			assert!(!self.obj().value());
		}
	}

}

glib::wrapper! {
	struct CustomObject(ObjectSubclass<imp::CustomObject>);
}

fn main() {
	let foo: CustomObject = glib::Object::new();

	// Setter is visible by inheriting struct visibility
	foo.set_value(true);
	// Getter is not visible
	assert!(!foo.value());
}
