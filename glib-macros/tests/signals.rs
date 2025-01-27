mod base {
    use glib::prelude::*;
    use glib::subclass::prelude::*;

    pub mod imp {
        use super::*;
        use glib::subclass::prelude::*;

        #[derive(Default)]
        pub struct Base {}

        #[glib::signals(wrapper_type = super::Base)]
        impl Base {
            #[signal(run_first, no_recurse, no_hooks)]
            fn one(&self) -> ();
            #[signal(run_last, action)]
            fn two(&self, pi: i32, pf: f32, ps: &str) -> i32;
            #[signal]
            fn three(&self, pf: f32) {
                println!("pf = {}", pf);
            }
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        #[glib::derived_signals]
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
