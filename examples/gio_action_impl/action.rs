use gio::{prelude::*, subclass::prelude::*};

mod imp {
    use super::*;
    use std::cell::OnceCell;

    #[derive(glib::Properties, Default)]
    #[properties(wrapper_type = super::RenamedAction)]
    pub struct RenamedAction {
        #[property(get, construct_only)]
        pub new_name: OnceCell<glib::GString>,

        #[property(get, construct_only)]
        pub action: OnceCell<gio::Action>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for RenamedAction {
        const NAME: &'static str = "ExampleRenamedAction";
        type Type = super::RenamedAction;
        type Interfaces = (gio::Action,);
    }

    #[glib::derived_properties]
    impl ObjectImpl for RenamedAction {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            if !self.delegate_set_property(id, value, pspec) {
                self.derived_set_property(id, value, pspec);
            }
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.delegate_get_property(id, pspec)
                .unwrap_or_else(|| self.derived_property(id, pspec))
        }
    }

    impl ActionImpl for RenamedAction {
        fn name(&self) -> glib::GString {
            self.obj().new_name()
        }

        fn parameter_type(&self) -> Option<glib::VariantType> {
            self.obj().action().parameter_type()
        }

        fn state_type(&self) -> Option<glib::VariantType> {
            self.obj().action().state_type()
        }

        fn state_hint(&self) -> Option<glib::Variant> {
            self.obj().action().state_hint()
        }

        fn is_enabled(&self) -> bool {
            self.obj().action().is_enabled()
        }

        fn state(&self) -> Option<glib::Variant> {
            self.obj().action().state()
        }

        fn change_state(&self, value: glib::Variant) {
            self.obj().action().change_state(&value);
        }

        fn activate(&self, parameter: Option<glib::Variant>) {
            self.obj().action().activate(parameter.as_ref());
        }
    }
}

glib::wrapper! {
    pub struct RenamedAction(ObjectSubclass<imp::RenamedAction>)
        @implements gio::Action;
}

impl RenamedAction {
    pub fn new(name: &str, action: &impl IsA<gio::Action>) -> Self {
        glib::Object::builder()
            .property("new-name", name)
            .property("action", action)
            .build()
    }
}
