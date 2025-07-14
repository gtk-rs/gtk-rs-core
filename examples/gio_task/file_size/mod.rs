pub mod ffi;
mod imp;

use gio::prelude::*;
use glib::subclass::prelude::*;

glib::wrapper! {
    pub struct FileSize(ObjectSubclass<imp::FileSize>);
}

unsafe impl Send for FileSize {}
unsafe impl Sync for FileSize {}

impl FileSize {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn retrieved_size(&self) -> Option<i64> {
        *self.imp().size.lock().unwrap()
    }

    pub fn file_size_async<Q: FnOnce(i64, &FileSize) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        let closure = move |task: gio::LocalTask<i64>, source_object: Option<&glib::Object>| {
            // SAFETY: this is safe because we call propagate just once and the
            // task sets the result as a value
            let value = unsafe { task.propagate() }.unwrap();
            let source_object = source_object.unwrap().downcast_ref::<FileSize>().unwrap();
            callback(value, source_object);
        };

        let task = unsafe {
            gio::LocalTask::new(
                Some(self.upcast_ref::<glib::Object>()),
                cancellable,
                closure,
            )
        };

        glib::MainContext::ref_thread_default().spawn_local(async move {
            let size = gio::File::for_path("Cargo.toml")
                .query_info_future(
                    "*",
                    gio::FileQueryInfoFlags::NONE,
                    glib::Priority::default(),
                )
                .await
                .unwrap()
                .size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .source_object()
                .unwrap();

            let source_object = source_object.downcast_ref::<FileSize>().unwrap().imp();

            *source_object.size.lock().unwrap() = Some(size);
            task.return_result(Ok(size));
        });
    }

    pub fn file_size_in_thread_async<Q: FnOnce(i64, &FileSize) + Send + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        let closure = move |task: gio::Task<i64>, source_object: Option<&FileSize>| {
            // SAFETY: this is safe because we call propagate just once and the
            // task sets the result as a value
            let value = unsafe { task.propagate().unwrap() };
            let source_object = source_object.unwrap().downcast_ref::<FileSize>().unwrap();
            callback(value, source_object);
        };

        let task = unsafe { gio::Task::new(Some(self), cancellable, closure) };

        let task_func = move |task: gio::Task<i64>,
                              source_object: Option<&FileSize>,
                              cancellable: Option<&gio::Cancellable>| {
            let size = gio::File::for_path("Cargo.toml")
                .query_info("*", gio::FileQueryInfoFlags::NONE, cancellable)
                .unwrap()
                .size();

            *source_object.unwrap().imp().size.lock().unwrap() = Some(size);

            // SAFETY: this is safe because we call return_result just once
            unsafe {
                task.return_result(Ok(size));
            }
        };

        task.run_in_thread(task_func);
    }
}

impl Default for FileSize {
    fn default() -> Self {
        Self::new()
    }
}
