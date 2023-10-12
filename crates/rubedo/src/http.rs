//! This module provides extensions to the [HTTP](https://crates.io/crates/http),
//! [Hyper](https://crates.io/crates/hyper), and [Axum](https://crates.io/crates/axum)
//! crates.
//! 
//! Hyper and Axum are built on top of the HTTP crate, and Axum uses parts of
//! Hyper, so it makes sense to combine all of these in one module.



//		Modules

#[cfg(test)]
#[path = "tests/http.rs"]
mod tests;



//		Packages

use crate::sugar::s;
use base64::{DecodeError, engine::{Engine as _, general_purpose::STANDARD as BASE64}};
use core::convert::Infallible;
use futures::executor;
use http::{Response, StatusCode};
use http_body::combinators::UnsyncBoxBody;
use hyper::{
	body::{Body as HyperBody, Bytes, to_bytes},
	HeaderMap,
	header::HeaderValue,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value as Json;
use std::{
	borrow::Cow,
	cmp::Ordering,
	error::Error,
	fmt::{Debug, Display, Write, self},
	ops::{Add, AddAssign},
	str::FromStr,
};



//		Enums

//		ContentType																
/// The content type of an HTTP response, for use by [`UnpackedResponseBody`].
/// 
/// The content type is used to determine how to represent and interpret the
/// response body when performing serialisation and deserialisation, including
/// for display.
/// 
/// The default content type is [`Text`](ContentType::Text).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum ContentType {
	/// The response body is text. It will be represented as an ordinary
	/// [`String`] when serialised.
	#[default]
	Text,
	
	/// The response body is binary. It will be represented as a [`String`]
	/// in base64 format when serialised.
	Binary,
}

//		ResponseError															
#[derive(Debug)]
pub enum ResponseError {
	ConversionError,
}

impl Display for ResponseError {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let description = match self {
			ResponseError::ConversionError => s!("Error encountered while converting response body to bytes"),
		};
		write!(f, "{}", description)
	}
}

impl Error for ResponseError {}



//		Structs

//		UnpackedResponse														
/// An HTTP response in comparison-friendly form for interrogation.
/// 
/// Data in [`hyper::Response`] (and indeed [`http::Response`] as well) is
/// stored in a specific form, made up of a header map object and a generic body
/// type, which can be empty, a [`String`], or a streaming body future. This
/// struct provides a way to use the data in a more accessible form, to allow it
/// to be checked and compared. This is useful for testing, as the entire set of
/// headers plus body can be checked all at once, and also for printing/logging.
/// 
/// If specific headers or body content needs to be checked, it is recommended
/// to use the standard functions as they will be more efficient and performant.
/// 
/// Note that the [`body`](UnpackedResponse.body) property, which is stored as a
/// vector of bytes, will get converted to a [`String`] if it is run through the
/// standard [`Debug`] or [`Display`] formatters. This is because human-readable
/// output is the intuitively-expected outcome in this situation. The behaviour
/// can be controlled with the [`ContentType`] enum, which is used to determine
/// whether the data is binary or text. If [`Text`](ContentType::Text), then the
/// conversion uses [`from_utf8_lossy()`](String::from_utf8_lossy()), so no
/// errors will occur, but if the body is not valid UTF8 then the resulting
/// [`String`] will not be exactly the same. If an accurate representation of
/// the body is required then it should be set to [`Binary`](ContentType::Binary),
/// or else it should be extracted and converted to a `Vec<u8>` and then run
/// through the [`Debug`] or [`Display`] formatters directly.
/// 
/// # See also
/// 
/// * [`axum::response`]
/// * [`axum::response::Response`]
/// * [`http::Response`]
/// * [`hyper::Response`]
/// * [`ResponseExt`]
/// * [`ResponseExt::unpack()`]
/// * [`UnpackedResponseHeader`]
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct UnpackedResponse {
	//		Public properties													
	/// The response status code. This is an enum, so is not directly comparable
	/// to a number. The standard [`Display`] formatter will convert it to a
	/// string in the format `"200 OK"`, but the standard [`FromStr`]
	/// implementation will error if this is given back to it, as it expects
	/// only `"200"`. Because this round-trip is basically broken, this struct
	/// provides custom serialisation and deserialisation functions to convert
	/// the status code to and from an actual number (a [`u16`]). This allows
	/// the struct to be serialised and deserialised in a round-trip without
	/// error, and is also the more intuitive representation of the status code
	/// in serialised form such as JSON.
	#[serde(serialize_with = "serialize_status_code", deserialize_with = "deserialize_status_code")]
	pub status: StatusCode,
	
	/// The response headers. These are in a vector rather than a hashmap
	/// because there may be multiple headers with the same name. They are
	/// sorted by name, and then by value, allowing for reliable comparison.
	/// Sorting does break the original order of the headers, but this should
	/// only very rarely matter, even when logging, and sorting allows
	/// duplicates to be spotted by eye more easily in logs.
	pub headers: Vec<UnpackedResponseHeader>,
	
	/// The response body. This originates from the response body as a [`Bytes`]
	/// container, but gets stored here as a vector of bytes for convenience.
	/// This may not be valid UTF8, so is not converted to a [`String`]. That
	/// step is left as optional for the caller, if required (and happens when
	/// running the [`UnpackedResponse`] struct through the [`Debug`] or
	/// [`Display`] formatters).
	pub body:    UnpackedResponseBody,
}

impl PartialEq for UnpackedResponse {
	//		eq																	
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.headers == other.headers && self.body == other.body
    }
}

//		UnpackedResponseHeader													
/// An HTTP response header.
/// 
/// A simple representation of an HTTP response header as a key-value pair. The
/// purpose of this struct is to formalise the data structure used by
/// [`UnpackedResponse`] for storing headers.
/// 
/// # See also
/// 
/// * [`UnpackedResponse`]
/// 
#[derive(Debug, Deserialize, Serialize)]
pub struct UnpackedResponseHeader {
	//		Public properties													
	/// The response header name.
	pub name:  String,
	
	/// The response header value.
	pub value: String,
}

impl PartialEq for UnpackedResponseHeader {
	//		eq																	
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.value == other.value
	}
}

//		UnpackedResponseBody													
/// An HTTP response body.
/// 
/// A simple representation of an HTTP response body as a vector of bytes. The
/// purpose of this struct is to formalise the data structure used by
/// [`UnpackedResponse`] for storing the body.
///
/// The data originates from the response body as a [`Bytes`] container, but
/// gets stored here as a vector of bytes for convenience. This may not be valid
/// UTF8, so is not converted to a [`String`]. That step is left as optional for
/// the caller, if required (and happens when running through the [`Debug`] or
/// [`Display`] formatters).
/// 
/// The conversion to a [`String`] when run through the [`Debug`] and
/// [`Display`] formatters is because human-readable output is the
/// intuitively-expected outcome in this situation. The behaviour can be
/// controlled with the [`ContentType`] enum, which is used to determine whether
/// the data is binary or text. If [`Text`](ContentType::Text), then the
/// conversion uses [`from_utf8_lossy()`](String::from_utf8_lossy()), so no
/// errors will occur, but if the body is not valid UTF8 then the resulting
/// [`String`] will not be exactly the same. If an accurate representation of
/// the body is required then it should be set to [`Binary`](ContentType::Binary),
/// or else it should be extracted and converted to a `Vec<u8>` and then run
/// through the [`Debug`] or [`Display`] formatters directly.
///
/// This struct is very similar in nature to the standard Rust [`String`]
/// struct, in that it is a wrapper around a vector of bytes, and so its design
/// and function names are modelled after it. The main difference is that it
/// does not require its contents to be valid UTF8, and also that it is a tuple
/// struct rather than a regular struct.
///
/// Note that serialisation/deserialisation of this struct directly will produce
/// and expect a [`String`], not a vector of bytes. This is because this is the
/// most useful and fitting behaviour for the intended purpose, as with the
/// implementations of [`Display`] and [`FromStr`]. As noted above, this is
/// lossy if the [`ContentType`] is [`Text`](ContentType::Text) and the data is
/// not valid UTF8, but not if set to [`Binary`](ContentType::Binary).
/// 
/// # See also
/// 
/// * [`UnpackedResponse`]
/// 
#[derive(Default)]
pub struct UnpackedResponseBody {
	//		Private properties													
	/// The response body as a vector of bytes. The data originates from the
	/// response body as a [`Bytes`] container, but gets stored here as a vector
	/// of bytes for convenience. This may not be valid UTF8, so is not
	/// converted to a [`String`]. That step is left as optional for the caller,
	/// if required (and happens when running through the [`Debug`] or
	/// [`Display`] formatters).
	body:         Vec<u8>,
	
	/// The content type of the response body. This is used to determine how to
	/// represent and interpret the response body when performing serialisation
	/// and deserialisation, including for display. The default content type is
	/// [`Text`](ContentType::Text).
	content_type: ContentType,
}

impl UnpackedResponseBody {
	//		new																	
	/// Creates a new response body instance.
	/// 
	/// # Parameters
	/// 
	/// * `data` - The response body as any type for which there is a [`From`]
	///            implementation.
	/// 
	pub fn new<T: Into<Self>>(data: T) -> Self {
		data.into()
	}
	
	//		content_type														
	/// Returns the content type of the response body.
	/// 
	/// # See also
	///
	/// * [`ContentType`]
	/// * [`UnpackedResponseBody::is_binary()`]
	/// * [`UnpackedResponseBody::is_text()`]
	/// * [`UnpackedResponseBody::set_content_type()`]
	/// 
	pub fn content_type(&self) -> ContentType {
		self.content_type
	}
	
	//		set_content_type													
	/// Sets the content type of the response body.
	/// 
	/// This method is chainable, as it returns a mutable reference to the
	/// response body instance.
	/// 
	/// # See also
	///
	/// * [`ContentType`]
	/// * [`UnpackedResponseBody::content_type()`]
	/// 
	pub fn set_content_type(&mut self, content_type: ContentType) -> &mut Self {
		self.content_type = content_type;
		self
	}
	
	//		is_binary															
	/// Returns whether the response body is binary.
	/// 
	/// # See also
	/// 
	/// * [`ContentType`]
	/// * [`UnpackedResponseBody::content_type()`]
	/// * [`UnpackedResponseBody::is_text()`]
	/// 
	pub fn is_binary(&self) -> bool {
		self.content_type == ContentType::Binary
	}
	
	//		is_text																
	/// Returns whether the response body is text.
	///
	/// # See also
	///
	/// * [`ContentType`]
	/// * [`UnpackedResponseBody::content_type()`]
	/// * [`UnpackedResponseBody::is_binary()`]
	///
	pub fn is_text(&self) -> bool {
		self.content_type == ContentType::Text
	}
	
	//		as_bytes															
	/// Returns a byte slice of the response body's contents.
	/// 
	/// Provides a read-only view of the byte data within the response body,
	/// without consuming the data. The returned slice is a reference to the
	/// actual data stored in the response body, not a copy. Because of this, it
	/// is not possible to mutate the contents of the response body through the
	/// returned slice. It does not allocate new memory or change the ownership
	/// of the byte data. This method is useful when you need to work with the
	/// bytes of the response body in a read-only fashion, or when you want to
	/// avoid copying the data.
	/// 
	///   - This method returns a slice (`&[u8]`) referencing the bytes of the
	///     response body contents.
	///   - The original response body value remains intact, and can still be
	///     used afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a view into the original memory.
	/// 
	/// Use this method when you need to work with the byte data in a
	/// non-destructive, read-only manner while keeping the original response
	/// body intact.
	///
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::as_mut_bytes()`]
	/// * [`UnpackedResponseBody::into_bytes()`]
	/// * [`UnpackedResponseBody::to_bytes()`]
	/// 
	pub fn as_bytes(&self) -> &[u8] {
		&self.body
	}
	
	//		as_mut_bytes														
	/// Returns a mutable referenced to the response body's contents.
	/// 
	/// Provides a mutable view of the byte data within the response body,
	/// without consuming the data. The returned vector is a reference to the
	/// actual data stored in the response body, not a copy. This method is
	/// useful when you need to work with, and modify, the bytes of the response
	/// body directly, without copying the data.
	/// 
	///   - This method returns a mutable vector (`&mut Vec<u8>`) referencing
	///     the bytes of the response body contents.
	///   - The original response body value remains intact, and can still be
	///     used afterward.
	///   - No reallocation or copying of data occurs since it's just providing
	///     a reference to the original memory.
	/// 
	/// Use this method when you need to work directly with the byte data in a
	/// mutable manner.
	/// 
	/// Note that unlike the function's [`String::as_mut_vec()`] counterpart,
	/// this method is not unsafe. This is because the response body is not
	/// required to be valid UTF8, so there is no risk of invalid UTF8 being
	/// created.
	/// 
	/// Note also that a better name for this method could be `as_mut_vec()`,
	/// which would be consistent with the standard library's
	/// [`String::as_mut_vec()`] method, which this method is modelled after,
	/// but that would break consistency with the other methods on this struct.
	/// In addition, there is another method called [`str::as_bytes_mut()`],
	/// which appears to be named quite inconsistently with other comparable
	/// methods, and so calling this method `as_mut_bytes()` might cause
	/// confusion, but is at least self-consistent.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::as_bytes()`]
	/// * [`UnpackedResponseBody::into_bytes()`]
	/// * [`UnpackedResponseBody::to_bytes()`]
	/// 
	pub fn as_mut_bytes(&mut self) -> &mut Vec<u8> {
		&mut self.body
	}
	
	//		into_bytes															
	/// Returns the response body as a vector of bytes.
	/// 
	/// This consumes the response body, without cloning or copying, and returns
	/// a new vector containing the bytes of the response body. It transfers
	/// ownership of the byte data from the response body to the new vector.
	/// This method is useful when you need to move the byte data out of the
	/// response body, for example to pass it to a function that expects a
	/// `Vec<u8>`, or when you want to modify the byte data in-place without
	/// affecting the original response body.
	/// 
	///   - This method consumes the response body contents and returns a
	///     `Vec<u8>` containing its bytes.
	///   - After calling this method, the original response body value is no
	///     longer available for use, because it has been moved.
	///   - Transforms the response body into a vector of bytes without any
	///     copying.
	/// 
	/// Use this method when you want to consume the response body and obtain
	/// ownership of its byte data in the form of a `Vec<u8>`. This is useful
	/// when you need to modify or move the byte data, or when you want to pass
	/// it to functions that expect a `Vec<u8>`.
	/// 
	/// Note that a better name for this method might be `into_vec()`, but that
	/// would be inconsistent with the standard library's
	/// [`String::into_bytes()`] method, which this method is modelled after.
	///
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::as_bytes()`]
	/// * [`UnpackedResponseBody::as_mut_bytes()`]
	/// * [`UnpackedResponseBody::to_bytes()`]
	/// 
	pub fn into_bytes(self) -> Vec<u8> {
		self.body
	}
	
	//		to_bytes															
	/// Returns a copy of the response body data converted to a vector of bytes.
	/// 
	/// This does not consume the response body, but clones it. Following Rust's
	/// naming conventions and idioms, this method "converts" the data content
	/// of the response body into a byte representation, in a `Vec<u8>`. (No
	/// actual conversion takes place because the data is already stored
	/// internally as a vector of bytes, but this is academic and could change
	/// in future, so "conversion" is implied and expected as a theoretical
	/// behaviour.) Ownership of the cloned and converted byte data is
	/// transferred to the caller, and there are no side effects on the internal
	/// state of the [`UnpackedResponseBody`] instance.
	/// 
	///   - This method returns a `Vec<u8>` vector of bytes without consuming
	///     the response body contents.
	///   - The original response body value remains intact, and can still be
	///     used afterward.
	///   - The response body data is copied, and converted/transformed into
	///     the output value returned.
	/// 
	/// Use this method when you need to obtain a copy of the response body's
	/// byte data in the form of a `Vec<u8>`, without consuming the response
	/// body itself. This is useful when you need to pass the byte data to a
	/// function that expects a `Vec<u8>`, or when you want to modify the byte
	/// data without affecting the original response body.
	/// 
	/// Note that a better name for this method might be `to_vec()`, but that
	/// would be inconsistent with the standard library's
	/// [`String::into_bytes()`] method.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::as_bytes()`]
	/// * [`UnpackedResponseBody::as_mut_bytes()`]
	/// * [`UnpackedResponseBody::into_bytes()`]
	/// 
	pub fn to_bytes(&self) -> Vec<u8> {
		self.body.clone()
	}
	
	//		to_base64															
	/// Returns the response body data converted to a base64-encoded [`String`].
	/// 
	/// This does not consume the response body, but clones it, as is necessary
	/// to perform the conversion to base64. It converts straight from bytes to
	/// base64, without converting to a [`String`] first, because the response
	/// body is binary data.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::from_base64()`]
	/// 
	pub fn to_base64(&self) -> String {
		BASE64.encode(&self.body)
	}
	
	//		from_base64															
	/// Converts a base64-encoded [`String`] to an [`UnpackedResponseBody`].
	/// 
	/// This method does not consume the input string, but clones it, as is
	/// necessary to perform the conversion from [`base64`]. It converts
	/// straight from base64 to bytes, without converting to a [`String`] first,
	/// because the response body is binary data. This means that no UTF8
	/// validation is performed.
	/// 
	/// Note that unlike the [`From`] type conversion implementations, this
	/// returns a [`Result`].
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::to_base64()`]
	/// 
	pub fn from_base64(encoded: &str) -> Result<Self, DecodeError> {
		let decoded = BASE64.decode(encoded)?;
		Ok(Self { body: decoded, content_type: ContentType::Binary })
	}
	
	//		clear																
	/// Removes all contents from the response body.
	/// 
	/// This method removes all data from the response body, resetting it to an
	/// empty state. This method has no effect on the capacity of the response
	/// body, and so does not affect any allocation.
	/// 
	pub fn clear(&mut self) {
		self.body.clear();
	}
	
	//		empty																
	/// Returns an empty response body.
	/// 
	/// This method returns an empty response body. This is equivalent to
	/// creating a new response body with [`UnpackedResponseBody::new()`], but
	/// without having to supply any parameters.
	/// 
	pub fn empty() -> Self {
		Self { body: Vec::new(), ..Default::default() }
	}
	
	//		is_empty															
	/// Returns whether the response body is empty.
	/// 
	/// This method returns whether the response body is empty. This is
	/// equivalent to checking whether the length of the response body is zero.
	/// 
	pub fn is_empty(&self) -> bool {
		self.body.is_empty()
	}
	
	//		len																	
	/// Returns the length of the response body.
	/// 
	/// This method returns the length of the response body, in bytes. This is
	/// equivalent to the length of the vector of bytes that the response body
	/// contains.
	/// 
	pub fn len(&self) -> usize {
		self.body.len()
	}
	
	//		push																
	/// Appends a byte to the response body.
	/// 
	/// Appends a given byte onto the end of the response body's existing byte
	/// data. The response body is not required to be valid UTF8, so this method
	/// does not check the validity of the byte before appending it.
	/// 
	/// This method accepts a [`u8`] instead of a [`char`] because a [`char`]
	/// represents a single Unicode scalar value. In Rust, a [`char`] is always
	/// 4 bytes long because it can represent any Unicode scalar value,
	/// including those outside the Basic Multilingual Plane. If `push()`
	/// accepted a [`char`], it would be signaling that [`UnpackedResponseBody`]
	/// is Unicode-aware and can handle any Unicode character - which is not the
	/// case. A [`u8`], on the other hand, represents a single byte. By having
	/// `push()` accept a [`u8`], it's signaling that [`UnpackedResponseBody`]
	/// is byte-oriented. A specific [`push_char()`](UnpackedResponseBody::push_char())
	/// method is also available, but `push()` is the most general method for
	/// appending bytes to the response body.
	/// 
	/// # Parameters
	/// 
	/// * `byte` - The byte to append to the response body.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::push_bytes()`]
	/// * [`UnpackedResponseBody::push_char()`]
	/// * [`UnpackedResponseBody::push_str()`]
	/// 
	pub fn push(&mut self, byte: u8) {
		self.body.push(byte)
	}
	
	//		push_bytes															
	/// Appends a byte slice to the response body.
	///
	/// Appends a given byte slice onto the end of the response body. The byte
	/// slice is appended to the end of the response body's existing byte data.
	/// The response body is not required to be valid UTF8, so this method does
	/// not check the validity of the byte slice before appending it.
	///
	/// # Parameters
	///
	/// * `bytes` - The byte slice to append to the response body.
	///
	/// # See also
	///
	/// * [`UnpackedResponseBody::push()`]
	/// * [`UnpackedResponseBody::push_char()`]
	/// * [`UnpackedResponseBody::push_str()`]
	///
	pub fn push_bytes(&mut self, bytes: &[u8]) {
		self.body.extend_from_slice(bytes);
	}
	
	//		push_char															
	/// Appends a [`char`] to the response body.
	///
	/// Appends a given character onto the end of the response body. The
	/// [`char`] is converted to bytes and then appended to the end of the
	/// response body's existing byte data.
	///
	/// # Parameters
	///
	/// * `char` - The [`char`] to append to the response body.
	///
	/// # See also
	///
	/// * [`UnpackedResponseBody::push()`]
	/// * [`UnpackedResponseBody::push_bytes()`]
	/// * [`UnpackedResponseBody::push_str()`]
	///
	pub fn push_char(&mut self, char: &char) {
		let mut bytes = [0; 4];
		let used      = char.encode_utf8(&mut bytes).len();
		self.body.extend(&bytes[..used]);
	}
	
	//		push_str															
	/// Appends a string slice to the response body.
	/// 
	/// Appends a given string slice onto the end of the response body. The
	/// string slice is converted to bytes and then appended to the end of the
	/// response body's existing byte data.
	/// 
	/// # Parameters
	/// 
	/// * `string` - The string slice to append to the response body.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::push()`]
	/// * [`UnpackedResponseBody::push_char()`]
	/// * [`UnpackedResponseBody::push_bytes()`]
	/// 
	pub fn push_str(&mut self, string: &str) {
		self.body.extend_from_slice(string.as_bytes());
	}
}

impl Add<&[u8]> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html) to
	/// an [`UnpackedResponseBody`].
	fn add(mut self, other: &[u8]) -> Self {
		self.push_bytes(other);
		self
	}
}

impl<const N: usize> Add<&[u8; N]> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to an [`UnpackedResponseBody`].
	fn add(mut self, other: &[u8; N]) -> Self {
		self.push_bytes(other);
		self
	}
}

impl Add<char> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`char`] to an [`UnpackedResponseBody`].
	fn add(mut self, other: char) -> Self {
		self.push_char(&other);
		self
	}
}

impl Add<&char> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&char`](char) to an [`UnpackedResponseBody`].
	fn add(mut self, other: &char) -> Self {
		self.push_char(other);
		self
	}
}

impl Add<&str> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&str`](str) to an [`UnpackedResponseBody`].
	fn add(mut self, other: &str) -> Self {
		self.push_str(other);
		self
	}
}

impl Add<String> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`String`] to an [`UnpackedResponseBody`].
	fn add(mut self, other: String) -> Self {
		self.push_str(&other);
		self
	}
}

impl Add<&String> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&String`](String) to an [`UnpackedResponseBody`].
	fn add(mut self, other: &String) -> Self {
		self.push_str(other);
		self
	}
}

impl Add<Box<str>> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [boxed](Box) [string](str) slice to an [`UnpackedResponseBody`].
	fn add(mut self, other: Box<str>) -> Self::Output {
		self.push_str(&other);
		self
	}
}

impl<'a> Add<Cow<'a, str>> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [clone-on-write](Cow) [string](str) to an
	/// [`UnpackedResponseBody`].
	fn add(mut self, other: Cow<'a, str>) -> Self::Output {
		self.push_str(&other);
		self
	}
}

impl Add<u8> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`u8`] to an [`UnpackedResponseBody`].
	fn add(mut self, other: u8) -> Self {
		self.push(other);
		self
	}
}

impl Add<Vec<u8>> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn add(mut self, other: Vec<u8>) -> Self {
		self.push_bytes(&other);
		self
	}
}

impl Add<&Vec<u8>> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds a [`&Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn add(mut self, other: &Vec<u8>) -> Self {
		self.push_bytes(other);
		self
	}
}

impl Add<UnpackedResponseBody> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds an [`UnpackedResponseBody`] to an [`UnpackedResponseBody`].
	fn add(mut self, other: Self) -> Self {
		self.push_bytes(&other.body);
		self
	}
}

impl Add<&UnpackedResponseBody> for UnpackedResponseBody {
	type Output = Self;
	
	//		add																	
	/// Adds an [`&UnpackedResponseBody`](UnpackedResponseBody) to an
	/// [`UnpackedResponseBody`].
	fn add(mut self, other: &Self) -> Self {
		self.push_bytes(other.as_bytes());
		self
	}
}

impl AddAssign<&[u8]> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html) to
	/// an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &[u8]) {
		self.push_bytes(other);
	}
}

impl<const N: usize> AddAssign<&[u8; N]> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &[u8; N]) {
		self.push_bytes(other);
	}
}

impl AddAssign<char> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`char`] to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: char) {
		self.push_char(&other);
	}
}

impl AddAssign<&char> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&char`](char) to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &char) {
		self.push_char(other);
	}
}

impl AddAssign<&str> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&str`](str) to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &str) {
		self.push_str(other);
	}
}

impl AddAssign<String> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`String`] to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: String) {
		self.push_str(&other);
	}
}

impl AddAssign<&String> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&String`](String) to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &String) {
		self.push_str(other);
	}
}

impl AddAssign<Box<str>> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [boxed](Box) [string](str) slice to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: Box<str>) {
		self.push_str(&other);
	}
}

impl<'a> AddAssign<Cow<'a, str>> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [clone-on-write](Cow) [string](str) to an
	/// [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: Cow<'a, str>){
		self.push_str(&other);
	}
}

impl AddAssign<u8> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`u8`] to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: u8) {
		self.push(other);
	}
}

impl AddAssign<Vec<u8>> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: Vec<u8>) {
		self.push_bytes(&other);
	}
}

impl AddAssign<&Vec<u8>> for UnpackedResponseBody {
	//		add_assign															
	/// Adds a [`&Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &Vec<u8>) {
		self.push_bytes(other);
	}
}

impl AddAssign<UnpackedResponseBody> for UnpackedResponseBody {
	//		add_assign															
	/// Adds an [`UnpackedResponseBody`] to an [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: Self) {
		self.push_bytes(&other.body);
	}
}

impl AddAssign<&UnpackedResponseBody> for UnpackedResponseBody {
	//		add_assign															
	/// Adds an [`&UnpackedResponseBody`](UnpackedResponseBody) to an
	/// [`UnpackedResponseBody`].
	fn add_assign(&mut self, other: &Self) {
		self.push_bytes(other.as_bytes());
	}
}

impl AsMut<[u8]> for UnpackedResponseBody {
	//		as_mut																
	fn as_mut(&mut self) -> &mut [u8] {
		self.as_mut_bytes()
	}
}

impl AsRef<[u8]> for UnpackedResponseBody {
	//		as_ref																
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl Clone for UnpackedResponseBody {
	//		clone																
	fn clone(&self) -> Self {
		Self { body: self.body.clone(), ..Default::default() }
	}
	
	//		clone_from															
	fn clone_from(&mut self, source: &Self) {
		self.body.clone_from(&source.body);
	}
}

impl Debug for UnpackedResponseBody {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("UnpackedResponseBody")
			.field("body",         &self.to_string())
			.field("content_type", &self.content_type)
			.finish()
	}
}

impl Display for UnpackedResponseBody {
	//		fmt																	
	/// Formats the response body for display.
	///
	/// This method serialises the response body based on the content type. If
	/// the content type is [`ContentType::Text`], then the response body is
	/// serialised to an ordinary [`String`]. If the content type is
	/// [`ContentType::Binary`], then the response body is serialised to a
	/// base64-encoded [`String`].
	///
	/// Note that as no validation checks are performed on the response body
	/// contents, it is not guaranteed to be UTF8, and therefore if not
	/// specified as binary it is possible that the serialised string will not
	/// totally match the original response body contents. This is because the
	/// conversion of the response body bytes to a UTF8 string will be lossy if
	/// there are invalid characters.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::serialize()`]
	/// * [`UnpackedResponseBody::to_base64()`]
	/// 
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let body                =  match self.content_type {
			ContentType::Text   => String::from_utf8_lossy(&self.body),
			ContentType::Binary => Cow::Owned(self.to_base64()),
		};
		write!(f, "{}", body)
	}
}

impl From<&[u8]> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&[u8]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to an [`UnpackedResponseBody`].
	fn from(b: &[u8]) -> Self {
		UnpackedResponseBody { body: b.to_vec(), ..Default::default() }
	}
}

impl<const N: usize> From<&[u8; N]> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&[u8; N]`](https://doc.rust-lang.org/std/primitive.slice.html)
	/// to an [`UnpackedResponseBody`].
	fn from(b: &[u8; N]) -> Self {
		UnpackedResponseBody { body: b.to_vec(), ..Default::default() }
	}
}

impl From<char> for UnpackedResponseBody {
	//		from																
	/// Converts a [`char`] to an [`UnpackedResponseBody`].
	/// 
	/// Note that it does this in the way that is most compatible with
	/// [`String`] conversion. The [`char`] type in Rust represents a Unicode
	/// scalar value. That means a single [`char`] value corresponds to one
	/// Unicode character. But Unicode characters can have a wide range of
	/// values, from `0` to `0x10FFFF` (this range excludes the surrogate
	/// pairs), and this value range doesn't fit into a single byte. That's why
	/// [`char`] in Rust is 4 bytes, because it has to accommodate any possible
	/// Unicode scalar value. The UTF-8 encoded representation of the `ñ`
	/// character is `[195, 177]`, but in memory, a [`char`] containing `'ñ'`
	/// does not hold the bytes `[195, 177]`. Instead, it holds the Unicode
	/// scalar value for `ñ`, which is `U+00F1`, or in integer terms, `241`.
	/// When we convert a [`char`] to a [`u32`] directly, we're taking this
	/// scalar value (like `241` for `ñ`) and representing it in memory as a
	/// 4-byte integer. So using code such as
	/// `(c as u32).to_le_bytes().to_vec()` would result in [241, 0, 0, 0], and
	/// not [195, 177]. This behaviour would not match expectation and would not
	/// match the behaviour of [`String`] conversion. To get the UTF-8 encoded
	/// bytes of a [`char`], we need to use encoding methods because we're
	/// effectively translating from the Unicode scalar value to its UTF-8 byte
	/// sequence. This is what the [`encode_utf8()`](char::encode_utf8()) method
	/// provides. To put it another way: [`char`] isn't storing bytes, it's
	/// storing a Unicode scalar value. UTF-8 is one of the ways to represent
	/// that value (and the most common one in Rust).
	/// 
	fn from(c: char) -> Self {
		let mut bytes = [0; 4];
		let used      = c.encode_utf8(&mut bytes).len();
		Self { body: bytes[..used].to_vec(), ..Default::default() }
	}
}

impl From<&char> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&char`](char) to an [`UnpackedResponseBody`].
	fn from(c: &char) -> Self {
		Self::from(c.to_owned())
	}
}

impl From<Json> for UnpackedResponseBody {
	//		from																
	/// Converts a [`serde_json::Value`] to an [`UnpackedResponseBody`].
	fn from(j: Json) -> Self {
		Self { body: j.to_string().into_bytes(), ..Default::default() }
	}
}

impl From<&Json> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&serde_json::Value`](serde_json::Value) to an
	/// [`UnpackedResponseBody`].
	fn from(j: &Json) -> Self {
		Self { body: j.to_string().into_bytes(), ..Default::default() }
	}
}

impl From<HyperBody> for UnpackedResponseBody {
	//		from																
	/// Converts a [`UnsyncBoxBody<Bytes, E>`](UnsyncBoxBody) to an
	/// [`UnpackedResponseBody`].
	fn from(b: HyperBody) -> Self {
		let bytes    =  executor::block_on(to_bytes(b));
		let body     =  match bytes {
			Ok(body) => body.to_vec(),
			Err(_)   => b"Conversion error".to_vec(),
		};
		Self { body, ..Default::default() }
	}
}

impl From<&str> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&str`](str) to an [`UnpackedResponseBody`].
	fn from(s: &str) -> Self {
		Self { body: s.to_owned().as_bytes().to_vec(), ..Default::default() }
	}
}

impl From<&mut str> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&mut str`](str) to an [`UnpackedResponseBody`].
	fn from(s: &mut str) -> Self {
		Self { body: s.to_owned().as_bytes().to_vec(), ..Default::default() }
	}
}

impl From<String> for UnpackedResponseBody {
	//		from																
	/// Converts a [`String`] to an [`UnpackedResponseBody`].
	fn from(s: String) -> Self {
		Self { body: s.into_bytes(), ..Default::default() }
	}
}

impl From<&String> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&String`](String) to an [`UnpackedResponseBody`].
	fn from(s: &String) -> Self {
		Self { body: s.as_str().as_bytes().to_vec(), ..Default::default() }
	}
}

impl From<Box<str>> for UnpackedResponseBody {
	//		from																
	/// Converts a [boxed](Box) [string](str) slice to an
	/// [`UnpackedResponseBody`].
	fn from(s: Box<str>) -> Self {
		Self { body: s.into_string().into_bytes(), ..Default::default() }
	}
}

impl<'a> From<Cow<'a, str>> for UnpackedResponseBody {
	//		from																
	/// Converts a [clone-on-write](Cow) [string](str) to an
	/// [`UnpackedResponseBody`].
	fn from(s: Cow<'a, str>) -> Self {
		Self { body: s.into_owned().into_bytes(), ..Default::default() }
	}
}

impl<E> From<UnsyncBoxBody<Bytes, E>> for UnpackedResponseBody
where
	E: Error + 'static,
{
	//		from																
	/// Converts a [`UnsyncBoxBody<Bytes, E>`](UnsyncBoxBody) to an
	/// [`UnpackedResponseBody`].
	fn from(b: UnsyncBoxBody<Bytes, E>) -> Self {
		let bytes    =  executor::block_on(to_bytes(b));
		let body     =  match bytes {
			Ok(body) => body.to_vec(),
			Err(_)   => b"Conversion error".to_vec(),
		};
		Self { body, ..Default::default() }
	}
}

impl From<u8> for UnpackedResponseBody {
	//		from																
	/// Converts a [`u8`] to an [`UnpackedResponseBody`].
	fn from(c: u8) -> Self {
		Self { body: Vec::from([c]), ..Default::default() }
	}
}

impl From<Vec<u8>> for UnpackedResponseBody {
	//		from																
	/// Converts a [`Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn from(v: Vec<u8>) -> Self {
		Self { body: v, ..Default::default() }
	}
}

impl From<&Vec<u8>> for UnpackedResponseBody {
	//		from																
	/// Converts a [`&Vec[u8]`](Vec) to an [`UnpackedResponseBody`].
	fn from(v: &Vec<u8>) -> Self {
		Self { body: v.clone(), ..Default::default() }
	}
}

impl FromStr for UnpackedResponseBody {
	type Err = Infallible;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(Self { body: s.as_bytes().to_vec(), ..Default::default() })
	}
}

impl PartialEq for UnpackedResponseBody {
	//		eq																	
	fn eq(&self, other: &Self) -> bool {
		self.body == other.body
	}
}

impl Serialize for UnpackedResponseBody {
	//		serialize															
	/// Serialises the response body to a [`String`].
	/// 
	/// This method serialises the response body based on the content type. If
	/// the content type is [`ContentType::Text`], then the response body is
	/// serialised to an ordinary [`String`]. If the content type is
	/// [`ContentType::Binary`], then the response body is serialised to a
	/// base64-encoded [`String`].
	/// 
	/// Note that as no validation checks are performed on the response body
	/// contents, it is not guaranteed to be UTF8, and therefore if not
	/// specified as binary it is possible that the serialised string will not
	/// totally match the original response body contents. This is because the
	/// conversion of the response body bytes to a UTF8 string will be lossy if
	/// there are invalid characters.
	/// 
	/// # See also
	/// 
	/// * [`UnpackedResponseBody::deserialize()`]
	/// * [`UnpackedResponseBody::<Display>fmt()`]
	/// * [`UnpackedResponseBody::to_base64()`]
	/// 
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

impl <'de> Deserialize<'de> for UnpackedResponseBody {
	//		deserialize															
	/// Deserialises the response body from a [`String`].
	/// 
	/// This method deserialises the response body based on the content type.
	/// However, as this method is not an instance method and it is not possible
	/// to specify the content type in advance, it has to try to detect it. It
	/// does this by attempting to decode the string as base64. If this succeeds
	/// then it will set the content type as [`ContentType::Binary`]. If this
	/// fails then it will assume the content type is [`ContentType::Text`], and
	/// deserialises the string in standard fashion.
	/// 
	/// Note that as the incoming data is from a [`String`], and Rust strings
	/// are are all valid UTF8, the resulting deserialised response body is
	/// guaranteed to be UTF8 if the content type is determined to be
	/// [`ContentType::Text`]. If base64 is detected then the deserialised bytes
	/// are not guaranteed to be valid UTF8, as no validation checks of that
	/// nature are performed against the response body.
	///
	/// # See also
	///
	/// * [`UnpackedResponseBody::deserialize()`]
	/// * [`UnpackedResponseBody::from_base64()`]
	///
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let string = String::deserialize(deserializer)?;
		match BASE64.decode(&string) {
			Ok(decoded) => Ok(Self { body: decoded,             content_type: ContentType::Binary }),
			Err(_)      => Ok(Self { body: string.into_bytes(), content_type: ContentType::Text }),
		}
	}
}

impl Write for UnpackedResponseBody {
	//		write_str															
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.push_str(s);
		Ok(())
	}
}



//		Traits

//§		ResponseExt																
/// This trait provides additional functionality to [`Response`].
pub trait ResponseExt {
	//		unpack																
	/// Returns an [`UnpackedResponse`] containing the unpacked response data.
	/// 
	/// This will unpack the response and provide the headers and body in a
	/// more accessible form, to allow it to be checked and compared. This is
	/// useful for testing, as the entire set of headers plus body can be
	/// checked all at once, and also for printing/logging.
	/// 
	/// If specific headers or body content needs to be checked, it is
	/// recommended to use the standard functions as they will be more
	/// efficient and performant. Notably, this function will consume the
	/// response body, which is necessary because the response might be
	/// streamed. In order to provide the full response, the whole body must be
	/// read first. This will obviously use more memory than would be used under
	/// normal circumstances, so it is not recommended to use this function
	/// without considering purpose and effect. For tests, ensuring a response
	/// body matches, this is fine, as the data is known and constrained, and
	/// memory/performance is less of a concern.
	/// 
	/// # See also
	/// 
	/// * [`axum::response`]
	/// * [`axum::response::Response`]
	/// * [`http::Response`]
	/// * [`hyper::Response`]
	/// * [`UnpackedResponse`]
	/// 
	fn unpack(&mut self) -> Result<UnpackedResponse, ResponseError>;
}

impl ResponseExt for Response<()> {
	//		unpack																
	fn unpack(&mut self) -> Result<UnpackedResponse, ResponseError> {
		Ok(convert_response(self.status(), self.headers(), Bytes::new()))
	}
}

impl<E> ResponseExt for Response<UnsyncBoxBody<Bytes, E>>
where
	E: Error + 'static,
{
	//		unpack																
	fn unpack(&mut self) -> Result<UnpackedResponse, ResponseError> {
		let body = executor::block_on(to_bytes(self.body_mut()));
		match body {
			Ok(body) => Ok(convert_response(self.status(), self.headers(), body)),
			Err(_)   => Err(ResponseError::ConversionError),
		}
	}
}

impl ResponseExt for Response<HyperBody> {
	//		unpack																
	fn unpack(&mut self) -> Result<UnpackedResponse, ResponseError> {
		let body = executor::block_on(to_bytes(self.body_mut()));
		match body {
			Ok(body) => Ok(convert_response(self.status(), self.headers(), body)),
			Err(_)   => Err(ResponseError::ConversionError),
		}
	}
}

impl ResponseExt for Response<String> {
	//		unpack																
	fn unpack(&mut self) -> Result<UnpackedResponse, ResponseError> {
		let body = executor::block_on(to_bytes(self.body_mut())).unwrap();  //  Infallible
		Ok(convert_response(self.status(), self.headers(), body))
	}
}



//		Functions

//		convert_headers															
/// Returns a vector of unpacked response headers.
/// 
/// These are returned in a vector rather than a hashmap because there may be
/// multiple headers with the same name. They are sorted by name, and then by
/// value, allowing for reliable comparison. Sorting does break the original
/// order of the headers, but this should only very rarely matter.
/// 
/// # See also
/// 
/// * [`ResponseExt::unpack()`]
/// * [`UnpackedResponse`]
/// * [`UnpackedResponseHeader`]
/// 
fn convert_headers(headermap: &HeaderMap<HeaderValue>) -> Vec<UnpackedResponseHeader> {
	let mut headers = vec![];
	for (name, value) in headermap {
		let name    = name.as_str().to_owned();
		let value   = String::from_utf8_lossy(value.as_bytes()).into_owned();
		headers.push(UnpackedResponseHeader { name, value });
	}
	headers.sort_by(|a, b| {
		match a.name.cmp(&b.name) {
			Ordering::Equal => a.value.cmp(&b.value),
			other           => other,
		}
	});
	headers
}

//		convert_response														
/// Returns an [`UnpackedResponse`] containing the unpacked response data.
/// 
/// This function carries out the common part of the conversion process for
/// [`ResponseExt::unpack()`]. As [`unpack()`](ResponseExt::unpack()) has a
/// number of implementations, the common code is abstracted out into this
/// function.
/// 
/// # Parameters
/// 
/// * `status`  - The response status code.
/// * `headers` - The response headers.
/// * `body`    - The response body.
/// 
/// # See also
///
/// * [`axum::response`]
/// * [`axum::response::Response`]
/// * [`http::Response`]
/// * [`hyper::Response`]
/// * [`ResponseExt::unpack()`]
/// * [`UnpackedResponse`]
/// * [`UnpackedResponseHeader`]
/// 
fn convert_response(
	status:  StatusCode,
	headers: &HeaderMap<HeaderValue>,
	body:    Bytes,
) -> UnpackedResponse {
	UnpackedResponse {
		status,
		headers: convert_headers(headers),
		body:    UnpackedResponseBody { body: body.to_vec(), ..Default::default() },
	}
}

//		serialize_status_code													
/// Returns the status code as a number.
///
/// This function is used by [`serde`] to serialise the status code as a number
/// rather than an enum. This is necessary because the [`UnpackedResponse`]
/// struct is used for comparison, and the status code is not directly
/// comparable to a number.
///
/// # Parameters
///
/// * `status_code` - The status code to serialise.
/// * `serializer`  - The serialiser to use.
///
/// # See also
///
/// * [`deserialize_status_code()`]
/// * [`http::StatusCode`]
/// * [`UnpackedResponse`]
///
fn serialize_status_code<S>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	serializer.serialize_u16(status_code.as_u16())
}

//		deserialize_status_code													
/// Returns the status code as an enum.
///
/// This function is used by [`serde`] to deserialise the status code as an
/// enum rather than a number. This is necessary because the
/// [`UnpackedResponse`] struct is used for comparison, and the status code is
/// not directly comparable to a number.
///
/// # Parameters
///
/// * `deserializer` - The deserialiser to use.
///
/// # See also
///
/// * [`http::StatusCode`]
/// * [`serialize_status_code()`]
/// * [`UnpackedResponse`]
///
fn deserialize_status_code<'de, D>(deserializer: D) -> Result<StatusCode, D::Error>
where
	D: Deserializer<'de>,
{
	let status_code_value: u16 = Deserialize::deserialize(deserializer)?;
	let status_code            = StatusCode::from_u16(status_code_value).map_err(serde::de::Error::custom)?;
	Ok(status_code)
}


