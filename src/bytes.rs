use core::fmt::{self, Debug};
use core::ops::Deref;

#[cfg(feature = "alloc")]
use alloc::borrow::ToOwned;

use serde::de::{Deserialize, Deserializer, Error, Visitor};
use serde::ser::{Serialize, Serializer};

/// Wrapper around `&[u8]` to serialize and deserialize efficiently.
///
/// ```
/// use std::collections::HashMap;
/// use std::io;
///
/// use serde_bytes::Bytes;
///
/// fn print_encoded_cache() -> bincode::Result<()> {
///     let mut cache = HashMap::new();
///     cache.insert(3, Bytes::new(b"three"));
///     cache.insert(2, Bytes::new(b"two"));
///     cache.insert(1, Bytes::new(b"one"));
///
///     bincode::serialize_into(&mut io::stdout(), &cache)
/// }
/// #
/// # fn main() {
/// #     print_encoded_cache().unwrap();
/// # }
/// ```
#[derive(Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(C)]
pub struct Bytes {
    bytes: [u8],
}

impl Bytes {
    /// Wrap an existing `&[u8]`.
    pub fn new(bytes: &[u8]) -> &Self {
        unsafe { &*(bytes as *const [u8] as *const Bytes) }
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.bytes, f)
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl ToOwned for Bytes {
    type Owned = crate::ByteBuf;

    fn to_owned(&self) -> Self::Owned {
        unimplemented!()
    }
}

impl Serialize for Bytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

struct BytesVisitor;

impl<'de> Visitor<'de> for BytesVisitor {
    type Value = &'de Bytes;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a borrowed byte array")
    }

    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Bytes::new(v))
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Bytes::new(v.as_bytes()))
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for &'a Bytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(BytesVisitor)
    }
}
