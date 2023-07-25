// You can copy/paste this file every time you need a simple GObject
// to hold some data

use glib::once_cell::sync::Lazy;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::subclass::Signal;
use glib::Properties;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(Properties, Default)]
    #[properties(wrapper_type = super::Author)]
    pub struct Author {
        #[property(get, set)]
        name: RefCell<String>,
        #[property(get, set)]
        surname: RefCell<String>,
    }

    #[glib::derived_properties]
    impl ObjectImpl for Author {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> =
                Lazy::new(|| vec![Signal::builder("awarded").build()]);
            SIGNALS.as_ref()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Author {
        const NAME: &'static str = "Author";
        type Type = super::Author;
    }
}

glib::wrapper! {
    pub struct Author(ObjectSubclass<imp::Author>);
}
impl Author {
    pub fn new(name: &str, surname: &str) -> Self {
        glib::Object::builder()
            .property("name", name)
            .property("surname", surname)
            .build()
    }
}
