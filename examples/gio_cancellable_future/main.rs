use futures_channel::oneshot;
use std::future::pending;
use std::time::Duration;

use gio::prelude::*;

use futures::prelude::*;

const TIMEOUT: Duration = Duration::from_secs(3);

/// A very long task. This task actually never ends.
async fn a_very_long_task() {
    println!("Very long task started");
    pending().await
}

#[glib::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    let cancellable = gio::Cancellable::new();

    // We wrap `a_very_long_task` inside a `CancellableFuture` controlled by `cancellable`.
    // The task is cancelled when `.cancel()` is invoked.
    let cancellable_task = gio::CancellableFuture::new(a_very_long_task(), cancellable.clone())
        .map(move |res| {
            if let Err(cancelled) = res {
                println!("{:?}", cancelled);
            }

            tx.send(()).unwrap();
        });

    // Spawn the cancellable task.
    glib::MainContext::default().spawn(cancellable_task);

    // We simulate a timeout here.
    // After `TIMEOUT` we cancel the pending task.
    glib::MainContext::default().spawn(async move {
        glib::timeout_future(TIMEOUT).await;

        println!(
            "Timeout ({:?}) elapsed! Cancelling pending task...",
            TIMEOUT
        );

        cancellable.cancel();
    });

    rx.await.unwrap();
}
