use gio::prelude::*;
use gio::subclass::prelude::*;
use gio::DBusProxy;
use glib::{object_subclass, Properties};

glib::wrapper! {
    pub(crate) struct UDisksDriveProxy(ObjectSubclass<imp::UDisksDriveProxy>)
        @extends gio::DBusProxy,
        @implements gio::AsyncInitable, gio::Initable, gio::DBusInterface;
}

mod imp {
    use super::*;
    use glib::derived_properties;
    use std::marker::PhantomData;

    #[derive(Default, Debug, Properties)]
    #[properties(wrapper_type = super::UDisksDriveProxy)]
    pub(crate) struct UDisksDriveProxy {
        #[property(name = "id", type = String, get = |this: &Self| this.get_dbus_property("Id"))]
        #[property(name = "vendor", type = String, get = |this: &Self| this.get_dbus_property("Vendor"))]
        #[property(name = "size", type = u64, get = |this: &Self| this.get_dbus_property("Size"))]
        #[property(name = "removable", type = bool, get = |this: &Self| this.get_dbus_property("Removable"))]
        #[property(name = "media-compatibility", type = Vec<String>, get = |this: &Self| this.get_dbus_property("MediaCompatibility"))]
        _marker: PhantomData<()>,
    }

    #[object_subclass]
    impl ObjectSubclass for UDisksDriveProxy {
        const NAME: &'static str = "SampleApplicationUDisksDriveProxy";
        type Type = super::UDisksDriveProxy;
        type ParentType = DBusProxy;
    }

    #[derived_properties]
    impl ObjectImpl for UDisksDriveProxy {}

    impl DBusProxyImpl for UDisksDriveProxy {
        fn g_properties_changed(
            &self,
            changed_properties: &glib::Variant,
            invalidated_properties: &glib::StrVRef,
        ) {
            self.parent_g_properties_changed(changed_properties, invalidated_properties);
            if let Ok(changed_properties) = changed_properties.array_iter_str() {
                let obj = self.obj();
                for prop in changed_properties {
                    match prop {
                        "Id" => obj.notify("id"),
                        "Vendor" => obj.notify("vendor"),
                        "Size" => obj.notify("size"),
                        "Removable" => obj.notify("removable"),
                        _ => {}
                    }
                }
            };
        }
    }

    impl UDisksDriveProxy {
        fn get_dbus_property<T: FromVariant + Default>(&self, name: &str) -> T {
            self.obj()
                .cached_property(name)
                .map(|v| v.get().expect("DBus Property to have correct type"))
                .unwrap_or_default()
        }
    }

    impl DBusInterfaceImpl for UDisksDriveProxy {}
    impl InitableImpl for UDisksDriveProxy {}
    impl AsyncInitableImpl for UDisksDriveProxy {}
}
