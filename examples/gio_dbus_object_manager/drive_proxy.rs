use std::marker::PhantomData;

use gio::prelude::*;
use gio::subclass::prelude::*;

use crate::utils::cached_dbus_property;

glib::wrapper! {
    pub(crate) struct UDisksDriveProxy(ObjectSubclass<imp::UDisksDriveProxy>)
        @extends gio::DBusProxy,
        @implements gio::AsyncInitable, gio::Initable, gio::DBusInterface;
}

mod imp {
    use super::*;

    #[derive(Default, Debug, glib::Properties)]
    #[properties(wrapper_type = super::UDisksDriveProxy)]
    pub(crate) struct UDisksDriveProxy {
        #[property(name = "id", type = String, get = |this: &Self| cached_dbus_property(&*this.obj(), "Id"))]
        #[property(name = "vendor", type = String, get = |this: &Self| cached_dbus_property(&*this.obj(), "Vendor"))]
        #[property(name = "size", type = u64, get = |this: &Self| cached_dbus_property(&*this.obj(), "Size"))]
        #[property(name = "removable", type = bool, get = |this: &Self| cached_dbus_property(&*this.obj(), "Removable"))]
        #[property(name = "media-compatibility", type = Vec<String>, get = |this: &Self| cached_dbus_property(&*this.obj(), "MediaCompatibility"))]
        _property: PhantomData<()>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UDisksDriveProxy {
        const NAME: &'static str = "SampleApplicationUDisksDriveProxy";
        type Type = super::UDisksDriveProxy;
        type ParentType = gio::DBusProxy;
    }

    #[glib::derived_properties]
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

    impl DBusInterfaceImpl for UDisksDriveProxy {}
    impl InitableImpl for UDisksDriveProxy {}
    impl AsyncInitableImpl for UDisksDriveProxy {}
}
