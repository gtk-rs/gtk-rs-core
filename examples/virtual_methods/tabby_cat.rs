use glib::subclass::prelude::*;

use super::cat::{Cat, CatImpl};
use super::pet::{Pet, PetImpl};
use super::purrable::{Purrable, PurrableImpl, PurrableImplExt};

mod imp {
    use crate::PetImplExt;

    use super::*;

    #[derive(Default)]
    pub struct TabbyCat {}

    #[glib::object_subclass]
    impl ObjectSubclass for TabbyCat {
        const NAME: &'static str = "TabbyCat";
        type Type = super::TabbyCat;
        type ParentType = Cat;

        /// We override a method of [`PurrableImpl`], so we must re-declare
        /// that we conform to the interface. Otherwise our implementation
        /// methods do not end up in the vtable.
        type Interfaces = (Purrable,);
    }

    impl ObjectImpl for TabbyCat {}
    impl PetImpl for TabbyCat {
        fn feed(&self) {
            println!("TabbyCat::feed");
            self.parent_feed()
        }
    }
    impl CatImpl for TabbyCat {}
    impl PurrableImpl for TabbyCat {
        fn is_purring(&self) -> bool {
            println!("TabbyCat::is_purring");
            self.parent_is_purring()
        }
    }
    impl TabbyCat {}
}

glib::wrapper! {
    pub struct TabbyCat(ObjectSubclass<imp::TabbyCat>)
    @extends Cat, Pet,
    @implements Purrable;
}

impl TabbyCat {}

impl Default for TabbyCat {
    fn default() -> Self {
        glib::Object::new()
    }
}
