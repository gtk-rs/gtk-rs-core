// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AsyncResult;
use crate::Cancellable;
use crate::Task;
use glib::object::IsA;
use glib::translate::*;
use glib::value::ValueType;
use libc::c_void;
use std::boxed::Box as Box_;
use std::ptr;

impl Task {
    #[doc(alias = "g_task_new")]
    pub fn new<P: IsA<Cancellable>, Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static>(
        source_object: Option<&glib::Object>,
        cancellable: Option<&P>,
        callback: Q,
    ) -> Task {
        let callback_data = Box_::new(callback);
        unsafe extern "C" fn trampoline<
            Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static,
        >(
            source_object: *mut glib::gobject_ffi::GObject,
            res: *mut ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
            let res = AsyncResult::from_glib_borrow(res);
            let callback: Box_<Q> = Box::from_raw(user_data as *mut _);
            callback(&res, source_object.as_ref().as_ref());
        }
        let callback = trampoline::<Q>;
        unsafe {
            from_glib_full(ffi::g_task_new(
                source_object.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(callback_data) as *mut _,
            ))
        }
    }

    #[doc(alias = "g_task_return_error")]
    pub fn return_error(&self, error: glib::Error) {
        unsafe {
            ffi::g_task_return_error(self.to_glib_none().0, error.to_glib_full() as *mut _);
        }
    }

    #[doc(alias = "get_priority")]
    #[doc(alias = "g_task_get_priority")]
    pub fn priority(&self) -> glib::source::Priority {
        unsafe { FromGlib::from_glib(ffi::g_task_get_priority(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_set_priority")]
    pub fn set_priority(&self, priority: glib::source::Priority) {
        unsafe {
            ffi::g_task_set_priority(self.to_glib_none().0, priority.into_glib());
        }
    }
}

pub trait TaskExtManual<T: ValueType> {
    #[doc(alias = "g_task_return_value")]
    fn return_value(&self, result: &T);

    #[doc(alias = "g_task_propagate_value")]
    fn propagate_value(&self) -> Result<T, glib::Error>;
}

impl<T: ValueType> TaskExtManual<T> for Task {
    fn return_value(&self, result: &T) {
        unsafe extern "C" fn value_free(value: *mut c_void) {
            let _: glib::Value = from_glib_full(value as *mut glib::gobject_ffi::GValue);
        }

        unsafe {
            ffi::g_task_return_pointer(
                self.to_glib_none().0,
                result.to_value().to_glib_full() as *mut _,
                Some(value_free),
            )
        }
    }

    fn propagate_value(&self) -> Result<T, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let value = ffi::g_task_propagate_pointer(self.to_glib_none().0, &mut error);
            if !error.is_null() {
                return Err(from_glib_full(error));
            }
            let value =
                Option::<glib::Value>::from_glib_full(value as *mut glib::gobject_ffi::GValue)
                    .unwrap_or_else(|| glib::Value::from_type(glib::types::Type::UNIT));
            Ok(T::from_value(&value))
        }
    }
}

pub trait TaskExtManualSend<T: ValueType + Send>: TaskExtManual<T> {
    #[doc(alias = "g_task_run_in_thread")]
    fn run_in_thread<Q>(&self, task_func: Q)
    where
        Q: FnOnce(&Self, Option<&glib::Object>, Option<&Cancellable>) -> Result<T, glib::Error>,
        Q: Send + 'static;
}

impl<T: ValueType + Send> TaskExtManualSend<T> for Task {
    fn run_in_thread<Q>(&self, task_func: Q)
    where
        Q: FnOnce(&Self, Option<&glib::Object>, Option<&Cancellable>) -> Result<T, glib::Error>,
        Q: Send + 'static,
    {
        let task_func_data = Box_::new(task_func);

        // We store the func pointer into the task data.
        // We intentionally do not expose a way to set the task data in the bindings.
        // If we detect that the task data is set, there is not much we can do, so we panic.
        unsafe {
            assert!(
                ffi::g_task_get_task_data(self.to_glib_none().0).is_null(),
                "Task data was manually set or the task was run thread multiple times"
            );

            ffi::g_task_set_task_data(
                self.to_glib_none().0,
                Box_::into_raw(task_func_data) as *mut _,
                None,
            );
        }

        unsafe extern "C" fn trampoline<Q, V>(
            task: *mut ffi::GTask,
            source_object: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
            cancellable: *mut ffi::GCancellable,
        ) where
            Q: FnOnce(&Task, Option<&glib::Object>, Option<&Cancellable>) -> Result<V, glib::Error>,
            Q: Send + 'static,
            V: ValueType + Send,
        {
            let task = Task::from_glib_borrow(task);
            let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
            let cancellable = Option::<Cancellable>::from_glib_borrow(cancellable);
            let task_func: Box_<Q> = Box::from_raw(user_data as *mut _);
            match task_func(
                task.as_ref(),
                source_object.as_ref().as_ref(),
                cancellable.as_ref().as_ref(),
            ) {
                Ok(v) => task.return_value(&v),
                Err(e) => task.return_error(e),
            }
        }

        let task_func = trampoline::<Q, T>;
        unsafe {
            ffi::g_task_run_in_thread(self.to_glib_none().0, Some(task_func));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;
    use crate::test_util::run_async_local;

    #[test]
    fn test_int_async_result() {
        match run_async_local(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            t.return_value(&100_i32.to_value());
        }) {
            Err(_) => panic!(),
            Ok(i) => {
                assert_eq!(i.get::<i32>().unwrap(), 100);
            }
        }
    }

    #[test]
    fn test_object_async_result() {
        use glib::subclass::prelude::*;
        pub struct MySimpleObjectPrivate {
            pub size: std::cell::RefCell<Option<i64>>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MySimpleObjectPrivate {
            const NAME: &'static str = "MySimpleObjectPrivate";
            type Type = MySimpleObject;

            fn new() -> Self {
                Self {
                    size: std::cell::RefCell::new(Some(100)),
                }
            }
        }

        impl ObjectImpl for MySimpleObjectPrivate {}

        glib::wrapper! {
            pub struct MySimpleObject(ObjectSubclass<MySimpleObjectPrivate>);
        }

        impl MySimpleObject {
            pub fn new() -> Self {
                glib::Object::new(&[]).expect("Failed to create MySimpleObject")
            }

            #[doc(alias = "get_size")]
            pub fn size(&self) -> Option<i64> {
                *self.imp().size.borrow()
            }

            pub fn set_size(&self, size: i64) {
                self.imp().size.borrow_mut().replace(size);
            }
        }

        impl Default for MySimpleObject {
            fn default() -> Self {
                Self::new()
            }
        }

        match run_async_local(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            let my_object = MySimpleObject::new();
            my_object.set_size(100);
            t.return_value(&my_object.upcast::<glib::Object>().to_value());
        }) {
            Err(_) => panic!(),
            Ok(o) => {
                let o = o
                    .get::<glib::Object>()
                    .unwrap()
                    .downcast::<MySimpleObject>()
                    .unwrap();

                assert_eq!(o.size(), Some(100));
            }
        }
    }

    #[test]
    fn test_error() {
        match run_async_local(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            t.return_error(glib::Error::new(
                crate::IOErrorEnum::WouldBlock,
                "WouldBlock",
            ));
        }) {
            Err(e) => match e.kind().unwrap() {
                crate::IOErrorEnum::WouldBlock => {}
                _ => panic!(),
            },
            Ok(_) => panic!(),
        }
    }

    #[test]
    fn test_cancelled() {
        match run_async_local(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            c.cancel();
            t.return_error_if_cancelled();
        }) {
            Err(e) => match e.kind().unwrap() {
                crate::IOErrorEnum::Cancelled => {}
                _ => panic!(),
            },
            Ok(_) => panic!(),
        }
    }
}
