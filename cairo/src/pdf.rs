// Take a look at the license at the top of the repository in the LICENSE file.

use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::fmt;
use std::io;
use std::mem;
use std::ops::Deref;
use std::path::Path;
use std::ptr;

#[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
use crate::enums::{PdfMetadata, PdfOutline};
use crate::enums::{PdfVersion, SurfaceType};
use crate::error::Error;
use crate::surface::Surface;
use crate::utils::status_to_result;

#[cfg(feature = "use_glib")]
use glib::translate::*;

impl PdfVersion {
    pub fn as_str(self) -> Option<&'static str> {
        unsafe {
            let res = ffi::cairo_pdf_version_to_string(self.into());
            res.as_ref()
                .and_then(|cstr| CStr::from_ptr(cstr as _).to_str().ok())
        }
    }
}

declare_surface!(PdfSurface, SurfaceType::Pdf);

impl PdfSurface {
    pub fn new<P: AsRef<Path>>(width: f64, height: f64, path: P) -> Result<Self, Error> {
        let path = path.as_ref().to_string_lossy().into_owned();
        let path = CString::new(path).unwrap();

        unsafe { Self::from_raw_full(ffi::cairo_pdf_surface_create(path.as_ptr(), width, height)) }
    }

    for_stream_constructors!(cairo_pdf_surface_create_for_stream);

    pub fn get_versions() -> impl Iterator<Item = PdfVersion> {
        let vers_slice = unsafe {
            let mut vers_ptr = ptr::null_mut();
            let mut num_vers = mem::MaybeUninit::uninit();
            ffi::cairo_pdf_get_versions(&mut vers_ptr, num_vers.as_mut_ptr());

            std::slice::from_raw_parts(vers_ptr, num_vers.assume_init() as _)
        };
        vers_slice.iter().map(|v| PdfVersion::from(*v))
    }

    pub fn restrict(&self, version: PdfVersion) -> Result<(), Error> {
        unsafe {
            ffi::cairo_pdf_surface_restrict_to_version(self.0.to_raw_none(), version.into());
        }
        self.status()
    }

    pub fn set_size(&self, width: f64, height: f64) -> Result<(), Error> {
        unsafe {
            ffi::cairo_pdf_surface_set_size(self.0.to_raw_none(), width, height);
        }
        self.status()
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_metadata(&self, metadata: PdfMetadata, value: &str) -> Result<(), Error> {
        let value = CString::new(value).unwrap();
        unsafe {
            ffi::cairo_pdf_surface_set_metadata(
                self.0.to_raw_none(),
                metadata.into(),
                value.as_ptr(),
            );
        }
        self.status()
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_page_label(&self, label: &str) -> Result<(), Error> {
        let label = CString::new(label).unwrap();
        unsafe {
            ffi::cairo_pdf_surface_set_page_label(self.0.to_raw_none(), label.as_ptr());
        }
        self.status()
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn set_thumbnail_size(&self, width: i32, height: i32) -> Result<(), Error> {
        unsafe {
            ffi::cairo_pdf_surface_set_thumbnail_size(
                self.0.to_raw_none(),
                width as _,
                height as _,
            );
        }
        self.status()
    }

    #[cfg(any(all(feature = "pdf", feature = "v1_16"), feature = "dox"))]
    pub fn add_outline(
        &self,
        parent_id: i32,
        name: &str,
        link_attribs: &str,
        flags: PdfOutline,
    ) -> Result<i32, Error> {
        let name = CString::new(name).unwrap();
        let link_attribs = CString::new(link_attribs).unwrap();

        let res = unsafe {
            ffi::cairo_pdf_surface_add_outline(
                self.0.to_raw_none(),
                parent_id,
                name.as_ptr(),
                link_attribs.as_ptr(),
                flags.bits() as _,
            ) as _
        };

        self.status()?;
        Ok(res)
    }

    fn status(&self) -> Result<(), Error> {
        let status = unsafe { ffi::cairo_surface_status(self.to_raw_none()) };
        status_to_result(status)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::context::*;
    use tempfile::tempfile;

    fn draw(surface: &Surface) {
        let cr = Context::new(surface);

        cr.set_line_width(25.0);

        cr.set_source_rgba(1.0, 0.0, 0.0, 0.5);
        cr.line_to(0., 0.);
        cr.line_to(100., 100.);
        cr.stroke();

        cr.set_source_rgba(0.0, 0.0, 1.0, 0.5);
        cr.line_to(0., 100.);
        cr.line_to(100., 0.);
        cr.stroke();
    }

    fn draw_in_buffer() -> Vec<u8> {
        let buffer: Vec<u8> = vec![];

        let surface = PdfSurface::for_stream(100., 100., buffer).unwrap();
        draw(&surface);
        *surface.finish_output_stream().unwrap().downcast().unwrap()
    }

    #[test]
    fn versions() {
        assert!(PdfSurface::get_versions().any(|v| v == PdfVersion::_1_4));
    }

    #[test]
    fn version_string() {
        let ver_str = PdfVersion::_1_4.as_str().unwrap();
        assert_eq!(ver_str, "PDF 1.4");
    }

    #[test]
    #[cfg(unix)]
    fn file() {
        let surface = PdfSurface::new(100., 100., "/dev/null").unwrap();
        draw(&surface);
        surface.finish();
    }

    #[test]
    fn writer() {
        let file = tempfile().expect("tempfile failed");
        let surface = PdfSurface::for_stream(100., 100., file).unwrap();

        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let file = stream.downcast::<std::fs::File>().unwrap();

        let buffer = draw_in_buffer();
        let file_size = file.metadata().unwrap().len();
        assert_eq!(file_size, buffer.len() as u64);
    }

    #[test]
    fn ref_writer() {
        let mut file = tempfile().expect("tempfile failed");
        let surface = unsafe { PdfSurface::for_raw_stream(100., 100., &mut file).unwrap() };

        draw(&surface);
        surface.finish_output_stream().unwrap();
        drop(file);
    }

    #[test]
    fn buffer() {
        let buffer = draw_in_buffer();

        let header = b"%PDF-1.5";
        assert_eq!(&buffer[..header.len()], header);
    }

    #[test]
    fn custom_writer() {
        struct CustomWriter(usize);

        impl io::Write for CustomWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0 += buf.len();
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let custom_writer = CustomWriter(0);

        let surface = PdfSurface::for_stream(20., 20., custom_writer).unwrap();
        surface.set_size(100., 100.).unwrap();
        draw(&surface);
        let stream = surface.finish_output_stream().unwrap();
        let custom_writer = stream.downcast::<CustomWriter>().unwrap();

        let buffer = draw_in_buffer();

        assert_eq!(custom_writer.0, buffer.len());
    }

    fn with_panicky_stream() -> PdfSurface {
        struct PanicWriter;

        impl io::Write for PanicWriter {
            fn write(&mut self, _buf: &[u8]) -> io::Result<usize> {
                panic!("panic in writer");
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let surface = PdfSurface::for_stream(20., 20., PanicWriter).unwrap();
        surface.finish();
        surface
    }

    #[test]
    #[should_panic]
    fn finish_stream_propagates_panic() {
        let _ = with_panicky_stream().finish_output_stream();
    }
}
