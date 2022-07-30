// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{HasParamSpec, Property, PropertyGet, PropertySet, PropertySetNested};

use once_cell::sync::OnceCell as SyncOnceCell;
use once_cell::unsync::OnceCell;
use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

macro_rules! define_construct {
    (@common $ident:ident, $container:ident, $inner:ty) => {
        // rustdoc-stripper-ignore-next
        /// Wrapper around a container type to be used for custom glib properties which get set during
        /// the construct phase. This is especially useful when used alongside the Properties macro.
        #[derive(Debug)]
        pub struct $ident<T>($container<$inner>);
        impl<T> Default for $ident<T> {
            fn default() -> Self {
                Self::new_empty()
            }
        }
        impl<T: Property + HasParamSpec> Property for $ident<T> {
            type Value = T;
        }

        impl<T> ::std::ops::Deref for $ident<T> {
            type Target = $container<$inner>;
                fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
    (@with_option_type $ident:ident, $container:ident) => {
        define_construct!(@common $ident, $container, Option<T>);
        impl<T> $ident<T> {
            pub fn new(value: T) -> Self {
                $ident($container::new(Some(value)))
            }
            pub fn new_empty() -> Self {
                $ident($container::default())
            }
        }

        impl<T> PropertySetNested for $ident<T> {
            type SetNestedValue = T;
            fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
                PropertySetNested::set_nested(&self.0, |v| f(&mut v.as_mut().unwrap()))
            }
        }
        impl<T> PropertyGet for $ident<T> {
            type Value = T;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                PropertyGet::get(&self.0, |v| f(v.as_ref().unwrap()))
            }
        }
    };
    // By "uninit type" I mean: a type that doesn't require an internal `Option`
    // and can be created as empty
    (@with_uninit_type $ident:ident, $container:ident, $inner:ty, $init:expr) => {
        impl<T> $ident<T> {
            pub fn new(value: T) -> Self {
                $ident($init(value))
            }
            pub fn new_empty() -> Self {
                $ident($container::default())
            }
        }

        define_construct!(@common $ident, $container, $inner);
        impl<T> PropertySet for $ident<T> {
            type SetValue = T;
            fn set(&self, v: T) {
                PropertySet::set(&self.0, v)
            }
        }
        impl<T> PropertyGet for $ident<T> {
            type Value = T;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                PropertyGet::get(&self.0, |v| f(v))
            }
        }
    }
}

define_construct!(@with_option_type ConstructRefCell, RefCell);
define_construct!(@with_option_type ConstructMutex, Mutex);
define_construct!(@with_option_type ConstructRwLock, RwLock);
define_construct!(@with_uninit_type ConstructOnceCell, OnceCell, T,
    |v: T| {
        let oc = OnceCell::new();
        oc.set(v).map_err(|_| "set failed in ConstructOnceCell").unwrap();
        oc
    }
);
define_construct!(@with_uninit_type ConstructSyncOnceCell, SyncOnceCell, T,
    |v: T| {
        let oc = SyncOnceCell::new();
        oc.set(v).map_err(|_| "set failed in ConstructSyncOnceCell").unwrap();
        oc
    }
);

// Manual implementation because Cell often requires `Copy`, so `Debug` can't be derived,
// `PropertyGet` and `PropertySet` can't be generated as with the other types...
pub struct ConstructCell<T>(Cell<Option<T>>);
impl<T> Default for ConstructCell<T> {
    fn default() -> Self {
        Self::new_empty()
    }
}
impl<T: Property + HasParamSpec> Property for ConstructCell<T> {
    type Value = T;
}

impl<T> std::ops::Deref for ConstructCell<T> {
    type Target = Cell<Option<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> ConstructCell<T> {
    pub fn new(value: T) -> Self {
        Self(Cell::new(Some(value)))
    }
    pub fn new_empty() -> Self {
        Self(Cell::default())
    }
}
impl<T: Copy + std::fmt::Debug> std::fmt::Debug for ConstructCell<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConstructCell").field("0", &self.0).finish()
    }
}
impl<T: Copy> PropertySet for ConstructCell<T> {
    type SetValue = T;
    fn set(&self, v: T) {
        PropertySet::set(&self.0, Some(v))
    }
}
impl<T: Copy> PropertyGet for ConstructCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        PropertyGet::get(&self.0, |v| f(v.as_ref().unwrap()))
    }
}
