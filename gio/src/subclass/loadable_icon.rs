// Take a look at the license at the top of the repository in the LICENSE file.

use std::{future::Future, pin::Pin};

use crate::{
    subclass::prelude::*, AsyncResult, Cancellable, GioFuture, InputStream, LoadableIcon, LocalTask,
};
use glib::{prelude::*, translate::*, GString};

pub trait LoadableIconImpl: IconImpl {
    fn load(
        &self,
        size: i32,
        cancellable: Option<&Cancellable>,
    ) -> Result<(InputStream, Option<GString>), glib::Error> {
        self.parent_load(size, cancellable)
    }

    fn load_future(
        &self,
        size: i32,
    ) -> Pin<Box<dyn Future<Output = Result<(InputStream, Option<GString>), glib::Error>> + 'static>>
    {
        self.parent_load_future(size)
    }
}

pub trait LoadableIconImplExt: ObjectSubclass {
    fn parent_load(
        &self,
        size: i32,
        cancellable: Option<&Cancellable>,
    ) -> Result<(InputStream, Option<GString>), glib::Error>;

    fn parent_load_async<C>(&self, size: i32, cancellable: Option<&Cancellable>, callback: C)
    where
        C: FnOnce(Result<(InputStream, Option<GString>), glib::Error>) + 'static;

    fn parent_load_future(
        &self,
        size: i32,
    ) -> Pin<Box<dyn Future<Output = Result<(InputStream, Option<GString>), glib::Error>> + 'static>>;
}

impl<T: LoadableIconImpl> LoadableIconImplExt for T {
    fn parent_load(
        &self,
        size: i32,
        cancellable: Option<&Cancellable>,
    ) -> Result<(InputStream, Option<GString>), glib::Error> {
        unsafe {
            let type_data = Self::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<LoadableIcon>()
                as *const ffi::GLoadableIconIface;

            let func = (*parent_iface)
                .load
                .expect("No parent iface implementation for \"load\"");
            let mut err = std::ptr::null_mut();
            let mut string = std::ptr::null_mut();
            let stream = func(
                self.obj()
                    .unsafe_cast_ref::<LoadableIcon>()
                    .to_glib_none()
                    .0,
                size,
                &mut string,
                cancellable.to_glib_none().0,
                &mut err,
            );
            if err.is_null() {
                Ok((from_glib_full(stream), from_glib_full(string)))
            } else {
                Err(from_glib_full(err))
            }
        }
    }

    fn parent_load_async<C>(&self, size: i32, cancellable: Option<&Cancellable>, callback: C)
    where
        C: FnOnce(Result<(InputStream, Option<GString>), glib::Error>) + 'static,
    {
        unsafe {
            let main_context = glib::MainContext::ref_thread_default();
            let is_main_context_owner = main_context.is_owner();
            let has_acquired_main_context = (!is_main_context_owner)
                .then(|| main_context.acquire().ok())
                .flatten();
            assert!(
                is_main_context_owner || has_acquired_main_context.is_some(),
                "Async operations only allowed if the thread is owning the MainContext"
            );

            let type_data = T::type_data();
            let parent_iface = type_data.as_ref().parent_interface::<LoadableIcon>()
                as *const ffi::GLoadableIconIface;
            let f = (*parent_iface)
                .load_async
                .expect("no parent \"load_async\" implementation");
            let finish = (*parent_iface)
                .load_finish
                .expect("no parent \"load_finish\" implementation");

            let user_data: Box<(glib::thread_guard::ThreadGuard<C>, _)> =
                Box::new((glib::thread_guard::ThreadGuard::new(callback), finish));

            unsafe extern "C" fn parent_load_async_trampoline<R>(
                source_object_ptr: *mut glib::gobject_ffi::GObject,
                res: *mut ffi::GAsyncResult,
                user_data: glib::ffi::gpointer,
            ) where
                R: FnOnce(Result<(InputStream, Option<GString>), glib::Error>) + 'static,
            {
                let mut error = std::ptr::null_mut();
                let cb: Box<(
                    glib::thread_guard::ThreadGuard<R>,
                    fn(
                        *mut ffi::GLoadableIcon,
                        *mut ffi::GAsyncResult,
                        *mut *mut libc::c_char,
                        *mut *mut glib::ffi::GError,
                    ) -> *mut ffi::GInputStream,
                )> = Box::from_raw(user_data as *mut _);
                let mut typeptr = std::ptr::null_mut();
                let stream = cb.1(source_object_ptr as _, res, &mut typeptr, &mut error);
                let result = if error.is_null() {
                    Ok((from_glib_full(stream), from_glib_full(typeptr)))
                } else {
                    Err(from_glib_full(error))
                };
                let cb = cb.0.into_inner();
                cb(result);
            }

            let callback = parent_load_async_trampoline::<C>;
            f(
                self.obj()
                    .unsafe_cast_ref::<LoadableIcon>()
                    .to_glib_none()
                    .0,
                size,
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    fn parent_load_future(
        &self,
        size: i32,
    ) -> Pin<Box<dyn Future<Output = Result<(InputStream, Option<GString>), glib::Error>> + 'static>>
    {
        Box::pin(GioFuture::new(
            &self.ref_counted(),
            move |imp, cancellable, send| {
                imp.parent_load_async(size, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }
}

unsafe impl<T: LoadableIconImpl> IsImplementable<T> for LoadableIcon {
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.load = Some(icon_load::<T>);
        iface.load_async = Some(icon_load_async::<T>);
        iface.load_finish = Some(icon_load_finish::<T>);
    }
}

unsafe extern "C" fn icon_load<T: LoadableIconImpl>(
    icon: *mut ffi::GLoadableIcon,
    size: i32,
    typeptr: *mut *mut libc::c_char,
    cancellableptr: *mut ffi::GCancellable,
    errorptr: *mut *mut glib::ffi::GError,
) -> *mut ffi::GInputStream {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.imp();

    let cancellable: Borrowed<Option<Cancellable>> = from_glib_borrow(cancellableptr);

    let ret = imp.load(size, cancellable.as_ref().as_ref());
    match ret {
        Ok((stream, icon_type)) => {
            if !typeptr.is_null() {
                *typeptr = icon_type.to_glib_none().0;
            }

            stream.to_glib_full()
        }
        Err(err) => {
            *errorptr = err.into_glib_ptr();
            *typeptr = std::ptr::null_mut();

            std::ptr::null_mut()
        }
    }
}

#[derive(Clone, glib::Boxed)]
#[boxed_type(name = "GLoadableIconReturnType")]
// Needed for having the required Value traits by GTask API
struct LoadableIconReturnType(InputStream, Option<GString>);

unsafe extern "C" fn icon_load_async<T: LoadableIconImpl>(
    icon: *mut ffi::GLoadableIcon,
    size: i32,
    cancellableptr: *mut ffi::GCancellable,
    callbackptr: ffi::GAsyncReadyCallback,
    dataptr: glib::ffi::gpointer,
) {
    let instance = &*(icon as *mut T::Instance);
    let imp = instance.imp();
    let wrap: LoadableIcon = from_glib_none(icon);
    let cancellable: Option<Cancellable> = from_glib_none(cancellableptr);

    let closure = move |task: LocalTask<LoadableIconReturnType>,
                        source_object: Option<&glib::Object>| {
        let result: *mut ffi::GAsyncResult = task.upcast_ref::<AsyncResult>().to_glib_none().0;
        let source_object: *mut glib::gobject_ffi::GObject = source_object.to_glib_none().0;
        callbackptr.unwrap()(source_object, result, dataptr)
    };

    let t = LocalTask::new(
        Some(wrap.upcast_ref::<glib::Object>()),
        cancellable.as_ref(),
        closure,
    );

    glib::MainContext::default().spawn_local(async move {
        let res = imp.load_future(size).await;
        t.return_result(res.map(|(s, t)| LoadableIconReturnType(s, t)));
    });
}

unsafe extern "C" fn icon_load_finish<T: LoadableIconImpl>(
    _icon: *mut ffi::GLoadableIcon,
    resultptr: *mut ffi::GAsyncResult,
    typeptr: *mut *mut libc::c_char,
    errorptr: *mut *mut glib::ffi::GError,
) -> *mut ffi::GInputStream {
    let res: AsyncResult = from_glib_none(resultptr);
    let t = res.downcast::<LocalTask<LoadableIconReturnType>>().unwrap();
    let ret = t.propagate();
    match ret {
        Ok(rt) => {
            let (stream, icon_type) = (rt.0, rt.1);
            if !typeptr.is_null() {
                *typeptr = icon_type.to_glib_full();
            }
            stream.to_glib_full()
        }
        Err(err) => {
            *errorptr = err.into_glib_ptr();
            std::ptr::null_mut()
        }
    }
}
