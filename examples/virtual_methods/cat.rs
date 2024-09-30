use glib::prelude::*;
use glib::subclass::prelude::*;

use super::pet::*;
use super::purrable::*;

mod imp {
    use std::cell::Cell;

    use super::*;

    #[derive(Default)]
    pub struct Cat {
        fed: Cell<bool>,
        pub(super) purring: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Cat {
        const NAME: &'static str = "Cat";
        type Type = super::Cat;
        type Interfaces = (Purrable,);
        type ParentType = Pet;
    }

    impl ObjectImpl for Cat {}
    impl PurrableImpl for Cat {
        fn is_purring(&self) -> bool {
            if self.purring.get() {
                println!("Cat::is_purring: *purr*");
                true
            } else {
                println!("Cat::is_purring: Chaining up to parent_is_purring");
                self.parent_is_purring()
            }
        }
    }
    impl PetImpl for Cat {
        /// Override the parent behaviour of `pet` to indicate a successful pet
        /// if we have been sufficiently fed
        fn pet(&self) -> bool {
            if self.fed.get() {
                println!("Cat::pet: *purr*");
                self.purring.set(true);
                true
            } else {
                println!("Cat::pet: *mrrp*");
                false
            }
        }

        fn feed(&self) {
            println!("Cat::feed: *mreeeow*");
            self.parent_feed();

            // Remember that we have been fed
            self.fed.set(true);
        }
    }

    impl Cat {
        pub(super) fn meow(&self) {
            // We can't be meowing and purring at the same time
            self.purring.set(false);
            println!("Cat::meow: *meow* *meow*");
        }
    }
}

glib::wrapper! {
    /// The `Cat` class ties the interface and the superclass together
    pub struct Cat(ObjectSubclass<imp::Cat>)
    @extends Pet,
    @implements Purrable;
}

/// Public methods of `Cat` classes
pub trait CatExt: IsA<Cat> {
    /// A regular public method.
    ///
    /// Resets the purring state.
    fn meow(&self) {
        self.upcast_ref::<Cat>().imp().meow();
    }
}

impl<T: IsA<Cat>> CatExt for T {}

impl Default for Cat {
    fn default() -> Self {
        glib::Object::new()
    }
}

/// Cat is also subclassable, but does not have any vfuncs.
///
/// By convention we still create an empty `CatImpl` trait, this allows us to add
/// 'protected' cat methods only available to be called by other Cats later.
pub trait CatImpl: PetImpl
where
    <Self as ObjectSubclass>::Type: IsA<glib::Object>,
    <Self as ObjectSubclass>::Type: IsA<Pet>,
    <Self as ObjectSubclass>::Type: IsA<Cat>,
{
}

/// To make this class subclassable we need to implement IsSubclassable
unsafe impl<Obj: CatImpl + PetImpl> IsSubclassable<Obj> for Cat
where
    <Obj as ObjectSubclass>::Type: IsA<glib::Object>,
    <Obj as ObjectSubclass>::Type: IsA<Pet>,
    <Obj as ObjectSubclass>::Type: IsA<Cat>,
{
}
