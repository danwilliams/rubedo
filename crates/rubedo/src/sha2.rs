//! This module provides extended functionality for the [SHA2](https://crates.io/crates/sha2)
//! crate.



//		Modules

#[cfg(test)]
#[path = "tests/sha2.rs"]
mod tests;



//		Packages

use crate::{
	std::ForceFrom,
	sugar::s,
};
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
use std::{
	borrow::Cow,
	error::Error,
};



//		Enums

//		HashError																
/// The possible errors that can occur when working with SHA2 hashes.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum HashError {
	/// The supplied data is longer than 32 bytes.
	DataTooLong,
	
	/// The supplied data is shorter than 32 bytes.
	DataTooShort,
	
	/// The supplied string is not in valid base64 format.
	InvalidBase64String,
	
	/// The supplied string is not in valid hexadecimal format.
	InvalidHexString,
}

impl Display for HashError {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match *self {
			Self::DataTooLong         => s!("The supplied data is longer than 32 bytes"),
			Self::DataTooShort        => s!("The supplied data is shorter than 32 bytes"),
			Self::InvalidBase64String => s!("The supplied data is not in valid base64 format"),
			Self::InvalidHexString    => s!("The supplied data is not in valid hexadecimal format"),
		};
		write!(f, "{description}")
	}
}

impl Error for HashError {}



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
	/// * [`Sha256Hash::from_bytes()`]
	/// 
	pub fn new<T: Into<Self>>(data: T) -> Self {
		data.into()
	}
	
	//		as_bytes															
	/// Returns a byte slice of the SHA256 hash's contents.
	/// 
	/// Provides a read-only view of the byte data within the SHA256 hash,
	/// without consuming the data. The returned slice is a reference to the
	/// actual data stored in the hash, not a copy. Because of this, it is not
	/// possible to mutate the contents of the hash through the returned slice.
	/// It does not allocate new memory or change the ownership of the byte
	/// data. This method is useful when you need to work with the bytes of the
	/// hash in a read-only fashion, or when you want to avoid copying the data.
	/// 
	///   - This method returns a slice (`&[u8; 32]`) referencing the bytes of
	///     the hash contents.
	///   - The original hash value remains intact, and can still be used
	///     afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a view into the original memory.
	/// 
	/// Use this method when you need to work with the byte data in a
	/// non-destructive, read-only manner while keeping the original hash
	/// intact.
	///
	/// # See also
	/// 
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::from_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::into_vec()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// * [`Sha256Hash::to_vec()`]
	/// 
	#[must_use]
	pub const fn as_bytes(&self) -> &[u8; 32] {
		&self.hash
	}
	
	//		as_mut_bytes														
	/// Returns a mutable reference to the SHA256 hash's contents.
	/// 
	/// Provides a mutable view of the byte data within the SHA256 hash, without
	/// consuming the data. The returned vector is a reference to the actual
	/// data stored in the hash, not a copy. This method is useful when you need
	/// to work with, and modify, the bytes of the hash directly, without
	/// copying the data.
	/// 
	///   - This method returns a mutable array (`&mut [u8; 32]`) referencing
	///     the bytes of the hash contents.
	///   - The original hash value remains intact, and can still be used
	///     afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a reference to the original memory.
	/// 
	/// Use this method when you need to work directly with the byte data in a
	/// mutable manner.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::from_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::into_vec()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// * [`Sha256Hash::to_vec()`]
	/// 
	pub fn as_mut_bytes(&mut self) -> &mut [u8; 32] {
		&mut self.hash
	}
	
	//		into_bytes															
	/// Returns the SHA256 hash as a fixed-length array of bytes.
	/// 
	/// This consumes the SHA256 hash, without cloning or copying, and returns a
	/// new fixed-length array containing the bytes of the hash. It transfers
	/// ownership of the byte data from the hash to the new array. This method
	/// is useful when you need to move the byte data out of the hash, or when
	/// you want to modify the byte data in-place without affecting the original
	/// hash.
	/// 
	///   - This method consumes the hash contents and returns a `[u8; 32]`
	///     containing its bytes.
	///   - After calling this method, the original hash value is no longer
	///     available for use, because it has been moved.
	/// 
	/// Use this method when you want to consume the hash and obtain ownership
	/// of its byte data in the form of a `[u8; 32]`. This is useful when you
	/// need to modify or move the byte data, or when you want to pass it to
	/// functions that expect a `[u8; 32]`.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::from_bytes()`]
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::into_vec()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// * [`Sha256Hash::to_vec()`]
	/// 
	#[must_use]
	pub const fn into_bytes(self) -> [u8; 32] {
		self.hash
	}
	
	//		to_bytes															
	/// Returns a copy of the SHA256 hash data as a fixed-length array of bytes.
	/// 
	/// This does not consume the SHA256 hash, but clones it. Following Rust's
	/// naming conventions and idioms, this method "converts" the data content
	/// of the hash into a byte representation, in a `[u8; 32]`. (No actual
	/// conversion takes place because the data is already stored internally as
	/// a fixed array of bytes, but this is academic and could change in future,
	/// so "conversion" is implied and expected as a theoretical behaviour.)
	/// Ownership of the cloned and converted byte data is transferred to the
	/// caller, and there are no side effects on the internal state of the
	/// [`Sha256Hash`] instance.
	/// 
	///   - This method returns a `[u8; 32]` array of bytes without consuming
	///     the hash contents.
	///   - The original hash value remains intact, and can still be used
	///     afterward.
	///   - The hash data is copied, and converted/transformed into the output
	///     value returned.
	/// 
	/// Use this method when you need to obtain a copy of the hash's byte data
	/// in the form of a `[u8; 32]`, without consuming the hash itself. This is
	/// useful when you need to pass the byte data to a function that expects a
	/// `[u8; 32]`, or when you want to modify the byte data without affecting
	/// the original hash.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::from_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::into_vec()`]
	/// * [`Sha256Hash::to_vec()`]
	/// 
	#[must_use]
	pub const fn to_bytes(&self) -> [u8; 32] {
		self.hash
	}
	
	//		from_bytes															
	/// Constructs a [`Sha256Hash`] from an array of bytes.
	/// 
	/// This method consumes the input array.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// 
	#[must_use]
	pub const fn from_bytes(bytes: [u8; 32]) -> Self {
		Self { hash: bytes }
	}
	
	//		to_base64															
	/// Returns the SHA256 hash data converted to a base64-encoded [`String`].
	/// 
	/// This does not consume the SHA256 hash, but clones it, as is necessary to
	/// perform the conversion to base64.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::from_base64()`]
	/// 
	#[must_use]
	pub fn to_base64(&self) -> String {
		BASE64.encode(self.hash)
	}
	
	//		from_base64															
	/// Converts a base64-encoded [`String`] to a [`Sha256Hash`].
	/// 
	/// This method does not consume the input string, but clones it, as is
	/// necessary to perform the conversion from [`base64`].
	/// 
	/// # Errors
	/// 
	/// This method will return an error if the input string is not valid
	/// base64. Such an error will be returned as a [`DecodeError`], which is
	/// passed through from the [`base64`] crate.
	/// 
	/// Note that if the incoming data results in a [`Vec<u8>`](Vec) that is too
	/// long to fit, it will be truncated without error or warning. If there is
	/// not enough data, it will be padded with zeroes. If this situation needs
	/// checking, decode from base64 manually and then use `try_from()` instead.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::to_base64()`]
	/// 
	pub fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		let decoded = BASE64.decode(encoded)?;
		Ok(Self::force_from(decoded))
	}
	
	//		to_hex																
	/// Returns the SHA256 hash data converted to a hex-encoded [`String`].
	/// 
	/// This does not consume the SHA256 hash, but clones it, as is necessary to
	/// perform the conversion to hexadecimal representation.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::from_hex()`]
	/// 
	#[must_use]
	pub fn to_hex(&self) -> String {
		hex::encode(self.hash)
	}
	
	//		from_hex															
	/// Converts a hex-encoded [`String`] to a [`Sha256Hash`].
	/// 
	/// This method does not consume the input string, but clones it, as is
	/// necessary to perform the conversion from hexadecimal representation.
	/// 
	/// # Errors
	/// 
	/// This method will return an error if the input string is not in valid
	/// hexadecimal format. Such an error will be returned as a
	/// [`FromHexError`], which is passed through from the [`hex`] crate.
	///
	/// Note that if the incoming data results in a [`Vec<u8>`](Vec) that is too
	/// long to fit, it will be truncated without error or warning. If there is
	/// not enough data, it will be padded with zeroes. If this situation needs
	/// checking, use `try_from()` instead.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::to_hex()`]
	/// 
	pub fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		let decoded = hex::decode(encoded)?;
		Ok(Self::force_from(decoded))
	}
	
	//		into_vec															
	/// Returns the SHA256 hash as a vector of bytes.
	/// 
	/// This consumes the SHA256 hash, and returns a new vector containing the
	/// bytes of the hash. It transfers ownership of the byte data from the hash
	/// to the new vector. This method is useful when you need to move the byte
	/// data out of the hash, for example to pass it to a function that expects
	/// a [`Vec<u8>`](Vec). Note, however, that because vectors are
	/// heap-allocated and can grow dynamically, whereas arrays are fixed-size
	/// and stack-allocated, there isn't a direct, zero-copy way to consume an
	/// array into a [`Vec`], and so this process does involve copying the data.
	/// 
	///   - This method consumes the hash contents and returns a [`Vec<u8>`](Vec)
	///     containing its bytes.
	///   - After calling this method, the original hash value is no longer
	///     available for use, because it has been moved.
	///   - Transforms the hash into a vector of bytes, but does copy the data.
	/// 
	/// Use this method when you want to consume the hash and obtain ownership
	/// of its byte data in the form of a [`Vec<u8>`](Vec). This is useful when
	/// you need to modify or move the byte data.
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// * [`Sha256Hash::to_vec()`]
	/// 
	#[must_use]
	pub fn into_vec(self) -> Vec<u8> {
		self.hash.into_iter().collect()
	}
	
	//		to_vec																
	/// Returns a copy of the SHA256 hash data converted to a vector of bytes.
	/// 
	/// This does not consume the SHA256 hash, but clones it. Following Rust's
	/// naming conventions and idioms, this method converts the data content of
	/// the hash into a byte representation, in a [`Vec<u8>`](Vec). Ownership of
	/// the cloned and converted byte data is transferred to the caller, and
	/// there are no side effects on the internal state of the [`Sha256Hash`]
	/// instance.
	/// 
	///   - This method returns a [`Vec<u8>`](Vec) vector of bytes without
	///     consuming the hash contents.
	///   - The original hash value remains intact, and can still be used
	///     afterward.
	///   - The hash data is copied, and converted/transformed into the output
	///     value returned.
	/// 
	/// Use this method when you need to obtain a copy of the hash's byte data
	/// in the form of a [`Vec<u8>`](Vec), without consuming the hash itself.
	/// This is useful when you need to pass the byte data to a function that
	/// expects a [`Vec<u8>`](Vec).
	/// 
	/// # See also
	/// 
	/// * [`Sha256Hash::as_bytes()`]
	/// * [`Sha256Hash::as_mut_bytes()`]
	/// * [`Sha256Hash::into_bytes()`]
	/// * [`Sha256Hash::into_vec()`]
	/// * [`Sha256Hash::to_bytes()`]
	/// 
	#[must_use]
	pub fn to_vec(&self) -> Vec<u8> {
		self.hash.to_vec()
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
	type Err = HashError;
	
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
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`Sha256Hash`].
	fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
		match b.len().cmp(&32) {
			Ordering::Greater => return Err(HashError::DataTooLong),
			Ordering::Less    => return Err(HashError::DataTooShort),
			Ordering::Equal   => {},
		}
		Ok(Self::force_from(b))
	}
}

impl TryFrom<&str> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`&str`](str) to a [`Sha256Hash`].
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::try_from(hex::decode(s).map_err(|_err| HashError::InvalidHexString)?)
	}
}

impl TryFrom<String> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`String`] to a [`Sha256Hash`].
	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(&s)
	}
}

impl TryFrom<&String> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`&String`](String) to a [`Sha256Hash`].
	fn try_from(s: &String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

impl TryFrom<Box<str>> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [boxed](Box) [string](str) slice to a [`Sha256Hash`].
	fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
		Self::try_from(&*s)
	}
}

impl<'a> TryFrom<Cow<'a, str>> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [clone-on-write](Cow) [string](str) to a [`Sha256Hash`].
	fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
		Self::try_from(s.as_ref())
	}
}

impl TryFrom<Vec<u8>> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`Sha256Hash`].
	fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(&*v)
	}
}

impl TryFrom<&Vec<u8>> for Sha256Hash {
	type Error = HashError;
	
	//		try_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`Sha256Hash`].
	fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(v.as_slice())
	}
}


