mod inner {
	use glib::{subclass::prelude::*, prelude::*};
	use std::cell::RefCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = CustomObject)]
    struct CustomObjectImp {
    	#[property(get, set)]
    	value: RefCell<bool>,
    }

    #[glib::object_subclass]
	impl ObjectSubclass for CustomObjectImp {
		const NAME: &'static str = "CustomObject";
		type Type = CustomObject;
	}

	#[glib::derived_properties]
	impl ObjectImpl for CustomObjectImp {
		fn constructed(&self) {
			self.parent_constructed();

			let obj = self.obj();

			// Both getter and setter are visible in this module
			assert!(!obj.value());
			obj.set_value(true);
		}
	}

	glib::wrapper! {
		struct CustomObject(ObjectSubclass<CustomObjectImp>);
	}

	pub fn new() -> CustomObject {
		glib::Object::new()
	}
}

fn main() {
	let foo = inner::new();

	// Getter is not visible
	assert!(foo.value());
	// Setter is not visible
	foo.set_value(false);
}
