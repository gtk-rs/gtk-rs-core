pub mod ffi;
mod imp;

use gio::prelude::*;
use glib::subclass::prelude::*;

glib::wrapper! {
    pub struct FileSize(ObjectSubclass<imp::FileSize>);
}

impl FileSize {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create FileSize")
    }

    pub fn retrieved_size(&self) -> Option<i64> {
        *self.imp().size.borrow()
    }

    pub fn file_size_async<Q: FnOnce(i64, &FileSize) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
            let value = result
                .downcast_ref::<gio::Task>()
                .unwrap()
                .propagate_value()
                .unwrap();
            let source_object = source_object.unwrap().downcast_ref::<FileSize>().unwrap();
            callback(value, source_object);
        };

        let task = gio::Task::new(
            Some(self.upcast_ref::<glib::Object>()),
            cancellable,
            closure,
        );

        glib::MainContext::default().spawn_local(async move {
            let size = gio::File::for_path("Cargo.toml")
                .query_info_future("*", gio::FileQueryInfoFlags::NONE, glib::PRIORITY_DEFAULT)
                .await
                .unwrap()
                .size();

            let source_object = task
                .upcast_ref::<gio::AsyncResult>()
                .source_object()
                .unwrap();

            let source_object = source_object.downcast_ref::<FileSize>().unwrap().imp();

            source_object.size.replace(Some(size));
            task.return_value(&size);
        });
    }

    pub fn file_size_in_thread_async<Q: FnOnce(i64, &FileSize) + 'static>(
        &self,
        cancellable: Option<&gio::Cancellable>,
        callback: Q,
    ) {
        let closure = move |result: &gio::AsyncResult, source_object: Option<&glib::Object>| {
            let value = result
                .downcast_ref::<gio::Task>()
                .unwrap()
                .propagate_value()
                .unwrap();
            let source_object = source_object.unwrap().downcast_ref::<FileSize>().unwrap();
            callback(value, source_object);
        };

        let task = gio::Task::new(
            Some(self.upcast_ref::<glib::Object>()),
            cancellable,
            closure,
        );

        let task_func = move |_task: &gio::Task,
                              source_object: Option<&glib::Object>,
                              cancellable: Option<&gio::Cancellable>| {
            let size = gio::File::for_path("Cargo.toml")
                .query_info("*", gio::FileQueryInfoFlags::NONE, cancellable)
                .unwrap()
                .size();

            let source_object = source_object
                .unwrap()
                .downcast_ref::<FileSize>()
                .unwrap()
                .imp();

            source_object.size.replace(Some(size));

            Ok(size)
        };

        task.run_in_thread(task_func);
    }
}

impl Default for FileSize {
    fn default() -> Self {
        Self::new()
    }
}
