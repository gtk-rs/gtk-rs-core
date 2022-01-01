use glib::subclass::prelude::*;

// FileSize is a simple object that will just contain the read file size.
// Initially the optional size field will be initialized to None.
// It uses a Mutex rather than a plain RefCell since we also want to
// use it for tasks that run in threads.
#[derive(Default)]
pub struct FileSize {
    pub size: std::sync::Mutex<Option<i64>>,
}

#[glib::object_subclass]
impl ObjectSubclass for FileSize {
    const NAME: &'static str = "FileSize";
    type Type = super::FileSize;
}

impl ObjectImpl for FileSize {}
