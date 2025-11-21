// Take a look at the license at the top of the repository in the LICENSE file.

use std::cell::RefCell;
use std::future::Future;
use std::mem::transmute;
use std::os::fd::{FromRawFd, RawFd};
use std::pin::Pin;

use glib::ffi::gpointer;
use glib::thread_guard::ThreadGuard;
use glib::translate::*;
use glib::{ControlFlow, IOCondition, Priority, Source, SourceFuture, SourceId, SourceStream};

use futures_core::stream::Stream;

fn into_raw<F: FnMut() -> ControlFlow + Send + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

fn into_raw_local<F: FnMut() -> ControlFlow + 'static>(func: F) -> gpointer {
    let func: Box<ThreadGuard<RefCell<F>>> = Box::new(ThreadGuard::new(RefCell::new(func)));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline<F: FnMut() -> ControlFlow + Send + 'static>(
    func: gpointer,
) -> glib::ffi::gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (*func.borrow_mut())().into_glib()
}

unsafe extern "C" fn trampoline_local<F: FnMut() -> ControlFlow + 'static>(
    func: gpointer,
) -> glib::ffi::gboolean {
    let func: &ThreadGuard<RefCell<F>> = &*(func as *const ThreadGuard<RefCell<F>>);
    (*func.get_ref().borrow_mut())().into_glib()
}

unsafe extern "C" fn destroy_closure<F: FnMut() -> ControlFlow + Send + 'static>(ptr: gpointer) {
    let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn destroy_closure_local<F: FnMut() -> ControlFlow + 'static>(ptr: gpointer) {
    let _ = Box::<ThreadGuard<RefCell<F>>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn trampoline_unix_fd<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
>(
    fd: i32,
    condition: glib::ffi::GIOCondition,
    func: gpointer,
) -> glib::ffi::gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (*func.borrow_mut())(fd, from_glib(condition)).into_glib()
}

unsafe extern "C" fn trampoline_unix_fd_local<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
>(
    fd: i32,
    condition: glib::ffi::GIOCondition,
    func: gpointer,
) -> glib::ffi::gboolean {
    let func: &ThreadGuard<RefCell<F>> = &*(func as *const ThreadGuard<RefCell<F>>);
    (*func.get_ref().borrow_mut())(fd, from_glib(condition)).into_glib()
}

unsafe extern "C" fn destroy_closure_unix_fd<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

unsafe extern "C" fn destroy_closure_unix_fd_local<
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
>(
    ptr: gpointer,
) {
    let _ = Box::<ThreadGuard<RefCell<F>>>::from_raw(ptr as *mut _);
}

fn into_raw_unix_fd<F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static>(
    func: F,
) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

fn into_raw_unix_fd_local<F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static>(
    func: F,
) -> gpointer {
    let func: Box<ThreadGuard<RefCell<F>>> = Box::new(ThreadGuard::new(RefCell::new(func)));
    Box::into_raw(func) as gpointer
}

#[inline(always)]
fn fnmut_callback_wrapper(
    func: impl FnOnce() + Send + 'static,
) -> impl FnMut() -> ControlFlow + Send + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning ControlFlow::Break");
        func();
        ControlFlow::Break
    }
}

#[inline(always)]
fn fnmut_callback_wrapper_local(
    func: impl FnOnce() + 'static,
) -> impl FnMut() -> ControlFlow + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning glib::ControlFlow::Break");
        func();
        ControlFlow::Break
    }
}

#[doc(alias = "g_unix_open_pipe")]
pub fn unix_open_pipe(flags: i32) -> Result<(RawFd, RawFd), glib::Error> {
    unsafe {
        let mut fds = [0, 2];
        let mut error = std::ptr::null_mut();
        let _ = ffi::g_unix_open_pipe(&mut fds, flags, &mut error);
        if error.is_null() {
            Ok((
                FromRawFd::from_raw_fd(fds[0]),
                FromRawFd::from_raw_fd(fds[1]),
            ))
        } else {
            Err(from_glib_full(error))
        }
    }
}

// rustdoc-stripper-ignore-next
/// Create a `Stream` that will provide a value whenever the given UNIX signal is raised
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn signal_stream(signum: i32) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    signal_stream_with_priority(Priority::default(), signum)
}

// rustdoc-stripper-ignore-next
/// Create a `Stream` that will provide a value whenever the given UNIX signal is raised
///
/// The `Stream` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn signal_stream_with_priority(
    priority: Priority,
    signum: i32,
) -> Pin<Box<dyn Stream<Item = ()> + Send + 'static>> {
    Box::pin(SourceStream::new(move |send| {
        signal_source_new(signum, None, priority, move || {
            if send.unbounded_send(()).is_err() {
                ControlFlow::Break
            } else {
                ControlFlow::Continue
            }
        })
    }))
}

// rustdoc-stripper-ignore-next
/// Create a `Future` that will resolve once the given UNIX signal is raised
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn signal_future_with_priority(
    priority: Priority,
    signum: i32,
) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    Box::pin(SourceFuture::new(move |send| {
        let mut send = Some(send);
        signal_source_new(signum, None, priority, move || {
            let _ = send.take().unwrap().send(());
            ControlFlow::Break
        })
    }))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_source_new")]
pub fn signal_source_new<F>(
    signum: i32,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> glib::Source
where
    F: FnMut() -> glib::ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_signal_source_new(signum);
        glib::ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        glib::ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            glib::ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
#[doc(alias = "g_unix_fd_source_new")]
pub fn fd_source_new<F>(
    fd: RawFd,
    condition: IOCondition,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_fd_source_new(fd, condition.into_glib());
        glib::ffi::g_source_set_callback(
            source,
            Some(transmute::<
                *const (),
                unsafe extern "C" fn(glib::ffi::gpointer) -> glib::ffi::gboolean,
            >(trampoline_unix_fd::<F> as *const ())),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        );
        glib::ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            glib::ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

// rustdoc-stripper-ignore-next
/// Create a `Future` that will resolve once the given UNIX signal is raised
///
/// The `Future` must be spawned on an `Executor` backed by a `glib::MainContext`.
pub fn signal_future(signum: i32) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>> {
    signal_future_with_priority(Priority::default(), signum)
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_signal_add_full(
            Priority::default().into_glib(),
            signum,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// In comparison to `unix_signal_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    unix_signal_add(signum, fnmut_callback_wrapper(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> ControlFlow + 'static,
{
    unsafe {
        let context = glib::MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_signal_add_full(
            Priority::default().into_glib(),
            signum,
            Some(trampoline_local::<F>),
            into_raw_local(func),
            Some(destroy_closure_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `unix_signal_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `ControlFlow::Break`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    unix_signal_add_local(signum, fnmut_callback_wrapper_local(func))
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_fd_add_full(
            Priority::default().into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly with `priority` while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_full<F>(
    fd: RawFd,
    priority: Priority,
    condition: IOCondition,
    func: F,
) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_fd_add_full(
            priority.into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_fd_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_local<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
{
    unsafe {
        let context = glib::MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_fd_add_full(
            Priority::default().into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd_local::<F>),
            into_raw_unix_fd_local(func),
            Some(destroy_closure_unix_fd_local::<F>),
        ))
    }
}

// rustdoc-stripper-ignore-next
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly with `priority` while the file descriptor matches the given IO condition
/// until it returns `ControlFlow::Break`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus, the closure is called on the main thread.
///
/// Different to `unix_fd_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_local_full<F>(
    fd: RawFd,
    priority: Priority,
    condition: IOCondition,
    func: F,
) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> ControlFlow + 'static,
{
    unsafe {
        let context = glib::MainContext::default();
        let _acquire = context
            .acquire()
            .expect("default main context already acquired by another thread");
        from_glib(ffi::g_unix_fd_add_full(
            priority.into_glib(),
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd_local::<F>),
            into_raw_unix_fd_local(func),
            Some(destroy_closure_unix_fd_local::<F>),
        ))
    }
}
