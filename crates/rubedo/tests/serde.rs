#![allow(unused_crate_dependencies)]

//	Lints specifically disabled for integration tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::tests_outside_test_module,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
))]



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


