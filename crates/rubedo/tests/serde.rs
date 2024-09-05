#![allow(non_snake_case)]
#![allow(unused_crate_dependencies)]

//		Packages

use rubedo::{
	serde::as_str,
	sugar::s,
	std::AsStr,
};
use serde::{Serialize, Serializer};



//		Structs

//		AsStrAsStr																
#[derive(Serialize)]
struct AsStrAsStr {
	#[serde(serialize_with = "as_str")]
	foo: String,
}

//		AsStrHelper																
#[derive(Serialize)]
struct AsStrHelper {
	#[serde(serialize_with = "helper")]
	foo: String,
}



//		Functions

//		helper																	
fn helper<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: AsStr,
	S: Serializer,
{
	serializer.serialize_str(value.as_str())
}



//		Tests

//		AsStr																	
#[test]
fn as_str__as_str() {
	let test = AsStrAsStr {
		foo: s!("Test"),
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Test"}"#);
}
#[test]
fn as_str__helper() {
	let test = AsStrHelper {
		foo: s!("Test"),
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Test"}"#);
}


