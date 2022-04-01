use crate::{Property, PropertyRead, PropertyWrite};
use std::cell::RefCell;
use std::sync::Mutex;

macro_rules! define_construct {
    ($name:ident, $container:ident, $inner:ty, $init:expr, $init_empty:expr) => {
        #[derive(Debug)]
        pub struct $name<T>($container<$inner>);
        impl<T> $name<T> {
            pub fn new(value: T) -> Self {
                $name($init(value))
            }
            pub fn new_empty() -> Self {
                $name($init_empty)
            }
        }

        impl<T> Default for $name<T> {
            fn default() -> Self {
                Self::new_empty()
            }
        }
        impl<T: Property> Property for $name<T>
        {
            type Value = T;
            type ParamSpec = T::ParamSpec;
        }
        impl<T> PropertyRead for $name<T>
        {
            type Value = T;
            fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
                PropertyRead::get(&self.0, |v| f(v.as_ref().unwrap()))
            }
        }
        impl<T> PropertyWrite for $name<T>
        {
            type Value = T;
            fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
                PropertyWrite::set(&self.0, |v| f(&mut v.as_mut().unwrap()))
            }
        }
    }
}

define_construct!(ConstructRefCell, RefCell, Option<T>, |v| RefCell::new(Some(v)), RefCell::new(None));
define_construct!(ConstructMutex, Mutex, Option<T>, |v| Mutex::new(Some(v)), Mutex::new(None));
define_construct!(ConstructRwLock, Mutex, Option<T>, |v| Mutex::new(Some(v)), Mutex::new(None));
// FIXME: define Construct for OnceCells. Needs some Property API redesign
