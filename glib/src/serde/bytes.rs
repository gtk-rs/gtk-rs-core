// Take a look at the license at the top of the repository in the LICENSE file.

use serde::Deserializer;

use super::*;

use crate::Bytes;

serialize_impl!(Bytes, Bytes(b) => b);

deserialize_impl! {
    Bytes,
    "a sequence of bytes",
    Deserializer::deserialize_seq => match impl {
        Bytes(b) => Ok(Bytes::from_owned(b.to_owned())),
        ByteBuf(buf) => Ok(Bytes::from_owned(buf)),
        Seq(s) => {
            let mut bytes = Vec::with_capacity(min(s.size_hint().unwrap_or(0), 4096));

            while let Some(byte) = s.next_element()? {
                bytes.push(byte)
            }

            Ok(Bytes::from_owned(bytes))
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{gformat, Bytes};

    #[test]
    fn serialization() {
        let json = match serde_json::to_value(Bytes::from_owned(
            gformat!("Lorem ipsum dolor sit amet").into_bytes(),
        )) {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        assert_ne!(json, None);
    }

    #[test]
    fn deserialization() {
        let json_str = r#"[76,111,114,101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101]"#;

        serde_json::from_str::<Bytes>(json_str).unwrap();
    }
}
