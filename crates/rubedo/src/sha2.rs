//! This module provides extended functionality for the [SHA2](https://crates.io/crates/sha2)
//! crate.



//		Modules

#[cfg(test)]
#[path = "tests/sha2.rs"]
mod tests;



//		Packages

use crate::std::{ByteSized, ByteSizedError, ByteSizedMut, ForceFrom};
use base64::{DecodeError, engine::{Engine as _, general_purpose::STANDARD as BASE64}};
use core::{
	cmp::Ordering,
	convert::TryFrom,
	fmt::{Debug, Display, self},
	str::FromStr,
};
use generic_array::{
	GenericArray,
	typenum::U32,
};
use hex::{FromHexError, self};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};
use std::borrow::Cow;



//		Structs

//		Sha256Hash																
/// A SHA256 hash.
/// 
/// A formalised representation of a SHA256 hash, with support for common
/// conversions, including serialisation and deserialisation using [Serde](https://crates.io/crates/serde).
/// 
/// The hash is stored internally as `[u8; 32]`, which is the correct and most
/// efficient format, as emitted by [`Sha256::digest()`](https://docs.rs/sha2/latest/sha2/type.Sha256.html).
///
/// The conversion to and from a [`String`] defaults to using hex strings rather
/// than base64-encoded strings, because this is more common, due to it being a
/// fixed-length string that is easy to read, verify, and transmit without any
/// compatibility issues. However, base64 conversion functions are also provided
/// for convenience in case that format is preferred.
/// 
#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Sha256Hash {
	//		Private properties													
	/// The SHA256 hash as a fixed-length array of bytes.
	hash: [u8; 32],
}

impl Sha256Hash {
	//		new																	
	/// Creates a new SHA256 hash instance.
	/// 
	/// # Parameters
	/// 
	/// * `data` - The SHA256 hash as any type for which there is a [`From`]
	///            implementation.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::from_bytes()`]
	/// 
	pub fn new<T: Into<Self>>(data: T) -> Self {
		data.into()
	}
}

impl ByteSized<32> for Sha256Hash {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; 32] {
		&self.hash
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; 32] {
		self.hash
	}
	
	//		from_bytes															
	fn from_bytes(bytes: [u8; 32]) -> Self {
		Self { hash: bytes }
	}
	
	//		to_base64															
	fn to_base64(&self) -> String {
		BASE64.encode(self.hash)
	}
	
	//		from_base64															
	fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		Ok(Self::force_from(BASE64.decode(encoded)?))
	}
	
	//		to_hex																
	fn to_hex(&self) -> String {
		hex::encode(self.hash)
	}
	
	//		from_hex															
	fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		Ok(Self::force_from(hex::decode(encoded)?))
	}
	
	//		to_vec																
	fn to_vec(&self) -> Vec<u8> {
		self.hash.to_vec()
	}
}

impl ByteSizedMut<32> for Sha256Hash {
	//		as_mut_bytes														
	fn as_mut_bytes(&mut self) -> &mut [u8; 32] {
		&mut self.hash
	}
	
	//		into_bytes															
	fn into_bytes(self) -> [u8; 32] {
		self.hash
	}
	
	//		into_vec															
	fn into_vec(self) -> Vec<u8> {
		self.hash.into_iter().collect()
	}
}

impl AsMut<[u8; 32]> for Sha256Hash {
	//		as_mut																
	fn as_mut(&mut self) -> &mut [u8; 32] {
		self.as_mut_bytes()
	}
}

impl AsRef<[u8; 32]> for Sha256Hash {
	//		as_ref																
	fn as_ref(&self) -> &[u8; 32] {
		self.as_bytes()
	}
}

impl Debug for Sha256Hash {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

impl Display for Sha256Hash {
	//		fmt																	
	/// Formats the SHA256 hash for display.
	///
	/// This method serialises the SHA256 hash into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::serialize()`]
	/// * [`Sha256Hash::to_base64()`]
	/// 
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

impl From<[u8; 32]> for Sha256Hash {
	//		from																
	/// Converts a [`[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	fn from(b: [u8; 32]) -> Self {
		Self::from_bytes(b)
	}
}

impl From<&[u8; 32]> for Sha256Hash {
	//		from																
	/// Converts a [`&[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	fn from(b: &[u8; 32]) -> Self {
		Self::from_bytes(*b)
	}
}

impl From<GenericArray<u8, U32>> for Sha256Hash {
	//		from																
	/// Converts a [`GenericArray<u8, U32>`](GenericArray) to a [`Sha256Hash`].
	fn from(a: GenericArray<u8, U32>) -> Self {
		Self::from(&a)
	}
}

impl From<&GenericArray<u8, U32>> for Sha256Hash {
	//		from																
	/// Converts a [`GenericArray<u8, U32>`](GenericArray) to a [`Sha256Hash`].
	fn from(a: &GenericArray<u8, U32>) -> Self {
		let mut hash = [0_u8; 32];
		hash.copy_from_slice(a.as_slice());
		Self::from_bytes(hash)
	}
}

impl FromStr for Sha256Hash {
	type Err = ByteSizedError;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
	}
}

impl ForceFrom<&[u8]> for Sha256Hash {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(b: &[u8]) -> Self {
		let mut array = [0_u8; 32];
		let len       = b.len().min(32);
		#[cfg_attr(    feature = "reasons",  allow(clippy::indexing_slicing, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::indexing_slicing))]
		array[..len].copy_from_slice(&b[..len]);
		Self::from(array)
	}
}

impl<const N: usize> ForceFrom<&[u8; N]> for Sha256Hash {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	/// 
	/// Note that if the incoming `[u8; N]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(b: &[u8; N]) -> Self {
		Self::force_from(&b[..])
	}
}

impl ForceFrom<Vec<u8>> for Sha256Hash {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`Sha256Hash`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(v: Vec<u8>) -> Self {
		Self::force_from(&*v)
	}
}

impl ForceFrom<&Vec<u8>> for Sha256Hash {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`Sha256Hash`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(v: &Vec<u8>) -> Self {
		Self::force_from(&**v)
	}
}

impl PartialEq<[u8; 32]> for Sha256Hash {
	//		eq																	
	fn eq(&self, other: &[u8; 32]) -> bool {
		&self.hash == other
	}
}

impl PartialEq<&[u8; 32]> for Sha256Hash {
	//		eq																	
	fn eq(&self, other: &&[u8; 32]) -> bool {
		&&self.hash == other
	}
}

impl Serialize for Sha256Hash {
	//		serialize															
	/// Serialises the SHA256 hash to a [`String`].
	/// 
	/// This method serialises the SHA256 hash into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::deserialize()`]
	/// * [`Sha256Hash::<Display>fmt()`]
	/// * [`Sha256Hash::to_base64()`]
	/// 
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for Sha256Hash {
	//		deserialize															
	/// Deserialises the SHA256 hash from a [`String`].
	/// 
	/// This method deserialises the SHA256 hash from hexadecimal string
	/// representation.
	/// 
	/// # See also
	///
	/// * [`Sha256Hash::deserialize()`]
	/// * [`Sha256Hash::from_base64()`]
	///
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_hex(&string).map_err(D::Error::custom)
	}
}

impl TryFrom<&[u8]> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
		match b.len().cmp(&32) {
			Ordering::Greater => return Err(ByteSizedError::DataTooLong(32)),
			Ordering::Less    => return Err(ByteSizedError::DataTooShort(32)),
			Ordering::Equal   => {},
		}
		Ok(Self::force_from(b))
	}
}

impl TryFrom<&str> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&str`](str) to a [`Sha256Hash`].
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::try_from(hex::decode(s).map_err(|_err| ByteSizedError::InvalidHexString)?)
	}
}

impl TryFrom<String> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`String`] to a [`Sha256Hash`].
	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

impl TryFrom<&String> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&String`](String) to a [`Sha256Hash`].
	fn try_from(s: &String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

impl TryFrom<Box<str>> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [boxed](Box) [string](str) slice to a [`Sha256Hash`].
	fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
		Self::try_from(&*s)
	}
}

impl<'a> TryFrom<Cow<'a, str>> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [clone-on-write](Cow) [string](str) to a [`Sha256Hash`].
	fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
		Self::try_from(s.as_ref())
	}
}

impl TryFrom<Vec<u8>> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`Sha256Hash`].
	fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(&*v)
	}
}

impl TryFrom<&Vec<u8>> for Sha256Hash {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`Sha256Hash`].
	fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(v.as_slice())
	}
}


