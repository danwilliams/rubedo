#![allow(non_snake_case)]

//		Packages

use super::*;
use claims::assert_err;
use serde::Serialize;
use std::fmt::{Debug, self};



//		Enums

//		Position																
#[derive(Copy, Clone, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
#[serde(into = "u8", try_from = "u8")]
enum Position {
	Zero = 0,
	One  = 1,
	Two  = 2,
}

impl Debug for Position {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::Zero => write!(f, "0: Zero"),
			Self::One  => write!(f, "1: One"),
			Self::Two  => write!(f, "2: Two"),
		}
	}
}

impl Display for Position {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::Zero => write!(f, "Position Zero"),
			Self::One  => write!(f, "Position One"),
			Self::Two  => write!(f, "Position Two"),
		}
	}
}

impl From<Position> for String {
	//		from																
	fn from(position: Position) -> Self {
		String::from(&position)
	}
}

impl From<&Position> for String {
	//		from																
	fn from(position: &Position) -> Self {
		match *position {
			Position::Zero => "Zero",
			Position::One  => "One",
			Position::Two  => "Two",
		}.to_owned()
	}
}

impl From<Position> for u8 {
	//		from																
	fn from(position: Position) -> Self {
		position as Self
	}
}

impl From<&Position> for u8 {
	//		from																
	fn from(position: &Position) -> Self {
		*position as Self
	}
}

impl TryFrom<String> for Position {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		match value.as_str() {
			"Zero" => Ok(Self::Zero),
			"One"  => Ok(Self::One),
			"Two"  => Ok(Self::Two),
			_      => Err(format!("Invalid value for Position: {}", value)),
		}
	}
}

impl TryFrom<u8> for Position {
	type Error = String;
	
	//		try_from															
	fn try_from(value: u8) -> Result<Self, Self::Error> {
		match value {
			0  => Ok(Self::Zero),
			1  => Ok(Self::One),
			2  => Ok(Self::Two),
			_  => Err(format!("Invalid value for Position: {}", value)),
		}
	}
}



//		Structs

//		PosIntoInt																
#[derive(Serialize)]
struct PosIntoInt {
	foo: Position,
}

//		PosIntoString															
#[derive(Serialize)]
struct PosIntoString {
	#[serde(serialize_with = "into_string")]
	foo: Position,
}

//		PosIntoIntGeneric														
#[derive(Serialize)]
struct PosIntoIntGeneric {
	#[serde(serialize_with = "into::<Position, u8, __S>")]
	foo: Position,
}

//		PosIntoStringGeneric													
#[derive(Serialize)]
struct PosIntoStringGeneric {
	#[serde(serialize_with = "into::<Position, String, __S>")]
	foo: Position,
}

//		PosTryFromInt															
#[derive(Deserialize)]
struct PosTryFromInt {
	foo: Position,
}

//		PosTryFromString														
#[derive(Debug, Deserialize)]
struct PosTryFromString {
	#[serde(deserialize_with = "try_from_string")]
	foo: Position,
}

//		PosTryFromIntGeneric													
#[derive(Debug, Deserialize)]
struct PosTryFromIntGeneric {
	#[serde(deserialize_with = "try_from::<Position, u8, __D>")]
	foo: Position,
}

//		PosTryFromStringGeneric													
#[derive(Debug, Deserialize)]
struct PosTryFromStringGeneric {
	#[serde(deserialize_with = "try_from::<Position, String, __D>")]
	foo: Position,
}



//		Tests

//		into_string																
#[test]
fn into_string__int() {
	//	This tests the default behaviour, i.e. without serialize_with applied
	let test = PosIntoInt {
		foo: Position::One,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":1}"#);
}
#[test]
fn into_string__str() {
	let test = PosIntoString {
		foo: Position::Two,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Two"}"#);
}

//		into__string_generic													
#[test]
fn into__int() {
	let test = PosIntoIntGeneric {
		foo: Position::One,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":1}"#);
}
#[test]
fn into__str() {
	let test = PosIntoStringGeneric {
		foo: Position::Two,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Two"}"#);
}

//		try_from_string															
#[test]
fn try_from_string__int() {
	//	This tests the default behaviour, i.e. without deserialize_with applied
	let test: PosTryFromInt = serde_json::from_str(r#"{"foo":1}"#).unwrap();
	assert_eq!(test.foo, Position::One);
}
#[test]
fn try_from_string__string() {
	let test: PosTryFromString = serde_json::from_str(r#"{"foo":"Two"}"#).unwrap();
	assert_eq!(test.foo, Position::Two);
}
#[test]
fn try_from_string__absent() {
	let test: Result<PosTryFromString, _> = serde_json::from_str(r#"{"foo":"Three"}"#);
	assert_err!(&test);
	//	The line and column come from Serde's deserialiser
	assert_eq!(test.unwrap_err().to_string(), "Invalid value for Position: Three at line 1 column 15");
}

//		try_from__string_generic												
#[test]
fn try_from__int_present() {
	let test: PosTryFromIntGeneric = serde_json::from_str(r#"{"foo":2}"#).unwrap();
	assert_eq!(test.foo, Position::Two);
}
#[test]
fn try_from__string_present() {
	let test: PosTryFromStringGeneric = serde_json::from_str(r#"{"foo":"One"}"#).unwrap();
	assert_eq!(test.foo, Position::One);
}
#[test]
fn try_from__int_absent() {
	let test: Result<PosTryFromIntGeneric, _> = serde_json::from_str(r#"{"foo":3}"#);
	assert_err!(&test);
	//	The line and column come from Serde's deserialiser
	assert_eq!(test.unwrap_err().to_string(), "Invalid value for Position: 3 at line 1 column 9");
}
#[test]
fn try_from__string_absent() {
	let test: Result<PosTryFromStringGeneric, _> = serde_json::from_str(r#"{"foo":"Three"}"#);
	assert_err!(&test);
	//	The line and column come from Serde's deserialiser
	assert_eq!(test.unwrap_err().to_string(), "Invalid value for Position: Three at line 1 column 15");
}


