use crate::{Property, PropertyRead, PropertyWrite, PropertyWriteNested};

use once_cell::sync::OnceCell as SyncOnceCell;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::RwLock;

macro_rules! define_construct {
    (@common $ident:ident, $container:ident, $inner:ty) => {
        #[derive(Debug)]
        pub struct $ident<T>($container<$inner>);
        impl<T> Default for $ident<T> {
            fn default() -> Self {
                Self::new_empty()
            }
        }
        impl<T: Property> Property for $ident<T> {
            type Value = T;
            type ParamSpec = T::ParamSpec;
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

        impl<T> PropertyWriteNested for $ident<T> {
            type SetNestedValue = T;
            fn set_nested<F: FnOnce(&mut Self::SetNestedValue)>(&self, f: F) {
                PropertyWriteNested::set_nested(&self.0, |v| f(&mut v.as_mut().unwrap()))
            }
        }
        impl<T> PropertyRead for $ident<T> {
            type Value = T;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                PropertyRead::get(&self.0, |v| f(v.as_ref().unwrap()))
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
        impl<T> PropertyWrite for $ident<T> {
            type SetValue = T;
            fn set(&self, v: T) {
                PropertyWrite::set(&self.0, v)
            }
        }
        impl<T> PropertyRead for $ident<T> {
            type Value = T;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                PropertyRead::get(&self.0, |v| f(v))
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
