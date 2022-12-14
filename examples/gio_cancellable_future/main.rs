use std::{future::pending, thread, time::Duration};

use futures::prelude::*;
use gio::prelude::*;

/// A very long task. This task actually never ends.
async fn a_very_long_task() {
    println!("Very long task started");
    pending().await
}

fn main() {
    const TIMEOUT: Duration = Duration::from_secs(3);

    let main_ctx = glib::MainContext::default();
    let main_loop = glib::MainLoop::new(Some(&main_ctx), false);
    let cancellable = gio::Cancellable::new();

    {
        let main_loop = main_loop.clone();

        // We wrap `a_very_long_task` inside a `CancellableFuture` controlled by `cancellable`.
        // The task is cancelled when `.cancel()` is invoked.
        let cancellable_task = gio::CancellableFuture::new(a_very_long_task(), cancellable.clone())
            .map(move |res| {
                if let Err(error) = res {
                    println!("{error:?}");
                }

                main_loop.quit();
            });

        main_ctx.spawn_local(cancellable_task);
    }

    // We simulate a timeout here.
    // After `TIMEOUT` we cancel the pending task.
    thread::spawn(move || {
        thread::sleep(TIMEOUT);

        println!("Timeout ({TIMEOUT:?}) elapsed! Cancelling pending task...",);

        cancellable.cancel();
    });

    main_loop.run();
}
