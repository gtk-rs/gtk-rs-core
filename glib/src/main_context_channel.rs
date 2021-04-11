// Take a look at the license at the top of the repository in the LICENSE file.

use crate::translate::{mut_override, FromGlibPtrFull, IntoGlib};
use crate::Continue;
use crate::MainContext;
use crate::Priority;
use crate::Source;
use crate::SourceId;
use crate::ThreadGuard;
use std::collections::VecDeque;
use std::fmt;
use std::mem;
use std::ptr;
use std::sync::mpsc;
use std::sync::{Arc, Condvar, Mutex};

enum ChannelSourceState {
    NotAttached,
    Attached(*mut ffi::GSource),
    Destroyed,
}

unsafe impl Send for ChannelSourceState {}
unsafe impl Sync for ChannelSourceState {}

struct ChannelInner<T> {
    queue: VecDeque<T>,
    source: ChannelSourceState,
    num_senders: usize,
}

impl<T> ChannelInner<T> {
    fn receiver_disconnected(&self) -> bool {
        match self.source {
            ChannelSourceState::Destroyed => true,
            // Receiver exists but is already destroyed
            ChannelSourceState::Attached(source)
                if unsafe { ffi::g_source_is_destroyed(source) } != ffi::GFALSE =>
            {
                true
            }
            // Not attached yet so the Receiver still exists
            ChannelSourceState::NotAttached => false,
            // Receiver still running
            ChannelSourceState::Attached(_) => false,
        }
    }

    #[doc(alias = "g_source_set_ready_time")]
    fn set_ready_time(&mut self, ready_time: i64) {
        if let ChannelSourceState::Attached(source) = self.source {
            unsafe {
                ffi::g_source_set_ready_time(source, ready_time);
            }
        }
    }
}

struct ChannelBound {
    bound: usize,
    cond: Condvar,
}

struct Channel<T>(Arc<(Mutex<ChannelInner<T>>, Option<ChannelBound>)>);

impl<T> Clone for Channel<T> {
    fn clone(&self) -> Channel<T> {
        Channel(self.0.clone())
    }
}

impl<T> Channel<T> {
    fn new(bound: Option<usize>) -> Channel<T> {
        Channel(Arc::new((
            Mutex::new(ChannelInner {
                queue: VecDeque::new(),
                source: ChannelSourceState::NotAttached,
                num_senders: 0,
            }),
            bound.map(|bound| ChannelBound {
                bound,
                cond: Condvar::new(),
            }),
        )))
    }

    fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        let mut inner = (self.0).0.lock().unwrap();

        // If we have a bounded channel then we need to wait here until enough free space is
        // available or the receiver disappears
        //
        // A special case here is a bound of 0: the queue must be empty for accepting
        // new data and then we will again wait later for the data to be actually taken
        // out
        if let Some(ChannelBound { bound, ref cond }) = (self.0).1 {
            while inner.queue.len() >= bound
                && !inner.queue.is_empty()
                && !inner.receiver_disconnected()
            {
                inner = cond.wait(inner).unwrap();
            }
        }

        // Error out directly if the receiver is disconnected
        if inner.receiver_disconnected() {
            return Err(mpsc::SendError(t));
        }

        // Store the item on our queue
        inner.queue.push_back(t);

        // and then wake up the GSource
        inner.set_ready_time(0);

        // If we have a bound of 0 we need to wait until the receiver actually
        // handled the data
        if let Some(ChannelBound { bound: 0, ref cond }) = (self.0).1 {
            while !inner.queue.is_empty() && !inner.receiver_disconnected() {
                inner = cond.wait(inner).unwrap();
            }

            // If the receiver was destroyed in the meantime take out the item and report an error
            if inner.receiver_disconnected() {
                // If the item is not in the queue anymore then the receiver just handled it before
                // getting disconnected and all is good
                if let Some(t) = inner.queue.pop_front() {
                    return Err(mpsc::SendError(t));
                }
            }
        }

        Ok(())
    }

    fn try_send(&self, t: T) -> Result<(), mpsc::TrySendError<T>> {
        let mut inner = (self.0).0.lock().unwrap();

        let ChannelBound { bound, ref cond } = (self.0)
            .1
            .as_ref()
            .expect("called try_send() on an unbounded channel");

        // Check if the queue is full and handle the special case of a 0 bound
        if inner.queue.len() >= *bound && !inner.queue.is_empty() {
            return Err(mpsc::TrySendError::Full(t));
        }

        // Error out directly if the receiver is disconnected
        if inner.receiver_disconnected() {
            return Err(mpsc::TrySendError::Disconnected(t));
        }

        // Store the item on our queue
        inner.queue.push_back(t);

        // and then wake up the GSource
        inner.set_ready_time(0);

        // If we have a bound of 0 we need to wait until the receiver actually
        // handled the data
        if *bound == 0 {
            while !inner.queue.is_empty() && !inner.receiver_disconnected() {
                inner = cond.wait(inner).unwrap();
            }

            // If the receiver was destroyed in the meantime take out the item and report an error
            if inner.receiver_disconnected() {
                // If the item is not in the queue anymore then the receiver just handled it before
                // getting disconnected and all is good
                if let Some(t) = inner.queue.pop_front() {
                    return Err(mpsc::TrySendError::Disconnected(t));
                }
            }
        }

        Ok(())
    }

    fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        let mut inner = (self.0).0.lock().unwrap();

        // Pop item if we have any
        if let Some(item) = inner.queue.pop_front() {
            // Wake up a sender that is currently waiting, if any
            if let Some(ChannelBound { ref cond, .. }) = (self.0).1 {
                cond.notify_one();
            }
            return Ok(item);
        }

        // If there are no senders left we are disconnected or otherwise empty. That's the case if
        // the only remaining strong reference is the one of the receiver
        if inner.num_senders == 0 {
            Err(mpsc::TryRecvError::Disconnected)
        } else {
            Err(mpsc::TryRecvError::Empty)
        }
    }
}

#[repr(C)]
struct ChannelSource<T, F: FnMut(T) -> Continue + 'static> {
    source: ffi::GSource,
    source_funcs: Option<Box<ffi::GSourceFuncs>>,
    channel: Option<Channel<T>>,
    callback: Option<ThreadGuard<F>>,
}

unsafe extern "C" fn dispatch<T, F: FnMut(T) -> Continue + 'static>(
    source: *mut ffi::GSource,
    callback: ffi::GSourceFunc,
    _user_data: ffi::gpointer,
) -> ffi::gboolean {
    let source = &mut *(source as *mut ChannelSource<T, F>);
    assert!(callback.is_none());

    // Set ready-time to -1 so that we won't get called again before a new item is added
    // to the channel queue.
    ffi::g_source_set_ready_time(&mut source.source, -1);

    // Get a reference to the callback. This will panic if we're called from a different
    // thread than where the source was attached to the main context.
    let callback = source
        .callback
        .as_mut()
        .expect("ChannelSource called before Receiver was attached")
        .get_mut();

    // Now iterate over all items that we currently have in the channel until it is
    // empty again. If all senders are disconnected at some point we remove the GSource
    // from the main context it was attached to as it will never ever be called again.
    let channel = source
        .channel
        .as_ref()
        .expect("ChannelSource without Channel");
    loop {
        match channel.try_recv() {
            Err(mpsc::TryRecvError::Empty) => break,
            Err(mpsc::TryRecvError::Disconnected) => return ffi::G_SOURCE_REMOVE,
            Ok(item) => {
                if callback(item) == Continue(false) {
                    return ffi::G_SOURCE_REMOVE;
                }
            }
        }
    }

    ffi::G_SOURCE_CONTINUE
}

unsafe extern "C" fn finalize<T, F: FnMut(T) -> Continue + 'static>(source: *mut ffi::GSource) {
    let source = &mut *(source as *mut ChannelSource<T, F>);

    // Drop all memory we own by taking it out of the Options
    let channel = source.channel.take().expect("Receiver without channel");

    {
        // Set the source inside the channel to None so that all senders know that there
        // is no receiver left and wake up the condition variable if any
        let mut inner = (channel.0).0.lock().unwrap();
        inner.source = ChannelSourceState::Destroyed;
        if let Some(ChannelBound { ref cond, .. }) = (channel.0).1 {
            cond.notify_all();
        }
    }

    let _ = source.source_funcs.take();

    // Take the callback out of the source. This will panic if the value is dropped
    // from a different thread than where the callback was created
    let _ = source.callback.take();
}

/// A `Sender` that can be used to send items to the corresponding main context receiver.
///
/// This `Sender` behaves the same as `std::sync::mpsc::Sender`.
///
/// See [`MainContext::channel()`] for how to create such a `Sender`.
///
/// [`MainContext::channel()`]: struct.MainContext.html#method.channel
pub struct Sender<T>(Channel<T>);

impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Sender").finish()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        Sender::new(&self.0)
    }
}

impl<T> Sender<T> {
    fn new(channel: &Channel<T>) -> Self {
        let mut inner = (channel.0).0.lock().unwrap();
        inner.num_senders += 1;
        Sender(channel.clone())
    }

    /// Sends a value to the channel.
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.0.send(t)
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // Decrease the number of senders and wake up the channel if this
        // was the last sender that was dropped.
        let mut inner = ((self.0).0).0.lock().unwrap();
        inner.num_senders -= 1;
        if inner.num_senders == 0 {
            inner.set_ready_time(0);
        }
    }
}

/// A `SyncSender` that can be used to send items to the corresponding main context receiver.
///
/// This `SyncSender` behaves the same as `std::sync::mpsc::SyncSender`.
///
/// See [`MainContext::sync_channel()`] for how to create such a `SyncSender`.
///
/// [`MainContext::sync_channel()`]: struct.MainContext.html#method.sync_channel
pub struct SyncSender<T>(Channel<T>);

impl<T> fmt::Debug for SyncSender<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SyncSender").finish()
    }
}

impl<T> Clone for SyncSender<T> {
    fn clone(&self) -> SyncSender<T> {
        SyncSender::new(&self.0)
    }
}

impl<T> SyncSender<T> {
    fn new(channel: &Channel<T>) -> Self {
        let mut inner = (channel.0).0.lock().unwrap();
        inner.num_senders += 1;
        SyncSender(channel.clone())
    }

    /// Sends a value to the channel and blocks if the channel is full.
    pub fn send(&self, t: T) -> Result<(), mpsc::SendError<T>> {
        self.0.send(t)
    }

    /// Sends a value to the channel.
    pub fn try_send(&self, t: T) -> Result<(), mpsc::TrySendError<T>> {
        self.0.try_send(t)
    }
}

impl<T> Drop for SyncSender<T> {
    fn drop(&mut self) {
        // Decrease the number of senders and wake up the channel if this
        // was the last sender that was dropped.
        let mut inner = ((self.0).0).0.lock().unwrap();
        inner.num_senders -= 1;
        if inner.num_senders == 0 {
            inner.set_ready_time(0);
        }
    }
}

/// A `Receiver` that can be attached to a main context to receive items from its corresponding
/// `Sender` or `SyncSender`.
///
/// See [`MainContext::channel()`] or [`MainContext::sync_channel()`] for how to create
/// such a `Receiver`.
///
/// [`MainContext::channel()`]: struct.MainContext.html#method.channel
/// [`MainContext::sync_channel()`]: struct.MainContext.html#method.sync_channel
pub struct Receiver<T>(Option<Channel<T>>, Priority);

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Receiver").finish()
    }
}

// It's safe to send the Receiver to other threads for attaching it as
// long as the items to be sent can also be sent between threads.
unsafe impl<T: Send> Send for Receiver<T> {}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // If the receiver was never attached to a main context we need to let all the senders know
        if let Some(channel) = self.0.take() {
            let mut inner = (channel.0).0.lock().unwrap();
            inner.source = ChannelSourceState::Destroyed;
            if let Some(ChannelBound { ref cond, .. }) = (channel.0).1 {
                cond.notify_all();
            }
        }
    }
}

impl<T> Receiver<T> {
    /// Attaches the receiver to the given `context` and calls `func` whenever an item is
    /// available on the channel.
    ///
    /// Passing `None` for the context will attach it to the thread default main context.
    ///
    /// # Panics
    ///
    /// This function panics if called from a thread that is not the owner of the provided
    /// `context`, or, if `None` is provided, of the thread default main context.
    pub fn attach<F: FnMut(T) -> Continue + 'static>(
        mut self,
        context: Option<&MainContext>,
        func: F,
    ) -> SourceId {
        unsafe {
            let channel = self.0.take().expect("Receiver without channel");

            let source_funcs = Box::new(ffi::GSourceFuncs {
                check: None,
                prepare: None,
                dispatch: Some(dispatch::<T, F>),
                finalize: Some(finalize::<T, F>),
                closure_callback: None,
                closure_marshal: None,
            });

            let source = ffi::g_source_new(
                mut_override(&*source_funcs),
                mem::size_of::<ChannelSource<T, F>>() as u32,
            ) as *mut ChannelSource<T, F>;
            assert!(!source.is_null());

            // Set up the GSource
            {
                let source = &mut *source;
                let mut inner = (channel.0).0.lock().unwrap();

                ffi::g_source_set_priority(mut_override(&source.source), self.1.into_glib());

                // We're immediately ready if the queue is not empty or if no sender is left at this point
                ffi::g_source_set_ready_time(
                    mut_override(&source.source),
                    if !inner.queue.is_empty() || inner.num_senders == 0 {
                        0
                    } else {
                        -1
                    },
                );
                inner.source = ChannelSourceState::Attached(&mut source.source);
            }

            // Store all our data inside our part of the GSource
            {
                let source = &mut *source;
                ptr::write(&mut source.channel, Some(channel));
                ptr::write(&mut source.callback, Some(ThreadGuard::new(func)));
                ptr::write(&mut source.source_funcs, Some(source_funcs));
            }

            let source = Source::from_glib_full(mut_override(&(*source).source));
            if let Some(context) = context {
                assert!(context.is_owner());
                source.attach(Some(context))
            } else {
                let context = MainContext::ref_thread_default();
                assert!(context.is_owner());
                source.attach(Some(&context))
            }
        }
    }
}

impl MainContext {
    /// Creates a channel for a main context.
    ///
    /// The `Receiver` has to be attached to a main context at a later time, together with a
    /// closure that will be called for every item sent to a `Sender`.
    ///
    /// The `Sender` can be cloned and both the `Sender` and `Receiver` can be sent to different
    /// threads as long as the item type implements the `Send` trait.
    ///
    /// When the last `Sender` is dropped the channel is removed from the main context. If the
    /// `Receiver` is dropped and not attached to a main context all sending to the `Sender`
    /// will fail.
    ///
    /// The returned `Sender` behaves the same as `std::sync::mpsc::Sender`.
    pub fn channel<T>(priority: Priority) -> (Sender<T>, Receiver<T>) {
        let channel = Channel::new(None);
        let receiver = Receiver(Some(channel.clone()), priority);
        let sender = Sender::new(&channel);

        (sender, receiver)
    }

    /// Creates a synchronous channel for a main context with a given bound on the capacity of the
    /// channel.
    ///
    /// The `Receiver` has to be attached to a main context at a later time, together with a
    /// closure that will be called for every item sent to a `SyncSender`.
    ///
    /// The `SyncSender` can be cloned and both the `SyncSender` and `Receiver` can be sent to different
    /// threads as long as the item type implements the `Send` trait.
    ///
    /// When the last `SyncSender` is dropped the channel is removed from the main context. If the
    /// `Receiver` is dropped and not attached to a main context all sending to the `SyncSender`
    /// will fail.
    ///
    /// The returned `SyncSender` behaves the same as `std::sync::mpsc::SyncSender`.
    pub fn sync_channel<T>(priority: Priority, bound: usize) -> (SyncSender<T>, Receiver<T>) {
        let channel = Channel::new(Some(bound));
        let receiver = Receiver(Some(channel.clone()), priority);
        let sender = SyncSender::new(&channel);

        (sender, receiver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MainLoop;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::thread;
    use std::time;

    #[test]
    fn test_channel() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::channel(Priority::default());

        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        let l_clone = l.clone();
        receiver.attach(Some(&c), move |item| {
            *sum_clone.borrow_mut() += item;
            if *sum_clone.borrow() == 6 {
                l_clone.quit();
                Continue(false)
            } else {
                Continue(true)
            }
        });

        sender.send(1).unwrap();
        sender.send(2).unwrap();
        sender.send(3).unwrap();

        l.run();

        assert_eq!(*sum.borrow(), 6);
    }

    #[test]
    fn test_drop_sender() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        struct Helper(MainLoop);
        impl Drop for Helper {
            fn drop(&mut self) {
                self.0.quit();
            }
        }

        let helper = Helper(l.clone());
        receiver.attach(Some(&c), move |_| {
            let _ = helper;

            Continue(true)
        });

        drop(sender);

        l.run();
    }

    #[test]
    fn test_drop_receiver() {
        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        drop(receiver);
        assert_eq!(sender.send(1), Err(mpsc::SendError(1)));
    }

    #[test]
    fn test_remove_receiver() {
        let c = MainContext::new();

        c.acquire();

        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        let source_id = receiver.attach(Some(&c), move |_| Continue(true));

        let source = c.find_source_by_id(&source_id).unwrap();
        source.destroy();

        assert_eq!(sender.send(1), Err(mpsc::SendError(1)));
    }

    #[test]
    fn test_remove_receiver_and_drop_source() {
        let c = MainContext::new();

        c.acquire();

        let (sender, receiver) = MainContext::channel::<i32>(Priority::default());

        struct Helper(Arc<AtomicBool>);
        impl Drop for Helper {
            fn drop(&mut self) {
                self.0.store(true, Ordering::Relaxed);
            }
        }

        let dropped = Arc::new(AtomicBool::new(false));
        let helper = Helper(dropped.clone());
        let source_id = receiver.attach(Some(&c), move |_| {
            let _helper = &helper;
            Continue(true)
        });

        let source = c.find_source_by_id(&source_id).unwrap();
        source.destroy();

        // This should drop the closure
        drop(source);

        assert!(dropped.load(Ordering::Relaxed));
        assert_eq!(sender.send(1), Err(mpsc::SendError(1)));
    }

    #[test]
    fn test_sync_channel() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::sync_channel(Priority::default(), 2);

        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        let l_clone = l.clone();
        receiver.attach(Some(&c), move |item| {
            *sum_clone.borrow_mut() += item;
            if *sum_clone.borrow() == 6 {
                l_clone.quit();
                Continue(false)
            } else {
                Continue(true)
            }
        });

        let (wait_sender, wait_receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            // The first two must succeed
            sender.try_send(1).unwrap();
            sender.try_send(2).unwrap();

            // This fills up the channel
            assert!(sender.try_send(3).is_err());
            wait_sender.send(()).unwrap();

            // This will block
            sender.send(3).unwrap();
        });

        // Wait until the channel is full, and then another
        // 50ms to make sure the sender is blocked now and
        // can wake up properly once an item was consumed
        assert!(wait_receiver.recv().is_ok());
        thread::sleep(time::Duration::from_millis(50));
        l.run();

        thread.join().unwrap();

        assert_eq!(*sum.borrow(), 6);
    }

    #[test]
    fn test_sync_channel_drop_wakeup() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::sync_channel(Priority::default(), 3);

        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        let l_clone = l.clone();
        receiver.attach(Some(&c), move |item| {
            *sum_clone.borrow_mut() += item;
            if *sum_clone.borrow() == 6 {
                l_clone.quit();
                Continue(false)
            } else {
                Continue(true)
            }
        });

        let (wait_sender, wait_receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            // The first three must succeed
            sender.try_send(1).unwrap();
            sender.try_send(2).unwrap();
            sender.try_send(3).unwrap();

            wait_sender.send(()).unwrap();
            for i in 4.. {
                // This will block at some point until the
                // receiver is removed from the main context
                if sender.send(i).is_err() {
                    break;
                }
            }
        });

        // Wait until the channel is full, and then another
        // 50ms to make sure the sender is blocked now and
        // can wake up properly once an item was consumed
        assert!(wait_receiver.recv().is_ok());
        thread::sleep(time::Duration::from_millis(50));
        l.run();

        thread.join().unwrap();

        assert_eq!(*sum.borrow(), 6);
    }

    #[test]
    fn test_sync_channel_drop_receiver_wakeup() {
        let c = MainContext::new();

        c.acquire();

        let (sender, receiver) = MainContext::sync_channel(Priority::default(), 2);

        let (wait_sender, wait_receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            // The first two must succeed
            sender.try_send(1).unwrap();
            sender.try_send(2).unwrap();
            wait_sender.send(()).unwrap();

            // This will block and then error out because the receiver is destroyed
            assert!(sender.send(3).is_err());
        });

        // Wait until the channel is full, and then another
        // 50ms to make sure the sender is blocked now and
        // can wake up properly once an item was consumed
        assert!(wait_receiver.recv().is_ok());
        thread::sleep(time::Duration::from_millis(50));
        drop(receiver);
        thread.join().unwrap();
    }

    #[test]
    fn test_sync_channel_rendezvous() {
        let c = MainContext::new();
        let l = MainLoop::new(Some(&c), false);

        c.acquire();

        let (sender, receiver) = MainContext::sync_channel(Priority::default(), 0);

        let (wait_sender, wait_receiver) = mpsc::channel();

        let thread = thread::spawn(move || {
            wait_sender.send(()).unwrap();
            sender.send(1).unwrap();
            wait_sender.send(()).unwrap();
            sender.send(2).unwrap();
            wait_sender.send(()).unwrap();
            sender.send(3).unwrap();
            wait_sender.send(()).unwrap();
        });

        // Wait until the thread is started, then wait another 50ms and
        // during that time it must not have proceeded yet to send the
        // second item because we did not yet receive the first item.
        assert!(wait_receiver.recv().is_ok());
        assert_eq!(
            wait_receiver.recv_timeout(time::Duration::from_millis(50)),
            Err(mpsc::RecvTimeoutError::Timeout)
        );

        let sum = Rc::new(RefCell::new(0));
        let sum_clone = sum.clone();
        let l_clone = l.clone();
        receiver.attach(Some(&c), move |item| {
            // We consumed one item so there should be one item on
            // the other receiver now.
            assert!(wait_receiver.recv().is_ok());
            *sum_clone.borrow_mut() += item;
            if *sum_clone.borrow() == 6 {
                // But as we didn't consume the next one yet, there must be no
                // other item available yet
                assert_eq!(
                    wait_receiver.recv_timeout(time::Duration::from_millis(50)),
                    Err(mpsc::RecvTimeoutError::Disconnected)
                );
                l_clone.quit();
                Continue(false)
            } else {
                // But as we didn't consume the next one yet, there must be no
                // other item available yet
                assert_eq!(
                    wait_receiver.recv_timeout(time::Duration::from_millis(50)),
                    Err(mpsc::RecvTimeoutError::Timeout)
                );
                Continue(true)
            }
        });
        l.run();

        thread.join().unwrap();

        assert_eq!(*sum.borrow(), 6);
    }
}
