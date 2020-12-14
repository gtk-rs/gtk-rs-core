// Take a look at the license at the top of the repository in the LICENSE file.

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::MemoryInputStream;
    use glib::Bytes;

    #[test]
    fn new() {
        let strm = MemoryInputStream::new();
        let ret = strm.skip(1, crate::NONE_CANCELLABLE);
        assert!(!ret.is_err());
        assert_eq!(ret.unwrap(), 0);

        let mut buf = vec![0; 10];
        let ret = strm.read(&mut buf, crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn from_bytes() {
        let b = Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::from_bytes(&b);
        let mut buf = vec![0; 10];
        let ret = strm.read(&mut buf, crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn add_bytes() {
        let strm = MemoryInputStream::new();
        let b = Bytes::from_owned(vec![1, 2, 3]);
        strm.add_bytes(&b);
        let mut buf = vec![0; 10];
        let ret = strm.read(&mut buf, crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 3);
        assert_eq!(buf[0], 1);
        assert_eq!(buf[1], 2);
        assert_eq!(buf[2], 3);

        let ret = strm.skip(10, crate::NONE_CANCELLABLE).unwrap();
        assert_eq!(ret, 0);
    }

    #[test]
    fn read_async_future() {
        use futures_util::future::TryFutureExt;

        let c = glib::MainContext::new();

        let buf = vec![0; 10];
        let b = glib::Bytes::from_owned(vec![1, 2, 3]);
        let strm = MemoryInputStream::from_bytes(&b);

        let res = c
            .block_on(
                strm.read_async_future(buf, glib::PRIORITY_DEFAULT)
                    .map_err(|(_buf, err)| err)
                    .map_ok(move |(mut buf, len)| {
                        buf.truncate(len);
                        buf
                    }),
            )
            .unwrap();

        assert_eq!(res, vec![1, 2, 3]);
    }
}
