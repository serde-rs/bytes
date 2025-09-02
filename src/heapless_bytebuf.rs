use core::borrow::{Borrow, BorrowMut};
use core::cmp::Ordering;
use core::convert::TryFrom;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::ops::{Deref, DerefMut};

use heapless::Vec;

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, Serializer};

use crate::Bytes;

/// Wrapper around `heapless::Vec<u8>` to serialize and deserialize efficiently.
///
/// ```
/// use std::collections::HashMap;
/// use std::io;
///
/// use serde_bytes::HeaplessByteBuf;
///
/// fn deserialize_bytebufs() -> Result<(), bincode::error::DecodeError> {
///     let example_data = [2, 2, 3, 116, 119, 111, 1, 3, 111, 110, 101];
///
///     let map: HashMap<u32, HeaplessByteBuf<11>>;
///     (map, _) = bincode::serde::decode_from_slice(
///         &example_data,
///         bincode::config::standard(),
///     )?;
///
///     println!("{:?}", map);
///
///     Ok(())
/// }
/// #
/// # fn main() {
/// #     deserialize_bytebufs().unwrap();
/// # }
/// ```
#[derive(Clone, Default, Eq, Ord)]
pub struct HeaplessByteBuf<const N: usize> {
    bytes: Vec<u8, N>,
}

impl<const N: usize> HeaplessByteBuf<N> {
    /// Construct a `HeaplessByteBuf`.
    pub fn new() -> Self {
        HeaplessByteBuf { bytes: Vec::new() }
    }

    /// Wrap existing bytes in a `HeaplessByteBuf`.
    pub fn from<T: Into<Vec<u8, N>>>(bytes: T) -> Self {
        HeaplessByteBuf {
            bytes: bytes.into(),
        }
    }

    /// Wrap existing bytes in a `HeaplessByteBuf`.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, HeaplessByteBufFullError<N>> {
        let bytes = Vec::from_slice(bytes).map_err(|_| HeaplessByteBufFullError::<N>)?;

        Ok(HeaplessByteBuf { bytes })
    }

    /// Wrap existing bytes in a `HeaplessByteBuf`.
    #[cfg(any(feature = "std", feature = "alloc"))]
    pub fn try_from<T: Into<alloc::vec::Vec<u8>>>(
        vec: T,
    ) -> Result<Self, HeaplessByteBufFullError<N>> {
        let std_vec = vec.into();
        let bytes = Vec::from_slice(&std_vec).map_err(|_| HeaplessByteBufFullError::<N>)?;

        Ok(HeaplessByteBuf { bytes })
    }

    /// Unwrap the vector of byte underlying this `HeaplessByteBuf`.
    pub fn into_inner(self) -> Vec<u8, N> {
        self.bytes
    }

    #[doc(hidden)]
    #[allow(clippy::should_implement_trait)]
    pub fn into_iter(self) -> <Vec<u8, N> as IntoIterator>::IntoIter {
        self.bytes.into_iter()
    }
}

#[derive(Debug)]
pub struct HeaplessByteBufFullError<const N: usize>;

impl<const N: usize> Display for HeaplessByteBufFullError<N> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "heapless::Vec<{N}> is too small")
    }
}

impl<const N: usize> core::error::Error for HeaplessByteBufFullError<N> {}

impl<const N: usize> Debug for HeaplessByteBuf<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.bytes, f)
    }
}

impl<const N: usize> AsRef<[u8]> for HeaplessByteBuf<N> {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl<const N: usize> AsMut<[u8]> for HeaplessByteBuf<N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl<const N: usize> Deref for HeaplessByteBuf<N> {
    type Target = Vec<u8, N>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl<const N: usize> DerefMut for HeaplessByteBuf<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}

impl<const N: usize> Borrow<Bytes> for HeaplessByteBuf<N> {
    fn borrow(&self) -> &Bytes {
        Bytes::new(&self.bytes)
    }
}

impl<const N: usize> BorrowMut<Bytes> for HeaplessByteBuf<N> {
    fn borrow_mut(&mut self) -> &mut Bytes {
        unsafe { &mut *(&mut self.bytes as &mut [u8] as *mut [u8] as *mut Bytes) }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
impl<const N: usize> TryFrom<alloc::vec::Vec<u8>> for HeaplessByteBuf<N> {
    type Error = HeaplessByteBufFullError<N>;

    fn try_from(bytes: alloc::vec::Vec<u8>) -> Result<Self, Self::Error> {
        HeaplessByteBuf::<N>::try_from(bytes)
    }
}

impl<Rhs, const N: usize> PartialEq<Rhs> for HeaplessByteBuf<N>
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn eq(&self, other: &Rhs) -> bool {
        self.as_ref().eq(other.as_ref())
    }
}

impl<Rhs, const N: usize> PartialOrd<Rhs> for HeaplessByteBuf<N>
where
    Rhs: ?Sized + AsRef<[u8]>,
{
    fn partial_cmp(&self, other: &Rhs) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<const N: usize> Hash for HeaplessByteBuf<N> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bytes.hash(state);
    }
}

impl<const N: usize> IntoIterator for HeaplessByteBuf<N> {
    type Item = u8;
    type IntoIter = <Vec<u8, N> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a HeaplessByteBuf<N> {
    type Item = &'a u8;
    type IntoIter = <&'a [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter()
    }
}

impl<'a, const N: usize> IntoIterator for &'a mut HeaplessByteBuf<N> {
    type Item = &'a mut u8;
    type IntoIter = <&'a mut [u8] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter_mut()
    }
}

impl<const N: usize> Serialize for HeaplessByteBuf<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.bytes)
    }
}

struct HeaplessByteBufVisitor<const N: usize>;

impl<'de, const N: usize> Visitor<'de> for HeaplessByteBufVisitor<N> {
    type Value = HeaplessByteBuf<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("byte array")
    }

    fn visit_seq<V>(self, mut visitor: V) -> Result<HeaplessByteBuf<N>, V::Error>
    where
        V: SeqAccess<'de>,
    {
        let mut bytes = Vec::<u8, N>::new();

        while let Some(b) = visitor.next_element()? {
            let result = bytes.push(b);

            // Push can only fail if the sequence is too large for the Vec
            if result.is_err() {
                let expected: &str = &format!("{N}");
                return Err(V::Error::invalid_length(N + 1, &expected));
            }
        }

        Ok(HeaplessByteBuf::from(bytes))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<HeaplessByteBuf<N>, E>
    where
        E: Error,
    {
        HeaplessByteBuf::try_from_slice(v).map_err(|_| {
            let expected: &str = &format!("{N}");
            Error::invalid_length(v.len(), &expected)
        })
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    fn visit_byte_buf<E>(self, v: alloc::vec::Vec<u8>) -> Result<HeaplessByteBuf<N>, E>
    where
        E: Error,
    {
        let len = v.len();
        HeaplessByteBuf::try_from(v).map_err(|_| {
            let expected: &str = &format!("{N}");
            Error::invalid_length(len, &expected)
        })
    }
}

impl<'de, const N: usize> Deserialize<'de> for HeaplessByteBuf<N> {
    fn deserialize<D>(deserializer: D) -> Result<HeaplessByteBuf<N>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_bytes(HeaplessByteBufVisitor::<N>)
    }
}
