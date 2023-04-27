// Take a look at the license at the top of the repository in the LICENSE file.

use serde::Deserializer;
use std::cmp::min;

macro_rules! serialize_impl {
    ($ty:ty, Bytes($bind:ident) => $expr:expr) => {
        impl ::serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                let $bind = self;

                serializer.serialize_bytes($expr)
            }
        }
    };
    ($ty:ty, str($bind:ident) => $expr:expr) => {
        impl ::serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                let $bind = self;

                serializer.serialize_str($expr)
            }
        }
    };
    ($ty:ident$(<$($generic:ident $(: $bound:tt $(+ $bound2:tt)*)?),+>)?, Sequence($bind:ident) => $expr:expr) => {
        impl$(<$($generic $(: ::serde::Serialize + $bound $(+ $bound2)*)?),+>)? ::serde::Serialize for $ty$(<$($generic),+>)? {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer
            {
                let $bind = self;

                serializer.collect_seq($expr)
            }
        }
    };
}

macro_rules! deserialize_impl {
    (
        $ty:ident$(<$($generic:ident$(: $bound:tt $(+ $bound2:tt)*)?),+>)?,
        $expecting:literal,
        $deserialize_target:expr => match impl {
            $(
                Bytes($bytes_arg:ident) => $visit_bytes:expr,
                ByteBuf($byte_buf_arg:ident) => $visit_byte_buf:expr,
            )?
            $(
                str($str_arg:ident) => $visit_str:expr,
                String($string_arg:ident) => $visit_string:expr,
            )?
            $(Seq($seq_arg:ident) => $visit_seq:expr,)?
            $(
                @in_place($inplace_self:ident) => match impl {
                    $(
                        Bytes($inplace_bytes_arg:ident) => $inplace_visit_bytes:expr,
                        ByteBuf($inplace_byte_buf_arg:ident) => $inplace_visit_byte_buf:expr,
                    )?
                    $(
                        str($inplace_str_arg:ident) => $inplace_visit_str:expr,
                        String($inplace_string_arg:ident) => $inplace_visit_string:expr,
                    )?
                    $(Seq($inplace_seq_arg:ident) => $inplace_visit_seq:expr,)?
                },
            )?
        }
    ) => {
        impl<'de, $($($generic $(: ::serde::Deserialize<'de> + $bound $(+ $bound2)*)?),+)?> ::serde::Deserialize<'de> for $ty$(<$($generic),+>)? {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                struct Visitor<'v$(, $($generic),+)?>(::std::marker::PhantomData<&'v ()>, $($(::std::marker::PhantomData<fn() -> $generic>),+)?)
                $(
                    where
                        $($generic: ::serde::Deserialize<'v> $(+ $bound $(+ $bound2)*)?),+
                )?;

                impl<'a$(, $($generic),+)?> ::serde::de::Visitor<'a> for Visitor<'a, $($($generic),+)?>
                $(
                    where
                        $($generic: ::serde::Deserialize<'a> $(+ $bound $(+ $bound2)*)?),+
                )?
                {
                    type Value = $ty$(<$($generic),+>)?;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str($expecting)
                    }

                    $(
                        fn visit_bytes<E>(self, $bytes_arg: &[u8]) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            $visit_bytes
                        }

                        fn visit_byte_buf<E>(self, $byte_buf_arg: Vec<u8>) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            $visit_byte_buf
                        }
                    )?

                    $(
                        fn visit_str<E>(self, $str_arg: &str) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            $visit_str
                        }

                        fn visit_string<E>(self, $string_arg: String) -> Result<Self::Value, E>
                        where
                            E: ::serde::de::Error,
                        {
                            $visit_string
                        }
                    )?

                    $(
                        fn visit_seq<A>(self, mut $seq_arg: A) -> Result<Self::Value, A::Error>
                        where
                            A: ::serde::de::SeqAccess<'a>,
                        {
                            $visit_seq
                        }
                    )?
                }

                $deserialize_target(deserializer, Visitor(::std::marker::PhantomData, $($(::std::marker::PhantomData::<fn() -> $generic>),+)?))
            }
        }
    };
}

mod byte_array;

mod bytes;

mod gstring;

mod collections;
