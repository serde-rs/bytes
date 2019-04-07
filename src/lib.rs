//! Wrapper types to enable optimized handling of `&[u8]` and `Vec<u8>`.
//!
//! Without specialization, Rust forces Serde to treat `&[u8]` just like any
//! other slice and `Vec<u8>` just like any other vector. In reality this
//! particular slice and vector can often be serialized and deserialized in a
//! more efficient, compact representation in many formats.
//!
//! When working with such a format, you can opt into specialized handling of
//! `&[u8]` by wrapping it in `serde_bytes::Bytes` and `Vec<u8>` by wrapping it
//! in `serde_bytes::ByteBuf`.
//!
//! This crate supports the Serde `with` attribute to enable efficient handling
//! of `&[u8]` and `Vec<u8>` in structs without needing a wrapper type.
//!
//! ```
//! # use serde_derive::{Serialize, Deserialize};
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize)]
//! struct Efficient<'a> {
//!     #[serde(with = "serde_bytes")]
//!     bytes: &'a [u8],
//!
//!     #[serde(with = "serde_bytes")]
//!     byte_buf: Vec<u8>,
//! }
//!
//! #[derive(Serialize, Deserialize)]
//! struct Packet {
//!     #[serde(with = "serde_bytes")]
//!     payload: Vec<u8>,
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/serde_bytes/0.10.5")]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "alloc", feature(alloc))]
#![deny(missing_docs)]

mod bytes;

#[cfg(any(feature = "std", feature = "alloc"))]
mod bytebuf;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

#[cfg(any(feature = "std", feature = "alloc"))]
use serde::{Deserialize, Deserializer};

use serde::Serializer;

pub use self::bytes::Bytes;

#[cfg(any(feature = "std", feature = "alloc"))]
pub use self::bytebuf::ByteBuf;

/// Serde `serialize_with` function to serialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes")]`
/// - `#[serde(serialize_with = "serde_bytes::serialize")]`
///
/// ```
/// # use serde_derive::Serialize;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Efficient<'a> {
///     #[serde(with = "serde_bytes")]
///     bytes: &'a [u8],
///
///     #[serde(with = "serde_bytes")]
///     byte_buf: Vec<u8>,
/// }
/// ```
pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ?Sized + AsRef<[u8]>,
    S: Serializer,
{
    serializer.serialize_bytes(bytes.as_ref())
}

/// Serde `deserialize_with` function to deserialize bytes efficiently.
///
/// This function can be used with either of the following Serde attributes:
///
/// - `#[serde(with = "serde_bytes")]`
/// - `#[serde(deserialize_with = "serde_bytes::deserialize")]`
///
/// ```
/// # use serde_derive::Deserialize;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct Packet {
///     #[serde(with = "serde_bytes")]
///     payload: Vec<u8>,
/// }
/// ```
#[cfg(any(feature = "std", feature = "alloc"))]
pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: From<Vec<u8>>,
    D: Deserializer<'de>,
{
    ByteBuf::deserialize(deserializer).map(|buf| Into::<Vec<u8>>::into(buf).into())
}
