// Take a look at the license at the top of the repository in the LICENSE file.

use std::{cell::Cell, rc::Rc};

use glib::object::ObjectExt;

mod base {
    use std::sync::LazyLock;

    use glib::object::ObjectSubclassIs;
    use glib::prelude::*;
    use glib::subclass::{prelude::*, SignalId};

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct Base {}

        #[glib::signals(wrapper_type = super::Base)]
        impl Base {
            #[signal(run_first, action)]
            fn run_first(&self) -> ();

            #[signal]
            fn has_params(&self, int: i32, float: f32) -> i32;
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        #[glib::derived_signals]
        impl ObjectImpl for Base {
            fn constructed(&self) {}
        }
    }

    glib::wrapper! {
        pub struct Base(ObjectSubclass<imp::Base>);
    }
}

#[test]
fn basic_test() {
    let foo = glib::Object::new::<base::Base>();

    let check: Rc<Cell<bool>> = Rc::new(Cell::new(false));

    let h_id = foo.connect_run_first({
        let check = Rc::clone(&check);
        move |_| {
            check.set(true);
        }
    });

    foo.emit_run_first();
    assert_eq!(check.get(), true, "Signal handler should have run");

    foo.disconnect(h_id);
    check.set(false);

    foo.emit_run_first();
    assert_eq!(check.get(), false, "Signal handler should not have run");
}
