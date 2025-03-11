// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{collections::List, prelude::*, subclass::prelude::*, translate::*, Error};
use libc::c_int;

use crate::{ffi, AsyncResult, Cancellable, FileEnumerator, FileInfo};

use std::boxed::Box as Box_;

pub trait FileEnumeratorImpl: ObjectImpl + ObjectSubclass<Type: IsA<FileEnumerator>> {
    fn next_file(&self, cancellable: Option<&Cancellable>) -> Result<Option<FileInfo>, Error> {
        self.parent_next_file(cancellable)
    }

    fn close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(cancellable)
    }

    fn next_files_async<P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static>(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&Cancellable>,
        callback: Option<P>,
    ) {
        self.parent_next_files_async(num_files, io_priority, cancellable, callback)
    }

    fn next_files_finish(&self, result: Option<&AsyncResult>) -> Result<List<FileInfo>, Error> {
        self.parent_next_files_finish(result)
    }

    fn close_async<P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&Cancellable>,
        callback: Option<P>,
    ) {
        self.parent_close_async(io_priority, cancellable, callback)
    }

    fn close_finish(&self, result: Option<&AsyncResult>) -> Result<(), Error> {
        self.parent_close_finish(result)
    }
}

pub trait FileEnumeratorImplExt: FileEnumeratorImpl {
    fn parent_next_file(
        &self,
        cancellable: Option<&Cancellable>,
    ) -> Result<Option<FileInfo>, Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .next_file
                .expect("No parent class implementation for \"next_file\"");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                cancellable.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(res))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .close_fn
                .expect("No parent class implementation for \"close_fn\"");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                cancellable.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_next_files_async<P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static>(
        &self,
        num_files: i32,
        io_priority: glib::Priority,
        cancellable: Option<&Cancellable>,
        callback: Option<P>,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .next_files_async
                .expect("No parent class implementation for \"next_files_async\"");

            let (callback, user_data): (ffi::GAsyncReadyCallback, glib::ffi::gpointer) = callback
                .map_or((None, std::ptr::null_mut()), |callback| {
                    let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
                        Box_::new(glib::thread_guard::ThreadGuard::new(callback));

                    unsafe extern "C" fn callback_trampoline<
                        P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static,
                    >(
                        source_object: *mut glib::gobject_ffi::GObject,
                        res: *mut ffi::GAsyncResult,
                        user_data: glib::ffi::gpointer,
                    ) {
                        let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
                        let result = AsyncResult::from_glib_borrow(res);
                        let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                            Box_::from_raw(user_data as *mut _);
                        let callback: P = callback.into_inner();
                        callback(source_object.as_ref().as_ref(), result.as_ref());
                    }
                    let callback = callback_trampoline::<P>;

                    (Some(callback), Box_::into_raw(user_data) as *mut _)
                });

            f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                num_files,
                io_priority.into_glib(),
                cancellable.as_ref().to_glib_none().0,
                callback,
                user_data,
            );
        }
    }

    fn parent_next_files_finish(
        &self,
        result: Option<&AsyncResult>,
    ) -> Result<List<FileInfo>, Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .next_files_finish
                .expect("No parent class implementation for \"next_files_finish\"");

            let mut error = std::ptr::null_mut();
            let res = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                result.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(List::from_glib_full(res))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn parent_close_async<P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static>(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&Cancellable>,
        callback: Option<P>,
    ) {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .close_async
                .expect("No parent class implementation for \"close_async\"");

            let (callback, user_data): (ffi::GAsyncReadyCallback, glib::ffi::gpointer) = callback
                .map_or((None, std::ptr::null_mut()), |callback| {
                    let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
                        Box_::new(glib::thread_guard::ThreadGuard::new(callback));

                    unsafe extern "C" fn callback_trampoline<
                        P: FnOnce(Option<&glib::Object>, &AsyncResult) + 'static,
                    >(
                        source_object: *mut glib::gobject_ffi::GObject,
                        res: *mut ffi::GAsyncResult,
                        user_data: glib::ffi::gpointer,
                    ) {
                        let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
                        let result = AsyncResult::from_glib_borrow(res);
                        let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                            Box_::from_raw(user_data as *mut _);
                        let callback: P = callback.into_inner();
                        callback(source_object.as_ref().as_ref(), result.as_ref());
                    }
                    let callback = callback_trampoline::<P>;

                    (Some(callback), Box_::into_raw(user_data) as *mut _)
                });

            f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                io_priority.into_glib(),
                cancellable.as_ref().to_glib_none().0,
                callback,
                user_data,
            );
        }
    }

    fn parent_close_finish(&self, result: Option<&AsyncResult>) -> Result<(), Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .close_finish
                .expect("No parent class implementation for \"close_finish\"");

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                self.obj()
                    .unsafe_cast_ref::<FileEnumerator>()
                    .to_glib_none()
                    .0,
                result.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<T: FileEnumeratorImpl> FileEnumeratorImplExt for T {}

unsafe impl<T: FileEnumeratorImpl> IsSubclassable<T> for FileEnumerator {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.next_file = Some(next_file::<T>);
        klass.close_fn = Some(close_fn::<T>);
        klass.next_files_async = Some(next_files_async::<T>);
        klass.next_files_finish = Some(next_files_finish::<T>);
        klass.close_async = Some(close_async::<T>);
        klass.close_finish = Some(close_finish::<T>);
    }
}

unsafe extern "C" fn next_file<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GFileInfo {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.next_file(cancellable.as_ref());

    match res {
        Ok(fileinfo) => fileinfo.to_glib_full(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn close_fn<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.close(cancellable.as_ref());

    match res {
        Ok(_) => true.into_glib(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            false.into_glib()
        }
    }
}

unsafe extern "C" fn next_files_async<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    num_files: c_int,
    io_priority: c_int,
    cancellable: *mut ffi::GCancellable,
    callback: ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);
    let callback = callback.map(move |callback| {
        move |source_obj: Option<&glib::Object>, res: &AsyncResult| unsafe {
            callback(source_obj.to_glib_none().0, res.to_glib_none().0, user_data)
        }
    });

    imp.next_files_async(
        num_files,
        from_glib(io_priority),
        cancellable.as_ref(),
        callback,
    );
}

unsafe extern "C" fn next_files_finish<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    result: *mut ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> *mut glib::ffi::GList {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let result = Option::<AsyncResult>::from_glib_none(result);

    let res = imp.next_files_finish(result.as_ref());

    match res {
        Ok(files) => files.to_glib_full(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn close_async<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    io_priority: c_int,
    cancellable: *mut ffi::GCancellable,
    callback: ffi::GAsyncReadyCallback,
    user_data: glib::ffi::gpointer,
) {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);
    let callback = callback.map(move |callback| {
        move |source_obj: Option<&glib::Object>, res: &AsyncResult| unsafe {
            callback(source_obj.to_glib_none().0, res.to_glib_none().0, user_data)
        }
    });

    imp.close_async(from_glib(io_priority), cancellable.as_ref(), callback);
}

unsafe extern "C" fn close_finish<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    result: *mut ffi::GAsyncResult,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let result = Option::<AsyncResult>::from_glib_none(result);

    let res = imp.close_finish(result.as_ref());

    match res {
        Ok(_) => true.into_glib(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            false.into_glib()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use futures_channel::oneshot;

    use super::*;
    use crate::{prelude::*, IOErrorEnum};

    mod imp {
        use std::sync::RwLock;

        use super::*;

        #[derive(Default)]
        pub struct MyFileEnumerator {
            pub index: RwLock<i8>,
        }

        #[glib::object_subclass]
        impl ObjectSubclass for MyFileEnumerator {
            const NAME: &'static str = "MyFileEnumerator";
            type Type = super::MyFileEnumerator;
            type ParentType = FileEnumerator;
        }

        impl ObjectImpl for MyFileEnumerator {}

        impl FileEnumeratorImpl for MyFileEnumerator {
            fn next_file(
                &self,
                _cancellable: Option<&Cancellable>,
            ) -> Result<Option<FileInfo>, glib::Error> {
                let mut index = self.index.write().unwrap();
                match *index {
                    -1 => Err(glib::Error::new(IOErrorEnum::Closed, "Closed")),
                    0..10 => {
                        let fileinfo = glib::Object::builder::<FileInfo>().build();
                        fileinfo.set_name(format!("file{}", *index));
                        *index += 1;
                        Ok(Some(fileinfo))
                    }
                    _ => Ok(None),
                }
            }

            fn close(&self, _cancellable: Option<&Cancellable>) -> Result<(), glib::Error> {
                let mut index = self.index.write().unwrap();
                *index = -1;
                Ok(())
            }
        }
    }

    glib::wrapper! {
        pub struct MyFileEnumerator(ObjectSubclass<imp::MyFileEnumerator>) @extends FileEnumerator;
    }

    #[test]
    fn test_next_file() {
        let file_enumerator = glib::Object::new::<MyFileEnumerator>();
        for i in 0..10 {
            let res = file_enumerator.next_file(Cancellable::NONE);
            assert!(res.is_ok(), "{}", res.err().unwrap());
            let res = res.unwrap();
            assert!(res.is_some(), "unexpected None");
            let fileinfo = res.unwrap();
            assert_eq!(fileinfo.name(), PathBuf::from(format!("file{}", i)));
        }
        let res = file_enumerator.next_file(Cancellable::NONE);
        assert!(res.is_ok(), "{}", res.err().unwrap());
        let res = res.unwrap();
        assert!(res.is_none(), "unexpected Some");
    }

    #[test]
    fn test_close() {
        let file_enumerator = glib::Object::new::<MyFileEnumerator>();
        for i in 0..10 {
            if i == 5 {
                let res = file_enumerator.close(Cancellable::NONE);
                assert!(res.is_ok(), "{}", res.err().unwrap());
            }
            let res = file_enumerator.next_file(Cancellable::NONE);
            if i < 5 {
                assert!(res.is_ok(), "{}", res.err().unwrap());
                let res = res.unwrap();
                assert!(res.is_some(), "unexpected None");
                let fileinfo = res.unwrap();
                assert_eq!(fileinfo.name(), PathBuf::from(format!("file{}", i)));
            } else {
                assert!(
                    res.as_ref().is_err_and(|err| err
                        .kind::<IOErrorEnum>()
                        .map(|io_err| io_err == IOErrorEnum::Closed)
                        .unwrap_or_default()),
                    "next file should have failed with error Closed, but is {:?}",
                    res
                );
            }
        }
    }

    #[test]
    fn test_async_next_file() {
        let main_context = glib::MainContext::new();
        let file_enumerator = glib::Object::new::<MyFileEnumerator>();
        for i in 0..10 {
            let (send, recv) = oneshot::channel();
            main_context.block_on(async {
                file_enumerator.next_files_async(
                    1,
                    glib::Priority::DEFAULT,
                    Cancellable::NONE,
                    move |res| {
                        assert!(res.is_ok(), "{}", res.err().unwrap());
                        let res = res.unwrap();
                        assert_eq!(res.len(), 1, "unexpected res");
                        let fileinfo = res.first().unwrap();
                        assert_eq!(fileinfo.name(), PathBuf::from(format!("file{}", i)));
                        send.send(()).unwrap();
                    },
                );
                recv.await.unwrap();
            })
        }
        let (send, recv) = oneshot::channel();
        main_context.block_on(async {
            file_enumerator.next_files_async(
                1,
                glib::Priority::DEFAULT,
                Cancellable::NONE,
                move |res| {
                    assert!(res.is_ok(), "{}", res.err().unwrap());
                    let res = res.unwrap();
                    assert!(res.is_empty(), "unexpected res");
                    send.send(()).unwrap();
                },
            );
            recv.await.unwrap();
        })
    }

    #[test]
    fn test_async_close() {
        let main_context = glib::MainContext::new();
        let file_enumerator = glib::Object::new::<MyFileEnumerator>();
        for i in 0..10 {
            if i == 5 {
                let (send, recv) = oneshot::channel();
                main_context.block_on(async {
                    file_enumerator.close_async(
                        glib::Priority::DEFAULT,
                        Cancellable::NONE,
                        move |res| {
                            assert!(res.is_ok(), "{}", res.err().unwrap());
                            send.send(()).unwrap();
                        },
                    );
                    recv.await.unwrap();
                })
            }
            let res = file_enumerator.next_file(Cancellable::NONE);
            if i < 5 {
                assert!(res.is_ok(), "{}", res.err().unwrap());
                let res = res.unwrap();
                assert!(res.is_some(), "unexpected None");
                let fileinfo = res.unwrap();
                assert_eq!(fileinfo.name(), PathBuf::from(format!("file{}", i)));
            } else {
                assert!(
                    res.as_ref().is_err_and(|err| err
                        .kind::<IOErrorEnum>()
                        .map(|io_err| io_err == IOErrorEnum::Closed)
                        .unwrap_or_default()),
                    "next file should have failed with error Closed, but is {:?}",
                    res
                );
            }
        }
    }
}
