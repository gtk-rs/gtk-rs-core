mod imp {
	use glib::{subclass::prelude::*, prelude::*};
	use std::cell::RefCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::CustomObject)]
    pub(super) struct CustomObject {
    	#[property(get, pub(self) set)]
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

			// Setter is visible in this module
			self.obj().set_value(true);
		}
	}
}

glib::wrapper! {
	struct CustomObject(ObjectSubclass<imp::CustomObject>);
}

fn main() {
	let foo: CustomObject = glib::Object::new();

	// Getter is visible by inheriting struct visibility
	assert!(!foo.value());
	// Setter is not visible
	foo.set_value(false);
}
