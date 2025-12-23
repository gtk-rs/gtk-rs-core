use std::marker::PhantomData;
use std::path::PathBuf;

use gio::prelude::*;
use gio::subclass::prelude::*;
use gio::DBusProxy;
use glib::{object_subclass, Properties};

glib::wrapper! {
    pub(crate) struct UDisksBlockProxy(ObjectSubclass<imp::UDisksBlockProxy>)
        @extends gio::DBusProxy,
        @implements gio::AsyncInitable, gio::Initable, gio::DBusInterface;
}

mod imp {
    use super::*;
    use glib::derived_properties;

    #[derive(Default, Debug, Properties)]
    #[properties(wrapper_type = super::UDisksBlockProxy)]
    pub(crate) struct UDisksBlockProxy {
        #[property(name = "device", type = PathBuf, get = |this: &Self| this.get_dbus_property("Device"))]
        _marker: PhantomData<()>,
    }

    #[object_subclass]
    impl ObjectSubclass for UDisksBlockProxy {
        const NAME: &'static str = "SampleApplicationUDisksBlockProxy";
        type Type = super::UDisksBlockProxy;
        type ParentType = DBusProxy;
    }

    #[derived_properties]
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

    impl UDisksBlockProxy {
        fn get_dbus_property<T: FromVariant + Default>(&self, name: &str) -> T {
            self.obj()
                .cached_property(name)
                .map(|v| v.get().expect("DBus Property to have correct type"))
                .unwrap_or_default()
        }
    }

    impl DBusInterfaceImpl for UDisksBlockProxy {}
    impl InitableImpl for UDisksBlockProxy {}
    impl AsyncInitableImpl for UDisksBlockProxy {}
}
