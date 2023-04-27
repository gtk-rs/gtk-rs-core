// Take a look at the license at the top of the repository in the LICENSE file.

use super::*;

use crate::{gformat, GStr, GString, GStringPtr};
use serde::de;

serialize_impl!(GStr, str(s) => s.as_str());

serialize_impl!(GString, str(s) => s.as_str());

deserialize_impl! {
    GString,
    "a valid UTF-8 string",
    Deserializer::deserialize_string => match impl {
        str(s) => Ok(gformat!("{s}")),
        String(s) => GString::from_string_checked(s).map_err(|e| de::Error::custom(e)),
    }
}

serialize_impl!(GStringPtr, str(s) => s.to_str());

#[cfg(test)]
mod tests {
    use crate::{translate::ToGlibPtr, GString, StrV};
    use serde_json::json;

    use crate::gformat;

    #[test]
    fn serialization() {
        let gstring = gformat!("Lorem ipsum dolor sit amet");
        let gstr = gstring.as_gstr();
        let gstringptr =
            &unsafe { StrV::from_glib_none(vec![gstring.to_owned()].to_glib_none().0) }[0];

        assert_eq!(json!(&gstring), json!(gstr));
        assert_eq!(json!(&gstring), json!(gstringptr));
        assert_eq!(json!(gstr), json!(gstringptr));
    }

    #[test]
    fn deserialization() {
        let json_str = r#""Lorem ipsum dolor sit amet""#;

        serde_json::from_str::<GString>(json_str).unwrap();
    }
}
