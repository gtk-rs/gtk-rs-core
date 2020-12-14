// Take a look at the license at the top of the repository in the LICENSE file.

use glib::object::IsA;
use glib::translate::*;
use glib::Error;
use libc::{c_uchar, c_void};
use std::io::Read;
use std::mem;
use std::path::Path;
use std::pin::Pin;
use std::ptr;
use std::slice;

use std::future::Future;

use crate::{Colorspace, Pixbuf, PixbufFormat};

impl Pixbuf {
    pub fn from_mut_slice<T: AsMut<[u8]>>(
        data: T,
        colorspace: Colorspace,
        has_alpha: bool,
        bits_per_sample: i32,
        width: i32,
        height: i32,
        row_stride: i32,
    ) -> Pixbuf {
        unsafe extern "C" fn destroy<T: AsMut<[u8]>>(_: *mut c_uchar, data: *mut c_void) {
            let _data: Box<T> = Box::from_raw(data as *mut T); // the data will be destroyed now
        }
        assert!(width > 0, "width must be greater than 0");
        assert!(height > 0, "height must be greater than 0");
        assert!(row_stride > 0, "row_stride must be greater than 0");
        assert!(
            bits_per_sample == 8,
            "bits_per_sample == 8 is the only supported value"
        );

        let width = width as usize;
        let height = height as usize;
        let row_stride = row_stride as usize;
        let bits_per_sample = bits_per_sample as usize;

        let n_channels = if has_alpha { 4 } else { 3 };
        let last_row_len = width * ((n_channels * bits_per_sample + 7) / 8);

        let mut data: Box<T> = Box::new(data);

        let ptr = {
            let data: &mut [u8] = (*data).as_mut();
            assert!(
                data.len() >= ((height - 1) * row_stride + last_row_len) as usize,
                "data.len() must fit the width, height, and row_stride"
            );
            data.as_mut_ptr()
        };

        unsafe {
            from_glib_full(ffi::gdk_pixbuf_new_from_data(
                ptr,
                colorspace.to_glib(),
                has_alpha.to_glib(),
                bits_per_sample as i32,
                width as i32,
                height as i32,
                row_stride as i32,
                Some(destroy::<T>),
                Box::into_raw(data) as *mut _,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a `Pixbuf` from a type implementing `Read` (like `File`).
    ///
    /// ```no_run
    /// use std::fs::File;
    /// use gdk_pixbuf::Pixbuf;
    ///
    /// let f = File::open("some_file.png").expect("failed to open image");
    /// let pixbuf = Pixbuf::from_read(f).expect("failed to load image");
    /// ```
    pub fn from_read<R: Read + Send + 'static>(r: R) -> Result<Pixbuf, Error> {
        Pixbuf::from_stream(&gio::ReadInputStream::new(r), None::<&gio::Cancellable>)
    }

    pub fn from_file<T: AsRef<Path>>(filename: T) -> Result<Pixbuf, Error> {
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file;
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_utf8 as gdk_pixbuf_new_from_file;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_file_at_size<T: AsRef<Path>>(
        filename: T,
        width: i32,
        height: i32,
    ) -> Result<Pixbuf, Error> {
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file_at_size;
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_at_size_utf8 as gdk_pixbuf_new_from_file_at_size;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_size(
                filename.as_ref().to_glib_none().0,
                width,
                height,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_file_at_scale<T: AsRef<Path>>(
        filename: T,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
    ) -> Result<Pixbuf, Error> {
        #[cfg(not(windows))]
        use ffi::gdk_pixbuf_new_from_file_at_scale;
        #[cfg(windows)]
        use ffi::gdk_pixbuf_new_from_file_at_scale_utf8 as gdk_pixbuf_new_from_file_at_scale;

        unsafe {
            let mut error = ptr::null_mut();
            let ptr = gdk_pixbuf_new_from_file_at_scale(
                filename.as_ref().to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.to_glib(),
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    pub fn from_stream_async<
        P: IsA<gio::InputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<Pixbuf, Error>) + Send + 'static,
    >(
        stream: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let cancellable = cancellable.map(|p| p.as_ref());
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn from_stream_async_trampoline<
            R: FnOnce(Result<Pixbuf, Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = from_stream_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_async(
                stream.as_ref().to_glib_none().0,
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn from_stream_async_future<P: IsA<gio::InputStream> + Clone + 'static>(
        stream: &P,
    ) -> Pin<Box<dyn Future<Output = Result<Pixbuf, Error>> + 'static>> {
        let stream = stream.clone();
        Box::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::from_stream_async(&stream, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    pub fn from_stream_at_scale_async<
        P: IsA<gio::InputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<Pixbuf, Error>) + Send + 'static,
    >(
        stream: &P,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let cancellable = cancellable.map(|p| p.as_ref());
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn from_stream_at_scale_async_trampoline<
            R: FnOnce(Result<Pixbuf, Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ptr = ffi::gdk_pixbuf_new_from_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ptr))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = from_stream_at_scale_async_trampoline::<R>;
        unsafe {
            ffi::gdk_pixbuf_new_from_stream_at_scale_async(
                stream.as_ref().to_glib_none().0,
                width,
                height,
                preserve_aspect_ratio.to_glib(),
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn from_stream_at_scale_async_future<P: IsA<gio::InputStream> + Clone + 'static>(
        stream: &P,
        width: i32,
        height: i32,
        preserve_aspect_ratio: bool,
    ) -> Pin<Box<dyn Future<Output = Result<Pixbuf, Error>> + 'static>> {
        let stream = stream.clone();
        Box::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::from_stream_at_scale_async(
                &stream,
                width,
                height,
                preserve_aspect_ratio,
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    #[allow(clippy::mut_from_ref)]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn get_pixels(&self) -> &mut [u8] {
        let mut len = 0;
        let ptr = ffi::gdk_pixbuf_get_pixels_with_length(self.to_glib_none().0, &mut len);
        slice::from_raw_parts_mut(ptr, len as usize)
    }

    pub fn put_pixel(&self, x: u32, y: u32, red: u8, green: u8, blue: u8, alpha: u8) {
        assert!(
            x < self.get_width() as u32,
            "x must be less than the pixbuf's width"
        );
        assert!(
            y < self.get_height() as u32,
            "y must be less than the pixbuf's height"
        );

        unsafe {
            let x = x as usize;
            let y = y as usize;
            let n_channels = self.get_n_channels() as usize;
            assert!(n_channels == 3 || n_channels == 4);
            let rowstride = self.get_rowstride() as usize;
            let pixels = self.get_pixels();
            let pos = y * rowstride + x * n_channels;

            pixels[pos] = red;
            pixels[pos + 1] = green;
            pixels[pos + 2] = blue;
            if n_channels == 4 {
                pixels[pos + 3] = alpha;
            }
        }
    }

    pub fn get_file_info<T: AsRef<Path>>(filename: T) -> Option<(PixbufFormat, i32, i32)> {
        unsafe {
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = ffi::gdk_pixbuf_get_file_info(
                filename.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            if !ret.is_null() {
                Some((
                    from_glib_none(ret),
                    width.assume_init(),
                    height.assume_init(),
                ))
            } else {
                None
            }
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    pub fn get_file_info_async<
        P: IsA<gio::Cancellable>,
        Q: FnOnce(Result<Option<(PixbufFormat, i32, i32)>, Error>) + Send + 'static,
        T: AsRef<Path>,
    >(
        filename: T,
        cancellable: Option<&P>,
        callback: Q,
    ) {
        let cancellable = cancellable.map(|p| p.as_ref());
        let user_data: Box<Q> = Box::new(callback);
        unsafe extern "C" fn get_file_info_async_trampoline<
            Q: FnOnce(Result<Option<(PixbufFormat, i32, i32)>, Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let mut width = mem::MaybeUninit::uninit();
            let mut height = mem::MaybeUninit::uninit();
            let ret = ffi::gdk_pixbuf_get_file_info_finish(
                res,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
                &mut error,
            );
            let result = if !error.is_null() {
                Err(from_glib_full(error))
            } else if ret.is_null() {
                Ok(None)
            } else {
                Ok(Some((
                    from_glib_none(ret),
                    width.assume_init(),
                    height.assume_init(),
                )))
            };
            let callback: Box<Q> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = get_file_info_async_trampoline::<Q>;
        unsafe {
            ffi::gdk_pixbuf_get_file_info_async(
                filename.as_ref().to_glib_none().0,
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_32", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_32")))]
    #[allow(clippy::type_complexity)]
    pub fn get_file_info_async_future<T: AsRef<Path> + Clone + 'static>(
        filename: T,
    ) -> Pin<Box<dyn Future<Output = Result<Option<(PixbufFormat, i32, i32)>, Error>> + 'static>>
    {
        Box::pin(gio::GioFuture::new(&(), move |_obj, send| {
            let cancellable = gio::Cancellable::new();
            Self::get_file_info_async(filename, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    pub fn save_to_bufferv(&self, type_: &str, options: &[(&str, &str)]) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut buffer = ptr::null_mut();
            let mut buffer_size = mem::MaybeUninit::uninit();
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_save_to_bufferv(
                self.to_glib_none().0,
                &mut buffer,
                buffer_size.as_mut_ptr(),
                type_.to_glib_none().0,
                option_keys.to_glib_none().0,
                option_values.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibContainer::from_glib_full_num(
                    buffer,
                    buffer_size.assume_init() as usize,
                ))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    pub fn save_to_streamv<P: IsA<gio::OutputStream>, Q: IsA<gio::Cancellable>>(
        &self,
        stream: &P,
        type_: &str,
        options: &[(&str, &str)],
        cancellable: Option<&Q>,
    ) -> Result<(), Error> {
        let cancellable = cancellable.map(|p| p.as_ref());
        unsafe {
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_save_to_streamv(
                self.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
                option_keys.to_glib_none().0,
                option_values.to_glib_none().0,
                cancellable.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    pub fn save_to_streamv_async<
        P: IsA<gio::OutputStream>,
        Q: IsA<gio::Cancellable>,
        R: FnOnce(Result<(), Error>) + Send + 'static,
    >(
        &self,
        stream: &P,
        type_: &str,
        options: &[(&str, &str)],
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let cancellable = cancellable.map(|p| p.as_ref());
        let user_data: Box<R> = Box::new(callback);
        unsafe extern "C" fn save_to_streamv_async_trampoline<
            R: FnOnce(Result<(), Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let _ = ffi::gdk_pixbuf_save_to_stream_finish(res, &mut error);
            let result = if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box<R> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = save_to_streamv_async_trampoline::<R>;
        unsafe {
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            ffi::gdk_pixbuf_save_to_streamv_async(
                self.to_glib_none().0,
                stream.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
                option_keys.to_glib_none().0,
                option_values.to_glib_none().0,
                cancellable.to_glib_none().0,
                Some(callback),
                Box::into_raw(user_data) as *mut _,
            );
        }
    }

    #[cfg(any(feature = "v2_36", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v2_36")))]
    pub fn save_to_streamv_async_future<P: IsA<gio::OutputStream> + Clone + 'static>(
        &self,
        stream: &P,
        type_: &str,
        options: &[(&str, &str)],
    ) -> Pin<Box<dyn Future<Output = Result<(), Error>> + 'static>> {
        let stream = stream.clone();
        let type_ = String::from(type_);
        let options = options
            .iter()
            .map(|&(k, v)| (String::from(k), String::from(v)))
            .collect::<Vec<(String, String)>>();
        Box::pin(gio::GioFuture::new(self, move |obj, send| {
            let cancellable = gio::Cancellable::new();
            let options = options
                .iter()
                .map(|&(ref k, ref v)| (k.as_str(), v.as_str()))
                .collect::<Vec<(&str, &str)>>();

            obj.save_to_streamv_async(
                &stream,
                &type_,
                options.as_slice(),
                Some(&cancellable),
                move |res| {
                    send.resolve(res);
                },
            );

            cancellable
        }))
    }

    pub fn savev<T: AsRef<Path>>(
        &self,
        filename: T,
        type_: &str,
        options: &[(&str, &str)],
    ) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let option_keys: Vec<&str> = options.iter().map(|o| o.0).collect();
            let option_values: Vec<&str> = options.iter().map(|o| o.1).collect();
            let _ = ffi::gdk_pixbuf_savev(
                self.to_glib_none().0,
                filename.as_ref().to_glib_none().0,
                type_.to_glib_none().0,
                option_keys.to_glib_none().0,
                option_values.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(())
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}
