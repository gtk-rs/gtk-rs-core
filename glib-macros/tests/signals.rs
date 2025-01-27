mod base {
    use glib::prelude::*;
    use glib::subclass::prelude::*;

    pub mod imp {
        use super::*;

        #[derive(Default)]
        pub struct Base {}

        #[glib::signals(wrapper_type = super::Base)]
        impl Base {
            #[signal(run_first, no_recurse, no_hooks)]
            fn void_signal(&self);
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        impl ObjectImpl for Base {
            fn constructed(&self) {
            }
        }
    }

    glib::wrapper! {
        pub struct Base(ObjectSubclass<imp::Base>);
    }
}

fn main() {}
