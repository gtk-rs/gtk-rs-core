// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::{from_glib, from_glib_full, FromGlib, IntoGlib, ToGlibPtr};
#[cfg(any(unix, feature = "dox"))]
use crate::IOCondition;
use ffi::{self, gboolean, gpointer};
#[cfg(all(not(unix), feature = "dox"))]
use libc::c_int as RawFd;
use std::cell::RefCell;
use std::mem::transmute;
use std::num::NonZeroU32;
#[cfg(unix)]
use std::os::unix::io::RawFd;
use std::time::Duration;

use crate::MainContext;
use crate::Source;

/// The id of a source that is returned by `idle_add` and `timeout_add`.
#[derive(Debug, Eq, PartialEq)]
pub struct SourceId(NonZeroU32);

impl SourceId {
    /// Returns the internal source ID.
    pub unsafe fn as_raw(&self) -> u32 {
        self.0.get()
    }
}

#[doc(hidden)]
impl FromGlib<u32> for SourceId {
    #[inline]
    unsafe fn from_glib(val: u32) -> SourceId {
        assert_ne!(val, 0);
        SourceId(NonZeroU32::new_unchecked(val))
    }
}

/// Process identificator
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Pid(pub ffi::GPid);

unsafe impl Send for Pid {}
unsafe impl Sync for Pid {}

#[doc(hidden)]
impl IntoGlib for Pid {
    type GlibType = ffi::GPid;

    #[inline]
    fn into_glib(self) -> ffi::GPid {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GPid> for Pid {
    #[inline]
    unsafe fn from_glib(val: ffi::GPid) -> Pid {
        Pid(val)
    }
}

/// Continue calling the closure in the future iterations or drop it.
///
/// This is the return type of `idle_add` and `timeout_add` closures.
///
/// `Continue(true)` keeps the closure assigned, to be rerun when appropriate.
///
/// `Continue(false)` disconnects and drops it.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Continue(pub bool);

#[doc(hidden)]
impl IntoGlib for Continue {
    type GlibType = gboolean;

    #[inline]
    fn into_glib(self) -> gboolean {
        self.0.into_glib()
    }
}

unsafe extern "C" fn trampoline<F: FnMut() -> Continue + 'static>(func: gpointer) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())().into_glib()
}

unsafe extern "C" fn destroy_closure<F: FnMut() -> Continue + 'static>(ptr: gpointer) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

fn into_raw<F: FnMut() -> Continue + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

unsafe extern "C" fn trampoline_child_watch<F: FnMut(Pid, i32) + 'static>(
    pid: ffi::GPid,
    status: i32,
    func: gpointer,
) {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())(Pid(pid), status)
}

unsafe extern "C" fn destroy_closure_child_watch<F: FnMut(Pid, i32) + 'static>(ptr: gpointer) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

fn into_raw_child_watch<F: FnMut(Pid, i32) + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
unsafe extern "C" fn trampoline_unix_fd<F: FnMut(RawFd, IOCondition) -> Continue + 'static>(
    fd: i32,
    condition: ffi::GIOCondition,
    func: gpointer,
) -> gboolean {
    let func: &RefCell<F> = &*(func as *const RefCell<F>);
    (&mut *func.borrow_mut())(fd, from_glib(condition)).into_glib()
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
unsafe extern "C" fn destroy_closure_unix_fd<F: FnMut(RawFd, IOCondition) -> Continue + 'static>(
    ptr: gpointer,
) {
    Box::<RefCell<F>>::from_raw(ptr as *mut _);
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
fn into_raw_unix_fd<F: FnMut(RawFd, IOCondition) -> Continue + 'static>(func: F) -> gpointer {
    let func: Box<RefCell<F>> = Box::new(RefCell::new(func));
    Box::into_raw(func) as gpointer
}

/// Transform a generic FnOnce into a closure that can be used as callback in various glib methods
///
/// The resulting function can only be called once and will panic otherwise. It will return `Continue(false)`
/// in order to prevent being called twice.
#[inline(always)]
fn fnmut_callback_wrapper(
    func: impl FnOnce() + Send + 'static,
) -> impl FnMut() -> Continue + Send + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning glib::Continue(false)");
        func();
        Continue(false)
    }
}

/// Transform a generic FnOnce into a closure that can be used as callback in various glib methods
///
/// The resulting function can only be called once and will panic otherwise. It will return `Continue(false)`
/// in order to prevent being called twice.
///
/// Different to `fnmut_callback_wrapper()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
#[inline(always)]
fn fnmut_callback_wrapper_local(
    func: impl FnOnce() + 'static,
) -> impl FnMut() -> Continue + 'static {
    let mut func = Some(func);
    move || {
        let func = func
            .take()
            .expect("GSource closure called after returning glib::Continue(false)");
        func();
        Continue(false)
    }
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add<F>(func: F) -> SourceId
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_idle_add_full(
            ffi::G_PRIORITY_DEFAULT_IDLE,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// In comparison to `idle_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_once<F>(func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    idle_add(fnmut_callback_wrapper(func))
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `idle_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_local<F>(func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_idle_add_full(
            ffi::G_PRIORITY_DEFAULT_IDLE,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `idle_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `idle_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_idle_add_full")]
pub fn idle_add_local_once<F>(func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    idle_add_local(fnmut_callback_wrapper_local(func))
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add<F>(interval: Duration, func: F) -> SourceId
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_timeout_add_full(
            ffi::G_PRIORITY_DEFAULT,
            interval.as_millis() as _,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// In comparison to `timeout_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_once<F>(interval: Duration, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    timeout_add(interval, fnmut_callback_wrapper(func))
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `timeout_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_local<F>(interval: Duration, func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_timeout_add_full(
            ffi::G_PRIORITY_DEFAULT,
            interval.as_millis() as _,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `timeout_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `timeout_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_timeout_add_full")]
pub fn timeout_add_local_once<F>(interval: Duration, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    timeout_add_local(interval, fnmut_callback_wrapper_local(func))
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_timeout_add_seconds_full(
            ffi::G_PRIORITY_DEFAULT,
            interval,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// In comparison to `timeout_add_seconds()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_once<F>(interval: u32, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    timeout_add_seconds(interval, fnmut_callback_wrapper(func))
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `timeout_add_seconds()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_local<F>(interval: u32, func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_timeout_add_seconds_full(
            ffi::G_PRIORITY_DEFAULT,
            interval,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

/// Adds a closure to be called by the default main loop at regular intervals
/// with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `timeout_add_seconds()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `timeout_add_seconds_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_timeout_add_seconds_full")]
pub fn timeout_add_seconds_local_once<F>(interval: u32, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    timeout_add_seconds_local(interval, fnmut_callback_wrapper_local(func))
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
#[doc(alias = "g_child_watch_add_full")]
pub fn child_watch_add<F>(pid: Pid, func: F) -> SourceId
where
    F: FnMut(Pid, i32) + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_child_watch_add_full(
            ffi::G_PRIORITY_DEFAULT,
            pid.0,
            Some(trampoline_child_watch::<F>),
            into_raw_child_watch(func),
            Some(destroy_closure_child_watch::<F>),
        ))
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
///
/// Different to `child_watch_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_child_watch_add_full")]
pub fn child_watch_add_local<F>(pid: Pid, func: F) -> SourceId
where
    F: FnMut(Pid, i32) + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_child_watch_add_full(
            ffi::G_PRIORITY_DEFAULT,
            pid.0,
            Some(trampoline_child_watch::<F>),
            into_raw_child_watch(func),
            Some(destroy_closure_child_watch::<F>),
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_signal_add_full(
            ffi::G_PRIORITY_DEFAULT,
            signum,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// In comparison to `unix_signal_add()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + Send + 'static,
{
    unix_signal_add(signum, fnmut_callback_wrapper(func))
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local<F>(signum: i32, func: F) -> SourceId
where
    F: FnMut() -> Continue + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_unix_signal_add_full(
            ffi::G_PRIORITY_DEFAULT,
            signum,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the default main loop whenever a UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `unix_signal_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
///
/// In comparison to `unix_signal_add_local()`, this only requires `func` to be
/// `FnOnce`, and will automatically return `Continue(false)`.
#[doc(alias = "g_unix_signal_add_full")]
pub fn unix_signal_add_local_once<F>(signum: i32, func: F) -> SourceId
where
    F: FnOnce() + 'static,
{
    unix_signal_add_local(signum, fnmut_callback_wrapper_local(func))
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> Continue + Send + 'static,
{
    unsafe {
        from_glib(ffi::g_unix_fd_add_full(
            ffi::G_PRIORITY_DEFAULT,
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `Continue(false)`.
///
/// The default main loop almost always is the main loop of the main thread.
/// Thus the closure is called on the main thread.
///
/// Different to `unix_fd_add()`, this does not require `func` to be
/// `Send` but can only be called from the thread that owns the main context.
///
/// This function panics if called from a different thread than the one that
/// owns the main context.
#[doc(alias = "g_unix_fd_add_full")]
pub fn unix_fd_add_local<F>(fd: RawFd, condition: IOCondition, func: F) -> SourceId
where
    F: FnMut(RawFd, IOCondition) -> Continue + 'static,
{
    unsafe {
        assert!(MainContext::default().is_owner());
        from_glib(ffi::g_unix_fd_add_full(
            ffi::G_PRIORITY_DEFAULT,
            fd,
            condition.into_glib(),
            Some(trampoline_unix_fd::<F>),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        ))
    }
}

/// Removes the source with the given id `source_id` from the default main context.
///
/// It is a programmer error to attempt to remove a non-existent source.
/// Note: source id are reused.
///
/// For historical reasons, the native function always returns true, so we
/// ignore it here.
#[allow(clippy::needless_pass_by_value)]
pub fn source_remove(source_id: SourceId) {
    unsafe {
        ffi::g_source_remove(source_id.as_raw());
    }
}

/// The priority of sources
///
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Priority(i32);

#[doc(hidden)]
impl IntoGlib for Priority {
    type GlibType = i32;

    #[inline]
    fn into_glib(self) -> i32 {
        self.0
    }
}

#[doc(hidden)]
impl FromGlib<i32> for Priority {
    #[inline]
    unsafe fn from_glib(val: i32) -> Priority {
        Priority(val)
    }
}

impl Default for Priority {
    fn default() -> Priority {
        PRIORITY_DEFAULT
    }
}

pub const PRIORITY_HIGH: Priority = Priority(ffi::G_PRIORITY_HIGH);
pub const PRIORITY_DEFAULT: Priority = Priority(ffi::G_PRIORITY_DEFAULT);
pub const PRIORITY_HIGH_IDLE: Priority = Priority(ffi::G_PRIORITY_HIGH_IDLE);
pub const PRIORITY_DEFAULT_IDLE: Priority = Priority(ffi::G_PRIORITY_DEFAULT_IDLE);
pub const PRIORITY_LOW: Priority = Priority(ffi::G_PRIORITY_LOW);

/// Adds a closure to be called by the main loop the return `Source` is attached to when it's idle.
///
/// `func` will be called repeatedly until it returns `Continue(false)`.
#[doc(alias = "g_idle_source_new")]
pub fn idle_source_new<F>(name: Option<&str>, priority: Priority, func: F) -> Source
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        let source = ffi::g_idle_source_new();
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with millisecond granularity.
///
/// `func` will be called repeatedly every `interval` milliseconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events. Prefer `timeout_add_seconds` when millisecond
/// precision is not necessary.
#[doc(alias = "g_timeout_source_new")]
pub fn timeout_source_new<F>(
    interval: Duration,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        let source = ffi::g_timeout_source_new(interval.as_millis() as _);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to at regular
/// intervals with second granularity.
///
/// `func` will be called repeatedly every `interval` seconds until it
/// returns `Continue(false)`. Precise timing is not guaranteed, the timeout may
/// be delayed by other events.
#[doc(alias = "g_timeout_source_new_seconds")]
pub fn timeout_source_new_seconds<F>(
    interval: u32,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        let source = ffi::g_timeout_source_new_seconds(interval);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

/// Adds a closure to be called by the main loop the returned `Source` is attached to when a child
/// process exits.
///
/// `func` will be called when `pid` exits
#[doc(alias = "g_child_watch_source_new")]
pub fn child_watch_source_new<F>(
    pid: Pid,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut(Pid, i32) + Send + 'static,
{
    unsafe {
        let source = ffi::g_child_watch_source_new(pid.0);
        ffi::g_source_set_callback(
            source,
            Some(transmute::<
                _,
                unsafe extern "C" fn(ffi::gpointer) -> ffi::gboolean,
            >(trampoline_child_watch::<F> as *const ())),
            into_raw_child_watch(func),
            Some(destroy_closure_child_watch::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX signal is raised.
///
/// `func` will be called repeatedly every time `signum` is raised until it
/// returns `Continue(false)`.
#[doc(alias = "g_unix_signal_source_new")]
pub fn unix_signal_source_new<F>(
    signum: i32,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut() -> Continue + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_signal_source_new(signum);
        ffi::g_source_set_callback(
            source,
            Some(trampoline::<F>),
            into_raw(func),
            Some(destroy_closure::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

#[cfg(any(unix, feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(unix)))]
/// Adds a closure to be called by the main loop the returned `Source` is attached to whenever a
/// UNIX file descriptor reaches the given IO condition.
///
/// `func` will be called repeatedly while the file descriptor matches the given IO condition
/// until it returns `Continue(false)`.
#[doc(alias = "g_unix_fd_source_new")]
pub fn unix_fd_source_new<F>(
    fd: RawFd,
    condition: IOCondition,
    name: Option<&str>,
    priority: Priority,
    func: F,
) -> Source
where
    F: FnMut(RawFd, IOCondition) -> Continue + Send + 'static,
{
    unsafe {
        let source = ffi::g_unix_fd_source_new(fd, condition.into_glib());
        ffi::g_source_set_callback(
            source,
            Some(transmute::<
                _,
                unsafe extern "C" fn(ffi::gpointer) -> ffi::gboolean,
            >(trampoline_unix_fd::<F> as *const ())),
            into_raw_unix_fd(func),
            Some(destroy_closure_unix_fd::<F>),
        );
        ffi::g_source_set_priority(source, priority.into_glib());

        if let Some(name) = name {
            ffi::g_source_set_name(source, name.to_glib_none().0);
        }

        from_glib_full(source)
    }
}

impl Source {
    #[doc(alias = "g_source_attach")]
    pub fn attach(&self, context: Option<&MainContext>) -> SourceId {
        unsafe {
            from_glib(ffi::g_source_attach(
                self.to_glib_none().0,
                context.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_source_remove")]
    pub fn remove(tag: SourceId) -> Result<(), crate::BoolError> {
        unsafe {
            result_from_gboolean!(
                ffi::g_source_remove(tag.as_raw()),
                "Failed to remove source"
            )
        }
    }
}
