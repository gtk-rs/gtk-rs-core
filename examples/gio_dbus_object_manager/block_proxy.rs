use std::marker::PhantomData;
use std::path::PathBuf;

use gio::prelude::*;
use gio::subclass::prelude::*;

use crate::utils::cached_dbus_property;

glib::wrapper! {
    pub(crate) struct UDisksBlockProxy(ObjectSubclass<imp::UDisksBlockProxy>)
        @extends gio::DBusProxy,
        @implements gio::AsyncInitable, gio::Initable, gio::DBusInterface;
}

mod imp {
    use super::*;

    #[derive(Default, Debug, glib::Properties)]
    #[properties(wrapper_type = super::UDisksBlockProxy)]
    pub(crate) struct UDisksBlockProxy {
        #[property(name = "device", type = PathBuf, get = |this: &Self| cached_dbus_property(&*this.obj(), "Device"))]
        _property: PhantomData<()>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UDisksBlockProxy {
        const NAME: &'static str = "SampleApplicationUDisksBlockProxy";
        type Type = super::UDisksBlockProxy;
        type ParentType = gio::DBusProxy;
    }

    #[glib::derived_properties]
    impl ObjectImpl for UDisksBlockProxy {}

    impl DBusProxyImpl for UDisksBlockProxy {
        fn g_properties_changed(
            &self,
            changed_properties: &glib::Variant,
            invalidated_properties: &glib::StrVRef,
        ) {
            self.parent_g_properties_changed(changed_properties, invalidated_properties);
            if let Ok(changed_properties) = changed_properties.array_iter_str() {
                let obj = self.obj();
                for prop in changed_properties {
                    #[allow(clippy::single_match)]
                    match prop {
                        "Device" => obj.notify("device"),
                        _ => {}
                    }
                }
            };
        }
    }

    impl DBusInterfaceImpl for UDisksBlockProxy {}
    impl InitableImpl for UDisksBlockProxy {}
    impl AsyncInitableImpl for UDisksBlockProxy {}
}
