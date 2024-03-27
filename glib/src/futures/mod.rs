use futures_channel::mpsc;
use futures_core::Stream;
use futures_task::FutureObj;
use futures_task::LocalFutureObj;
use futures_task::LocalSpawn;
use futures_task::Poll;
use futures_task::Spawn;
use futures_util::future::Either;
use futures_util::future::Select;
use futures_util::pin_mut;
use futures_util::StreamExt;

use crate::Continue;
use crate::FutureWithTimeoutError;
use crate::JoinHandle;
use crate::MainContext;
use crate::Priority;
use crate::Source;
use crate::SpawnWithinJoinHandle;
use std::future::{Future, IntoFuture};
use std::{pin::Pin, time::Duration};

#[derive(Default, Copy, Clone, Debug, Eq, PartialEq)]
pub enum SchedulingPrecision {
    #[default]
    Millisecond,
    Second,
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct Sleep {
    duration: Duration,
    priority: Priority,
    precision: SchedulingPrecision,
}

impl IntoFuture for Sleep {
    type Output = ();

    type IntoFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;

    fn into_future(self) -> Self::IntoFuture {
        use SchedulingPrecision::*;
        match self.precision {
            Millisecond => crate::timeout_future_with_priority(self.priority, self.duration),
            Second => crate::timeout_future_seconds_with_priority(
                self.priority,
                self.duration.as_secs() as u32,
            ),
        }
    }
}

impl Sleep {
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    pub fn precision(mut self, precision: SchedulingPrecision) -> Self {
        self.precision = precision;
        self
    }
}

pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        priority: crate::PRIORITY_DEFAULT,
        duration,
        precision: SchedulingPrecision::Millisecond,
    }
}

// rustdoc-stripper-ignore-next
/// Options to build a future that will run until the specified `duration` passes.
#[derive(Default, Debug, Eq, PartialEq)]
pub struct Timeout<F: Future> {
    duration: Duration,
    priority: Priority,
    precision: SchedulingPrecision,
    future: F,
}
pub struct TimeoutFuture<F> {
    select: Select<F, Pin<Box<dyn Future<Output = ()> + Send + 'static>>>,
}

impl<F: Future + Unpin> Future for TimeoutFuture<F> {
    type Output = Result<F::Output, FutureWithTimeoutError>;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut futures_task::Context<'_>,
    ) -> futures_task::Poll<Self::Output> {
        let select = &mut self.as_mut().select;
        pin_mut!(select);
        match select.poll(cx) {
            Poll::Ready(res) => match res {
                Either::Left(value) => Poll::Ready(Ok(value.0)),
                Either::Right(_timedout) => Poll::Ready(Err(FutureWithTimeoutError)),
            },
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<F: Future + std::marker::Unpin + 'static> IntoFuture for Timeout<F> {
    type Output = Result<F::Output, FutureWithTimeoutError>;

    type IntoFuture = TimeoutFuture<F>;

    fn into_future(self) -> Self::IntoFuture {
        let sleep = Sleep {
            duration: self.duration,
            precision: self.precision,
            priority: self.priority,
        };
        TimeoutFuture {
            select: futures_util::future::select(self.future, sleep.into_future()),
        }
    }
}

impl<F: Future> Timeout<F> {
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    pub fn precision(mut self, precision: SchedulingPrecision) -> Self {
        self.precision = precision;
        self
    }
}

pub fn timeout<F: Future>(
    duration: Duration,
    future: F,
) -> Timeout<impl Future<Output = F::Output>> {
    Timeout {
        duration,
        priority: crate::PRIORITY_DEFAULT,
        future: Box::pin(future),
        precision: SchedulingPrecision::Millisecond,
    }
}

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
pub enum MissedTickBehavior {
    #[default]
    Burst,
    Delay,
    Skip,
}

// Inspired from tokio's [interval](https://docs.rs/tokio/latest/tokio/time/struct.Interval.html)
pub struct Interval {
    builder: IntervalBuilder,
    source: Source,
    source_chan: mpsc::UnboundedReceiver<()>,
}

impl Interval {
    pub async fn tick(&mut self) {
        use MissedTickBehavior::*;
        match self.builder.missed_tick_behavior {
            Burst => {
                self.source_chan.next().await;
            }
            Delay => {
                self.reset();
                self.source_chan.next().await;
            }
            Skip => {
                while let Ok(Some(_)) = self.source_chan.try_next() {}
                self.source_chan.next().await;
            }
        }
    }
    pub fn reset(&mut self) {
        self.source.destroy();
        // Is there another way to reset the GSource other than recreating it?
        (self.source, self.source_chan) = self.builder.build_source();
    }
    pub fn missed_tick_behavior(&self) -> MissedTickBehavior {
        self.builder.missed_tick_behavior
    }
    pub fn set_missed_tick_behavior(&mut self, missed_tick_behavior: MissedTickBehavior) {
        self.builder.missed_tick_behavior = missed_tick_behavior;
    }
    pub fn into_stream(self) -> impl Stream<Item = ()> + Send + 'static {
        self.source_chan
    }
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct IntervalBuilder {
    priority: Priority,
    precision: SchedulingPrecision,
    period: Duration,
    missed_tick_behavior: MissedTickBehavior,
}

impl IntervalBuilder {
    pub fn new(period: Duration) -> Self {
        Self {
            priority: crate::PRIORITY_DEFAULT,
            precision: SchedulingPrecision::default(),
            period,
            missed_tick_behavior: MissedTickBehavior::default(),
        }
    }
    pub fn precision(mut self, precision: SchedulingPrecision) -> Self {
        self.precision = precision;
        self
    }
    pub fn priority(mut self, priority: Priority) -> Self {
        self.priority = priority;
        self
    }
    pub fn missed_tick_behavior(mut self, missed_tick_behavior: MissedTickBehavior) -> Self {
        self.missed_tick_behavior = missed_tick_behavior;
        self
    }
    fn build_source(&self) -> (Source, mpsc::UnboundedReceiver<()>) {
        let (send, recv) = mpsc::unbounded();
        let cb = move || {
            if send.unbounded_send(()).is_err() {
                Continue(false)
            } else {
                Continue(true)
            }
        };
        use SchedulingPrecision::*;
        let source = match self.precision {
            Millisecond => crate::timeout_source_new(self.period, None, self.priority, cb),
            Second => crate::timeout_source_new_seconds(
                self.period.as_secs() as u32,
                None,
                self.priority,
                cb,
            ),
        };
        (source, recv)
    }
    fn build(self) -> Interval {
        let (source, source_chan) = self.build_source();
        Interval {
            source,
            source_chan,
            builder: self,
        }
    }
}

pub fn interval(period: Duration) -> Interval {
    IntervalBuilder::new(period).build()
}

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct SpawnOptions {
    priority: Priority,
    context: Option<crate::MainContext>,
}

impl SpawnOptions {
    pub fn new() -> Self {
        Self {
            priority: crate::PRIORITY_DEFAULT,
            context: None,
        }
    }
    pub fn priority(&mut self, priority: Priority) -> &mut Self {
        self.priority = priority;
        self
    }
    pub fn context(&mut self, context: MainContext) -> &mut Self {
        self.context = Some(context);
        self
    }
    pub fn spawn_local<F: Future + 'static>(&self, future: F) -> JoinHandle<<F as Future>::Output> {
        self.context
            .as_ref()
            .unwrap_or(&MainContext::default())
            .spawn_local_with_priority(self.priority, future)
    }
    pub fn spawn<R: Send + 'static, F: Future<Output = R> + Send + 'static>(
        &self,
        future: F,
    ) -> JoinHandle<R> {
        self.context
            .as_ref()
            .unwrap_or(&MainContext::default())
            .spawn_with_priority(self.priority, future)
    }
    pub fn spawn_from_within<F: Future + 'static>(
        &self,
        func: impl FnOnce() -> F + Send + 'static,
    ) -> SpawnWithinJoinHandle<<F as Future>::Output> {
        self.context
            .as_ref()
            .unwrap_or(&MainContext::default())
            .spawn_from_within_with_priority(self.priority, func)
    }
}

impl From<MainContext> for SpawnOptions {
    fn from(value: MainContext) -> Self {
        let mut opts = SpawnOptions::new();
        opts.context(value);
        opts
    }
}

// The following trait implementations will reuse the methods from `SpawnOptions`, so the spawned
// futures will have the correct priority chosen by the user.
// This is an improvement compared to `MainContext::spawn_obj`, which doesn't let you specify the
// priority.
impl Spawn for SpawnOptions {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), futures_task::SpawnError> {
        self.spawn(future);
        Ok(())
    }
}
impl LocalSpawn for SpawnOptions {
    fn spawn_local_obj(
        &self,
        future: LocalFutureObj<'static, ()>,
    ) -> Result<(), futures_task::SpawnError> {
        self.spawn_local(future);
        Ok(())
    }
}

#[test]
fn test_sleep() {
    use crate::MainContext;

    let c = MainContext::new();

    c.block_on(async {
        sleep(Duration::from_millis(10)).await;
        sleep(Duration::from_secs(1))
            .priority(crate::PRIORITY_HIGH)
            .precision(SchedulingPrecision::Second)
            .await;
    });
}

#[test]
fn test_timeout() {
    use crate::{MainContext, MainLoop};
    use std::future::ready;

    let c = MainContext::new();
    let l = MainLoop::new(Some(&c), false);

    let tt = timeout(Duration::from_millis(10), ready(()));
    let l_clone = l.clone();
    c.spawn_local(async move {
        tt.await.unwrap();
        l_clone.quit();
    });
    l.run();

    let tt = timeout(Duration::from_millis(10), async move { 2 }).priority(crate::PRIORITY_HIGH);
    let l_clone = l.clone();
    c.spawn(async move {
        tt.await.unwrap();
        l_clone.quit();
    });
    l.run();
}

#[test]
fn test_interval() {
    interval(Duration::from_millis(1));
    IntervalBuilder::new(Duration::from_secs(1))
        .priority(crate::PRIORITY_HIGH)
        .precision(SchedulingPrecision::Second)
        .build();
}

#[test]
fn test_spawn() {
    use crate::{MainContext, MainLoop};

    let c = MainContext::new();
    let l = MainLoop::new(Some(&c), false);

    let l_clone = l.clone();
    SpawnOptions::new().spawn(async move {
        2;
        l_clone.quit();
    });
    l.run();

    let l_clone = l.clone();
    SpawnOptions::new()
        .context(c)
        .priority(crate::PRIORITY_HIGH)
        .spawn_local(async move {
            2;
            l_clone.quit();
        });
    l.run();
}
