use crate::Bytes;
use serde::Deserializer;

#[cfg(any(feature = "std", feature = "alloc"))]
use crate::ByteBuf;

#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(all(feature = "std", not(feature = "alloc")))]
use std::borrow::Cow;

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(feature = "heapless")]
use heapless::{ArrayLength, Vec};

/// Types that can be deserialized via `#[serde(with = "serde_bytes")]`.
pub trait Deserialize<'de>: Sized {
    #[allow(missing_docs)]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>;
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a [u8] {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Via the serde::Deserialize impl for &[u8].
        serde::Deserialize::deserialize(deserializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Vec<u8> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(ByteBuf::into_vec)
    }
}

#[cfg(feature = "heapless")]
impl<'de, N> Deserialize<'de> for Vec<u8, N>
where
    N: ArrayLength<u8>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(|bytes| {
            let mut vec: Vec<u8, N> = Vec::new();
            // TODO: can we map errors?
            vec.extend_from_slice(bytes).unwrap();
            vec
        })
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for &'a Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Bytes::new)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for ByteBuf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Via the serde::Deserialize impl for ByteBuf.
        serde::Deserialize::deserialize(deserializer)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, [u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Cow::Borrowed)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de: 'a, 'a> Deserialize<'de> for Cow<'a, Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Cow::Borrowed)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Box<[u8]> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Deserialize::deserialize(deserializer).map(Vec::into_boxed_slice)
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<'de> Deserialize<'de> for Box<Bytes> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Box<[u8]> = Deserialize::deserialize(deserializer)?;
        Ok(bytes.into())
    }
}
