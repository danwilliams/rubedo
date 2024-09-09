//! This module provides extensions to the Rust standard library.



//		Modules

#[cfg(test)]
#[path = "tests/std.rs"]
mod tests;



//		Packages

use crate::sugar::s;
use base64::DecodeError;
use core::{
	convert::TryFrom,
	error::Error,
	fmt::{Debug, Display, self},
	hash::Hash,
	str::FromStr,
};
use hex::FromHexError;
use rust_decimal::{
	Decimal,
	prelude::ToPrimitive,
};
use serde::{Deserialize, Serialize};
use std::{
	borrow::Cow,
	env,
	ffi::OsString,
	path::{Component as PathComponent, Path, PathBuf},
};

#[cfg(feature = "crypto")]
use crate::crypto::Hashed;
#[cfg(feature = "crypto")]
use ::{
	core::future::Future,
	digest::Digest,
	std::{
		fs::File,
		io::{BufReader, Error as IoError, Read},
	},
	tokio::{
		fs::File as AsyncFile,
		io::{AsyncReadExt, BufReader as AsyncBufReader},
	},
};



//		Enums

//		ByteSizedError															
/// The possible errors that can occur when working with [`ByteSized`] types.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ByteSizedError {
	/// The supplied data is longer than `ByteSized::SIZE` bytes.
	DataTooLong(usize),
	
	/// The supplied data is shorter than `ByteSized::SIZE` bytes.
	DataTooShort(usize),
	
	/// The supplied string is not in valid base64 format.
	InvalidBase64String,
	
	/// The supplied string is not in valid hexadecimal format.
	InvalidHexString,
}

//󰭅		Display																	
impl Display for ByteSizedError {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match *self {
			Self::DataTooLong(size)   => format!("The supplied data is longer than {size} bytes"),
			Self::DataTooShort(size)  => format!("The supplied data is shorter than {size} bytes"),
			Self::InvalidBase64String => s!(     "The supplied data is not in valid base64 format"),
			Self::InvalidHexString    => s!(     "The supplied data is not in valid hexadecimal format"),
		};
		write!(f, "{description}")
	}
}

//󰭅		Error																	
impl Error for ByteSizedError {}



//		Structs

//		LimitIterator															
/// This struct provides an iterator that limits the number of items returned.
/// 
/// This will be returned from the [`limit()`](IteratorExt::limit()) method, and
/// will generally not be used directly.
/// 
/// # See also
/// 
/// * [`IteratorExt::limit()`]
/// 
#[derive(Clone, Debug)]
pub struct LimitIterator<I> {
	//		Private properties													
	/// The iterator to limit.
	iter:  I,
	
	/// The maximum number of items to return.
	limit: Option<usize>,
	
	/// The number of items returned so far.
	count: usize,
}

//󰭅		Iterator																
impl<I: Iterator> Iterator for LimitIterator<I> {
	type Item = I::Item;
	
	//		next																
	fn next(&mut self) -> Option<Self::Item> {
		#[expect(clippy::arithmetic_side_effects, reason = "Range is controlled")]
		if let Some(limit) = self.limit {
			if self.count >= limit {
				return None;
			}
			//	In this location, the count is guaranteed to not exceed the limit, so
			//	this will not overflow and a checked_add() is not required.
			self.count += 1;
		}
		self.iter.next()
	}
}



//		Traits

//§		AsStr																	
/// This trait provides an [`as_str()`](AsStr::as_str()) method.
/// 
/// This trait requires the presence of an [`as_str()`](AsStr::as_str()) method.
/// It's not possible to apply this trait purely as a marker to the existing
/// types such as [`String`] that already have an [`as_str()`](AsStr::as_str())
/// method and have it recognised that they already have it, due to Rust's
/// implementation determination allowing multiple methods of the same name,
/// differentiated by trait. In other words, our trait could define a method
/// with the same name and signature as another trait, but an implementation of
/// the function would not be considered to satisfy both. Both traits would have
/// to have their methods specifically implemented, even if identical, and then
/// the conflict would be resolved at call-time by specifying which trait's
/// method is being called.
/// 
/// However, it is possible to apply this trait and call the underlying method
/// on the type, for such cases as this may be required. This trait should
/// therefore be applied to any types of interest, for which the [`as_str()`](crate::serde::as_str())
/// serialisation function provided by the [`serde`](crate::serde) module is
/// intended to be specified. Suitable standard and common types such as
/// [`String`] and [`str`] have already had this trait implemented, and those
/// implementations will be brought into scope when this trait is used.
/// 
/// In reality, implementations onto standard types should not really be
/// necessary, as this trait exists primarily for use with the
/// [`serde::as_str()`](crate::serde::as_str()) method, and Serde already knows
/// how to handle such types so there is no real advantage to be gained by
/// implementing this trait for such types. The intent and purpose of this trait
/// is to provide a way to specify a string representation for types that do not
/// already have one, such as dual-nature enums, i.e. where they can be
/// represented as either a string or a number. Still, the trait has been
/// applied to some common types for consistency and completeness.
/// 
/// The only current drawback is that trait functions cannot currently be
/// declared as `const`, and the scope of the [`as_str()`](AsStr::as_str())
/// method is usually such that it could be declared as `const` otherwise.
/// 
pub trait AsStr {
	//		as_str																
	/// Provides a string slice representation of the type.
	#[must_use]
	fn as_str(&self) -> &str;
}

//󰭅		String																	
impl AsStr for String {
	//		as_str																
	fn as_str(&self) -> &str {
		//	This simply calls the existing method, i.e. String.as_str(), but is
		//	required to allow the trait to be applied to the type.
		self.as_str()
	}
}

//󰭅		str																		
impl AsStr for str {
	//		as_str																
	fn as_str(&self) -> &str {
		//	This simply returns the existing value, i.e. self, but is required
		//	to allow the trait to be applied to the type.
		self
	}
}

//§		ByteSized																
/// Fixed-size byte container functionality.
/// 
/// This trait provides a formalised representation of a fixed-size byte array,
/// with support for common conversions, including to and from hex and base64
/// formats.
/// 
/// Notably, it is defined in a way that allows implementation onto third-party
/// types, i.e. those from other libraries, boosting their functionality with
/// additional methods. Meanwhile, if applying to an owned type, whether an
/// original or a wrapper, the full range of trait implementations for various
/// conversions and similar is available via the [`ByteSizedFull`] trait.
/// 
/// The container is expected to be stored internally as `[u8; N]`, where `N` is
/// defined upon the implementation of this trait — but the actual internal type
/// is arbitrary. Because there may or may not be control over the internal type
/// (for instance when implementing for a third-party type), the methods that
/// require the ability to mutate or consume the internal type are split out
/// into a separate [`ByteSizedMut`] trait.
///
/// The conversion to and from a [`String`] defaults to using hex strings rather
/// than base64-encoded strings, because this is more common for the primary use
/// case of hashes and keys, due to it being a fixed-length string that is easy
/// to read, verify, and transmit without any compatibility issues. However,
/// base64 conversion functions are also provided for convenience in case that
/// format is preferred.
/// 
/// # See also
/// 
/// * [`ByteSizedFull`]
/// * [`ByteSizedMut`]
/// 
#[expect(clippy::trait_duplication_in_bounds, reason = "Not actually duplicates")]
pub trait ByteSized<const SIZE: usize>:
	Sized
	+ Clone
	+ for<'a> ForceFrom<&'a [u8]>
//	+ for<'a> ForceFrom<&'a [u8; N]>  //  Cannot specify this as a constraint due to N
	+ ForceFrom<Vec<u8>>
	+ for<'a> ForceFrom<&'a Vec<u8>>
{
	//		as_bytes															
	/// Returns a byte slice of the container's contents.
	/// 
	/// Provides a read-only view of the byte data within the container, without
	/// consuming the data. The returned slice is a reference to the actual data
	/// stored in the container, not a copy. Because of this, it is not possible
	/// to mutate the contents of the container through the returned slice. It
	/// does not allocate new memory or change the ownership of the byte data.
	/// This method is useful when you need to work with the bytes of the
	/// container in a read-only fashion, or when you want to avoid copying the
	/// data.
	/// 
	///   - This method returns a slice (`&[u8; Self::SIZE]`) referencing the
	///     bytes of the container contents.
	///   - The original container value remains intact, and can still be used
	///     afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a view into the original memory.
	/// 
	/// Use this method when you need to work with the byte data in a
	/// non-destructive, read-only manner while keeping the original container
	/// intact.
	///
	/// # See also
	/// 
	/// * [`ByteSized::from_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSized::to_vec()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// * [`ByteSizedMut::into_vec()`]
	/// 
	#[must_use]
	fn as_bytes(&self) -> &[u8; SIZE];
	
	//		to_bytes															
	/// Returns a copy of the container data as a fixed-length array of bytes.
	/// 
	/// This does not consume the container, but clones it. Following Rust's
	/// naming conventions and idioms, this method "converts" the data content
	/// of the container into a byte representation, in a `[u8; SIZE]`. (No
	/// actual conversion takes place if the data is already stored internally
	/// as a fixed array of bytes, but this is academic, so "conversion" is
	/// implied and expected as a theoretical behaviour.) Ownership of the
	/// cloned and converted byte data is transferred to the caller, and there
	/// are no side effects on the internal state of the [`ByteSized`] instance.
	/// 
	///   - This method returns a `[u8; SIZE]` array of bytes without consuming
	///     the container contents.
	///   - The original container value remains intact, and can still be used
	///     afterward.
	///   - The container data is copied, and converted/transformed into the
	///     output value returned.
	/// 
	/// Use this method when you need to obtain a copy of the container's byte
	/// data in the form of a `[u8; SIZE]`, without consuming the container
	/// itself. This is useful when you need to pass the byte data to a function
	/// that expects a `[u8; SIZE]`, or when you want to modify the byte data
	/// without affecting the original container.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::from_bytes()`]
	/// * [`ByteSized::to_vec()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// * [`ByteSizedMut::into_vec()`]
	/// 
	#[must_use]
	fn to_bytes(&self) -> [u8; SIZE];
	
	//		from_bytes															
	/// Constructs a [`ByteSized`] type from an array of bytes.
	/// 
	/// This method consumes the input array.
	/// 
	/// # Parameters
	/// 
	/// * `bytes` - The array of bytes to convert into the [`ByteSized`] type.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// 
	#[must_use]
	fn from_bytes(bytes: [u8; SIZE]) -> Self;
	
	//		to_base64															
	/// Returns the container data converted to a base64-encoded [`String`].
	/// 
	/// This does not consume the container, but clones it, as is necessary to
	/// perform the conversion to base64.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::from_base64()`]
	/// 
	#[must_use]
	fn to_base64(&self) -> String;
	
	//		from_base64															
	/// Converts a base64-encoded [`String`] to a [`ByteSized`] type.
	/// 
	/// This method does not consume the input string, but clones it, as is
	/// necessary to perform the conversion from [`base64`].
	/// 
	/// # Parameters
	/// 
	/// * `encoded` - The base64-encoded [`String`] to convert into a
	///               [`ByteSized`] type.
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
	/// * [`ByteSized::to_base64()`]
	/// 
	fn from_base64(encoded: &str) -> Result<Self, DecodeError>;
	
	//		to_hex																
	/// Returns the container data converted to a hex-encoded [`String`].
	/// 
	/// This does not consume the container, but clones it, as is necessary to
	/// perform the conversion to hexadecimal representation.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::from_hex()`]
	/// 
	#[must_use]
	fn to_hex(&self) -> String;
	
	//		from_hex															
	/// Converts a hex-encoded [`String`] to a [`ByteSized`].
	/// 
	/// This method does not consume the input string, but clones it, as is
	/// necessary to perform the conversion from hexadecimal representation.
	/// 
	/// # Parameters
	/// 
	/// * `encoded` - The hex-encoded [`String`] to convert into a [`ByteSized`]
	///               type.
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
	/// * [`ByteSized::to_hex()`]
	/// 
	fn from_hex(encoded: &str) -> Result<Self, FromHexError>;
	
	//		to_vec																
	/// Returns a copy of the container data converted to a vector of bytes.
	/// 
	/// This does not consume the container, but clones it. Following Rust's
	/// naming conventions and idioms, this method converts the data content of
	/// the container into a byte representation, in a [`Vec<u8>`](Vec).
	/// Ownership of the cloned and converted byte data is transferred to the
	/// caller, and there are no side effects on the internal state of the
	/// [`ByteSized`] instance.
	/// 
	///   - This method returns a [`Vec<u8>`](Vec) vector of bytes without
	///     consuming the container contents.
	///   - The original container value remains intact, and can still be used
	///     afterward.
	///   - The container data is copied, and converted/transformed into the
	///     output value returned.
	/// 
	/// Use this method when you need to obtain a copy of the container's byte
	/// data in the form of a [`Vec<u8>`](Vec), without consuming the container
	/// itself. This is useful when you need to pass the byte data to a function
	/// that expects a [`Vec<u8>`](Vec).
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// * [`ByteSizedMut::into_vec()`]
	/// 
	#[must_use]
	fn to_vec(&self) -> Vec<u8>;
}

//§		ByteSizedFull															
/// Full conversion functionality for [`ByteSized`]-based types.
/// 
/// This trait provides no methods, but establishes required trait
/// implementations that should be present for a full implementation of
/// [`ByteSized`] functionality onto an owned type or wrapper. This includes
/// support for common conversions, including serialisation and deserialisation
/// using [Serde](https://crates.io/crates/serde). The traits that cannot be
/// implemented for third-party types due to the orphan rule are therefore
/// listed under this trait as constraints.
/// 
/// Because there may or may not be control over the internal type (for instance
/// when implementing onto a third-party type, or for a wrapper), the
/// implementations that require the ability to mutate or consume the internal
/// type are specified in the separate [`ByteSizedMut`] trait.
/// 
/// # See also
/// 
/// * [`ByteSized`]
/// * [`ByteSizedMut`]
/// 
#[expect(clippy::trait_duplication_in_bounds, reason = "Not actually duplicates")]
pub trait ByteSizedFull<const SIZE: usize>:
	ByteSized<SIZE>
	+ AsRef<[u8; SIZE]>
	+ Debug
	+ Default
	+ Display
	+ From<[u8; SIZE]>
	+ for<'a> From<&'a [u8; SIZE]>
	+ FromStr
	+ Hash
	+ PartialEq
	+ Serialize
	+ for<'de> Deserialize<'de>
	+ for<'a> TryFrom<&'a [u8]>
	+ for<'a> TryFrom<&'a str>
	+ TryFrom<String>
	+ for<'a> TryFrom<&'a String>
	+ TryFrom<Box<str>>
	+ for<'a> TryFrom<Cow<'a, str>>
	+ TryFrom<Vec<u8>>
	+ for<'a> TryFrom<&'a Vec<u8>>
{}

//§		ByteSizedMut															
/// Mutating and consuming functionality for [`ByteSized`].
/// 
/// This trait provides methods that mutate and/or consume the underlying data
/// type represented, expected to be a `[u8; N]`, where `N` is defined upon the
/// implementation of the [`ByteSized`] trait — but the actual internal type is
/// arbitrary.
/// 
/// Because there may or may not be control over the internal type (for instance
/// when implementing for a third-party type), the methods that require the
/// ability to mutate or consume the internal type are split out into this
/// separate [`ByteSizedMut`] trait, with the read-only methods and constructors
/// being in the main [`ByteSized`] trait, and the traits that cannot be
/// implemented for third-party types due to the orphan rule being specified
/// under the [`ByteSizedFull`] trait.
/// 
/// # See also
/// 
/// * [`ByteSized`]
/// * [`ByteSizedFull`]
/// 
pub trait ByteSizedMut<const SIZE: usize>:
	ByteSized<SIZE>
	+ AsMut<[u8; SIZE]>
{
	//		as_mut_bytes														
	/// Returns a mutable reference to the container's contents.
	/// 
	/// Provides a mutable view of the byte data within the container, without
	/// consuming the data. The returned vector is a reference to the actual
	/// data stored in the container, not a copy. This method is useful when you
	/// need to work with, and modify, the bytes of the container directly,
	/// without copying the data.
	/// 
	///   - This method returns a mutable array (`&mut [u8; SIZE]`) referencing
	///     the bytes of the container contents.
	///   - The original container value remains intact, and can still be used
	///     afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a reference to the original memory.
	/// 
	/// Use this method when you need to work directly with the byte data in a
	/// mutable manner.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::from_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSized::to_vec()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// * [`ByteSizedMut::into_vec()`]
	/// 
	fn as_mut_bytes(&mut self) -> &mut [u8; SIZE];
	
	//		into_bytes															
	/// Returns the container as a fixed-length array of bytes.
	/// 
	/// This consumes the container, without cloning or copying, and returns a
	/// new fixed-length array containing the bytes of the container. It
	/// transfers ownership of the byte data from the container to the new
	/// array. This method is useful when you need to move the byte data out of
	/// the container, or when you want to modify the byte data in-place without
	/// affecting the original container.
	/// 
	///   - This method consumes the container contents and returns a
	///     `[u8; SIZE]` containing its bytes.
	///   - After calling this method, the original container value is no longer
	///     available for use, because it has been moved.
	/// 
	/// Use this method when you want to consume the container and obtain
	/// ownership of its byte data in the form of a `[u8; SIZE]`. This is useful
	/// when you need to modify or move the byte data, or when you want to pass
	/// it to functions that expect a `[u8; SIZE]`.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::from_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSized::to_vec()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_vec()`]
	/// 
	#[must_use]
	fn into_bytes(self) -> [u8; SIZE];
	
	//		into_vec															
	/// Returns the container as a vector of bytes.
	/// 
	/// This consumes the container, and returns a new vector containing the
	/// bytes of the container. It transfers ownership of the byte data from the
	/// container to the new vector. This method is useful when you need to move
	/// the byte data out of the container, for example to pass it to a function
	/// that expects a [`Vec<u8>`](Vec). Note, however, that because vectors are
	/// heap-allocated and can grow dynamically, whereas arrays are fixed-size
	/// and stack-allocated, there isn't a direct, zero-copy way to consume an
	/// array into a [`Vec`], and so this process does involve copying the data.
	/// 
	///   - This method consumes the container contents and returns a
	///     [`Vec<u8>`](Vec) containing its bytes.
	///   - After calling this method, the original container value is no longer
	///     available for use, because it has been moved.
	///   - Transforms the container into a vector of bytes, but does copy the
	///     data.
	/// 
	/// Use this method when you want to consume the container and obtain
	/// ownership of its byte data in the form of a [`Vec<u8>`](Vec). This is
	/// useful when you need to modify or move the byte data.
	/// 
	/// # See also
	/// 
	/// * [`ByteSized::as_bytes()`]
	/// * [`ByteSized::to_bytes()`]
	/// * [`ByteSized::to_vec()`]
	/// * [`ByteSizedMut::as_mut_bytes()`]
	/// * [`ByteSizedMut::into_bytes()`]
	/// 
	#[must_use]
	fn into_vec(self) -> Vec<u8>;
}

//§		FileExt																	
/// This trait provides additional functionality to [`File`].
#[cfg(feature = "crypto")]
pub trait FileExt {
	/// Hashes the contents of a file.
	/// 
	/// This function reads the contents of a file and hashes it using the
	/// hashing algorithm associated to the hash type specified. The resulting
	/// hash is returned as the specified formal [`Hashed`] type.
	/// 
	/// # Parameters
	/// 
	/// * `path` - The path to the file to hash.
	/// 
	/// # Errors
	/// 
	/// This function will return an error if the file cannot be opened, or if
	/// there is a problem reading from the file.
	/// 
	fn hash<T: Hashed>(path: &Path) -> Result<T, IoError>;
}

//󰭅		File																	
#[cfg(feature = "crypto")]
impl FileExt for File {
	fn hash<T: Hashed>(path: &Path) -> Result<T, IoError> {
		let file       = Self::open(path)?;
		let mut reader = BufReader::new(file);
		let mut hasher = T::Algorithm::new();
		let mut buffer = [0; 0x2000];  //  8KB buffer
		loop {
			let count = reader.read(&mut buffer)?;
			if count == 0 {
				break;
			}
			#[expect(clippy::indexing_slicing, reason = "Infallible")]
			hasher.update(&buffer[..count]);
		}
		Ok(T::from_digest(hasher.finalize()))
	}
}

//§		AsyncFileExt															
/// This trait provides additional functionality to [`AsyncFile`].
#[cfg(feature = "crypto")]
pub trait AsyncFileExt {
	/// Hashes the contents of a file asynchronously.
	/// 
	/// This function reads the contents of a file and hashes it using the
	/// hashing algorithm associated to the hash type specified. The resulting
	/// hash is returned as the specified formal [`Hashed`] type.
	/// 
	/// # Parameters
	/// 
	/// * `path` - The path to the file to hash.
	/// 
	/// # Errors
	/// 
	/// This function will return an error if the file cannot be opened, or if
	/// there is a problem reading from the file.
	/// 
	//	Cannot use the async keyword here due to needing to specify Send as a
	//	constraint.
	fn hash<T: Hashed>(path: &Path) -> impl Future<Output = Result<T, IoError>> + Send;
}

//󰭅		AsyncFile																
#[cfg(feature = "crypto")]
impl AsyncFileExt for AsyncFile {
	async fn hash<T: Hashed>(path: &Path) -> Result<T, IoError> {
		let file       = Self::open(path).await?;
		let mut reader = AsyncBufReader::new(file);
		let mut hasher = T::Algorithm::new();
		let mut buffer = [0; 0x2000];  //  8KB buffer
		loop {
			let count = reader.read(&mut buffer).await?;
			if count == 0 {
				break;
			}
			#[expect(clippy::indexing_slicing, reason = "Infallible")]
			hasher.update(&buffer[..count]);
		}
		Ok(T::from_digest(hasher.finalize()))
	}
}

//§		FromIntWithScale														
/// Converts from an integer to a floating-point number with a specified scale.
/// 
/// This trait requires the presence of a [`from_int_with_scale()`](FromIntWithScale::from_int_with_scale())
/// method, which converts from an integer to a floating-point number with a
/// specified scale, i.e. a certain number of decimal places. For example, if
/// the scale is `2`, then the integer `1234` would be converted to the
/// floating-point number `12.34`. This is most useful when dealing with
/// currencies.
/// 
/// The trait is implemented for the standard floating-point types, i.e. [`f32`]
/// and [`f64`], and for the [`Decimal`] type from the [`rust_decimal`](https://crates.io/crates/rust_decimal)
/// crate. For the corresponding integer types expressed as the generic `T`, it
/// is implemented for the standard integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`].
/// 
/// Note that not all of these integer types can have their full range
/// represented by all of the floating-point types, and so naive conversion may
/// result in them being truncated or rounded. To avoid this happening
/// invisibly, the conversion will return [`None`] if the input number cannot be
/// accurately represented in the output type. Care should be taken to assess
/// the likelihood of this occurring, and to ensure that the correct types are
/// used. This cannot be guaranteed by the compiler, as the outcome depends
/// partly on the type and partly on the scale factor, and so an assessment has
/// to be made at runtime.
/// 
pub trait FromIntWithScale<T>: Sized {
	//		from_int_with_scale													
	/// Converts from an integer to a floating-point number with a specified
	/// scale.
	/// 
	/// This function converts from an integer to a floating-point number with a
	/// specified scale, i.e. a certain number of decimal places. For example,
	/// if the scale is `2`, then the integer `1234` would be converted to the
	/// floating-point number `12.34`. This is most useful when dealing with
	/// currencies.
	/// 
	/// Note that not all integer types can have their full range represented by
	/// all of the floating-point types, and so naive conversion may result in
	/// them being truncated or rounded. To avoid this happening invisibly, the
	/// conversion will return [`None`] if the input number cannot be accurately
	/// represented in the output type. Care should be taken to assess the
	/// likelihood of this occurring, and to ensure that the correct types are
	/// used. This cannot be guaranteed by the compiler, as the outcome depends
	/// partly on the type and partly on the scale factor, and so an assessment
	/// has to be made at runtime.
	/// 
	/// # Parameters
	/// 
	/// * `value` - The integer value to convert.
	/// * `scale` - The scale factor, i.e. the number of decimal places. Note
	///             that this is essentially limited to a maximum of 19 DP of
	///             movement for an [`f32`] or [`f64`] without overflowing, and
	///             28 DP for a [`Decimal`].
	/// 
	/// # See also
	/// 
	/// * [`ToIntWithScale::to_int_with_scale()`]
	/// 
	fn from_int_with_scale(value: T, scale: u8) -> Option<Self>;
}

//		impl_from_int_with_scale_for_float										
/// Implements the [`FromIntWithScale`] trait for floating-point types.
macro_rules! impl_from_int_with_scale_for_float {
	($t:ty, f32) => {
		//󰭅		Integer for f32													
		impl FromIntWithScale<$t> for f32 {
			//		from_int_with_scale											
			#[allow(clippy::allow_attributes, reason = "Multiple possibilities through the macro invocation")]
			#[allow(clippy::cast_lossless,    reason = "Being potentially lossy does not matter here")]
			fn from_int_with_scale(value: $t, scale: u8) -> Option<Self> {
				let factor = 10_u32.checked_pow(u32::from(scale))?;
				#[allow(clippy::cast_precision_loss, reason = "Losing precision does not matter here")]
				let scaled = value as f32 / factor as f32;
				//	We need to manually check if the value exceeds the range of integer
				//	values supported by an f32, as that will result in a loss of precision.
				#[allow(trivial_numeric_casts,              reason = "Trivial casts here are due to the macro permutations")]
				#[allow(clippy::cast_sign_loss,             reason = "Loss of sign does not matter here, as we are checking for overflow")]
				#[allow(clippy::cast_possible_wrap,         reason = "Possible wrapping does not matter here, as we are checking for underflow")]
				#[allow(clippy::invalid_upcast_comparisons, reason = "Superfluous upcast comparisons here are due to the macro permutations")]
				if scaled.is_infinite() || (value as u128) > 0x0100_0000_u128 || (value as i128) < -0x0100_0000_i128 {
					None
				} else {
					Some(scaled)
				}
			}
		}
	};
	($t:ty, f64) => {
		//󰭅		Integer for f64													
		impl FromIntWithScale<$t> for f64 {
			//		from_int_with_scale											
			#[allow(clippy::allow_attributes, reason = "Multiple possibilities through the macro invocation")]
			#[allow(clippy::cast_lossless,    reason = "Being potentially lossy does not matter here")]
			fn from_int_with_scale(value: $t, scale: u8) -> Option<Self> {
				let factor = 10_u64.checked_pow(u32::from(scale))?;
				#[allow(clippy::cast_precision_loss, reason = "Losing precision does not matter here")]
				let scaled = value as f64 / factor as f64;
				//	We need to manually check if the value exceeds the range of integer
				//	values supported by an f64, as that will result in a loss of precision.
				#[allow(trivial_numeric_casts,              reason = "Trivial casts here are due to the macro permutations")]
				#[allow(clippy::cast_sign_loss,             reason = "Loss of sign does not matter here, as we are checking for overflow")]
				#[allow(clippy::cast_possible_wrap,         reason = "Possible wrapping does not matter here, as we are checking for underflow")]
				#[allow(clippy::invalid_upcast_comparisons, reason = "Superfluous upcast comparisons here are due to the macro permutations")]
				if scaled.is_infinite() || (value as u128) > 0x0020_0000_0000_0000_u128 || (value as i128) < -0x0020_0000_0000_0000_i128 {
					None
				} else {
					Some(scaled)
				}
			}
		}
	};
}

impl_from_int_with_scale_for_float!(i8,   f32);
impl_from_int_with_scale_for_float!(i16,  f32);
impl_from_int_with_scale_for_float!(i32,  f32);
impl_from_int_with_scale_for_float!(i64,  f32);
impl_from_int_with_scale_for_float!(i128, f32);
impl_from_int_with_scale_for_float!(i8,   f64);
impl_from_int_with_scale_for_float!(i16,  f64);
impl_from_int_with_scale_for_float!(i32,  f64);
impl_from_int_with_scale_for_float!(i64,  f64);
impl_from_int_with_scale_for_float!(i128, f64);
impl_from_int_with_scale_for_float!(u8,   f32);
impl_from_int_with_scale_for_float!(u16,  f32);
impl_from_int_with_scale_for_float!(u32,  f32);
impl_from_int_with_scale_for_float!(u64,  f32);
impl_from_int_with_scale_for_float!(u128, f32);
impl_from_int_with_scale_for_float!(u8,   f64);
impl_from_int_with_scale_for_float!(u16,  f64);
impl_from_int_with_scale_for_float!(u32,  f64);
impl_from_int_with_scale_for_float!(u64,  f64);
impl_from_int_with_scale_for_float!(u128, f64);

//		impl_from_int_with_scale_for_decimal									
/// Implements the [`FromIntWithScale`] trait for the [`Decimal`] type.
macro_rules! impl_from_int_with_scale_for_decimal {
	(i128) => {
		//󰭅		i128 for Decimal												
		impl FromIntWithScale<i128> for Decimal {
			//		from_int_with_scale											
			fn from_int_with_scale(value: i128, scale: u8) -> Option<Self> {
				//	We should be able to rely upon Decimal::try_from_i128_with_scale() to
				//	perform the necessary checks, but it currently has issues with numbers
				//	larger than the supported 96-bit range, so we need to check manually.
				if value > Decimal::MAX.to_i128().unwrap() || value < Decimal::MIN.to_i128().unwrap() {
					None
				} else {
					Decimal::try_from_i128_with_scale(value, u32::from(scale)).ok()
				}
			}
		}
	};
	(u128) => {
		//󰭅		u128 for Decimal												
		impl FromIntWithScale<u128> for Decimal {
			//		from_int_with_scale											
			#[allow(clippy::allow_attributes, reason = "Multiple possibilities through the macro invocation")]
			#[allow(clippy::cast_lossless,    reason = "Being potentially lossy does not matter here")]
			fn from_int_with_scale(value: u128, scale: u8) -> Option<Self> {
				//	We should be able to rely upon Decimal::try_from_i128_with_scale() to
				//	perform the necessary checks, but it currently has issues with numbers
				//	larger than the supported 96-bit range, so we need to check manually.
				//	Regardless of this, we would have to check if the value is larger than
				//	supported by an i128 in any case.
				#[allow(clippy::cast_possible_wrap, reason = "Possible wrapping does not matter here, as we are checking for underflow")]
				if value > Decimal::MAX.to_u128().unwrap() || (value as i128) < Decimal::MIN.to_i128().unwrap() {
					None
				} else {
					Decimal::try_from_i128_with_scale(value as i128, u32::from(scale)).ok()
				}
			}
		}
	};
	($t:ty) => {
		//󰭅		Integer for Decimal												
		impl FromIntWithScale<$t> for Decimal {
			//		from_int_with_scale											
			#[allow(clippy::allow_attributes, reason = "Multiple possibilities through the macro invocation")]
			#[allow(clippy::cast_lossless,    reason = "Being potentially lossy does not matter here")]
			fn from_int_with_scale(value: $t, scale: u8) -> Option<Self> {
				//	Everything less than 128 bits will fit safely into the Decimal's range.
				Decimal::try_from_i128_with_scale(value as i128, u32::from(scale)).ok()
			}
		}
	};
}

impl_from_int_with_scale_for_decimal!(i8);
impl_from_int_with_scale_for_decimal!(i16);
impl_from_int_with_scale_for_decimal!(i32);
impl_from_int_with_scale_for_decimal!(i64);
impl_from_int_with_scale_for_decimal!(i128);
impl_from_int_with_scale_for_decimal!(u8);
impl_from_int_with_scale_for_decimal!(u16);
impl_from_int_with_scale_for_decimal!(u32);
impl_from_int_with_scale_for_decimal!(u64);
impl_from_int_with_scale_for_decimal!(u128);

//§		ToIntWithScale															
/// Converts from a floating-point number to an integer with a specified scale.
/// 
/// This trait requires the presence of a [`to_int_with_scale()`](ToIntWithScale::to_int_with_scale())
/// method, which converts from a floating-point number to an integer with a
/// specified scale, i.e. a certain number of decimal places. For example, if
/// the scale is `2`, then the floating-point number `12.34` would be converted
/// to the integer `1234`. This is most useful when dealing with currencies.
/// 
/// The trait is implemented for the standard floating-point types, i.e. [`f32`]
/// and [`f64`], and for the [`Decimal`] type from the [`rust_decimal`](https://crates.io/crates/rust_decimal)
/// crate. For the corresponding integer types expressed as the generic `T`, it
/// is implemented for the standard integer types [`i8`], [`i16`], [`i32`],
/// [`i64`], [`i128`], [`u8`], [`u16`], [`u32`], [`u64`], and [`u128`].
/// 
/// Note that not all of these floating-point types can have their full range
/// represented by all of the integer types, and so naive conversion may result
/// in them being truncated or rounded. To avoid this happening invisibly, the
/// conversion will return [`None`] if the input number cannot be accurately
/// represented in the output type. Care should be taken to assess the
/// likelihood of this occurring, and to ensure that the correct types are used.
/// This cannot be guaranteed by the compiler, as the outcome depends partly on
/// the type and partly on the scale factor, and so an assessment has to be made
/// at runtime.
/// 
pub trait ToIntWithScale<T>: Sized {
	//		to_int_with_scale													
	/// Converts from a floating-point number to an integer with a specified
	/// scale.
	/// 
	/// This function converts from a floating-point number to an integer with a
	/// specified scale, i.e. a certain number of decimal places. For example,
	/// if the scale is `2`, then the integer `1234` would be converted to the
	/// floating-point number `12.34`. This is most useful when dealing with
	/// currencies.
	/// 
	/// Note that not all floating-point types can have their full range
	/// represented by all of the integer types, and so naive conversion may
	/// result in them being truncated or rounded. To avoid this happening
	/// invisibly, the conversion will return [`None`] if the input number
	/// cannot be accurately represented in the output type. Care should be
	/// taken to assess the likelihood of this occurring, and to ensure that the
	/// correct types are used. This cannot be guaranteed by the compiler, as
	/// the outcome depends partly on the type and partly on the scale factor,
	/// and so an assessment has to be made at runtime.
	/// 
	/// # Parameters
	///
	/// * `scale` - The scale factor, i.e. the number of decimal places. Note
	///             that this is essentially limited to a maximum of 19 DP of
	///             movement without overflowing.
	/// 
	/// # See also
	/// 
	/// * [`FromIntWithScale::from_int_with_scale()`]
	/// 
	fn to_int_with_scale(&self, scale: u8) -> Option<T>;
}

//		impl_to_int_with_scale_for_float										
/// Implements the [`ToIntWithScale`] trait for floating-point types.
macro_rules! impl_to_int_with_scale_for_float {
	($t:ty, $f:ty) => {
		//󰭅		Integer for Float												
		impl ToIntWithScale<$t> for $f {
			//		to_int_with_scale											
			#[allow(clippy::allow_attributes,    reason = "Multiple possibilities through the macro invocation")]
			#[allow(clippy::cast_lossless,       reason = "Being potentially lossy does not matter here")]
			#[allow(clippy::cast_precision_loss, reason = "Losing precision does not matter here")]
			fn to_int_with_scale(&self, scale: u8) -> Option<$t> {
				let factor = 10_u64.checked_pow(u32::from(scale))?;
				let scaled = (self * factor as $f).round();
				if scaled.is_infinite() || scaled > <$t>::MAX as $f || scaled < <$t>::MIN as $f {
					None
				} else {
					#[allow(clippy::cast_possible_truncation, reason = "Possible truncation does not matter here")]
					#[allow(clippy::cast_sign_loss,           reason = "Loss of sign will not occur here, as we are casting to a float")]
					Some(scaled as $t)
				}
			}
		}
	};
}

impl_to_int_with_scale_for_float!(i8,   f32);
impl_to_int_with_scale_for_float!(i16,  f32);
impl_to_int_with_scale_for_float!(i32,  f32);
impl_to_int_with_scale_for_float!(i64,  f32);
impl_to_int_with_scale_for_float!(i128, f32);
impl_to_int_with_scale_for_float!(i8,   f64);
impl_to_int_with_scale_for_float!(i16,  f64);
impl_to_int_with_scale_for_float!(i32,  f64);
impl_to_int_with_scale_for_float!(i64,  f64);
impl_to_int_with_scale_for_float!(i128, f64);
impl_to_int_with_scale_for_float!(u8,   f32);
impl_to_int_with_scale_for_float!(u16,  f32);
impl_to_int_with_scale_for_float!(u32,  f32);
impl_to_int_with_scale_for_float!(u64,  f32);
impl_to_int_with_scale_for_float!(u128, f32);
impl_to_int_with_scale_for_float!(u8,   f64);
impl_to_int_with_scale_for_float!(u16,  f64);
impl_to_int_with_scale_for_float!(u32,  f64);
impl_to_int_with_scale_for_float!(u64,  f64);
impl_to_int_with_scale_for_float!(u128, f64);

//		impl_to_int_with_scale_for_decimal										
/// Implements the [`ToIntWithScale`] trait for the [`Decimal`] type.
macro_rules! impl_to_int_with_scale_for_decimal {
	(i128) => {
		//󰭅		i128 for Decimal												
		impl ToIntWithScale<i128> for Decimal {
			fn to_int_with_scale(&self, scale: u8) -> Option<i128> {
				//	The integer range of the Decimal type is less than that of an i128, but
				//	we cannot convert first and then scale, because the floating-point
				//	component will be truncated and lost. We therefore need to scale first,
				//	but this restricts the range of the final outcome to that of the Decimal
				//	type, which is 96 bits.
				let factor = 10_u64.checked_pow(u32::from(scale))?;
				(self.checked_mul(Decimal::from(factor))?.round()).to_i128()
			}
		}
	};
	(u128) => {
		//󰭅		u128 for Decimal												
		impl ToIntWithScale<u128> for Decimal {
			fn to_int_with_scale(&self, scale: u8) -> Option<u128> {
				//	The integer range of the Decimal type is less than that of an i128, but
				//	we cannot convert first and then scale, because the floating-point
				//	component will be truncated and lost. We therefore need to scale first,
				//	but this restricts the range of the final outcome to that of the Decimal
				//	type, which is 96 bits.
				let factor = 10_u64.checked_pow(u32::from(scale))?;
				(self.checked_mul(Decimal::from(factor))?.round()).to_u128()
			}
		}
	};
	($t:ty) => {
		//󰭅		Integer for Decimal												
		impl ToIntWithScale<$t> for Decimal {
			fn to_int_with_scale(&self, scale: u8) -> Option<$t> {
				let factor = 10_u64.checked_pow(u32::from(scale))?;
				let scaled = self.checked_mul(Decimal::from(factor))?.round();
				//	Everything less than 128 bits will fit safely into the Decimal's range.
				if scaled > Decimal::from(<$t>::MAX) || scaled < Decimal::from(<$t>::MIN) {
					None
				} else {
					scaled.to_i128().and_then(|value| value.try_into().ok())
				}
			}
		}
	};
}

impl_to_int_with_scale_for_decimal!(i8);
impl_to_int_with_scale_for_decimal!(i16);
impl_to_int_with_scale_for_decimal!(i32);
impl_to_int_with_scale_for_decimal!(i64);
impl_to_int_with_scale_for_decimal!(i128);
impl_to_int_with_scale_for_decimal!(u8);
impl_to_int_with_scale_for_decimal!(u16);
impl_to_int_with_scale_for_decimal!(u32);
impl_to_int_with_scale_for_decimal!(u64);
impl_to_int_with_scale_for_decimal!(u128);

//§		ForceFrom																
/// Simple and safe forced infallible type conversion.
/// 
/// Rust's [`From`] trait provides an infallible (and lossless) mechanism to
/// convert one type to another, and [`TryFrom`] provides the equivalent
/// fallible mechanism. However, it isn't possible to implement both at the same
/// time, because implementing [`From`] brings [`Into`] along for free for the
/// reverse operation, and Rust implements a blanket `impl<T, U> TryFrom<U> for
/// T, where U: Into<T>` meaning that [`TryFrom`] is available (yet infallible)
/// for all implementations of [`From`].
/// 
/// Therefore, this trait exists in order to provide a non-conflicting mechanism
/// for implementing [`From`]-style conversions in situations that *can* fail,
/// but in which failure is not necessarily important.
/// 
/// A good example is that of converting a base64-encoded string into a
/// fixed-length array of bytes: it will likely be important to deal with
/// decoding errors, but the possible truncation may not matter. Therefore,
/// [`TryFrom`] could be implemented for both [`String`] and [`Vec<u8>`](Vec),
/// but additionally [`ForceFrom`] could be implemented for [`Vec<u8>`](Vec).
/// This then ensures that all [`String`] decoding issues will be caught and
/// thought about, but byte data can be chosen to be truncated invisibly, or
/// handled as an error, depending on context.
/// 
/// Handling this as a separate, clearly-signposted approach is more idiomatic
/// than obscuring it behind [`From`], which should always be lossless as well
/// as needing to be infallible. [`ForceFrom`] is essentially a lossy version of
/// [`From`] — which means it should not be used for situations of error that do
/// not relate to loss-associated situations.
/// 
pub trait ForceFrom<T> {
	/// Performs the conversion to this type from the input type.
	fn force_from(value: T) -> Self;
}

//§		IteratorExt																
/// This trait provides additional functionality to [`Iterator`].
pub trait IteratorExt: Iterator {
	//		limit																
	/// Limits the number of items returned by an iterator.
	/// 
	/// This is the same as [`Iterator::take()`], but accepts an [`Option`], so
	/// that the limit does not have to be specified. It allows a match such as
	/// `foo.iter().take(match limit { Some(n) => n, None => foo.len() })`
	/// to be simplified to `foo.iter().limit(limit)`, and is especially useful
	/// when `foo` is of unknown or infinite length.
	/// 
	/// # Parameters
	/// 
	/// * `limit` - The maximum number of items to return. If [`None`], no limit
	///             will be applied.
	/// 
	fn limit(self, limit: Option<usize>) -> LimitIterator<Self> where Self: Sized {
		LimitIterator { iter: self, limit, count: 0 }
	}
}

//󰭅		Iterator																
impl<I: Iterator> IteratorExt for I {}

//§		PathExt																	
/// This trait provides additional functionality to [`Path`].
pub trait PathExt {
	//		append																
	/// Appends a string to a path.
	/// 
	/// Adds a string to the end of a path, and returns the result as a new
	/// path. This is specifically different to both [`push()`](PathBuf::push())
	/// and [`join()`](Path::join()), as it simply appends the string without
	/// having any further effect on the path. By contrast, [`push()`](PathBuf::push())
	/// and [`join()`](Path::join()) will append a new string as a new path
	/// component, which will then be normalized, and will also replace the path
	/// entirely if the string is an absolute path.
	/// 
	/// # Parameters
	/// 
	/// * `suffix` - The string to append to the path.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Path::join()`]
	/// * [`std::path::PathBuf::push()`]
	/// 
	fn append<P: AsRef<Path>>(&self, suffix: P) -> PathBuf;
	
	//		is_subjective														
	/// Checks if the path is specifically relative to the current directory.
	/// 
	/// Returns `true` if the path starts with a reference to the current
	/// directory, i.e. `.` or `..` (as `..` is the parent of the current
	/// directory and therefore related to it), making it specifically and
	/// explicitly related to the current working directory. This can be
	/// described as a subjective relative path, as opposed to an objective
	/// relative path which is generically relative because it lacks a root
	/// component.
	/// 
	/// A path that is subjective is also always relative. It is not possible to
	/// have a subjective absolute path, as that would be a contradiction in
	/// terms. However, objective paths may be either absolute or relative.
	/// There is therefore no method `is_objective()`, as it does not currently
	/// appear to have a useful purpose.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Path::is_absolute()`]
	/// * [`std::path::Path::is_relative()`]
	/// 
	fn is_subjective(&self) -> bool;
	
	//		normalize															
	/// Normalizes the path.
	/// 
	/// Computes the canonicalized, absolute path of a file or directory, but
	/// without expanding symlinks or checking existence. A path that starts
	/// with `.` or without an initial separator will be interpreted relative to
	/// the current working directory (or the filesystem root if the current
	/// working directory is not accessible). Empty paths and paths of `.` alone
	/// will result in the current working directory being returned.
	/// 
	/// This function will normalize the path by removing any `.` and `..`
	/// segments and returning the "real" path. It does this without touching
	/// the filesystem, and so is an abstract but also simpler version of
	/// [`canonicalize()`](Path::canonicalize()), which does a number of
	/// filesystem checks. It does check for the current working directory, on
	/// which to base relative paths, but does not perform any other checks.
	/// 
	/// Key differences are that [`canonicalize()`](Path::canonicalize()) will
	/// return an error if the path does not exist, and will resolve symlinks.
	/// This function will remove `.` segments, and will remove the parent
	/// segment along with the current segment for `..` segments.
	/// 
	/// # See also
	/// 
	/// * [`restrict()`](PathExt::restrict())
	/// * [`std::fs::canonicalize()`]
	/// * [`std::path::Path::canonicalize()`]
	/// 
	fn normalize(&self) -> PathBuf;
	
	//		restrict															
	/// Restricts the path.
	/// 
	/// Computes the canonicalized, absolute path of a file or directory, but
	/// without allowing parent directory traversal to go beyond the base path.
	/// If no base path is specified, the current working directory will be
	/// used. If the path starts with `.` then this will be interpreted relative
	/// to the base path.
	/// 
	/// This function calls [`normalize()`](PathExt::normalize()), and so the
	/// fundamental behaviour of the resolution performed is the same as that
	/// function. The difference is that this function will not allow the path
	/// to go beyond the base path, and so any `..` segments will simply be
	/// removed from the path if they would otherwise go beyond the anchor
	/// point.
	/// 
	/// This does have the effect that if a path does try to traverse too far,
	/// it may lose additional components. For example, a path of `../foo` will
	/// end up losing the `foo` component, as the logic will be that `foo` is
	/// intended to be a sibling to the base path and not a child of it, and is
	/// therefore invalid. So if the base directory is `/home/user` then a path
	/// of `../foo` will be resolved to `/home/user` and not `/home/user/foo`.
	/// The effect of this continues further, in that all children of `foo` will
	/// also be deemed invalid. So `../foo/bar` will also be resolved to
	/// `/home/user`, and not `/home/user/foo/bar` or `/home/user/bar`. Care
	/// should therefore be taken when using this function to ensure that the
	/// path returned is valid for the intended use.
	/// 
	/// In the case of the path being absolute, it will be resolved and then
	/// compared against the base path. If the path is a child of the base path
	/// then it will be returned - otherwise the base path will be returned, as
	/// the path is invalid. For example, if the base directory is `/home/user`
	/// then a path of `/home/user/foo` will be returned, but a path of
	/// `/home/otheruser` will return `/home/user`.
	/// 
	/// Note that this function does not touch the filesystem, does not expand
	/// symlinks, and does not check that the path exists - including the
	/// base path. Hence when this documentation talks about base directory,
	/// it does so interchangeably with base path, as the valid intent would be
	/// for the base path to be a directory, but this is not actually checked.
	/// 
	/// # Parameters
	/// 
	/// * `base` - The base path to use. If this is [`None`] then the current
	///            working directory will be used.
	/// 
	/// # See also
	/// 
	/// * [`normalize()`](PathExt::normalize())
	/// 
	fn restrict<P: AsRef<Path>>(&self, base: P) -> PathBuf;
	
	//		strip_parentdirs													
	/// Removes references to parent directories, i.e. `..`.
	/// 
	/// Removes any [`ParentDir`](std::path::Component::ParentDir) components
	/// from either the beginning of the path or anywhere in the path.
	/// 
	/// This function does not touch the filesystem, or check if the path is
	/// valid or exists. It will also not attempt to resolve the parent
	/// directory references that it removes, so they will be taken out with no
	/// effect on the rest of the path.
	/// 
	/// # Parameters
	/// 
	/// * `remove_all` - If `true` then all parent directory references will be
	///                  removed, otherwise only those at the beginning of the
	///                  path will be removed.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Component`]
	/// * [`std::path::Path::components()`]
	/// 
	fn strip_parentdirs(&self, remove_all: bool) -> PathBuf;
	
	//		strip_root															
	/// Makes the path relative by removing the root and/or prefix components.
	/// 
	/// Removes any components from the path that are considered to be the root
	/// or prefix of the path. The prefix is this context is not the same as in
	/// [`strip_prefix()`](Path::strip_prefix()), which removes a specific
	/// string prefix from the path. Rather, the prefix here is a
	/// [`PrefixComponent`](std::path::PrefixComponent). A path is considered to
	/// be absolute if it has a root on Unix, or if it has both root and prefix
	/// on Windows. Therefore, in order to convert the path to be relative, both
	/// the root and prefix must be removed.
	/// 
	/// This function does not touch the filesystem, or check if the path is
	/// valid or exists. It will also not attempt to resolve special directory
	/// references such as `.` or `..`.
	/// 
	/// # See also
	/// 
	/// * [`std::path::Path::components()`]
	/// * [`std::path::Path::has_root()`]
	/// * [`std::path::Path::is_absolute()`]
	/// * [`std::path::Path::strip_prefix()`]
	/// * [`std::path::Prefix`]
	/// * [`std::path::PrefixComponent`]
	/// 
	fn strip_root(&self) -> PathBuf;
}

//󰭅		Path																	
impl PathExt for Path {
	//		append																
	fn append<P: AsRef<Self>>(&self, suffix: P) -> PathBuf {
		PathBuf::from([
			self.as_os_str().to_os_string(),
			OsString::from(suffix.as_ref()),
		].into_iter().collect::<OsString>())
	}
	
	//		is_subjective														
	fn is_subjective(&self) -> bool {
		self.is_relative() && {
			let mut components = self.components();
			matches!(components.next(), Some(PathComponent::CurDir | PathComponent::ParentDir))
		}
	}
	
	//		normalize															
	fn normalize(&self) -> PathBuf {
		let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
		if self.as_os_str().is_empty() {
			return cwd;
		}
		let mut segments: Vec<OsString> = vec![];
		for (i, component) in self.components().enumerate() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   => {
					if i == 0 {
						segments.push(component.as_os_str().to_os_string());
					}
				},
				PathComponent::CurDir    |
				PathComponent::ParentDir => {
					if i == 0 {
						segments.append(
							cwd.components()
								.map(|c| c.as_os_str().to_os_string())
								.collect::<Vec<OsString>>()
								.as_mut()
						);
					}
					if component == PathComponent::ParentDir && segments.len() > 1 {
						drop(segments.pop());
					}
				},
				PathComponent::Normal(_) => {
					if i == 0 {
						segments.push(cwd.as_os_str().to_os_string());
					}
					segments.push(component.as_os_str().to_os_string());
				},
			}
		}
		segments.iter().collect()
	}
	
	//		restrict															
	fn restrict<P: AsRef<Self>>(&self, base: P) -> PathBuf {
		let basepath = base.as_ref().normalize();
		if self.as_os_str().is_empty() {
			return basepath;
		}
		let mut path = if self.is_absolute() {
			self.to_path_buf()
		} else {
			basepath.join(self)
		}.normalize();
		if !path.starts_with(&basepath) {
			path = basepath;
		}
		path
	}
	
	//		strip_parentdirs													
	fn strip_parentdirs(&self, remove_all: bool) -> PathBuf {
		if self.as_os_str().is_empty() || (!remove_all && self.is_absolute()) {
			return self.to_owned();
		}
		let mut at_start = true;
		let mut segments: Vec<OsString> = vec![];
		for component in self.components() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   |
				PathComponent::CurDir    |
				PathComponent::Normal(_) => {
					segments.push(component.as_os_str().to_os_string());
					at_start = false;
				},
				PathComponent::ParentDir => {
					if !remove_all && !at_start {
						segments.push(component.as_os_str().to_os_string());
					}
				},
			}
		}
		segments.iter().collect()
	}
	
	//		strip_root															
	fn strip_root(&self) -> PathBuf {
		if self.as_os_str().is_empty() || self.is_relative() {
			return self.to_owned();
		}
		let mut segments: Vec<OsString> = vec![];
		for component in self.components() {
			match component {
				PathComponent::Prefix(_) |
				PathComponent::RootDir   => {},
				PathComponent::CurDir    |
				PathComponent::ParentDir |
				PathComponent::Normal(_) => {
					segments.push(component.as_os_str().to_os_string());
				},
			}
		}
		segments.iter().collect()
	}
}


