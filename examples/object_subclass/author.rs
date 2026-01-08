// You can copy/paste this file every time you need a simple GObject
// to hold some data

use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::subclass::Signal;
use glib::subclass::object::DerivedObjectSignals;
use std::cell::RefCell;

mod imp {
    use super::*;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::Author)]
    pub struct Author {
        /// The name of the author
        ///
        /// Just their given name, not their surname.
        #[property(get, set)]
        /// A helpful name-surname combination.
        #[property(name = "name-surname", get = |author: &Self| format!("{} {}", author.name.borrow(), author.surname.borrow()))]
        name: RefCell<String>,
        /// # Getter
        ///
        /// This is how you can get the surname of the author.
        ///
        /// # Setter
        ///
        /// You can change the surname of the author too if you want.
        #[property(get, set)]
        surname: RefCell<String>,
    }

    #[glib::signals(wrapper_type = super::Author)]
    impl Author {

        #[signal]
        fn awarded(&self) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for Author {
        fn signals() -> &'static [Signal] {
            Self::derived_signals()
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
