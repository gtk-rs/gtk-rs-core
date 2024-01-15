// Take a look at the license at the top of the repository in the LICENSE file.

use super::*;

use crate::{
    translate::{TransparentPtrType, TransparentType},
    List, PtrSlice, SList, Slice, StrV,
};

serialize_impl!(Slice<T: TransparentType>, Sequence(iter) => iter);

deserialize_impl! {
    Slice<T: TransparentType>,
    "a sequence of GLib transparent values",
    Deserializer::deserialize_seq => match impl {
        Seq(s) => {
            let mut slice = Slice::with_capacity(min(s.size_hint().unwrap_or(0), 4096));

            while let Some(item) = s.next_element()? {
                slice.push(item)
            }

            Ok(slice)
        },
    }
}

serialize_impl!(PtrSlice<T: TransparentPtrType>, Sequence(iter) => iter);

deserialize_impl! {
    PtrSlice<T: TransparentPtrType>,
    "a sequence of GLib transparent pointer values",
    Deserializer::deserialize_seq => match impl {
        Seq(s) => {
            let mut slice = PtrSlice::with_capacity(min(s.size_hint().unwrap_or(0), 4096));

            while let Some(item) = s.next_element()? {
                slice.push(item)
            }

            Ok(slice)
        },
    }
}

serialize_impl!(List<T: TransparentPtrType>, Sequence(iter) => iter.iter());

deserialize_impl! {
    List<T: TransparentPtrType>,
    "a sequence of GLib transparent pointer values",
    Deserializer::deserialize_seq => match impl {
        Seq(s) => {
            let mut list = List::new();

            while let Some(item) = s.next_element()? {
                list.push_front(item)
            }
            list.reverse();

            Ok(list)
        },
    }
}

serialize_impl!(SList<T: TransparentPtrType>, Sequence(iter) => iter.iter());

deserialize_impl! {
    SList<T: TransparentPtrType>,
    "a sequence of GLib transparent pointer values",
    Deserializer::deserialize_seq => match impl {
        Seq(s) => {
            let mut list = SList::new();

            while let Some(item) = s.next_element()? {
                list.push_front(item)
            }
            list.reverse();

            Ok(list)
        },
    }
}

serialize_impl!(StrV, Sequence(iter) => iter);

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{gformat, Bytes, List, PtrSlice, SList, Slice};

    #[test]
    fn serialization() {
        let bytes = gformat!("Lorem ipsum dolor sit amet").into_bytes();

        let slice = Slice::from([
            Bytes::from_owned(bytes[..].to_vec()),
            Bytes::from_owned(bytes[1..].to_vec()),
            Bytes::from_owned(bytes[2..].to_vec()),
            Bytes::from_owned(bytes[3..].to_vec()),
        ]);

        let ptr_slice = PtrSlice::from([
            Bytes::from_owned(bytes[..].to_vec()),
            Bytes::from_owned(bytes[1..].to_vec()),
            Bytes::from_owned(bytes[2..].to_vec()),
            Bytes::from_owned(bytes[3..].to_vec()),
        ]);

        let mut list = List::<Bytes>::new();
        list.push_front(Bytes::from_owned(bytes[..].to_vec()));
        list.push_front(Bytes::from_owned(bytes[1..].to_vec()));
        list.push_front(Bytes::from_owned(bytes[2..].to_vec()));
        list.push_front(Bytes::from_owned(bytes[3..].to_vec()));
        list.reverse();

        let mut slist = SList::<Bytes>::new();
        slist.push_front(Bytes::from_owned(bytes[..].to_vec()));
        slist.push_front(Bytes::from_owned(bytes[1..].to_vec()));
        slist.push_front(Bytes::from_owned(bytes[2..].to_vec()));
        slist.push_front(Bytes::from_owned(bytes[3..].to_vec()));
        slist.reverse();

        assert_eq!(json!(&slice), json!(&list));
        assert_eq!(json!(&slice), json!(&slist));
        assert_eq!(json!(&ptr_slice), json!(&list));
        assert_eq!(json!(&ptr_slice), json!(&slist));
        assert_eq!(json!(&slice), json!(&ptr_slice));
        assert_eq!(json!(&list), json!(&slist));
    }

    #[test]
    fn deserialization() {
        let json_str = r#"
[
    [76,111,114,101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101],
    [111,114,101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101],
    [114,101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101],
    [101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101]
]"#;
        serde_json::from_str::<Slice<Bytes>>(json_str).unwrap();
        serde_json::from_str::<PtrSlice<Bytes>>(json_str).unwrap();
        serde_json::from_str::<List<Bytes>>(json_str).unwrap();
        serde_json::from_str::<SList<Bytes>>(json_str).unwrap();
    }
}
