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
mod unpacked_response_body__struct {
	use super::super::*;
	
	//		new																	
	#[test]
	fn new() {
		let body = UnpackedResponseBody::new(b"This is a test".to_vec());
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let body       = UnpackedResponseBody(b"This is a test".to_vec());
		let byte_slice = body.as_bytes();
		
		//	Ensure the byte slice matches the original response body's bytes.
		assert_eq!(byte_slice, b"This is a test".to_vec());
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original response body.
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		as_mut_bytes														
	#[test]
	fn as_mut_bytes() {
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		let byte_vec = body.as_mut_bytes();
		
		//	Ensure the byte vector matches the original response body's bytes.
		assert_eq!(*byte_vec, b"This is a test".to_vec());
		
		// We can modify the byte vector.
		byte_vec[10] = 84;
		assert_eq!(*byte_vec, b"This is a Test".to_vec());
		
		//	as_mut_bytes() doesn't consume the original response body, but modifying
		//	the returned vector will have affected the response body's contents.
		assert_eq!(body, UnpackedResponseBody(b"This is a Test".to_vec()));
	}
	
	//		into_bytes															
	#[test]
	fn into_bytes() {
		let body         = UnpackedResponseBody(b"This is a test".to_vec());
		let mut byte_vec = body.into_bytes();
		
		//	Ensure the byte vector matches the original response body's bytes.
		assert_eq!(byte_vec, b"This is a test".to_vec());
		
		// We can modify the byte vector.
		byte_vec[10]     = 84;
		assert_eq!(byte_vec, b"This is a Test".to_vec());
		
		//	We can't use the original response body after calling into_bytes(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let body           = UnpackedResponseBody(b"This is a test".to_vec());
		let mut byte_clone = body.to_bytes();
		
		//	Ensure the clone matches the original response body's bytes.
		assert_eq!(byte_clone, b"This is a test".to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_eq!(byte_clone, b"This is a Test".to_vec());
		
		//	to_bytes() doesn't consume or affect the original response body.
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(body.to_string(), "This is a test");
	}
	
	//		clear																
	#[test]
	fn clear() {
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		body.clear();
		assert_eq!(body, UnpackedResponseBody(b"".to_vec()));
	}
	
	//		empty																
	#[test]
	fn empty() {
		let body = UnpackedResponseBody::empty();
		assert_eq!(body, UnpackedResponseBody(b"".to_vec()));
	}
	
	//		is_empty															
	#[test]
	fn is_empty() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(body.is_empty(), false);
		let body = UnpackedResponseBody(b"".to_vec());
		assert_eq!(body.is_empty(), true);
	}
	
	//		len																	
	#[test]
	fn len() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(body.len(), 14);
	}
	
	//		push																
	#[test]
	fn push() {
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		body.push(33);
		assert_eq!(body, UnpackedResponseBody(b"This is a test!".to_vec()));
	}
	
	//		push_bytes															
	#[test]
	fn push_bytes__byte_array() {
		let mut body = UnpackedResponseBody(b"This".to_vec());
		body.push_bytes(b" is a test");
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn push_bytes__byte_slice() {
		let mut body = UnpackedResponseBody(b"This".to_vec());
		body.push_bytes(&b" is a test"[..]);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		push_char															
	#[test]
	fn push_char() {
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		body.push_char(&'!');
		assert_eq!(body, UnpackedResponseBody(b"This is a test!".to_vec()));
	}
	
	//		push_str															
	#[test]
	fn push_str() {
		let mut body = UnpackedResponseBody(b"This is".to_vec());
		body.push_str(" a test");
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
}

#[cfg(test)]
mod unpacked_response_body__traits {
	use super::super::*;
	use assert_json_diff::assert_json_eq;
	use claims::{assert_ok, assert_ok_eq};
	use serde_json::json;
	
	//		add																	
	#[test]
	fn add__byte_array() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + b" a test", UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__byte_slice() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + &b" a test"[..], UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__char_one_byte() {
		let body = UnpackedResponseBody(b"This is ".to_vec());
		assert_eq!(body + 'A', UnpackedResponseBody(s!("This is A").into_bytes()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(s!("This is ").into_bytes()));
	}
	#[test]
	fn add__char_two_byte() {
		let body = UnpackedResponseBody(b"This is ".to_vec());
		assert_eq!(body + 'ñ', UnpackedResponseBody(s!("This is ñ").into_bytes()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(s!("This is ").into_bytes()));
	}
	#[test]
	fn add__char_three_byte() {
		let body = UnpackedResponseBody(b"This is ".to_vec());
		assert_eq!(body + 'Ḁ', UnpackedResponseBody(s!("This is Ḁ").into_bytes()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(s!("This is ").into_bytes()));
	}
	#[test]
	fn add__char_four_byte() {
		let body = UnpackedResponseBody(b"This is ".to_vec());
		assert_eq!(body + '𐍈', UnpackedResponseBody(s!("This is 𐍈").into_bytes()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(s!("This is ").into_bytes()));
	}
	#[test]
	fn add__char_ref() {
		let body = UnpackedResponseBody(b"This is ".to_vec());
		let char = 'A';
		assert_eq!(body + &char, UnpackedResponseBody(s!("This is A").into_bytes()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(s!("This is ").into_bytes()));
	}
	#[test]
	fn add__str() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + " a test", UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__str_ref() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		let str  = " a test";
		assert_eq!(body + str, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__string() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + s!(" a test"), UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__string_ref() {
		let body   = UnpackedResponseBody(b"This is".to_vec());
		let string = s!(" a test");
		assert_eq!(body + &string, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__box_str() {
		let body   = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + s!(" a test").into_boxed_str(), UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__cow_borrowed() {
		let body              = UnpackedResponseBody(b"This is".to_vec());
		let cow: Cow<'_, str> = Cow::Borrowed(" a test");
		assert_eq!(body + cow, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__cow_owned() {
		let body              = UnpackedResponseBody(b"This is".to_vec());
		let cow: Cow<'_, str> = Cow::Owned(s!(" a test"));
		assert_eq!(body + cow, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__u8() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(body + 33, UnpackedResponseBody(b"This is a test!".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add__vec_u8() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		assert_eq!(body + b" a test".to_vec(), UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__vec_u8_ref() {
		let body = UnpackedResponseBody(b"This is".to_vec());
		let vec  = b" a test".to_vec();
		assert_eq!(body + &vec, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__unpacked_response_body() {
		let body1 = UnpackedResponseBody(b"This is".to_vec());
		let body2 = UnpackedResponseBody(b" a test".to_vec());
		assert_eq!(body1 + body2, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body1, UnpackedResponseBody(b"This is".to_vec()));
	}
	#[test]
	fn add__unpacked_response_body_ref() {
		let body1 = UnpackedResponseBody(b"This is".to_vec());
		let body2 = UnpackedResponseBody(b" a test".to_vec());
		assert_eq!(body1 + &body2, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original response body after using the +
		//	operator, because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(body1, UnpackedResponseBody(b"This is".to_vec()));
	}
	
	//		add_assign															
	#[test]
	fn add_assign__byte_array() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += b" a test";
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__byte_slice() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += &b" a test"[..];
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__char_one_byte() {
		let mut body  = UnpackedResponseBody(b"This is ".to_vec());
		body         += 'A';
		assert_eq!(body, UnpackedResponseBody(s!("This is A").into_bytes()));
	}
	#[test]
	fn add_assign__char_two_byte() {
		let mut body  = UnpackedResponseBody(b"This is ".to_vec());
		body         += 'ñ';
		assert_eq!(body, UnpackedResponseBody(s!("This is ñ").into_bytes()));
	}
	#[test]
	fn add_assign__char_three_byte() {
		let mut body  = UnpackedResponseBody(b"This is ".to_vec());
		body         += 'Ḁ';
		assert_eq!(body, UnpackedResponseBody(s!("This is Ḁ").into_bytes()));
	}
	#[test]
	fn add_assign__char_four_byte() {
		let mut body  = UnpackedResponseBody(b"This is ".to_vec());
		body         += '𐍈';
		assert_eq!(body, UnpackedResponseBody(s!("This is 𐍈").into_bytes()));
	}
	#[test]
	fn add_assign__char_ref() {
		let mut body  = UnpackedResponseBody(b"This is ".to_vec());
		let char      = 'A';
		body         += &char;
		assert_eq!(body, UnpackedResponseBody(s!("This is A").into_bytes()));
	}
	#[test]
	fn add_assign__str() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += " a test";
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__str_ref() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		let str       = " a test";
		body         += str;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__string() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += s!(" a test");
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__string_ref() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		let string    = s!(" a test");
		body         += &string;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__box_str() {
		let mut body = UnpackedResponseBody(b"This is".to_vec());
		let box_str  = s!(" a test").into_boxed_str();
		body        += box_str;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__cow_borrowed() {
		let mut body           = UnpackedResponseBody(b"This is".to_vec());
		let cow: Cow<'_, str>  = Cow::Borrowed(" a test");
		body                  += cow;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__cow_owned() {
		let mut body           = UnpackedResponseBody(b"This is".to_vec());
		let cow: Cow<'_, str>  = Cow::Owned(s!(" a test"));
		body                  += cow;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__u8() {
		let mut body  = UnpackedResponseBody(b"This is a test".to_vec());
		body         += 33;
		assert_eq!(body, UnpackedResponseBody(b"This is a test!".to_vec()));
	}
	#[test]
	fn add_assign__vec_u8() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += b" a test".to_vec();
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__vec_u8_ref() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		let vec       = b" a test".to_vec();
		body         += &vec;
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__unpacked_response_body() {
		let mut body  = UnpackedResponseBody(b"This is".to_vec());
		body         += UnpackedResponseBody(b" a test".to_vec());
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn add_assign__unpacked_response_body_ref() {
		let mut body1  = UnpackedResponseBody(b"This is".to_vec());
		let body2      = UnpackedResponseBody(b" a test".to_vec());
		body1          += &body2;
		assert_eq!(body1, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let body       = UnpackedResponseBody(b"This is a test".to_vec());
		let byte_slice = body.as_ref();
		assert_eq!(byte_slice, b"This is a test".to_vec());
		assert_eq!(body,       UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		as_mut																
	#[test]
	fn as_mut() {
		//	Same tests as for as_mut_bytes().
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		let byte_vec = body.as_mut();
		assert_eq!(*byte_vec, b"This is a test".to_vec());
		byte_vec[10] = 84;
		assert_eq!(*byte_vec, b"This is a Test".to_vec());
		assert_eq!(body,      UnpackedResponseBody(b"This is a Test".to_vec()));
	}
	
	//		clone																
	#[test]
	fn clone() {
		let mut body = UnpackedResponseBody(b"This is a test".to_vec());
		let clone    = body.clone();
		assert_eq!(clone, UnpackedResponseBody(b"This is a test".to_vec()));
		body.clear();
		body.push_str("This is a different test");
		assert_eq!(body,  UnpackedResponseBody(b"This is a different test".to_vec()));
		assert_eq!(clone, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let mut body  = UnpackedResponseBody(b"This is a test".to_vec());
		let mut clone = UnpackedResponseBody(b"This is another test".to_vec());
		clone.clone_from(&body);
		assert_eq!(body,  UnpackedResponseBody(b"This is a test".to_vec()));
		assert_eq!(clone, UnpackedResponseBody(b"This is a test".to_vec()));
		body.clear();
		body.push_str("This is a different test");
		assert_eq!(body,  UnpackedResponseBody(b"This is a different test".to_vec()));
		assert_eq!(clone, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	
	//		debug																
	#[test]
	fn debug() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(format!("{:?}", body), r#"UnpackedResponseBody("This is a test")"#);
	}
	
	//		default																
	#[test]
	fn default() {
		let body = UnpackedResponseBody::default();
		assert_eq!(body, UnpackedResponseBody(b"".to_vec()));
	}
	
	//		display																
	#[test]
	fn display() {
		let body = UnpackedResponseBody(b"This is a test".to_vec());
		assert_eq!(format!("{}", body), r#"This is a test"#);
	}
	
	//		from																
	#[test]
	fn from__byte_array() {
		let body       = UnpackedResponseBody::from(b"This is a test");
		assert_eq!(body,       UnpackedResponseBody(b"This is a test".to_vec()));
		let byte_array = b"This is another test";
		let body       = UnpackedResponseBody::from(byte_array);
		assert_eq!(body,       UnpackedResponseBody(b"This is another test".to_vec()));
		assert_eq!(byte_array, b"This is another test");
	}
	#[test]
	fn from__byte_slice() {
		let body       = UnpackedResponseBody::from(&b"This is a test"[..]);
		assert_eq!(body,       UnpackedResponseBody(b"This is a test".to_vec()));
		let byte_slice = &b"This is another test"[..];
		let body       = UnpackedResponseBody::from(byte_slice);
		assert_eq!(body,       UnpackedResponseBody(b"This is another test".to_vec()));
		assert_eq!(byte_slice, b"This is another test");
	}
	#[test]
	fn from__char() {
		let body = UnpackedResponseBody::from('A');
		assert_eq!(body, UnpackedResponseBody(b"A".to_vec()));
		let char = 'B';
		let body = UnpackedResponseBody::from(char);
		assert_eq!(body, UnpackedResponseBody(b"B".to_vec()));
		assert_eq!(char, 'B');
	}
	#[test]
	fn from__char_ref() {
		let char = 'A';
		let body = UnpackedResponseBody::from(&char);
		assert_eq!(body, UnpackedResponseBody(b"A".to_vec()));
		assert_eq!(char, 'A');
	}
	#[test]
	fn from__char_one_byte() {
		let body = UnpackedResponseBody::from('A');
		assert_eq!(body, UnpackedResponseBody(vec![65]));
		assert_eq!(body, UnpackedResponseBody::from(s!("A")));
		assert_eq!(body, UnpackedResponseBody(s!("A").into_bytes()));
	}
	#[test]
	fn from__char_two_byte() {
		let body = UnpackedResponseBody::from('ñ');
		assert_eq!(body, UnpackedResponseBody(vec![195, 177]));
		assert_eq!(body, UnpackedResponseBody::from(s!("ñ")));
		assert_eq!(body, UnpackedResponseBody(s!("ñ").into_bytes()));
	}
	#[test]
	fn from__char_three_byte() {
		let three_byte_single_width = UnpackedResponseBody::from('Ḁ');
		assert_eq!(three_byte_single_width, UnpackedResponseBody(vec![225, 184, 128]));
		assert_eq!(three_byte_single_width, UnpackedResponseBody::from(s!("Ḁ")));
		assert_eq!(three_byte_single_width, UnpackedResponseBody(s!("Ḁ").into_bytes()));
		let three_byte_double_width = UnpackedResponseBody::from('你');
		assert_eq!(three_byte_double_width, UnpackedResponseBody(vec![228, 189, 160]));
		assert_eq!(three_byte_double_width, UnpackedResponseBody::from(s!("你")));
		assert_eq!(three_byte_double_width, UnpackedResponseBody(s!("你").into_bytes()));
	}
	#[test]
	fn from__char_four_byte() {
		let body = UnpackedResponseBody::from('𐍈');
		assert_eq!(body, UnpackedResponseBody(vec![240, 144, 141, 136]));
		assert_eq!(body, UnpackedResponseBody::from(s!("𐍈")));
		assert_eq!(body, UnpackedResponseBody(s!("𐍈").into_bytes()));
	}
	#[test]
	fn from__str() {
		let body = UnpackedResponseBody::from("This is a test");
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
	}
	#[test]
	fn from__str_ref() {
		let str  = "This is a test";
		let body = UnpackedResponseBody::from(str);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		assert_eq!(str,  "This is a test");
	}
	#[test]
	fn from__mut_str_ref() {
		let mut string = s!("This is a test");
		let mut_str    = string.as_mut_str();
		let body       = UnpackedResponseBody::from(mut_str);
		assert_eq!(body,   UnpackedResponseBody(b"This is a test".to_vec()));
		assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__string() {
		let string = s!("This is a test");
		let body   = UnpackedResponseBody::from(string);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original string after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__string_ref() {
		let string = s!("This is a test");
		let body   = UnpackedResponseBody::from(&string);
		assert_eq!(body,   UnpackedResponseBody(b"This is a test".to_vec()));
		assert_eq!(string, "This is a test");
	}
	#[test]
	fn from__box_str() {
		let box_str = s!("This is a test").into_boxed_str();
		let body    = UnpackedResponseBody::from(box_str);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original box_str after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(box_str, s!("This is a test").into_boxed_str());
	}
	#[test]
	fn from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed("This is a test");
		let body              = UnpackedResponseBody::from(cow);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original cow after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(cow, "This is a test");
	}
	#[test]
	fn from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(s!("This is a test"));
		let body              = UnpackedResponseBody::from(cow);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		//	We cannot compare to the original cow after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(cow, "This is a test");
	}
	#[test]
	fn from__u8() {
		let body = UnpackedResponseBody::from(65);
		assert_eq!(body, UnpackedResponseBody(vec![65]));
		assert_eq!(body, UnpackedResponseBody(b"A".to_vec()));
	}
	#[test]
	fn from__vec_u8() {
		let body = UnpackedResponseBody::from(b"This is a test".to_vec());
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		let vec  = b"This is another test".to_vec();
		let body = UnpackedResponseBody::from(vec);
		assert_eq!(body, UnpackedResponseBody(b"This is another test".to_vec()));
		//	We cannot compare to the original vec after calling from(),
		//	because it has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		//assert_eq!(vec,  b"This is another test".to_vec());
		
	}
	#[test]
	fn from__vec_u8_ref() {
		let vec  = b"This is a test".to_vec();
		let body = UnpackedResponseBody::from(&vec);
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
		assert_eq!(vec,  b"This is a test".to_vec());
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		//	Basic ASCII string
		assert_ok_eq!(UnpackedResponseBody::from_str("Test"), UnpackedResponseBody(b"Test".to_vec()));
		//	Strings containing different sizes of UTF8 characters
		assert_ok_eq!(UnpackedResponseBody::from_str("ñ"),    UnpackedResponseBody(s!("ñ").into_bytes()));
		assert_ok_eq!(UnpackedResponseBody::from_str("Ḁ"),    UnpackedResponseBody(s!("Ḁ").into_bytes()));
		assert_ok_eq!(UnpackedResponseBody::from_str("𐍈"),    UnpackedResponseBody(s!("𐍈").into_bytes()));
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
	
	//		write_str															
	#[test]
	fn write_str() {
		let mut body = UnpackedResponseBody(b"This is".to_vec());
		assert_ok!(body.write_str(" a test"));
		assert_eq!(body, UnpackedResponseBody(b"This is a test".to_vec()));
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


