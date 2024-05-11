mod cat;
mod pet;
mod purrable;
mod tabby_cat;

use cat::*;
use glib::object::Cast;
use pet::*;
use purrable::*;
use tabby_cat::*;

/// This example provides a class [`Pet`] with the virtual methods [`PetImpl::pet`] and
/// [`PetImpl::feed`], an interface [`Purrable`] with the method [`PurrableImpl::is_purring`],
/// an implementation class [`Cat`] to tie them all together, and a trivial subclass [`TabbyCat`]
/// to show that chaining up vfuncs works as expected.
fn main() {
    println!("\n=== Cat implementation ===");
    // Instantiate the subclass `Cat``
    let cat = Cat::default();
    cat.meow();
    dbg!(cat.pet());
    dbg!(cat.is_purring());

    cat.feed();
    dbg!(cat.pet());
    dbg!(cat.is_purring());
    cat.meow();
    dbg!(cat.is_purring());

    println!("\n=== Tabby Cat implementation ===");
    // Now instantiate the subclass `TabbyCat` and ensure that the parent class
    // functionality still works as expected and all methods chain up correctly.
    let tabby_cat = TabbyCat::default();
    tabby_cat.feed();
    tabby_cat.pet();

    // Even if we cast this as `Purrable` this calls the implementation in `TabbyCat`
    let purrable = tabby_cat.upcast_ref::<Purrable>();
    dbg!(purrable.is_purring());
    tabby_cat.meow();
}
