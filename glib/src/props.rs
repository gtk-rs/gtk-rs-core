// Take a look at the license at the top of the repository in the LICENSE file.

use once_cell::sync::OnceCell;
use std::cell::Cell;
use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::RwLock;

use crate::HasParamSpec;

pub trait PropType {
    type HasSpecType;
}
impl<T: HasParamSpec> PropType for T {
    type HasSpecType = T;
}
impl<T: PropType> PropType for Option<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for PhantomData<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for RefCell<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for Cell<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for Mutex<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for Rc<T> {
    type HasSpecType = T::HasSpecType;
}
impl<T: PropType> PropType for Arc<T> {
    type HasSpecType = T::HasSpecType;
}

pub trait PropRead {
    type Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R;
}
pub trait PropWrite {
    type Value;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F);
}

impl<T> PropRead for RefCell<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.borrow())
    }
}
impl<T> PropWrite for RefCell<T> {
    type Value = T;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        f(&mut self.borrow_mut());
    }
}

impl<T> PropRead for Mutex<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.lock().unwrap())
    }
}
impl<T> PropWrite for Mutex<T> {
    type Value = T;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        f(&mut self.lock().unwrap());
    }
}

impl<T> PropRead for RwLock<T> {
    type Value = T;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        f(&self.read().unwrap())
    }
}
impl<T> PropWrite for RwLock<T> {
    type Value = T;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        f(&mut self.write().unwrap());
    }
}

impl<T: PropRead> PropRead for Rc<T> {
    type Value = T::Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        (**self).get(f)
    }
}
impl<T: PropWrite> PropWrite for Rc<T> {
    type Value = T::Value;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        (**self).set(f)
    }
}

impl<T: PropRead> PropRead for Arc<T> {
    type Value = T::Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        (**self).get(f)
    }
}
impl<T: PropWrite> PropWrite for Arc<T> {
    type Value = T::Value;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        (**self).set(f)
    }
}

impl<T: PropRead> PropRead for OnceCell<T> {
    type Value = T::Value;
    fn get<R, F: Fn(&Self::Value) -> R>(&self, f: F) -> R {
        self.get().unwrap().get(f)
    }
}
// This implemenation is a bit of a stretch...
// `ParamStoreWrite` requires a function taking Self::Value, but OnceCell doesn't have
// an internal value before being init. Still, I'm implementing it so that the derive `Props`
// macro can easily work with it
impl<T: PropWrite + Default> PropWrite for OnceCell<T> {
    type Value = T;
    fn set<F: FnOnce(&mut Self::Value)>(&self, f: F) {
        let mut v = Self::Value::default();
        f(&mut v);
        // I can't use `unwrap` because I would have to add a `Debug` bound to _v
        if let Err(_v) = self.set(v) {
            panic!("can't set value of OnceCell multiple times")
        };
    }
}
