//! This module provides extensions to the [HTTP](https://crates.io/crates/http),
//! [Hyper](https://crates.io/crates/hyper), and [Axum](https://crates.io/crates/axum)
//! crates.
//! 
//! Hyper and Axum are built on top of the HTTP crate, and Axum uses parts of
//! Hyper, so it makes sense to combine all of these in one module.



//		Modules

#[cfg(test)]
#[path = "tests/hyper.rs"]
mod tests;



//		Packages

use crate::sugar::s;
use futures::executor;
use http::{Response, StatusCode};
use http_body::combinators::UnsyncBoxBody;
use hyper::{
	body::{Body as HyperBody, Bytes, to_bytes},
	HeaderMap,
	header::HeaderValue,
};
use std::{
	cmp::Ordering,
	error::Error,
	fmt::{Debug, Display, self},
};



//		Enums

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
/// standard [`Debug`] formatter. This is because human-readable output is the
/// intuitively-expected outcome in this situation. The conversion uses
/// [`from_utf8_lossy()`](String::from_utf8_lossy()), so no errors will occur,
/// but if the body is not valid UTF8 then the resulting `String` will not be
/// exactly the same. If an accurate representation of the body is required,
/// then it should be run through the `Debug` formatter directly.
/// 
/// # See Also
/// 
/// * [`axum::response`]
/// * [`axum::response::Response`]
/// * [`http::Response`]
/// * [`hyper::Response`]
/// * [`ResponseExt`]
/// * [`ResponseExt::unpack()`]
/// * [`UnpackedResponseHeader`]
/// 
pub struct UnpackedResponse {
	//		Public properties													
	/// The response status code. This is an enum, so is not directly comparable
	/// to a number. It can be converted to a number, but this is not done here
	/// because it is not necessary for the purposes of this struct.
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
	/// running the `UnpackedResponse` struct through the [`Debug`] formatter).
	pub body:    Vec<u8>,
}

impl PartialEq for UnpackedResponse {
	//		eq																	
    fn eq(&self, other: &Self) -> bool {
        self.status == other.status && self.headers == other.headers && self.body == other.body
    }
}

impl Debug for UnpackedResponse {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let body = String::from_utf8_lossy(&self.body);
		f.debug_struct("UnpackedResponse")
			.field("status",  &self.status)
			.field("headers", &self.headers)
			.field("body",    &body)
			.finish()
	}
}

//		UnpackedResponseHeader													
/// An HTTP response header.
/// 
/// A simple representation of an HTTP response header as a key-value pair. The
/// purpose of this struct is to formalise the data structure used by
/// [`UnpackedResponse`] for storing headers.
/// 
/// # See Also
/// 
/// * [`UnpackedResponse`]
/// 
#[derive(Debug)]
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
	/// # See Also
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
/// # See Also
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
/// [`unpack()`]. As `unpack()` has a number of implementations, the common code
/// is abstracted out into this function.
/// 
/// # Parameters
/// 
/// * `status`  - The response status code.
/// * `headers` - The response headers.
/// * `body`    - The response body.
/// 
/// # See Also
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
		body:    body.to_vec(),
	}
}


