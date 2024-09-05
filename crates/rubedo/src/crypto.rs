//! This module provides extended functionality for the [SHA2](https://crates.io/crates/sha2)
//! crate.



//		Modules

#[cfg(test)]
#[path = "tests/crypto.rs"]
mod tests;



//		Packages

use crate::std::{ByteSized, ByteSizedError, ByteSizedFull, ByteSizedMut, ForceFrom};
use base64::{DecodeError, engine::{Engine as _, general_purpose::STANDARD as BASE64}};
use core::{
	cmp::Ordering,
	convert::TryFrom,
	fmt::{Debug, Display, self},
	hash::{Hash, Hasher},
	ops::Deref,
	str::FromStr,
};
use digest::Digest;
use sha2::{Sha256, Sha512};
use ed25519_dalek::{SigningKey as RealSigningKey, VerifyingKey as RealVerifyingKey};
use generic_array::{
	ArrayLength,
	GenericArray,
	typenum::{U32, U64, Unsigned},
};
use hex::{FromHexError, self};
use rand_core::CryptoRngCore;
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

//󰭅		Sha256Hash																
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

//󰭅		Hashed																	
impl Hashed for Sha256Hash {
	type Algorithm = Sha256;
	type OutputSize = U32;
	
	//		from_digest															
	fn from_digest(output: GenericArray<u8, Self::OutputSize>) -> Self {
		let mut hash = [0_u8; 32];
		hash.copy_from_slice(output.as_slice());
		Self::from_bytes(hash)
	}
}

//		Sha512Hash																
/// A SHA512 hash.
/// 
/// A formalised representation of a SHA512 hash, with support for common
/// conversions, including serialisation and deserialisation using [Serde](https://crates.io/crates/serde).
/// 
/// The hash is stored internally as `[u8; 64]`, which is the correct and most
/// efficient format, as emitted by [`Sha512::digest()`](https://docs.rs/sha2/latest/sha2/type.Sha512.html).
///
/// The conversion to and from a [`String`] defaults to using hex strings rather
/// than base64-encoded strings, because this is more common, due to it being a
/// fixed-length string that is easy to read, verify, and transmit without any
/// compatibility issues. However, base64 conversion functions are also provided
/// for convenience in case that format is preferred.
/// 
#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Sha512Hash {
	//		Private properties													
	/// The SHA512 hash as a fixed-length array of bytes.
	hash: [u8; 64],
}

//󰭅		Sha512Hash																
impl Sha512Hash {
	//		new																	
	/// Creates a new SHA512 hash instance.
	/// 
	/// # Parameters
	/// 
	/// * `data` - The SHA512 hash as any type for which there is a [`From`]
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

//󰭅		Default																	
impl Default for Sha512Hash {
	//		default																
	fn default() -> Self {
		Self { hash: [0; 64] }
	}
}

//󰭅		Hashed																	
impl Hashed for Sha512Hash {
	type Algorithm = Sha512;
	type OutputSize = U64;
	
	//		from_digest															
	fn from_digest(output: GenericArray<u8, Self::OutputSize>) -> Self {
		let mut hash = [0_u8; 64];
		hash.copy_from_slice(output.as_slice());
		Self::from_bytes(hash)
	}
}

//		impl_traits_for_hashed_type												
/// Implements common traits for [`Hashed`] types.
macro_rules! impl_traits_for_hashed_type { ($t:ty, $o:ty, $s:expr) => {
//󰭅		ByteSized																
impl ByteSized<$s> for $t {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; $s] {
		&self.hash
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; $s] {
		self.hash
	}
	
	//		from_bytes															
	fn from_bytes(bytes: [u8; $s]) -> Self {
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

//󰭅		ByteSizedMut															
impl ByteSizedMut<$s> for $t {
	//		as_mut_bytes														
	fn as_mut_bytes(&mut self) -> &mut [u8; $s] {
		&mut self.hash
	}
	
	//		into_bytes															
	fn into_bytes(self) -> [u8; $s] {
		self.hash
	}
	
	//		into_vec															
	fn into_vec(self) -> Vec<u8> {
		self.hash.into_iter().collect()
	}
}

//󰭅		AsMut [u8; $s]															
impl AsMut<[u8; $s]> for $t {
	//		as_mut																
	fn as_mut(&mut self) -> &mut [u8; $s] {
		self.as_mut_bytes()
	}
}

//󰭅		AsRef [u8; $s]															
impl AsRef<[u8; $s]> for $t {
	//		as_ref																
	fn as_ref(&self) -> &[u8; $s] {
		self.as_bytes()
	}
}

//󰭅		Debug																	
impl Debug for $t {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		Display																	
impl Display for $t {
	//		fmt																	
	/// Formats the SHA256 hash for display.
	///
	/// This method serialises the SHA256 hash into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`$t::serialize()`]
	/// * [`$t::to_base64()`]
	/// 
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		From [u8; $s]															
impl From<[u8; $s]> for $t {
	//		from																
	/// Converts a [`[u8; $s]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`$t`].
	fn from(b: [u8; $s]) -> Self {
		Self::from_bytes(b)
	}
}

//󰭅		From &[u8; $s]															
impl From<&[u8; $s]> for $t {
	//		from																
	/// Converts a [`&[u8; $s]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`$t`].
	fn from(b: &[u8; $s]) -> Self {
		Self::from_bytes(*b)
	}
}

//󰭅		From GenericArray<u8, $o>												
impl From<GenericArray<u8, $o>> for $t {
	//		from																
	/// Converts a [`GenericArray<u8, $o>`](GenericArray) to a [`$t`].
	fn from(a: GenericArray<u8, $o>) -> Self {
		Self::from(&a)
	}
}

//󰭅		From &GenericArray<u8, $o>												
impl From<&GenericArray<u8, $o>> for $t {
	//		from																
	/// Converts a [`GenericArray<u8, $o>`](GenericArray) to a [`$t`].
	fn from(a: &GenericArray<u8, $o>) -> Self {
		let mut hash = [0_u8; $s];
		hash.copy_from_slice(a.as_slice());
		Self::from_bytes(hash)
	}
}

//󰭅		FromStr																	
impl FromStr for $t {
	type Err = ByteSizedError;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
	}
}

//󰭅		ForceFrom &[u8]															
impl ForceFrom<&[u8]> for $t {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`$t`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(b: &[u8]) -> Self {
		let mut array = [0_u8; $s];
		let len       = b.len().min($s);
		#[cfg_attr(    feature = "reasons",  allow(clippy::indexing_slicing, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::indexing_slicing))]
		array[..len].copy_from_slice(&b[..len]);
		Self::from(array)
	}
}

//󰭅		ForceFrom &[u8; N]														
impl<const N: usize> ForceFrom<&[u8; N]> for $t {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`$t`].
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

//󰭅		ForceFrom Vec<u8>														
impl ForceFrom<Vec<u8>> for $t {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`$t`].
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

//󰭅		ForceFrom &Vec<u8>														
impl ForceFrom<&Vec<u8>> for $t {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`$t`].
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

//󰭅		PartialEq [u8; $s]														
impl PartialEq<[u8; $s]> for $t {
	//		eq																	
	fn eq(&self, other: &[u8; $s]) -> bool {
		&self.hash == other
	}
}

//󰭅		PartialEq &[u8; $s]														
impl PartialEq<&[u8; $s]> for $t {
	//		eq																	
	fn eq(&self, other: &&[u8; $s]) -> bool {
		&&self.hash == other
	}
}

//󰭅		Serialize																
impl Serialize for $t {
	//		serialize															
	/// Serialises the SHA256 hash to a [`String`].
	/// 
	/// This method serialises the SHA256 hash into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`$t::deserialize()`]
	/// * [`$t::<Display>fmt()`]
	/// * [`$t::to_base64()`]
	/// 
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

//󰭅		Deserialize																
impl<'de> Deserialize<'de> for $t {
	//		deserialize															
	/// Deserialises the SHA256 hash from a [`String`].
	/// 
	/// This method deserialises the SHA256 hash from hexadecimal string
	/// representation.
	/// 
	/// # See also
	///
	/// * [`$t::deserialize()`]
	/// * [`$t::from_base64()`]
	///
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_hex(&string).map_err(D::Error::custom)
	}
}

//󰭅		TryFrom &[u8]															
impl TryFrom<&[u8]> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`$t`].
	fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
		match b.len().cmp(&$s) {
			Ordering::Greater => return Err(ByteSizedError::DataTooLong($s)),
			Ordering::Less    => return Err(ByteSizedError::DataTooShort($s)),
			Ordering::Equal   => {},
		}
		Ok(Self::force_from(b))
	}
}

//󰭅		TryFrom &str															
impl TryFrom<&str> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&str`](str) to a [`$t`].
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::try_from(hex::decode(s).map_err(|_err| ByteSizedError::InvalidHexString)?)
	}
}

//󰭅		TryFrom String															
impl TryFrom<String> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`String`] to a [`$t`].
	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom &String															
impl TryFrom<&String> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&String`](String) to a [`$t`].
	fn try_from(s: &String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom Box<str>														
impl TryFrom<Box<str>> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [boxed](Box) [string](str) slice to a [`$t`].
	fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
		Self::try_from(&*s)
	}
}

//󰭅		TryFrom Cow<str>														
impl<'a> TryFrom<Cow<'a, str>> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [clone-on-write](Cow) [string](str) to a [`$t`].
	fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
		Self::try_from(s.as_ref())
	}
}

//󰭅		TryFrom Vec<u8>															
impl TryFrom<Vec<u8>> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`$t`].
	fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(&*v)
	}
}

//󰭅		TryFrom &Vec<u8>														
impl TryFrom<&Vec<u8>> for $t {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`$t`].
	fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(v.as_slice())
	}
}
};}

impl_traits_for_hashed_type!(Sha256Hash, U32, 32);
impl_traits_for_hashed_type!(Sha512Hash, U64, 64);

//		SigningKey																
/// An ed25519 signing key which can be used to produce signatures.
/// 
/// This is a wrapper around [`ed25519_dalek::SigningKey`], which provides
/// additional functionality, including serialisation and deserialisation using
/// [Serde](https://crates.io/crates/serde), via the implementation of the
/// [`ByteSized`] and [`ByteSizedFull`] traits.
/// 
/// # See also
/// 
/// * [`ed25519_dalek::SigningKey`]
/// 
#[derive(Clone, Eq, PartialEq)]
pub struct SigningKey {
	//		Private properties													
	/// The actual signing key.
	key: RealSigningKey,
}

//󰭅		SigningKey																
impl SigningKey {
	//		generate															
	/// Generates an ed25519 [`SigningKey`].
	/// 
	/// This function exists to return the wrapper type [`SigningKey`] rather
	/// than the inner type [`ed25519_dalek::SigningKey`].
	/// 
	#[must_use]
	pub fn generate<R: CryptoRngCore + ?Sized>(csprng: &mut R) -> Self {
		Self::from(RealSigningKey::generate(csprng))
	}
	
	//		into_inner															
	/// Consumes the [`SigningKey`] and returns the inner
	/// [`ed25519_dalek::SigningKey`].
	#[must_use]
	pub fn into_inner(self) -> RealSigningKey {
		self.key
	}
	
	//		verifying_key														
	/// Returns the [`VerifyingKey`] for this [`SigningKey`].
	/// 
	/// This function exists to return the wrapper type [`VerifyingKey`] rather
	/// than the inner type [`ed25519_dalek::VerifyingKey`].
	/// 
	#[must_use]
	pub fn verifying_key(&self) -> VerifyingKey {
		VerifyingKey::from(&self.key.verifying_key())
	}
}

//󰭅		ByteSized																
impl ByteSized<32> for SigningKey {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; 32] {
		self.key.as_bytes()
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; 32] {
		self.key.to_bytes()
	}
	
	//		from_bytes															
	fn from_bytes(bytes: [u8; 32]) -> Self {
		Self { key: RealSigningKey::from_bytes(&bytes) }
	}
	
	//		to_base64															
	fn to_base64(&self) -> String {
		self.key.to_base64()
	}
	
	//		from_base64															
	fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		RealSigningKey::from_base64(encoded).map(|key| Self { key })
	}
	
	//		to_hex																
	fn to_hex(&self) -> String {
		self.key.to_hex()
	}
	
	//		from_hex															
	fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		RealSigningKey::from_hex(encoded).map(|key| Self { key })
	}
	
	//		to_vec																
	fn to_vec(&self) -> Vec<u8> {
		self.key.to_vec()
	}
}

//󰭅		ByteSizedFull															
impl ByteSizedFull<32> for SigningKey {}

//󰭅		AsRef [u8; 32]															
impl AsRef<[u8; 32]> for SigningKey {
	//		as_ref																
	fn as_ref(&self) -> &[u8; 32] {
		self.as_bytes()
	}
}

//󰭅		Debug																	
impl Debug for SigningKey {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		Default																	
impl Default for SigningKey {
	//		default																
	fn default() -> Self {
		Self { key: RealSigningKey::from_bytes(&[0; 32]) }
	}
}

//󰭅		Deref																	
impl Deref for SigningKey {
    type Target = RealSigningKey;

	//		deref																
    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

//󰭅		Display																	
impl Display for SigningKey {
	//		fmt																	
	/// Formats the signing key for display.
	///
	/// This method serialises the signing key into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`SigningKey::serialize()`]
	/// * [`SigningKey::to_base64()`]
	/// 
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		From RealSigningKey														
impl From<RealSigningKey> for SigningKey {
	//		from																
	/// Converts a [`ed25519_dalek::SigningKey`] to a [`SigningKey`].
	fn from(key: RealSigningKey) -> Self {
		Self { key }
	}
}

//󰭅		From &RealSigningKey													
impl From<&RealSigningKey> for SigningKey {
	//		from																
	/// Converts a [`&ed25519_dalek::SigningKey`](ed25519_dalek::SigningKey) to
	/// a [`SigningKey`].
	fn from(key: &RealSigningKey) -> Self {
		Self { key: key.clone() }
	}
}

//󰭅		From [u8; 32]															
impl From<[u8; 32]> for SigningKey {
	//		from																
	/// Converts a [`[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`SigningKey`].
	fn from(b: [u8; 32]) -> Self {
		Self::from_bytes(b)
	}
}

//󰭅		From &[u8; 32]															
impl From<&[u8; 32]> for SigningKey {
	//		from																
	/// Converts a [`&[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`SigningKey`].
	fn from(b: &[u8; 32]) -> Self {
		Self::from_bytes(*b)
	}
}

//󰭅		FromStr																	
impl FromStr for SigningKey {
	type Err = ByteSizedError;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
	}
}

//󰭅		ForceFrom &[u8]															
impl ForceFrom<&[u8]> for SigningKey {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`SigningKey`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8]) -> Self {
		Self { key: RealSigningKey::force_from(value) }
	}
}

//󰭅		ForceFrom &[u8; N]														
impl<const N: usize> ForceFrom<&[u8; N]> for SigningKey {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`SigningKey`].
	/// 
	/// Note that if the incoming `[u8; N]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8; N]) -> Self {
		Self::force_from(&value[..])
	}
}

//󰭅		ForceFrom Vec<u8>														
impl ForceFrom<Vec<u8>> for SigningKey {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`SigningKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: Vec<u8>) -> Self {
		Self::force_from(&*value)
	}
}

//󰭅		ForceFrom &Vec<u8>														
impl ForceFrom<&Vec<u8>> for SigningKey {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`SigningKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &Vec<u8>) -> Self {
		Self::force_from(&**value)
	}
}

//󰭅		Hash																	
impl Hash for SigningKey {
	//		hash																
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.key.as_bytes().hash(state);
	}
}

//󰭅		Serialize																
impl Serialize for SigningKey {
	//		serialize															
	/// Serialises the signing key to a [`String`].
	/// 
	/// This method serialises the signing key into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`SigningKey::deserialize()`]
	/// * [`SigningKey::<Display>fmt()`]
	/// * [`SigningKey::to_base64()`]
	/// 
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

//󰭅		Deserialize																
impl<'de> Deserialize<'de> for SigningKey {
	//		deserialize															
	/// Deserialises the signing key from a [`String`].
	/// 
	/// This method deserialises the signing key from hexadecimal string
	/// representation.
	/// 
	/// # See also
	///
	/// * [`SigningKey::deserialize()`]
	/// * [`SigningKey::from_base64()`]
	///
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_hex(&string).map_err(D::Error::custom)
	}
}

//󰭅		TryFrom &[u8]															
impl TryFrom<&[u8]> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`SigningKey`].
	fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
		match b.len().cmp(&32) {
			Ordering::Greater => return Err(ByteSizedError::DataTooLong(32)),
			Ordering::Less    => return Err(ByteSizedError::DataTooShort(32)),
			Ordering::Equal   => {},
		}
		Ok(Self::force_from(b))
	}
}

//󰭅		TryFrom &str															
impl TryFrom<&str> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&str`](str) to a [`SigningKey`].
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::try_from(hex::decode(s).map_err(|_err| ByteSizedError::InvalidHexString)?)
	}
}

//󰭅		TryFrom String															
impl TryFrom<String> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`String`] to a [`SigningKey`].
	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom &String															
impl TryFrom<&String> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&String`](String) to a [`SigningKey`].
	fn try_from(s: &String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom Box<str>														
impl TryFrom<Box<str>> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [boxed](Box) [string](str) slice to a [`SigningKey`].
	fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
		Self::try_from(&*s)
	}
}

//󰭅		TryFrom Cow<str>														
impl<'a> TryFrom<Cow<'a, str>> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [clone-on-write](Cow) [string](str) to a [`SigningKey`].
	fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
		Self::try_from(s.as_ref())
	}
}

//󰭅		TryFrom Vec<u8>															
impl TryFrom<Vec<u8>> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`SigningKey`].
	fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(&*v)
	}
}

//󰭅		TryFrom &Vec<u8>														
impl TryFrom<&Vec<u8>> for SigningKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`SigningKey`].
	fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(v.as_slice())
	}
}

//		VerifyingKey															
/// An ed25519 verifying key which can be used to produce signatures.
/// 
/// This is a wrapper around [`ed25519_dalek::VerifyingKey`], which provides
/// additional functionality, including serialisation and deserialisation using
/// [Serde](https://crates.io/crates/serde), via the implementation of the
/// [`ByteSized`] and [`ByteSizedFull`] traits.
/// 
/// # See also
/// 
/// * [`ed25519_dalek::VerifyingKey`]
/// 
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VerifyingKey {
	//		Private properties													
	/// The actual verifying key.
	key: RealVerifyingKey,
}

//󰭅		VerifyingKey															
impl VerifyingKey {
	//		into_inner															
	/// Consumes the [`VerifyingKey`] and returns the inner
	/// [`ed25519_dalek::VerifyingKey`].
	#[must_use]
	pub const fn into_inner(self) -> RealVerifyingKey {
		self.key
	}
}

//󰭅		ByteSized																
impl ByteSized<32> for VerifyingKey {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; 32] {
		self.key.as_bytes()
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; 32] {
		self.key.to_bytes()
	}
	
	//		from_bytes															
	/// Converts a `[u8; 32]` to a [`VerifyingKey`].
	/// 
	/// Note that this is a direct conversion, and does not check the validity
	/// of the bytes. If the bytes are not a valid verifying key, the key will
	/// be created as empty. To check the validity of the bytes, use
	/// [`VerifyingKey::from_bytes()`](RealVerifyingKey::from_bytes()) instead.
	/// 
	fn from_bytes(bytes: [u8; 32]) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::option_if_let_else,
			reason = "Using map_or_else() here would not be as clear, and no more concise"
		))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::option_if_let_else))]
		match RealVerifyingKey::from_bytes(&bytes) {
			Ok(key) => Self { key },
			Err(_)  => Self::default(),
		}
	}
	
	//		to_base64															
	fn to_base64(&self) -> String {
		self.key.to_base64()
	}
	
	//		from_base64															
	fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		RealVerifyingKey::from_base64(encoded).map(|key| Self { key })
	}
	
	//		to_hex																
	fn to_hex(&self) -> String {
		self.key.to_hex()
	}
	
	//		from_hex															
	fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		RealVerifyingKey::from_hex(encoded).map(|key| Self { key })
	}
	
	//		to_vec																
	fn to_vec(&self) -> Vec<u8> {
		self.key.to_vec()
	}
}

//󰭅		ByteSizedFull															
impl ByteSizedFull<32> for VerifyingKey {}

//󰭅		AsRef [u8; 32]															
impl AsRef<[u8; 32]> for VerifyingKey {
	//		as_ref																
	fn as_ref(&self) -> &[u8; 32] {
		self.as_bytes()
	}
}

//󰭅		Debug																	
impl Debug for VerifyingKey {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		Default																	
impl Default for VerifyingKey {
	//		default																
	fn default() -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self { key: RealVerifyingKey::from_bytes(&[0; 32]).unwrap() }
	}
}

//󰭅		Deref																	
impl Deref for VerifyingKey {
    type Target = RealVerifyingKey;

	//		deref																
    fn deref(&self) -> &Self::Target {
        &self.key
    }
}

//󰭅		Display																	
impl Display for VerifyingKey {
	//		fmt																	
	/// Formats the verifying key for display.
	///
	/// This method serialises the verifying key into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`VerifyingKey::serialize()`]
	/// * [`VerifyingKey::to_base64()`]
	/// 
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.to_hex())
	}
}

//󰭅		From RealVerifyingKey													
impl From<RealVerifyingKey> for VerifyingKey {
	//		from																
	/// Converts a [`ed25519_dalek::VerifyingKey`] to a [`VerifyingKey`].
	fn from(key: RealVerifyingKey) -> Self {
		Self { key }
	}
}

//󰭅		From &RealVerifyingKey													
impl From<&RealVerifyingKey> for VerifyingKey {
	//		from																
	/// Converts a [`&ed25519_dalek::VerifyingKey`](ed25519_dalek::VerifyingKey)
	/// to a [`VerifyingKey`].
	fn from(key: &RealVerifyingKey) -> Self {
		Self { key: *key }
	}
}

//󰭅		From [u8; 32]															
impl From<[u8; 32]> for VerifyingKey {
	//		from																
	/// Converts a [`[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`VerifyingKey`].
	fn from(b: [u8; 32]) -> Self {
		Self::from_bytes(b)
	}
}

//󰭅		From &[u8; 32]															
impl From<&[u8; 32]> for VerifyingKey {
	//		from																
	/// Converts a [`&[u8; 32]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`VerifyingKey`].
	fn from(b: &[u8; 32]) -> Self {
		Self::from_bytes(*b)
	}
}

//󰭅		FromStr																	
impl FromStr for VerifyingKey {
	type Err = ByteSizedError;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Self::try_from(s)
	}
}

//󰭅		ForceFrom &[u8]															
impl ForceFrom<&[u8]> for VerifyingKey {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`VerifyingKey`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8]) -> Self {
		Self { key: RealVerifyingKey::force_from(value) }
	}
}

//󰭅		ForceFrom &[u8; N]														
impl<const N: usize> ForceFrom<&[u8; N]> for VerifyingKey {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`VerifyingKey`].
	/// 
	/// Note that if the incoming `[u8; N]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8; N]) -> Self {
		Self::force_from(&value[..])
	}
}

//󰭅		ForceFrom Vec<u8>														
impl ForceFrom<Vec<u8>> for VerifyingKey {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`VerifyingKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: Vec<u8>) -> Self {
		Self::force_from(&*value)
	}
}

//󰭅		ForceFrom &Vec<u8>														
impl ForceFrom<&Vec<u8>> for VerifyingKey {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`VerifyingKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &Vec<u8>) -> Self {
		Self::force_from(&**value)
	}
}

//󰭅		Hash																	
impl Hash for VerifyingKey {
	//		hash																
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.key.as_bytes().hash(state);
	}
}

//󰭅		Serialize																
impl Serialize for VerifyingKey {
	//		serialize															
	/// Serialises the verifying key to a [`String`].
	/// 
	/// This method serialises the verifying key into hexadecimal string
	/// representation.
	/// 
	/// # See also
	/// 
	/// * [`VerifyingKey::deserialize()`]
	/// * [`VerifyingKey::<Display>fmt()`]
	/// * [`VerifyingKey::to_base64()`]
	/// 
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

//󰭅		Deserialize																
impl<'de> Deserialize<'de> for VerifyingKey {
	//		deserialize															
	/// Deserialises the verifying key from a [`String`].
	/// 
	/// This method deserialises the verifying key from hexadecimal string
	/// representation.
	/// 
	/// # See also
	///
	/// * [`VerifyingKey::deserialize()`]
	/// * [`VerifyingKey::from_base64()`]
	///
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		Self::from_hex(&string).map_err(D::Error::custom)
	}
}

//󰭅		TryFrom &[u8]															
impl TryFrom<&[u8]> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`VerifyingKey`].
	fn try_from(b: &[u8]) -> Result<Self, Self::Error> {
		match b.len().cmp(&32) {
			Ordering::Greater => return Err(ByteSizedError::DataTooLong(32)),
			Ordering::Less    => return Err(ByteSizedError::DataTooShort(32)),
			Ordering::Equal   => {},
		}
		Ok(Self::force_from(b))
	}
}

//󰭅		TryFrom &str															
impl TryFrom<&str> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&str`](str) to a [`VerifyingKey`].
	fn try_from(s: &str) -> Result<Self, Self::Error> {
		Self::try_from(hex::decode(s).map_err(|_err| ByteSizedError::InvalidHexString)?)
	}
}

//󰭅		TryFrom String															
impl TryFrom<String> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`String`] to a [`VerifyingKey`].
	fn try_from(s: String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom &String															
impl TryFrom<&String> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&String`](String) to a [`VerifyingKey`].
	fn try_from(s: &String) -> Result<Self, Self::Error> {
		Self::try_from(s.as_str())
	}
}

//󰭅		TryFrom Box<str>														
impl TryFrom<Box<str>> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [boxed](Box) [string](str) slice to a [`VerifyingKey`].
	fn try_from(s: Box<str>) -> Result<Self, Self::Error> {
		Self::try_from(&*s)
	}
}

//󰭅		TryFrom Cow<str>														
impl<'a> TryFrom<Cow<'a, str>> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [clone-on-write](Cow) [string](str) to a [`VerifyingKey`].
	fn try_from(s: Cow<'a, str>) -> Result<Self, Self::Error> {
		Self::try_from(s.as_ref())
	}
}

//󰭅		TryFrom Vec<u8>															
impl TryFrom<Vec<u8>> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`VerifyingKey`].
	fn try_from(v: Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(&*v)
	}
}

//󰭅		TryFrom &Vec<u8>														
impl TryFrom<&Vec<u8>> for VerifyingKey {
	type Error = ByteSizedError;
	
	//		try_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`VerifyingKey`].
	fn try_from(v: &Vec<u8>) -> Result<Self, Self::Error> {
		Self::try_from(v.as_slice())
	}
}



//		Traits

//§		Hashed																	
/// This trait provides a formal representation of actual hash values.
/// 
/// This trait is called `Hashed` because Rust already has a trait called
/// [`Hash`], which is used for hashing keys in hash maps. To avoid confusion,
/// this trait is called `Hashed` which hopefully makes it clear that it
/// represents the result of a hashing algorithm, and not the hashable type or
/// the algorithm itself.
/// 
pub trait Hashed: Sized {
	/// The hashing algorithm to use produce the hash.
	type Algorithm: Digest<OutputSize = Self::OutputSize> + Send;
	
	/// The output size of the hashing algorithm.
	type OutputSize: ArrayLength<u8> + Unsigned;
	
	//		from_digest															
	/// Converts the output of the hashing algorithm to the [`Hashed`] type.
	/// 
	/// # Parameters
	/// 
	/// * `output` - The output of the hashing algorithm, taken as input here.
	/// 
	fn from_digest(output: GenericArray<u8, Self::OutputSize>) -> Self;
}

//§		SigningKeyExt															
/// This trait provides additional functionality to
/// [`ed25519_dalek::SigningKey`].
/// 
/// At present, this trait specifies the implementation of [`ByteSized`] to
/// apply to [`ed25519_dalek::SigningKey`]. When wishing to use the original,
/// "real" type, which is [`ed25519_dalek::SigningKey`], this trait can be
/// brought into scope to extend it with the additional functionality provided.
/// However, if the full range of functionality provided by [`ByteSizedFull`] is
/// required, then the [`SigningKey`] type should be used instead, as this wraps
/// the original type and provides the full range of functionality.
/// 
/// The conversion to and from a [`String`] defaults to using hex strings rather
/// than base64-encoded strings, because this is more common, due to it being a
/// fixed-length string that is easy to read, verify, and transmit without any
/// compatibility issues. However, base64 conversion functions are also provided
/// for convenience in case that format is preferred.
/// 
pub trait SigningKeyExt: ByteSized<32> {}

//󰭅		SigningKeyExt															
impl SigningKeyExt for RealSigningKey {}

//󰭅		ByteSized																
impl ByteSized<32> for RealSigningKey {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; 32] {
		self.as_bytes()
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; 32] {
		self.to_bytes()
	}
	
	//		from_bytes															
	fn from_bytes(bytes: [u8; 32]) -> Self {
		Self::from_bytes(&bytes)
	}
	
	//		to_base64															
	fn to_base64(&self) -> String {
		BASE64.encode(self.as_bytes())
	}
	
	//		from_base64															
	fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		Ok(Self::force_from(BASE64.decode(encoded)?))
	}
	
	//		to_hex																
	fn to_hex(&self) -> String {
		hex::encode(self.as_bytes())
	}
	
	//		from_hex															
	fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		Ok(Self::force_from(hex::decode(encoded)?))
	}
	
	//		to_vec																
	fn to_vec(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}

//󰭅		ForceFrom &[u8]															
impl ForceFrom<&[u8]> for RealSigningKey {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`ed25519_dalek::SigningKey`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8]) -> Self {
		let mut array = [0_u8; 32];
		let len       = value.len().min(32);
		#[cfg_attr(    feature = "reasons",  allow(clippy::indexing_slicing, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::indexing_slicing))]
		array[..len].copy_from_slice(&value[..len]);
		Self::from(array)
	}
}

//󰭅		ForceFrom &[u8; N]														
impl<const N: usize> ForceFrom<&[u8; N]> for RealSigningKey {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`ed25519_dalek::SigningKey`].
	/// 
	/// Note that if the incoming `[u8; N]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8; N]) -> Self {
		Self::force_from(&value[..])
	}
}

//󰭅		ForceFrom Vec<u8>														
impl ForceFrom<Vec<u8>> for RealSigningKey {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`ed25519_dalek::SigningKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: Vec<u8>) -> Self {
		Self::force_from(&*value)
	}
}

//󰭅		ForceFrom &Vec<u8>														
impl ForceFrom<&Vec<u8>> for RealSigningKey {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`ed25519_dalek::SigningKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &Vec<u8>) -> Self {
		Self::force_from(&**value)
	}
}

//§		VerifyingKeyExt															
/// This trait provides additional functionality to
/// [`ed25519_dalek::VerifyingKey`].
/// 
/// At present, this trait specifies the implementation of [`ByteSized`] to
/// apply to [`ed25519_dalek::VerifyingKey`]. When wishing to use the original,
/// "real" type, which is [`ed25519_dalek::VerifyingKey`], this trait can be
/// brought into scope to extend it with the additional functionality provided.
/// However, if the full range of functionality provided by [`ByteSizedFull`] is
/// required, then the [`VerifyingKey`] type should be used instead, as this wraps
/// the original type and provides the full range of functionality.
/// 
/// The conversion to and from a [`String`] defaults to using hex strings rather
/// than base64-encoded strings, because this is more common, due to it being a
/// fixed-length string that is easy to read, verify, and transmit without any
/// compatibility issues. However, base64 conversion functions are also provided
/// for convenience in case that format is preferred.
/// 
pub trait VerifyingKeyExt: ByteSized<32> {}

//󰭅		VerifyingKeyExt															
impl VerifyingKeyExt for RealVerifyingKey {}

//󰭅		ByteSized																
impl ByteSized<32> for RealVerifyingKey {
	//		as_bytes															
	fn as_bytes(&self) -> &[u8; 32] {
		self.as_bytes()
	}
	
	//		to_bytes															
	fn to_bytes(&self) -> [u8; 32] {
		self.to_bytes()
	}
	
	//		from_bytes															
	/// Converts a `[u8; 32]` to a [`ed25519_dalek::VerifyingKey`](RealVerifyingKey).
	/// 
	/// Note that this is a direct conversion, and does not check the validity
	/// of the bytes. If the bytes are not a valid verifying key, the key will
	/// be created as empty. To check the validity of the bytes, use
	/// [`VerifyingKey::from_bytes()`](RealVerifyingKey::from_bytes) instead
	/// (which will be default unless this method is specifically called).
	/// 
	fn from_bytes(bytes: [u8; 32]) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::from_bytes(&bytes).unwrap_or_else(|_| Self::from_bytes(&[0_u8; 32]).unwrap())
	}
	
	//		to_base64															
	fn to_base64(&self) -> String {
		BASE64.encode(self.as_bytes())
	}
	
	//		from_base64															
	fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		Ok(Self::force_from(BASE64.decode(encoded)?))
	}
	
	//		to_hex																
	fn to_hex(&self) -> String {
		hex::encode(self.as_bytes())
	}
	
	//		from_hex															
	fn from_hex(encoded: &str) -> Result<Self, FromHexError> {
		Ok(Self::force_from(hex::decode(encoded)?))
	}
	
	//		to_vec																
	fn to_vec(&self) -> Vec<u8> {
		self.as_bytes().to_vec()
	}
}

//󰭅		ForceFrom &[u8]															
impl ForceFrom<&[u8]> for RealVerifyingKey {
	//		force_from															
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`ed25519_dalek::VerifyingKey`].
	/// 
	/// Note that if the incoming `[u8]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	/// Note that this is a direct conversion, and does not check the validity
	/// of the bytes. If the bytes are not a valid verifying key, the key will
	/// be created as empty. To check the validity of the bytes, use
	/// [`VerifyingKey::from_bytes()`](RealVerifyingKey::from_bytes) instead
	/// (which will be default unless this method is specifically called).
	/// 
	fn force_from(value: &[u8]) -> Self {
		let mut array = [0_u8; 32];
		let len       = value.len().min(32);
		#[cfg_attr(    feature = "reasons",  allow(clippy::indexing_slicing, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::indexing_slicing))]
		array[..len].copy_from_slice(&value[..len]);
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::from_bytes(&array).unwrap_or_else(|_| Self::from_bytes(&[0_u8; 32]).unwrap())
	}
}

//󰭅		ForceFrom &[u8; N]														
impl<const N: usize> ForceFrom<&[u8; N]> for RealVerifyingKey {
	//		force_from															
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to a [`ed25519_dalek::VerifyingKey`].
	/// 
	/// Note that if the incoming `[u8; N]` is too long to fit, it will be
	/// truncated without error or warning. If there is not enough data, it will
	/// be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &[u8; N]) -> Self {
		Self::force_from(&value[..])
	}
}

//󰭅		ForceFrom Vec<u8>														
impl ForceFrom<Vec<u8>> for RealVerifyingKey {
	//		force_from															
	/// Converts a [`Vec<u8>`](Vec) to a [`ed25519_dalek::VerifyingKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: Vec<u8>) -> Self {
		Self::force_from(&*value)
	}
}

//󰭅		ForceFrom &Vec<u8>														
impl ForceFrom<&Vec<u8>> for RealVerifyingKey {
	//		force_from															
	/// Converts a [`&Vec[u8]`](Vec) to a [`ed25519_dalek::VerifyingKey`].
	/// 
	/// Note that if the incoming [`Vec<u8>`](Vec) is too long to fit, it will
	/// be truncated without error or warning. If there is not enough data, it
	/// will be padded with zeroes. If this situation needs checking, use
	/// `try_from()` instead.
	/// 
	fn force_from(value: &Vec<u8>) -> Self {
		Self::force_from(&**value)
	}
}


