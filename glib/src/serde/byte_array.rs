// Take a look at the license at the top of the repository in the LICENSE file.

use serde::Deserializer;

use super::*;

use crate::ByteArray;

serialize_impl!(ByteArray, Bytes(b) => b);

deserialize_impl! {
    ByteArray,
    "a sequence of bytes",
    Deserializer::deserialize_seq => match impl {
        Bytes(b) => Ok(ByteArray::from(b)),
        ByteBuf(buf) => Ok(ByteArray::from(buf.as_slice())),
        Seq(s) => {
            // See https://docs.rs/serde/1.0.159/src/serde/de/impls.rs.html#1038
            // and https://docs.rs/serde/1.0.159/src/serde/private/size_hint.rs.html#13
            let mut bytes = Vec::with_capacity(min(s.size_hint().unwrap_or(0), 4096));

            while let Some(byte) = s.next_element()? {
                bytes.push(byte)
            }

            Ok(ByteArray::from(bytes.as_slice()))
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::{gformat, ByteArray};

    #[test]
    fn serialization() {
        let json = match serde_json::to_value(ByteArray::from(
            gformat!("Lorem ipsum dolor sit amet").as_bytes(),
        )) {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        assert_ne!(json, None);
    }

    #[test]
    fn deserialization() {
        let json_str = r#"[76,111,114,101,109,32,105,112,115,117,109,32,100,111,108,111,114,32,115,105,116,32,97,109,101]"#;

        serde_json::from_str::<ByteArray>(json_str).unwrap();
    }
}
