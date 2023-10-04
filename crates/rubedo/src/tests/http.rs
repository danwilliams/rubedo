#![allow(non_snake_case)]

//		Tests

//		ResponseError															
#[cfg(test)]
mod response_error {
	use super::super::*;
	use claims::assert_err;
	
	//		debug																
	#[test]
	fn debug() {
		let err = Err::<ResponseError, _>(ResponseError::ConversionError);
		assert_err!(&err);
		assert_eq!(format!("{:?}", err), "Err(ConversionError)");
	}
	
	//		display																
	#[test]
	fn display() {
		let err = Err::<ResponseError, _>(ResponseError::ConversionError);
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Error encountered while converting response body to bytes");
	}
}

//		UnpackedResponse														

#[cfg(test)]
mod unpacked_response {
	use super::super::*;
	use crate::sugar::s;
	use assert_json_diff::assert_json_eq;
	use claims::assert_ok_eq;
	use serde_json::json;
	
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
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		};
		assert_eq!(format!("{:?}", response), r#"UnpackedResponse { status: 200, headers: [UnpackedResponseHeader { name: "foo", value: "bar" }], body: UnpackedResponseBody("This is a test") }"#);
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
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		};
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::NOT_FOUND,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		});
		assert_eq!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("baz"),
				},
			],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("baz"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		});
		assert_ne!(response, UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody(b"This is different".to_vec()),
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
			body:            UnpackedResponseBody(b"This is a test".to_vec()),
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
			body:            UnpackedResponseBody(b"This is a test".to_vec()),
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
mod unpacked_response_header {
	use super::super::*;
	use assert_json_diff::assert_json_eq;
	use claims::assert_ok_eq;
	use serde_json::json;
	
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
mod unpacked_response_body {
	use super::super::*;
	use assert_json_diff::assert_json_eq;
	use claims::assert_ok_eq;
	use serde_json::json;
	
	//		debug																
	#[test]
	fn debug() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(format!("{:?}", body), r#"UnpackedResponseBody("This is a test")"#);
	}
	
	//		display																
	#[test]
	fn display() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(format!("{}", body), r#"This is a test"#);
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		let body = UnpackedResponseBody::from_str("This is a test");
		assert_ok_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		assert_ne!(body, UnpackedResponseBody(b"This is different".to_vec()));
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		let json = json!(b"This is a test".to_vec());
		assert_json_eq!(json!(body), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		let json = json!(b"This is a test".to_vec()).to_string();
		assert_ok_eq!(serde_json::from_str::<UnpackedResponseBody>(&json), body);
	}
}

//§		ResponseExt																
#[cfg(test)]
mod response_ext {
	use super::super::*;
	use crate::sugar::s;
	use axum::response::IntoResponse;
	use claims::assert_ok_eq;
	
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
			body:          UnpackedResponseBody(b"".to_vec()),
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
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		};
		assert_ok_eq!(unpacked, crafted);
	}
	#[test]
	fn unpack__hyper_body() {
		let mut response = hyper::Response::builder()
			.status(StatusCode::OK)
			.body(HyperBody::from("This is a test"))
			.unwrap()
		;
		let unpacked     = response.unpack();
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
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
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		};
		assert_ok_eq!(unpacked, crafted);
	}
}

//		Functions																
#[cfg(test)]
mod functions {
	use super::super::*;
	use crate::sugar::s;
	use claims::assert_ok_eq;
	use serde_assert::{
		Deserializer as TestDeserializer,
		Serializer as TestSerializer,
		Token,
		Tokens,
	};
	
	//		convert_headers														
	#[test]
	fn convert_headers__basic() {
		let mut headers = HeaderMap::new();
		headers.insert("foo", HeaderValue::from_static("bar"));
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
		headers.insert("Foo", HeaderValue::from_static("Bar"));
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
		headers.insert("foo", HeaderValue::from_static("bar"));
		headers.insert("bar", HeaderValue::from_static("baz"));
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
		headers.append("foo", HeaderValue::from_static("bar"));
		headers.append("bar", HeaderValue::from_static("baz"));
		headers.append("foo", HeaderValue::from_static("baz"));
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
		headers.insert("foo", HeaderValue::from_static("bar"));
		headers.insert("bar", HeaderValue::from_static("baz"));
		headers.insert("foo", HeaderValue::from_static("baz"));
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
		headers.insert("foo", HeaderValue::from_static("bar"));
		let converted    = convert_response(StatusCode::OK, &headers, Bytes::from("This is a test"));
		let crafted      = UnpackedResponse {
			status:        StatusCode::OK,
			headers:       vec![
				UnpackedResponseHeader {
					name:  s!("foo"),
					value: s!("bar"),
				},
			],
			body:          UnpackedResponseBody(b"This is a test".to_vec()),
		};
		assert_eq!(converted, crafted);
	}
	
	//		serialize_status_code												
	#[test]
	fn serialize_status_code__basic() {
		let status_code = StatusCode::OK;
		let serializer  = TestSerializer::builder().build();
		let result      = serialize_status_code(&status_code, &serializer);
		assert_ok_eq!(result, Tokens(vec![Token::U16(200)]));
	}
	
	//		deserialize_status_code												
	#[test]
	fn deserialize_status_code__basic() {
		let mut deserializer = TestDeserializer::builder()
			.tokens(Tokens(vec![Token::U16(200)]))
			.build()
		;
		let result           = deserialize_status_code(&mut deserializer);
		assert_ok_eq!(result, StatusCode::OK);
	}
}


