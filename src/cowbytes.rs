use crate::{ByteBuf, Bytes};

use core::fmt;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(all(feature = "std", not(feature = "alloc")))]
use std::borrow::Cow;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use serde::{de::Visitor, Deserialize, Deserializer, Serialize, Serializer};

pub struct CowBytes<'a> {
    bytes: Cow<'a, [u8]>,
}

impl<'a> CowBytes<'a> {
    pub fn from<I>(bytes: I) -> Self
    where
        I: Into<Cow<'a, [u8]>>,
    {
        Self {
            bytes: bytes.into(),
        }
    }

    pub fn into_cow(self) -> Cow<'a, [u8]> {
        self.bytes
    }

    pub fn into_cow_bytes(self) -> Cow<'a, Bytes> {
        match self.bytes {
            Cow::Borrowed(bytes) => Cow::Borrowed(Bytes::new(bytes)),
            Cow::Owned(buf) => Cow::Owned(ByteBuf::from(buf)),
        }
    }
}

struct CowBytesVisitor;

impl<'de> Visitor<'de> for CowBytesVisitor {
    type Value = CowBytes<'de>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("bytes")
    }

    fn visit_bytes<E>(self, bytes: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(bytes.to_owned()))
    }

    fn visit_borrowed_bytes<E>(self, bytes: &'de [u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(bytes))
    }

    fn visit_byte_buf<E>(self, buf: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(buf))
    }

    fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(str.as_bytes().to_owned()))
    }

    fn visit_borrowed_str<E>(self, str: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(str.as_bytes()))
    }

    fn visit_string<E>(self, string: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(CowBytes::from(string.into_bytes()))
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut buf: Vec<u8> = if let Some(length) = seq.size_hint() {
            Vec::with_capacity(length)
        } else {
            Vec::new()
        };

        while let Some(byte) = seq.next_element()? {
            buf.push(byte)
        }

        Ok(CowBytes::from(buf))
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for CowBytes<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(CowBytesVisitor)
    }
}

impl Serialize for CowBytes<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}
