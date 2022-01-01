// Take a look at the license at the top of the repository in the LICENSE file.

use crate::AsyncResult;
use crate::Cancellable;
use glib::object::IsA;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::value::ValueType;
use glib::Cast;
use libc::c_void;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

// Implemented manually to make it generic over the return type to ensure the API
// is sound when the task is moved across threads.

glib::wrapper! {
    #[doc(alias = "GTask")]
    pub struct Task<V: ValueType>(Object<ffi::GTask, ffi::GTaskClass>) @implements AsyncResult;

    match fn {
        type_ => || ffi::g_task_get_type(),
    }
}

impl<V: ValueType> Task<V> {
    #[doc(alias = "g_task_new")]
    pub fn new<P, Q>(
        source_object: Option<&glib::Object>,
        cancellable: Option<&P>,
        callback: Q,
    ) -> Self
    where
        P: IsA<Cancellable>,
        Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static,
    {
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

    #[doc(alias = "g_task_get_cancellable")]
    #[doc(alias = "get_cancellable")]
    pub fn cancellable(&self) -> Cancellable {
        unsafe { from_glib_none(ffi::g_task_get_cancellable(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_check_cancellable")]
    #[doc(alias = "get_check_cancellable")]
    pub fn is_check_cancellable(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_check_cancellable(self.to_glib_none().0)) }
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

    #[doc(alias = "g_task_get_completed")]
    #[doc(alias = "get_completed")]
    pub fn is_completed(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_completed(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> glib::MainContext {
        unsafe { from_glib_none(ffi::g_task_get_context(self.to_glib_none().0)) }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_task_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_task_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_get_return_on_cancel")]
    #[doc(alias = "get_return_on_cancel")]
    pub fn is_return_on_cancel(&self) -> bool {
        unsafe { from_glib(ffi::g_task_get_return_on_cancel(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_had_error")]
    pub fn had_error(&self) -> bool {
        unsafe { from_glib(ffi::g_task_had_error(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_return_error_if_cancelled")]
    pub fn return_error_if_cancelled(&self) -> bool {
        unsafe { from_glib(ffi::g_task_return_error_if_cancelled(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_task_set_check_cancellable")]
    pub fn set_check_cancellable(&self, check_cancellable: bool) {
        unsafe {
            ffi::g_task_set_check_cancellable(self.to_glib_none().0, check_cancellable.into_glib());
        }
    }

    #[cfg(any(feature = "v2_60", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_60")))]
    #[doc(alias = "g_task_set_name")]
    pub fn set_name(&self, name: Option<&str>) {
        unsafe {
            ffi::g_task_set_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "g_task_set_return_on_cancel")]
    pub fn set_return_on_cancel(&self, return_on_cancel: bool) -> bool {
        unsafe {
            from_glib(ffi::g_task_set_return_on_cancel(
                self.to_glib_none().0,
                return_on_cancel.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_task_is_valid")]
    pub fn is_valid(
        result: &impl IsA<AsyncResult>,
        source_object: Option<&impl IsA<glib::Object>>,
    ) -> bool {
        unsafe {
            from_glib(ffi::g_task_is_valid(
                result.as_ref().to_glib_none().0,
                source_object.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "completed")]
    pub fn connect_completed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_completed_trampoline<V, F>(
            this: *mut ffi::GTask,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            V: ValueType,
            F: Fn(&Task<V>) + 'static,
        {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::completed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_completed_trampoline::<V, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "g_task_return_error")]
    pub fn return_error(&self, error: glib::Error) {
        unsafe {
            ffi::g_task_return_error(self.to_glib_none().0, error.to_glib_full() as *mut _);
        }
    }

    #[doc(alias = "g_task_return_value")]
    pub fn return_value(&self, result: &V) {
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

    #[doc(alias = "g_task_propagate_value")]
    pub fn propagate_value(&self) -> Result<V, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let value = ffi::g_task_propagate_pointer(self.to_glib_none().0, &mut error);
            if error.is_null() {
                let value =
                    Option::<glib::Value>::from_glib_full(value as *mut glib::gobject_ffi::GValue)
                        .expect("Task::propagate() called before Task::return_result()");
                Ok(V::from_value(&value))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<V: ValueType + Send> Task<V> {
    #[doc(alias = "g_task_run_in_thread")]
    pub fn run_in_thread<S, Q>(&self, task_func: Q)
    where
        S: IsA<glib::Object> + Send,
        Q: FnOnce(&Self, Option<&S>, Option<&Cancellable>),
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

        unsafe extern "C" fn trampoline<V, S, Q>(
            task: *mut ffi::GTask,
            source_object: *mut glib::gobject_ffi::GObject,
            user_data: glib::ffi::gpointer,
            cancellable: *mut ffi::GCancellable,
        ) where
            V: ValueType,
            S: IsA<glib::Object> + Send,
            Q: FnOnce(&Task<V>, Option<&S>, Option<&Cancellable>),
            Q: Send + 'static,
        {
            let task = Task::from_glib_borrow(task);
            let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
            let cancellable = Option::<Cancellable>::from_glib_borrow(cancellable);
            let task_func: Box_<Q> = Box::from_raw(user_data as *mut _);
            task_func(
                task.as_ref(),
                source_object.as_ref().as_ref().map(|s| s.unsafe_cast_ref()),
                cancellable.as_ref().as_ref(),
            );
        }

        let task_func = trampoline::<V, S, Q>;
        unsafe {
            ffi::g_task_run_in_thread(self.to_glib_none().0, Some(task_func));
        }
    }
}

unsafe impl<V: ValueType + Send> Send for Task<V> {}
unsafe impl<V: ValueType + Send> Sync for Task<V> {}

impl<V: ValueType> fmt::Display for Task<V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Task")
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
                    let t = a.downcast_ref::<crate::Task<i32>>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            t.return_value(&100_i32);
        }) {
            Err(_) => panic!(),
            Ok(i) => assert_eq!(i, 100),
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
                    let t = a.downcast_ref::<crate::Task<glib::Object>>().unwrap();
                    tx.send(t.propagate_value()).unwrap();
                    l.quit();
                },
            );
            let my_object = MySimpleObject::new();
            my_object.set_size(100);
            t.return_value(&my_object.upcast::<glib::Object>());
        }) {
            Err(_) => panic!(),
            Ok(o) => {
                let o = o.downcast::<MySimpleObject>().unwrap();
                assert_eq!(o.size(), Some(100));
            }
        }
    }

    #[test]
    fn test_error() {
        match run_async_local(|tx, l| {
            let c = crate::Cancellable::new();
            let t = crate::Task::<i32>::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task<i32>>().unwrap();
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
            let t = crate::Task::<i32>::new(
                None,
                Some(&c),
                move |a: &AsyncResult, _b: Option<&glib::Object>| {
                    let t = a.downcast_ref::<crate::Task<i32>>().unwrap();
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
