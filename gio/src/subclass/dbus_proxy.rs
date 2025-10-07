// Take a look at the license at the top of the repository in the LICENSE file.

#![deny(unsafe_op_in_unsafe_fn)]

use glib::{prelude::*, subclass::prelude::*};

use crate::subclass::prelude::DBusInterfaceImpl;
use crate::DBusProxy;

pub trait DBusProxyImpl:
    ObjectImpl + DBusInterfaceImpl + ObjectSubclass<Type: IsA<DBusProxy>>
{
}

pub trait DBusProxyImplExt: DBusProxyImpl {}

impl<T: DBusProxyImpl> DBusProxyImplExt for T {}

unsafe impl<T: DBusProxyImpl> IsSubclassable<T> for DBusProxy {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
