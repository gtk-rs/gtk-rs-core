// Take a look at the license at the top of the repository in the LICENSE file.

use glib::{prelude::*, subclass::prelude::*, translate::*, Error};

use crate::{ffi, traits::FileEnumeratorExt, Cancellable, FileEnumerator, FileInfo, IOErrorEnum};

// Support custom implementation of virtual functions defined in `gio::ffi::GFileEnumeratorClass` except pairs `xxx_async/xxx_finish` for which GIO provides a default implementation.
pub trait FileEnumeratorImpl: ObjectImpl + ObjectSubclass<Type: IsA<FileEnumerator>> {
    fn next_file(&self, cancellable: Option<&Cancellable>) -> Result<Option<FileInfo>, Error> {
        self.parent_next_file(cancellable)
    }

    fn close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        self.parent_close(cancellable)
    }
}

// Support parent implementation of virtual functions defined in `gio::ffi::GFileEnumeratorClass` except pairs `xxx_async/xxx_finish` for which GIO provides a default implementation.
pub trait FileEnumeratorImplExt: FileEnumeratorImpl {
    fn parent_next_file(
        &self,
        cancellable: Option<&Cancellable>,
    ) -> Result<Option<FileInfo>, Error> {
        if self.obj().is_closed() {
            Err(Error::new::<IOErrorEnum>(
                IOErrorEnum::Closed,
                "Enumerator is closed",
            ))
        } else {
            unsafe {
                let data = Self::type_data();
                let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

                let f = (*parent_class)
                    .next_file
                    .expect("No parent class implementation for \"next_file\"");

                let mut error = std::ptr::null_mut();
                let res = f(
                    self.obj()
                        .unsafe_cast_ref::<FileEnumerator>()
                        .to_glib_none()
                        .0,
                    cancellable.as_ref().to_glib_none().0,
                    &mut error,
                );
                if error.is_null() {
                    Ok(from_glib_full(res))
                } else {
                    Err(from_glib_full(error))
                }
            }
        }
    }

    fn parent_close(&self, cancellable: Option<&Cancellable>) -> Result<(), Error> {
        unsafe {
            let data = Self::type_data();
            let parent_class = data.as_ref().parent_class() as *const ffi::GFileEnumeratorClass;

            let f = (*parent_class)
                .close_fn
                .expect("No parent class implementation for \"close_fn\"");

            // get the corresponding object instance without checking the reference count because the object might just be finalized.
            let obj = {
                let offset = -data.as_ref().impl_offset();
                let ptr = self as *const Self as usize;
                let ptr = if offset < 0 {
                    ptr - (-offset) as usize
                } else {
                    ptr + offset as usize
                } as *const <Self::Type as ObjectType>::GlibType;
                glib::BorrowedObject::<Self::Type>::new(mut_override(ptr))
            };

            let mut error = std::ptr::null_mut();
            let is_ok = f(
                obj.unsafe_cast_ref::<FileEnumerator>().to_glib_none().0,
                cancellable.as_ref().to_glib_none().0,
                &mut error,
            );
            debug_assert_eq!(is_ok == glib::ffi::GFALSE, !error.is_null());
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

impl<T: FileEnumeratorImpl> FileEnumeratorImplExt for T {}

// Implement virtual functions defined in `gio::ffi::GFileEnumeratorClass` except pairs `xxx_async/xxx_finish` for which GIO provides a default implementation.
unsafe impl<T: FileEnumeratorImpl> IsSubclassable<T> for FileEnumerator {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.next_file = Some(next_file::<T>);
        klass.close_fn = Some(close_fn::<T>);
        // `GFileEnumerator` already implements `xxx_async/xxx_finish` vfuncs and this should be ok.
        // TODO: if necessary override the `GFileEnumerator` implementation of the following vfuncs:
        // klass.next_files_async = Some(next_files_async::<T>);
        // klass.next_files_finish = Some(next_files_finish::<T>);
        // klass.close_async = Some(close_async::<T>);
        // klass.close_finish = Some(close_finish::<T>);
    }
}

unsafe extern "C" fn next_file<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> *mut ffi::GFileInfo {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.next_file(cancellable.as_ref());

    match res {
        Ok(fileinfo) => fileinfo.to_glib_full(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            std::ptr::null_mut()
        }
    }
}

unsafe extern "C" fn close_fn<T: FileEnumeratorImpl>(
    enumerator: *mut ffi::GFileEnumerator,
    cancellable: *mut ffi::GCancellable,
    error: *mut *mut glib::ffi::GError,
) -> glib::ffi::gboolean {
    let instance = &*(enumerator as *mut T::Instance);
    let imp = instance.imp();
    let cancellable = Option::<Cancellable>::from_glib_none(cancellable);

    let res = imp.close(cancellable.as_ref());

    match res {
        Ok(_) => true.into_glib(),
        Err(err) => {
            if !error.is_null() {
                *error = err.to_glib_full()
            }
            false.into_glib()
        }
    }
}
