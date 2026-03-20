// Take a look at the license at the top of the repository in the LICENSE file.

use std::ffi::CStr;

use crate::ffi;
use crate::{DBusInterfaceInfo, DBusPropertyInfo};
use glib::translate::*;

impl DBusInterfaceInfo {
    pub fn name(&self) -> &str {
        unsafe {
            let c_obj = self.as_ptr();
            let name = (*c_obj).name;
            assert!(!name.is_null());
            let c_str = CStr::from_ptr(name);
            c_str.to_str().unwrap()
        }
    }

    pub fn properties(&self) -> DBusInterfaceInfoPropertiesIter<'_> {
        DBusInterfaceInfoPropertiesIter::new(self)
    }
}

pub struct DBusInterfaceInfoPropertiesIter<'a> {
    _stash: Stash<'a, *mut ffi::GDBusInterfaceInfo, DBusInterfaceInfo>,
    next_property: *mut *mut ffi::GDBusPropertyInfo,
}

impl<'a> DBusInterfaceInfoPropertiesIter<'a> {
    fn new(info: &'a DBusInterfaceInfo) -> Self {
        let stash: Stash<*mut ffi::GDBusInterfaceInfo, _> = info.to_glib_none();
        // SAFETY:
        // * the stash is stored in the struct to keep the pointer valid
        // * though not explicitly documented, this struct is assumed to be immutable after creation
        //   (with the exception of ref_count of course). See usage in gdbusconnection.c
        let next_property = unsafe { *stash.0 }.properties;
        Self {
            _stash: stash,
            next_property,
        }
    }
}

impl<'a> Iterator for DBusInterfaceInfoPropertiesIter<'a> {
    type Item = DBusPropertyInfo;

    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: `self.next_property` is a pointer to a NULL-terminated
        // array of pointers to GDBusPropertyInfo.
        unsafe {
            assert!(!self.next_property.is_null());
            let property = *self.next_property;
            if !property.is_null() {
                self.next_property = self.next_property.add(1);
                Some(from_glib_none(property))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::DBusNodeInfo;

    #[test]
    fn iterate_properties() {
        const XML: &str = r#"
        <node>
            <interface name='com.github.gtk_rs.Test'>
                <property name='Name' type='s' access='read' />
                <property name='Count' type='x' access='write' />
            </interface>
        </node>
        "#;
        let node_info = DBusNodeInfo::for_xml(XML).unwrap();
        let interface = node_info
            .lookup_interface("com.github.gtk_rs.Test")
            .unwrap();

        let mut properties = interface.properties();
        let property = properties.next().unwrap();
        assert_eq!("Name", property.name());
        let property = properties.next().unwrap();
        assert_eq!("Count", property.name());
        assert!(properties.next().is_none());
    }

    #[test]
    fn iterate_empty_properties() {
        const XML: &str = r#"
        <node>
            <interface name='com.github.gtk_rs.Test' />
        </node>
        "#;
        let node_info = DBusNodeInfo::for_xml(XML).unwrap();
        let interface = node_info
            .lookup_interface("com.github.gtk_rs.Test")
            .unwrap();

        let mut properties = interface.properties();
        assert!(properties.next().is_none());
    }

    #[test]
    fn iterator_is_sealed() {
        const XML: &str = r#"
        <node>
            <interface name='com.github.gtk_rs.Test'>
                <property name='Count' type='x' access='write' />
            </interface>
        </node>
        "#;
        let node_info = DBusNodeInfo::for_xml(XML).unwrap();
        let interface = node_info
            .lookup_interface("com.github.gtk_rs.Test")
            .unwrap();

        let mut properties = interface.properties();
        let _property = properties.next().unwrap();
        assert!(properties.next().is_none());

        assert!(properties.next().is_none());
    }
}
