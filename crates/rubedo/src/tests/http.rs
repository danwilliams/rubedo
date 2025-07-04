//		Packages																										

use super::*;
use crate::sugar::s;
use assert_json_diff::assert_json_eq;
use axum::response::IntoResponse as _;
use bytes::Bytes;
use claims::{assert_err, assert_ok, assert_ok_eq};
use core::{
	convert::Infallible,
	future::Future,
	net::SocketAddr,
	pin::Pin,
	str::from_utf8,
};
use http::{Request, Response};
use http_body_util::Full;
use hyper::{
	body::Incoming,
	server::conn::http1,
	service::Service,
};
use hyper_util::{
	client::legacy::Client,
	rt::{TokioExecutor, TokioIo},
};
use serde_assert::{
	Deserializer as TestDeserializer,
	Serializer   as TestSerializer,
	token::Token,
};
use serde_json::json;
use tokio::{
	net::TcpListener,
	spawn,
};



//		Structs																											

//		TestService																
struct TestService(Bytes);

//󰭅		TestService																
impl TestService {
	const fn new(data: Bytes) -> Self {
		Self(data)
	}
}

//󰭅		Service																	
impl Service<Request<Incoming>> for TestService {
	type Response = Response<Full<Bytes>>;
	type Error    = Infallible;
	type Future   = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
	
	//		call																
	fn call(&self, _req: Request<Incoming>) -> Self::Future {
		let data = self.0.clone();
		Box::pin(async move { Ok(Response::new(Full::new(data))) })
	}
}



//		Functions																										

//		create_incoming_for_testing												
async fn create_incoming_for_testing(data: Bytes) -> Incoming {
	//	Bind to a local address on a random port. It isn't ideal to have to use
	//	actual network calls to test Incoming, but there is no way to create an
	//	Instance directly, and this is the only way to actually obtain one.
	let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))).await.unwrap();
	let address  = listener.local_addr().unwrap();
	
	//	Spawn the server task to listen and send the specified data back. This
	//	will respond to a single request and then stop.
	let handle   = spawn(async move {
		let (stream, _) = listener.accept().await.unwrap();
		let io          = TokioIo::new(stream);
		let service     = TestService::new(data.clone());
		
		http1::Builder::new().serve_connection(io, service).await.expect("Error serving connection");
	});
	
	//	Create a client and send a request
	let incoming = Client::builder(TokioExecutor::new())
		.build_http::<Full<Bytes>>()
		.get(format!("http://{address}").parse().unwrap()).await.unwrap()
		.into_body()
	;
	
	handle.await.unwrap();
	incoming
}



//		Tests																											

//		ResponseError															
#[cfg(test)]
mod response_error {
	use super::*;
	
	#[derive(Debug)]
	struct TestError;
	
	impl Display for TestError {
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			write!(f, "Test error")
		}
	}
	
	impl Error for TestError {}
	
	//		debug																
	#[test]
	fn debug() {
		let err = Err::<ResponseError, _>(ResponseError::ConversionError(Box::new(TestError)));
		assert_err!(&err);
		assert_eq!(format!("{err:?}"), "Err(ConversionError(TestError))");
	}
	
	//		display																
	#[test]
	fn display() {
		let err = ResponseError::ConversionError(Box::new(TestError));
		assert_eq!(err.to_string(), "Error encountered while converting response body to bytes: Test error");
	}
}

//		UnpackedResponse														
#[cfg(test)]
mod unpacked_response__struct {
	use super::*;
	
	//		new																	
	#[test]
	fn new() {
		let headers  = vec![
			(s!("Foo"), s!("Bar")),
			(s!("Bar"), s!("Baz")),
		];
		let body     = "This is a test";
		let unpacked = UnpackedResponse::new(StatusCode::OK, headers, body);
		assert_eq!(unpacked, UnpackedResponse {
			status:  StatusCode::OK,
			headers: vec![
				UnpackedResponseHeader { name: s!("Foo"), value: s!("Bar") },
				UnpackedResponseHeader { name: s!("Bar"), value: s!("Baz") },
			],
			body:    UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
	}
	
	//		new_from_parts														
	#[test]
	fn new_from_parts() {
		let headers  = vec![
			UnpackedResponseHeader::new(s!("Foo"), s!("Bar")),
			UnpackedResponseHeader::new(s!("Bar"), s!("Baz")),
		];
		let body     = UnpackedResponseBody::new("This is a test");
		let unpacked = UnpackedResponse::new_from_parts(StatusCode::OK, headers, body);
		assert_eq!(unpacked, UnpackedResponse {
			status:  StatusCode::OK,
			headers: vec![
				UnpackedResponseHeader { name: s!("Foo"), value: s!("Bar") },
				UnpackedResponseHeader { name: s!("Bar"), value: s!("Baz") },
			],
			body:    UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
	}
}

#[cfg(test)]
mod unpacked_response__traits {
	use super::*;
	
	//		debug																
	#[test]
	fn debug() {
		let response     = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_eq!(format!("{response:?}"), r#"UnpackedResponse { status: 200, headers: [UnpackedResponseHeader { name: "foo", value: "bar" }], body: UnpackedResponseBody { body: "This is a test", content_type: Text } }"#);
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let response     = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::NOT_FOUND,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
		assert_eq!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("baz"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("baz"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is different".to_vec(), ..Default::default() },
		});
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let response       = UnpackedResponse {
			status:          StatusCode::OK,
			headers:         vec![
				UnpackedResponseHeader {
					name:    s!("foo"),
					value:   s!("bar"),
				},
			],
			body:            UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		let json           = json!({
			"status":        200,
			"headers":       [
				{
					"name":  "foo",
					"value": "bar",
				},
			],
			"body":          "This is a test",
		});
		assert_json_eq!(json!(response), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let response       = UnpackedResponse {
			status:          StatusCode::OK,
			headers:         vec![
				UnpackedResponseHeader {
					name:    s!("foo"),
					value:   s!("bar"),
				},
			],
			body:            UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		let json           = json!({
			"status":        200,
			"headers":       [
				{
					"name":  "foo",
					"value": "bar",
				},
			],
			"body":          "This is a test",
		}).to_string();
		assert_ok_eq!(serde_json::from_str::<UnpackedResponse>(&json), response);
	}
}

//		UnpackedResponseHeader													
#[cfg(test)]
mod unpacked_response_header__struct {
	use super::*;
	
	//		new																	
	#[test]
	fn new() {
		let header = UnpackedResponseHeader::new(s!("Foo"), s!("Bar"));
		assert_eq!(header, UnpackedResponseHeader { name: s!("Foo"), value: s!("Bar") });
	}
}

#[cfg(test)]
mod unpacked_response_header__traits {
	use super::*;
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let header = UnpackedResponseHeader {
			name:    s!("foo"),
			value:   s!("bar"),
		};
		assert_eq!(header, UnpackedResponseHeader {
			name:    s!("foo"),
			value:   s!("bar"),
		});
		assert_ne!(header, UnpackedResponseHeader {
			name:    s!("foo"),
			value:   s!("baz"),
		});
		assert_ne!(header, UnpackedResponseHeader {
			name:    s!("baz"),
			value:   s!("bar"),
		});
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let headers    = vec![
			UnpackedResponseHeader {
				name:    s!("foo"),
				value:   s!("bar"),
			},
		];
		let json       = json!([
			{
				"name":  "foo",
				"value": "bar",
			},
		]);
		assert_json_eq!(json!(headers), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let headers    = vec![
			UnpackedResponseHeader {
				name:    s!("foo"),
				value:   s!("bar"),
			},
		];
		let json       = json!([
			{
				"name":  "foo",
				"value": "bar",
			},
		]).to_string();
		assert_ok_eq!(serde_json::from_str::<Vec<UnpackedResponseHeader>>(&json), headers);
	}
}

//		UnpackedResponseBody													
#[cfg(test)]
mod unpacked_response_body__struct {
	use super::*;
	
	//		new																	
	#[test]
	fn new() {
		let body1 = UnpackedResponseBody::new(b"This is a test".to_vec());
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let body2 = UnpackedResponseBody::new("This is a test");
		assert_eq!(body2, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		content_type														
	#[test]
	fn content_type() {
		let body1       = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text
		};
		assert_eq!(body1.content_type(), ContentType::Text);
		
		let body2       = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Binary
		};
		assert_eq!(body2.content_type(), ContentType::Binary);
	}
	
	//		set_content_type													
	#[test]
	fn set_content_type() {
		let mut body = UnpackedResponseBody { body: b"".to_vec(), ..Default::default() };
		assert_eq!(body.content_type(), ContentType::Text);
		
		_ = body.set_content_type(ContentType::Binary);
		assert_eq!(body.content_type(), ContentType::Binary);
		
		_ = body.set_content_type(ContentType::Text);
		assert_eq!(body.content_type(), ContentType::Text);
		
		let mut clone = body.clone();
		assert_eq!(clone.set_content_type(ContentType::Text), &body);
		
		assert_eq!(body.set_content_type(ContentType::Binary).content_type(), ContentType::Binary);
	}
	
	//		is_binary															
	#[test]
	fn is_binary() {
		let mut body = UnpackedResponseBody { body: b"".to_vec(), ..Default::default() };
		assert!(!body.is_binary());
		
		_ = body.set_content_type(ContentType::Binary);
		assert!( body.is_binary());
		
		_ = body.set_content_type(ContentType::Text);
		assert!(!body.is_binary());
	}
	
	//		is_text																
	#[test]
	fn is_text() {
		let mut body = UnpackedResponseBody { body: b"".to_vec(), ..Default::default() };
		assert!( body.is_text());
		
		_ = body.set_content_type(ContentType::Binary);
		assert!(!body.is_text());
		
		_ = body.set_content_type(ContentType::Text);
		assert!( body.is_text());
	}
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let body       = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let byte_slice = body.as_bytes();
		
		//	Ensure the byte slice matches the original response body's bytes.
		assert_eq!(byte_slice, b"This is a test".to_vec());
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original response body.
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		as_mut_bytes														
	#[test]
	fn as_mut_bytes() {
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let byte_vec = body.as_mut_bytes();
		
		//	Ensure the byte vector matches the original response body's bytes.
		assert_eq!(*byte_vec, b"This is a test".to_vec());
		
		// We can modify the byte vector.
		byte_vec[10] = 84;
		assert_eq!(*byte_vec, b"This is a Test".to_vec());
		
		//	as_mut_bytes() doesn't consume the original response body, but modifying
		//	the returned vector will have affected the response body's contents.
		assert_eq!(body, UnpackedResponseBody { body: b"This is a Test".to_vec(), ..Default::default() });
	}
	
	//		into_bytes															
	#[test]
	fn into_bytes() {
		let body         = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let mut byte_vec = body.into_bytes();
		
		//	Ensure the byte vector matches the original response body's bytes.
		assert_eq!(byte_vec, b"This is a test".to_vec());
		
		// We can modify the byte vector.
		byte_vec[10]     = 84;
		assert_eq!(byte_vec, b"This is a Test".to_vec());
		
		//	We can't use the original response body after calling into_bytes(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let body           = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let mut byte_clone = body.to_bytes();
		
		//	Ensure the clone matches the original response body's bytes.
		assert_eq!(byte_clone, b"This is a test".to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_eq!(byte_clone, b"This is a Test".to_vec());
		
		//	to_bytes() doesn't consume or affect the original response body.
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		assert_eq!(body.to_string(), "This is a test");
	}
	
	//		to_base64															
	#[expect(invalid_from_utf8, reason = "Needed for the test")]
	#[test]
	fn to_base64() {
		let mut body    = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Binary,
		};
		assert_eq!(body.to_base64(), "VGhpcyBpcyBhIHRlc3Q=");
		
		body.clear();
		assert_eq!(body.to_base64(), "");
		
		body.body       = vec![0x80];
		assert_err!(from_utf8(&[0x80]));
		assert_eq!(body.to_base64(), "gA==");
	}
	
	//		from_base64															
	#[expect(invalid_from_utf8, reason = "Needed for the test")]
	#[test]
	fn from_base64__valid() {
		let body1 = UnpackedResponseBody::from_base64("VGhpcyBpcyBhIHRlc3Q=").unwrap();
		assert_eq!(body1.body,         b"This is a test");
		assert_eq!(body1.content_type, ContentType::Binary);
		
		let body2 = UnpackedResponseBody::from_base64("").unwrap();
		assert!(body2.body.is_empty());
		assert_eq!(body2.content_type, ContentType::Binary);
		
		let body3 = UnpackedResponseBody::from_base64("gA==").unwrap();
		assert_eq!(body3.body,         vec![0x80]);
		assert_eq!(body3.content_type, ContentType::Binary);
		assert_err!(from_utf8(&[0x80]));
	}
	
	#[test]
	fn from_base64__invalid() {
		assert_err!(UnpackedResponseBody::from_base64("invalid@@base64"));
	}
	
	//		clear																
	#[test]
	fn clear() {
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		body.clear();
		assert_eq!(body, UnpackedResponseBody { body: b"".to_vec(), ..Default::default() });
	}
	
	//		empty																
	#[test]
	fn empty() {
		let body = UnpackedResponseBody::empty();
		assert_eq!(body, UnpackedResponseBody { body: b"".to_vec(), ..Default::default() });
	}
	
	//		is_empty															
	#[test]
	fn is_empty() {
		let body1 = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		assert!(!body1.is_empty());
		
		let body2 = UnpackedResponseBody { body: b"".to_vec(), ..Default::default() };
		assert!( body2.is_empty());
	}
	
	//		len																	
	#[test]
	fn len() {
		let body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		assert_eq!(body.len(), 14);
	}
	
	//		push																
	#[test]
	fn push() {
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		body.push(33);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test!".to_vec(), ..Default::default() });
	}
	
	//		push_bytes															
	#[test]
	fn push_bytes__byte_array() {
		let mut body = UnpackedResponseBody { body: b"This".to_vec(), ..Default::default() };
		body.push_bytes(b" is a test");
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn push_bytes__byte_slice() {
		let mut body = UnpackedResponseBody { body: b"This".to_vec(), ..Default::default() };
		body.push_bytes(&b" is a test"[..]);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		push_char															
	#[test]
	fn push_char() {
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		body.push_char(&'!');
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test!".to_vec(), ..Default::default() });
	}
	
	//		push_str															
	#[test]
	fn push_str() {
		let mut body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body.push_str(" a test");
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
}

#[cfg(test)]
mod unpacked_response_body__traits {
	use super::*;
	
	//		add																	
	#[test]
	fn add__byte_array() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + b" a test", UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__byte_slice() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + &b" a test"[..], UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[expect(clippy::string_lit_as_bytes, reason = "Needed for the test")]
	#[test]
	fn add__char_one_byte() {
		let body = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		assert_eq!(body + 'A', UnpackedResponseBody { body: s!("This is A").into_bytes(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: s!("This is ").into_bytes() });
	}
	#[test]
	fn add__char_two_byte() {
		let body = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		assert_eq!(body + 'ñ', UnpackedResponseBody { body: s!("This is ñ").into_bytes(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: s!("This is ").into_bytes() });
	}
	#[test]
	fn add__char_three_byte() {
		let body = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		assert_eq!(body + 'Ḁ', UnpackedResponseBody { body: s!("This is Ḁ").into_bytes(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: s!("This is ").into_bytes() });
	}
	#[test]
	fn add__char_four_byte() {
		let body = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		assert_eq!(body + '𐍈', UnpackedResponseBody { body: s!("This is 𐍈").into_bytes(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: s!("This is ").into_bytes() });
	}
	#[expect(clippy::string_lit_as_bytes, reason = "Needed for the test")]
	#[test]
	fn add__char_ref() {
		let body = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		let char = 'A';
		assert_eq!(body + char, UnpackedResponseBody { body: s!("This is A").into_bytes(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: s!("This is ").into_bytes() });
	}
	#[test]
	fn add__str() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + " a test", UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__str_ref() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let str  = " a test";
		assert_eq!(body + str, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__string() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + s!(" a test"), UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__string_ref() {
		let body   = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let string = s!(" a test");
		assert_eq!(body + &string, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__box_str() {
		let body   = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + s!(" a test").into_boxed_str(), UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__cow_borrowed() {
		let body              = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let cow: Cow<'_, str> = Cow::Borrowed(" a test");
		assert_eq!(body + cow, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__cow_owned() {
		let body              = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let cow: Cow<'_, str> = Cow::Owned(s!(" a test"));
		assert_eq!(body + cow, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__u8() {
		let body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		assert_eq!(body + 33, UnpackedResponseBody { body: b"This is a test!".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__vec_u8() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_eq!(body + b" a test".to_vec(), UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__vec_u8_ref() {
		let body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let vec  = b" a test".to_vec();
		assert_eq!(body + &vec, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__unpacked_response_body() {
		let body1 = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let body2 = UnpackedResponseBody { body: b" a test".to_vec(), ..Default::default() };
		assert_eq!(body1 + body2, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body1, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	#[test]
	fn add__unpacked_response_body_ref() {
		let body1 = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let body2 = UnpackedResponseBody { body: b" a test".to_vec(), ..Default::default() };
		assert_eq!(body1 + &body2, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body1, UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() });
	}
	
	//		add_assign															
	#[test]
	fn add_assign__byte_array() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += b" a test";
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__byte_slice() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += &b" a test"[..];
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[expect(clippy::string_lit_as_bytes, reason = "Needed for the test")]
	#[test]
	fn add_assign__char_one_byte() {
		let mut body  = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		body         += 'A';
		assert_eq!(body, UnpackedResponseBody { body: s!("This is A").into_bytes(), ..Default::default() });
	}
	#[test]
	fn add_assign__char_two_byte() {
		let mut body  = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		body         += 'ñ';
		assert_eq!(body, UnpackedResponseBody { body: s!("This is ñ").into_bytes(), ..Default::default() });
	}
	#[test]
	fn add_assign__char_three_byte() {
		let mut body  = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		body         += 'Ḁ';
		assert_eq!(body, UnpackedResponseBody { body: s!("This is Ḁ").into_bytes(), ..Default::default() });
	}
	#[test]
	fn add_assign__char_four_byte() {
		let mut body  = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		body         += '𐍈';
		assert_eq!(body, UnpackedResponseBody { body: s!("This is 𐍈").into_bytes(), ..Default::default() });
	}
	#[expect(clippy::string_lit_as_bytes, reason = "Needed for the test")]
	#[test]
	fn add_assign__char_ref() {
		let mut body  = UnpackedResponseBody { body: b"This is ".to_vec(), ..Default::default() };
		let char      = 'A';
		body         += &char;
		assert_eq!(body, UnpackedResponseBody { body: s!("This is A").into_bytes(), ..Default::default() });
	}
	#[test]
	fn add_assign__str() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += " a test";
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__str_ref() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let str       = " a test";
		body         += str;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__string() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += s!(" a test");
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__string_ref() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let string    = s!(" a test");
		body         += &string;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__box_str() {
		let mut body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let box_str  = s!(" a test").into_boxed_str();
		body        += box_str;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__cow_borrowed() {
		let mut body           = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let cow: Cow<'_, str>  = Cow::Borrowed(" a test");
		body                  += cow;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__cow_owned() {
		let mut body           = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let cow: Cow<'_, str>  = Cow::Owned(s!(" a test"));
		body                  += cow;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__u8() {
		let mut body  = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		body         += 33;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test!".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__vec_u8() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += b" a test".to_vec();
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__vec_u8_ref() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let vec       = b" a test".to_vec();
		body         += &vec;
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__unpacked_response_body() {
		let mut body  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		body         += UnpackedResponseBody { body: b" a test".to_vec(), ..Default::default() };
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn add_assign__unpacked_response_body_ref() {
		let mut body1  = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		let body2      = UnpackedResponseBody { body: b" a test".to_vec(), ..Default::default() };
		body1          += &body2;
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let body       = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let byte_slice = body.as_ref();
		assert_eq!(byte_slice, b"This is a test".to_vec());
		assert_eq!(body,       UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		as_mut																
	#[test]
	fn as_mut() {
		//	Same tests as for as_mut_bytes().
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let byte_vec = body.as_mut();
		assert_eq!(*byte_vec, b"This is a test".to_vec());
		
		byte_vec[10] = 84;
		assert_eq!(*byte_vec, b"This is a Test".to_vec());
		assert_eq!(body,      UnpackedResponseBody { body: b"This is a Test".to_vec(), ..Default::default() });
	}
	
	//		clone																
	#[test]
	fn clone() {
		let mut body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let clone    = body.clone();
		assert_eq!(clone, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		body.clear();
		body.push_str("This is a different test");
		assert_eq!(body,  UnpackedResponseBody { body: b"This is a different test".to_vec(), ..Default::default() });
		assert_eq!(clone, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let mut body  = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		let mut clone = UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() };
		clone.clone_from(&body);
		assert_eq!(body,  UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_eq!(clone, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		body.clear();
		body.push_str("This is a different test");
		assert_eq!(body,  UnpackedResponseBody { body: b"This is a different test".to_vec(), ..Default::default() });
		assert_eq!(clone, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	
	//		debug																
	#[test]
	fn debug__binary() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Binary,
		};
		assert_eq!(format!("{body:?}"), r#"UnpackedResponseBody { body: "VGhpcyBpcyBhIHRlc3Q=", content_type: Binary }"#);
	}
	#[test]
	fn debug__text() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text,
		};
		assert_eq!(format!("{body:?}"), r#"UnpackedResponseBody { body: "This is a test", content_type: Text }"#);
	}
	
	//		default																
	#[test]
	fn default() {
		let body = UnpackedResponseBody::default();
		assert_eq!(body, UnpackedResponseBody { body: b"".to_vec(), ..Default::default() });
	}
	
	//		display																
	#[test]
	fn display__binary() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Binary,
		};
		assert_eq!(format!("{body}"), r"VGhpcyBpcyBhIHRlc3Q=");
	}
	#[test]
	fn display__text() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text,
		};
		assert_eq!(format!("{body}"), r"This is a test");
	}
	
	//		from																
	#[test]
	fn from__axum_body() {
		let body1 = UnpackedResponseBody::from(AxumBody::from("This is a test"));
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let axum  = AxumBody::from("This is another test");
		let body2 = UnpackedResponseBody::from(axum);
		assert_eq!(body2, UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		//	We cannot compare to the original AxumBody after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(axum, "This is another test");
	}
	#[test]
	fn from__byte_array() {
		let body1      = UnpackedResponseBody::from(b"This is a test");
		assert_eq!(body1,      UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let byte_array = b"This is another test";
		let body2      = UnpackedResponseBody::from(byte_array);
		assert_eq!(body2,      UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		assert_eq!(byte_array, b"This is another test");
	}
	#[test]
	fn from__byte_slice() {
		let body1       = UnpackedResponseBody::from(&b"This is a test"[..]);
		assert_eq!(body1,       UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let byte_slice = &b"This is another test"[..];
		let body2      = UnpackedResponseBody::from(byte_slice);
		assert_eq!(body2,      UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		assert_eq!(byte_slice, b"This is another test");
	}
	#[test]
	fn from__char() {
		let body1 = UnpackedResponseBody::from('A');
		assert_eq!(body1, UnpackedResponseBody { body: b"A".to_vec(), ..Default::default() });
		
		let char  = 'B';
		let body2 = UnpackedResponseBody::from(char);
		assert_eq!(body2, UnpackedResponseBody { body: b"B".to_vec(), ..Default::default() });
		assert_eq!(char,  'B');
	}
	#[test]
	fn from__char_ref() {
		let char = 'A';
		let body = UnpackedResponseBody::from(&char);
		assert_eq!(body, UnpackedResponseBody { body: b"A".to_vec(), ..Default::default() });
		assert_eq!(char, 'A');
	}
	#[expect(clippy::string_lit_as_bytes, reason = "Needed for the test")]
	#[test]
	fn from__char_one_byte() {
		let body = UnpackedResponseBody::from('A');
		assert_eq!(body, UnpackedResponseBody { body: vec![65], ..Default::default() });
		assert_eq!(body, UnpackedResponseBody::from(s!("A")));
		assert_eq!(body, UnpackedResponseBody { body: s!("A").into_bytes(), ..Default::default() });
	}
	#[test]
	fn from__char_two_byte() {
		let body = UnpackedResponseBody::from('ñ');
		assert_eq!(body, UnpackedResponseBody { body: vec![195, 177], ..Default::default() });
		assert_eq!(body, UnpackedResponseBody::from(s!("ñ")));
		assert_eq!(body, UnpackedResponseBody { body: s!("ñ").into_bytes(), ..Default::default() });
	}
	#[test]
	fn from__char_three_byte() {
		let three_byte_single_width = UnpackedResponseBody::from('Ḁ');
		assert_eq!(three_byte_single_width, UnpackedResponseBody { body: vec![225, 184, 128], ..Default::default() });
		assert_eq!(three_byte_single_width, UnpackedResponseBody::from(s!("Ḁ")));
		assert_eq!(three_byte_single_width, UnpackedResponseBody { body: s!("Ḁ").into_bytes(), ..Default::default() });
		
		let three_byte_double_width = UnpackedResponseBody::from('你');
		assert_eq!(three_byte_double_width, UnpackedResponseBody { body: vec![228, 189, 160], ..Default::default() });
		assert_eq!(three_byte_double_width, UnpackedResponseBody::from(s!("你")));
		assert_eq!(three_byte_double_width, UnpackedResponseBody { body: s!("你").into_bytes(), ..Default::default() });
	}
	#[test]
	fn from__char_four_byte() {
		let body = UnpackedResponseBody::from('𐍈');
		assert_eq!(body, UnpackedResponseBody { body: vec![240, 144, 141, 136], ..Default::default() });
		assert_eq!(body, UnpackedResponseBody::from(s!("𐍈")));
		assert_eq!(body, UnpackedResponseBody { body: s!("𐍈").into_bytes(), ..Default::default() });
	}
	#[test]
	fn from__full() {
		let body1 = UnpackedResponseBody::from(Full::new(Bytes::from("This is a test")));
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let full  = Full::new(Bytes::from("This is another test"));
		let body2 = UnpackedResponseBody::from(full);
		assert_eq!(body2, UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		//	We cannot compare to the original Full after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(full, "This is another test");
	}
	#[tokio::test]
	async fn from__incoming() {
		let body1    = UnpackedResponseBody::from(create_incoming_for_testing(Bytes::from("This is a test")).await);
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let incoming = create_incoming_for_testing(Bytes::from("This is another test")).await;
		let body2    = UnpackedResponseBody::from(incoming);
		assert_eq!(body2, UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		//	We cannot compare to the original incoming after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(incoming, "This is another test");
	}
	#[test]
	fn from__json() {
		let body1 = UnpackedResponseBody::from(json!({
			"foo": "bar",
			"baz": 2,
		}));
		assert_json_eq!(json!(body1), r#"{"foo":"bar","baz":2}"#);
		
		let json = json!({
			"str":   "foo",
			"int":   99,
			"float": 1.234,
			"bool":  true,
		});
		let body2 = UnpackedResponseBody::from(json);
		assert_json_eq!(json!(body2), r#"{"str":"foo","int":99,"float":1.234,"bool":true}"#);
		//	We cannot compare to the original JSON after calling from(),
		//	because it has been consumed.
		//	Uncommenting the lines below would cause a compilation error:
		// assert_json_eq!(json, json!({
		// 	"str":   "foo",
		// 	"int":   99,
		// 	"float": 1.234,
		// 	"bool":  true,
		// }));
	}
	#[test]
	fn from__json_ref() {
		let json = json!({
			"foo": "bar",
			"baz": 2,
		});
		let body = UnpackedResponseBody::from(&json);
		assert_json_eq!(json!(body), r#"{"foo":"bar","baz":2}"#);
		assert_json_eq!(json, json!({
			"foo": "bar",
			"baz": 2,
		}));
	}
	#[test]
	fn from__str() {
		let body = UnpackedResponseBody::from("This is a test");
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
	#[test]
	fn from__str_ref() {
		let str  = "This is a test";
		let body = UnpackedResponseBody::from(str);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_eq!(str,  "This is a test");
	}
	#[test]
	fn from__mut_str_ref() {
		let mut string = s!("This is a test");
		let mut_str    = string.as_mut_str();
		let body       = UnpackedResponseBody::from(mut_str);
		assert_eq!(body,   UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__string() {
		let string = s!("This is a test");
		let body   = UnpackedResponseBody::from(string);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original string after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__string_ref() {
		let string = s!("This is a test");
		let body   = UnpackedResponseBody::from(&string);
		assert_eq!(body,   UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__box_str() {
		let box_str = s!("This is a test").into_boxed_str();
		let body    = UnpackedResponseBody::from(box_str);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original box_str after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(box_str, s!("This is a test").into_boxed_str());
	}
	#[test]
	fn from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed("This is a test");
		let body              = UnpackedResponseBody::from(cow);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original cow after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(cow, "This is a test");
	}
	#[test]
	fn from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(s!("This is a test"));
		let body              = UnpackedResponseBody::from(cow);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		//	We cannot compare to the original cow after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(cow, "This is a test");
	}
	#[test]
	fn from__u8() {
		let body = UnpackedResponseBody::from(65);
		assert_eq!(body, UnpackedResponseBody { body: vec![65], ..Default::default() });
		assert_eq!(body, UnpackedResponseBody { body: b"A".to_vec(), ..Default::default() });
	}
	#[test]
	fn from__vec_u8() {
		let body1 = UnpackedResponseBody::from(b"This is a test".to_vec());
		assert_eq!(body1, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		
		let vec   = b"This is another test".to_vec();
		let body2 = UnpackedResponseBody::from(vec);
		assert_eq!(body2, UnpackedResponseBody { body: b"This is another test".to_vec(), ..Default::default() });
		//	We cannot compare to the original vec after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(vec,  b"This is another test".to_vec());
		
	}
	#[test]
	fn from__vec_u8_ref() {
		let vec  = b"This is a test".to_vec();
		let body = UnpackedResponseBody::from(&vec);
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_eq!(vec,  b"This is a test".to_vec());
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		//	Basic ASCII string
		assert_ok_eq!(UnpackedResponseBody::from_str("Test"), UnpackedResponseBody { body: b"Test".to_vec(), ..Default::default() });
		//	Strings containing different sizes of UTF8 characters
		assert_ok_eq!(UnpackedResponseBody::from_str("ñ"),    UnpackedResponseBody { body: s!("ñ").into_bytes(), ..Default::default() });
		assert_ok_eq!(UnpackedResponseBody::from_str("Ḁ"),    UnpackedResponseBody { body: s!("Ḁ").into_bytes(), ..Default::default() });
		assert_ok_eq!(UnpackedResponseBody::from_str("𐍈"),    UnpackedResponseBody { body: s!("𐍈").into_bytes(), ..Default::default() });
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let body = UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() };
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
		assert_ne!(body, UnpackedResponseBody { body: b"This is different".to_vec(), ..Default::default() });
	}
	
	//		serialize															
	#[test]
	fn serialize__binary() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Binary,
		};
		let json        = json!("VGhpcyBpcyBhIHRlc3Q=");
		assert_json_eq!(json!(body), json);
	}
	#[test]
	fn serialize__text() {
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text,
		};
		let json        = json!("This is a test");
		assert_json_eq!(json!(body), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize__binary() {
		let json        = r#""VGhpcyBpcyBhIHRlc3Q=""#;
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text,
		};
		assert_ok_eq!(serde_json::from_str::<UnpackedResponseBody>(json), body);
	}
	#[test]
	fn deserialize__text() {
		let json        = r#""This is a test""#;
		let body        = UnpackedResponseBody {
			body:         b"This is a test".to_vec(),
			content_type: ContentType::Text,
		};
		assert_ok_eq!(serde_json::from_str::<UnpackedResponseBody>(json), body);
	}
	
	//		write_str															
	#[test]
	fn write_str() {
		let mut body = UnpackedResponseBody { body: b"This is".to_vec(), ..Default::default() };
		assert_ok!(body.write_str(" a test"));
		assert_eq!(body, UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() });
	}
}

//§		ResponseExt																
#[cfg(test)]
mod response_ext {
	use super::*;
	
	//		unpack																
	#[test]
	fn unpack__basic() {
		let mut response = Response::builder()
			.status(StatusCode::OK)
			.header("foo", "bar")
			.body(())
			.unwrap()
		;
		let unpacked     = response.unpack();
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"".to_vec(), ..Default::default() },
		};
		assert_ok_eq!(unpacked, crafted);
	}
	#[test]
	fn unpack__string() {
		let mut response = Response::builder()
			.status(StatusCode::OK)
			.body(s!("This is a test"))
			.unwrap()
		;
		let unpacked     = response.unpack();
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_ok_eq!(unpacked, crafted);
	}
	#[test]
	fn unpack__hyper_body() {
		let mut response = Response::builder()
			.status(StatusCode::OK)
			.body(Full::new(Bytes::from("This is a test")))
			.unwrap()
		;
		let unpacked     = response.unpack();
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_ok_eq!(unpacked, crafted);
	}
	#[test]
	fn unpack__axum_body() {
		let mut response = (
			StatusCode::OK,
			"This is a test",
		).into_response();
		let unpacked     = response.unpack();
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				//  Axum automatically adds a content-type header.
				UnpackedResponseHeader {
					name:  s!("content-type"),
					value: s!("text/plain; charset=utf-8"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_ok_eq!(unpacked, crafted);
	}
}

//		Functions																
#[cfg(test)]
mod functions {
	use super::*;
	
	//		convert_headers														
	#[test]
	fn convert_headers__basic() {
		let mut headers = HeaderMap::new();
		drop(headers.insert("foo", HeaderValue::from_static("bar")));
		let converted   = convert_headers(&headers);
		let crafted     = vec![
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("bar"),
			},
		];
		assert_eq!(converted, crafted);
	}
	#[test]
	fn convert_headers__textcase() {
		let mut headers = HeaderMap::new();
		drop(headers.insert("Foo", HeaderValue::from_static("Bar")));
		let converted   = convert_headers(&headers);
		let crafted     = vec![
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("Bar"),
			},
		];
		assert_eq!(converted, crafted);
	}
	#[test]
	fn convert_headers__order() {
		let mut headers = HeaderMap::new();
		drop(headers.insert("foo", HeaderValue::from_static("bar")));
		drop(headers.insert("bar", HeaderValue::from_static("baz")));
		let converted   = convert_headers(&headers);
		let crafted1    = vec![
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("bar"),
			},
			UnpackedResponseHeader {
				name:     s!("bar"),
				value:    s!("baz"),
			},
		];
		let crafted2    = vec![
			UnpackedResponseHeader {
				name:     s!("bar"),
				value:    s!("baz"),
			},
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("bar"),
			},
		];
		assert_ne!(converted, crafted1);
		assert_eq!(converted, crafted2);
	}
	#[test]
	fn convert_headers__duplicates() {
		let mut headers = HeaderMap::new();
		_               = headers.append("foo", HeaderValue::from_static("bar"));
		_               = headers.append("bar", HeaderValue::from_static("baz"));
		_               = headers.append("foo", HeaderValue::from_static("baz"));
		let converted   = convert_headers(&headers);
		let crafted     = vec![
			UnpackedResponseHeader {
				name:     s!("bar"),
				value:    s!("baz"),
			},
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("bar"),
			},
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("baz"),
			},
		];
		assert_eq!(converted, crafted);
	}
	#[test]
	fn convert_headers__no_duplicates() {
		let mut headers = HeaderMap::new();
		drop(headers.insert("foo", HeaderValue::from_static("bar")));
		drop(headers.insert("bar", HeaderValue::from_static("baz")));
		drop(headers.insert("foo", HeaderValue::from_static("baz")));
		let converted   = convert_headers(&headers);
		let crafted     = vec![
			UnpackedResponseHeader {
				name:     s!("bar"),
				value:    s!("baz"),
			},
			UnpackedResponseHeader {
				name:     s!("foo"),
				value:    s!("baz"),
			},
		];
		assert_eq!(converted, crafted);
	}
	
	//		convert_response													
	#[test]
	fn convert_response__basic() {
		let mut headers  = HeaderMap::new();
		drop(headers.insert("foo", HeaderValue::from_static("bar")));
		let converted    = convert_response(StatusCode::OK, &headers, &Bytes::from("This is a test"));
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody { body: b"This is a test".to_vec(), ..Default::default() },
		};
		assert_eq!(converted, crafted);
	}
	
	//		serialize_status_code												
	#[test]
	fn serialize_status_code__basic() {
		let status_code = StatusCode::OK;
		let serializer  = TestSerializer::builder().build();
		let result      = serialize_status_code(&status_code, &serializer);
		assert_ok_eq!(result, vec![Token::U16(200)]);
	}
	
	//		deserialize_status_code												
	#[test]
	fn deserialize_status_code__basic() {
		let mut deserializer = TestDeserializer::builder(vec![Token::U16(200)]).build();
		let result           = deserialize_status_code(&mut deserializer);
		assert_ok_eq!(result, StatusCode::OK);
	}
}


