// Take a look at the license at the top of the repository in the LICENSE file.

use crate::prelude::*;
use crate::subclass::prelude::*;
use crate::OutputStream;
use glib::subclass;

use std::any::Any;
use std::io::{Seek, Write};

use crate::read_input_stream::std_error_to_gio_error;

mod imp {
    use super::*;
    use std::cell::RefCell;

    pub(super) enum Writer {
        Write(AnyWriter),
        WriteSeek(AnyWriter),
    }

    pub struct WriteOutputStream {
        pub(super) write: RefCell<Option<Writer>>,
    }

    impl ObjectSubclass for WriteOutputStream {
        const NAME: &'static str = "WriteOutputStream";
        type Type = super::WriteOutputStream;
        type ParentType = OutputStream;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib::glib_object_subclass!();

        fn new() -> Self {
            Self {
                write: RefCell::new(None),
            }
        }

        fn type_init(type_: &mut subclass::InitializingType<Self>) {
            type_.add_interface::<crate::Seekable>();
        }
    }

    impl ObjectImpl for WriteOutputStream {}

    impl OutputStreamImpl for WriteOutputStream {
        fn write(
            &self,
            _stream: &Self::Type,
            buffer: &[u8],
            _cancellable: Option<&crate::Cancellable>,
        ) -> Result<usize, glib::Error> {
            let mut write = self.write.borrow_mut();
            let write = match *write {
                None => {
                    return Err(glib::Error::new(
                        crate::IOErrorEnum::Closed,
                        "Alwritey closed",
                    ));
                }
                Some(Writer::Write(ref mut write)) => write,
                Some(Writer::WriteSeek(ref mut write)) => write,
            };

            loop {
                match std_error_to_gio_error(write.write(buffer)) {
                    None => continue,
                    Some(res) => return res,
                }
            }
        }

        fn flush(
            &self,
            _stream: &Self::Type,
            _cancellable: Option<&crate::Cancellable>,
        ) -> Result<(), glib::Error> {
            let mut write = self.write.borrow_mut();
            let write = match *write {
                None => {
                    return Err(glib::Error::new(
                        crate::IOErrorEnum::Closed,
                        "Alwritey closed",
                    ));
                }
                Some(Writer::Write(ref mut write)) => write,
                Some(Writer::WriteSeek(ref mut write)) => write,
            };

            loop {
                match std_error_to_gio_error(write.flush()) {
                    None => continue,
                    Some(res) => return res,
                }
            }
        }

        fn close(
            &self,
            _stream: &Self::Type,
            _cancellable: Option<&crate::Cancellable>,
        ) -> Result<(), glib::Error> {
            let _ = self.write.borrow_mut().take();
            Ok(())
        }
    }

    impl SeekableImpl for WriteOutputStream {
        fn tell(&self, _seekable: &Self::Type) -> i64 {
            // XXX: stream_position is not stable yet
            // let mut write = self.write.borrow_mut();
            // match *write {
            //     Some(Writer::WriteSeek(ref mut write)) => {
            //         write.stream_position().map(|pos| pos as i64).unwrap_or(-1)
            //     },
            //     _ => -1,
            // };
            -1
        }

        fn can_seek(&self, _seekable: &Self::Type) -> bool {
            let write = self.write.borrow();
            matches!(*write, Some(Writer::WriteSeek(_)))
        }

        fn seek(
            &self,
            _seekable: &Self::Type,
            offset: i64,
            type_: glib::SeekType,
            _cancellable: Option<&crate::Cancellable>,
        ) -> Result<(), glib::Error> {
            use std::io::SeekFrom;

            let mut write = self.write.borrow_mut();
            match *write {
                Some(Writer::WriteSeek(ref mut write)) => {
                    let pos = match type_ {
                        glib::SeekType::Cur => SeekFrom::Current(offset),
                        glib::SeekType::Set => {
                            if offset < 0 {
                                return Err(glib::Error::new(
                                    crate::IOErrorEnum::InvalidArgument,
                                    "Invalid Argument",
                                ));
                            } else {
                                SeekFrom::Start(offset as u64)
                            }
                        }
                        glib::SeekType::End => SeekFrom::End(offset),
                        _ => unimplemented!(),
                    };

                    loop {
                        match std_error_to_gio_error(write.seek(pos)) {
                            None => continue,
                            Some(res) => return res.map(|_| ()),
                        }
                    }
                }
                _ => Err(glib::Error::new(
                    crate::IOErrorEnum::NotSupported,
                    "Truncating not supported",
                )),
            }
        }

        fn can_truncate(&self, _seekable: &Self::Type) -> bool {
            false
        }

        fn truncate(
            &self,
            _seekable: &Self::Type,
            _offset: i64,
            _cancellable: Option<&crate::Cancellable>,
        ) -> Result<(), glib::Error> {
            Err(glib::Error::new(
                crate::IOErrorEnum::NotSupported,
                "Truncating not supported",
            ))
        }
    }
}

glib::glib_wrapper! {
    pub struct WriteOutputStream(ObjectSubclass<imp::WriteOutputStream>) @extends crate::OutputStream, @implements crate::Seekable;
}

impl WriteOutputStream {
    pub fn new<W: Write + Send + Any + 'static>(write: W) -> WriteOutputStream {
        let obj = glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create write input stream")
            .downcast()
            .expect("Created write input stream is of wrong type");

        let imp = imp::WriteOutputStream::from_instance(&obj);
        *imp.write.borrow_mut() = Some(imp::Writer::Write(AnyWriter::new(write)));
        obj
    }

    pub fn new_seekable<W: Write + Seek + Send + Any + 'static>(write: W) -> WriteOutputStream {
        let obj = glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create write input stream")
            .downcast()
            .expect("Created write input stream is of wrong type");

        let imp = imp::WriteOutputStream::from_instance(&obj);
        *imp.write.borrow_mut() = Some(imp::Writer::WriteSeek(AnyWriter::new_seekable(write)));
        obj
    }

    pub fn close_and_take(&self) -> Box<dyn Any + Send + 'static> {
        let imp = imp::WriteOutputStream::from_instance(self);
        let inner = imp.write.borrow_mut().take();

        let ret = match inner {
            None => {
                panic!("Stream already closed or inner taken");
            }
            Some(imp::Writer::Write(write)) => write.writer,
            Some(imp::Writer::WriteSeek(write)) => write.writer,
        };

        let _ = self.close(crate::NONE_CANCELLABLE);

        match ret {
            AnyOrPanic::Any(w) => w,
            AnyOrPanic::Panic(p) => std::panic::resume_unwind(p),
        }
    }
}

enum AnyOrPanic {
    Any(Box<dyn Any + Send + 'static>),
    Panic(Box<dyn Any + Send + 'static>),
}

// Helper struct for dynamically dispatching to any kind of Writer and
// catching panics along the way
struct AnyWriter {
    writer: AnyOrPanic,
    write_fn: fn(s: &mut AnyWriter, buffer: &[u8]) -> std::io::Result<usize>,
    flush_fn: fn(s: &mut AnyWriter) -> std::io::Result<()>,
    seek_fn: Option<fn(s: &mut AnyWriter, pos: std::io::SeekFrom) -> std::io::Result<u64>>,
}

impl AnyWriter {
    fn new<W: Write + Any + Send + 'static>(w: W) -> Self {
        AnyWriter {
            writer: AnyOrPanic::Any(Box::new(w)),
            write_fn: Self::write_fn::<W>,
            flush_fn: Self::flush_fn::<W>,
            seek_fn: None,
        }
    }

    fn new_seekable<W: Write + Seek + Any + Send + 'static>(w: W) -> Self {
        AnyWriter {
            writer: AnyOrPanic::Any(Box::new(w)),
            write_fn: Self::write_fn::<W>,
            flush_fn: Self::flush_fn::<W>,
            seek_fn: Some(Self::seek_fn::<W>),
        }
    }

    fn write_fn<W: Write + 'static>(s: &mut AnyWriter, buffer: &[u8]) -> std::io::Result<usize> {
        s.with_inner(|w: &mut W| w.write(buffer))
    }

    fn flush_fn<W: Write + 'static>(s: &mut AnyWriter) -> std::io::Result<()> {
        s.with_inner(|w: &mut W| w.flush())
    }

    fn seek_fn<W: Seek + 'static>(
        s: &mut AnyWriter,
        pos: std::io::SeekFrom,
    ) -> std::io::Result<u64> {
        s.with_inner(|w: &mut W| w.seek(pos))
    }

    fn with_inner<W: 'static, T, F: FnOnce(&mut W) -> std::io::Result<T>>(
        &mut self,
        func: F,
    ) -> std::io::Result<T> {
        match self.writer {
            AnyOrPanic::Any(ref mut writer) => {
                let w = writer.downcast_mut::<W>().unwrap();
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| func(w))) {
                    Ok(res) => res,
                    Err(panic) => {
                        self.writer = AnyOrPanic::Panic(panic);
                        Err(std::io::Error::new(std::io::ErrorKind::Other, "Panicked"))
                    }
                }
            }
            AnyOrPanic::Panic(_) => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Panicked before",
            )),
        }
    }

    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        (self.write_fn)(self, buffer)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        (self.flush_fn)(self)
    }

    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        if let Some(ref seek_fn) = self.seek_fn {
            seek_fn(self, pos)
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_write() {
        let cursor = Cursor::new(vec![]);
        let stream = WriteOutputStream::new(cursor);

        assert_eq!(
            stream.write(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], crate::NONE_CANCELLABLE),
            Ok(10)
        );

        let inner = stream.close_and_take();
        assert!(inner.is::<Cursor<Vec<u8>>>());
        let inner = inner.downcast_ref::<Cursor<Vec<u8>>>().unwrap();
        assert_eq!(inner.get_ref(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn test_write_seek() {
        let cursor = Cursor::new(vec![]);
        let stream = WriteOutputStream::new_seekable(cursor);

        assert_eq!(
            stream.write(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10], crate::NONE_CANCELLABLE),
            Ok(10)
        );

        assert!(stream.can_seek());
        assert_eq!(
            stream.seek(0, glib::SeekType::Set, crate::NONE_CANCELLABLE),
            Ok(())
        );

        assert_eq!(
            stream.write(
                &[11, 12, 13, 14, 15, 16, 17, 18, 19, 20],
                crate::NONE_CANCELLABLE
            ),
            Ok(10)
        );

        let inner = stream.close_and_take();
        assert!(inner.is::<Cursor<Vec<u8>>>());
        let inner = inner.downcast_ref::<Cursor<Vec<u8>>>().unwrap();
        assert_eq!(inner.get_ref(), &[11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    }
}
