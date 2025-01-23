mod base {
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glib::subclass::Signal;
    use glib::Value;
    use glib_macros::signals;
    use std::marker::PhantomData;

    pub mod imp {
        use std::sync::OnceLock;

        use super::*;

        #[derive(Default)]
        pub struct Base {}

        // #[signals(wrapper_type = super::Base)]
        impl Base {
            fn void_signal(&self) {}
        }

        #[glib::object_subclass]
        impl ObjectSubclass for Base {
            const NAME: &'static str = "MyBase";
            type Type = super::Base;
        }

        impl ObjectImpl for Base {
            fn constructed(&self) {
            }

            fn signals() -> &'static [Signal] {
                static SIGNALS: OnceLock<[Signal; 1]> = OnceLock::new();
                SIGNALS.get_or_init(|| {
                    [Signal::builder("void-signal")
                        .class_handler(|values| {
                            let this = values[0].get::<<Self as ObjectSubclass>::Type>().unwrap();
                            this.imp().void_signal();
                            None
                        })
                        .action()
                        .build()]
                })
            }
        }
    }

    glib::wrapper! {
        pub struct Base(ObjectSubclass<imp::Base>);
    }
}

fn main() {}
