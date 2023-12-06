#![allow(non_snake_case)]

//		Packages

use super::*;
use crate::sugar::s;
use claims::{assert_err, assert_ok};
use serde::Serialize;
use std::fmt::{Debug, self};



//		Enums

//		Position																
///	This enum is used to test the `into` and `try_from` functions. It represents
/// the typical use case for enums, as deserialisation is not guaranteed to give
/// a valid result.
#[derive(Copy, Clone, Deserialize, PartialEq, Serialize)]
#[repr(u8)]
#[serde(into = "u8", try_from = "u8")]
enum Position {
	Zero = 0,
	One  = 1,
	Two  = 2,
}

impl AsStr for Position {
	//		as_str																
	#[must_use]
	fn as_str(&self) -> &'static str {
		match *self {
			Self::Zero => "Zero",
			Self::One  => "One",
			Self::Two  => "Two",
		}
	}
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
		write!(f, "Position {}", self.as_str())
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
		position.as_str().to_owned()
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

impl FromStr for Position {
	type Err = String;
	
	//		from_str															
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.as_str() {
			"Zero" => Ok(Self::Zero),
			"One"  => Ok(Self::One),
			"Two"  => Ok(Self::Two),
			_      => Err(format!("Invalid value for Position: {}", s)),
		}
	}
}

impl TryFrom<String> for Position {
	type Error = String;
	
	//		try_from															
	fn try_from(value: String) -> Result<Self, Self::Error> {
		value.as_str().parse()
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

//		PositionInfallible														
///	This enum is used to test the `from` function. It represents an atypical use
/// case for enums, as deserialisation is not guaranteed to give a valid result,
/// but the `from` function is infallible. It only has the implementations that
/// are necessary for the tests, and sets a default to use in case of no match.
#[derive(Copy, Clone, Default, Deserialize, PartialEq)]
#[repr(u8)]
#[serde(into = "u8", try_from = "u8")]
enum PositionInfallible {
	#[default]
	Zero = 0,
	One  = 1,
	Two  = 2,
}

impl Debug for PositionInfallible {
	//		fmt																	
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::Zero => write!(f, "0: Zero"),
			Self::One  => write!(f, "1: One"),
			Self::Two  => write!(f, "2: Two"),
		}
	}
}

impl From<String> for PositionInfallible {
	fn from(value: String) -> Self {
		match value.as_str() {
			"Zero" => Self::Zero,
			"One"  => Self::One,
			"Two"  => Self::Two,
			_      => Self::default(),
		}
	}
}

impl From<u8> for PositionInfallible {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Zero,
			1 => Self::One,
			2 => Self::Two,
			_ => Self::default(),
		}
	}
}



//		Structs

//		StringStandard															
#[derive(Serialize)]
struct StringStandard {
	foo: String,
}

//		StringAsStr																
#[derive(Serialize)]
struct StringAsStr {
	#[serde(serialize_with = "as_str")]
	foo: String,
}

//		PosAsStr																
#[derive(Serialize)]
struct PosAsStr {
	#[serde(serialize_with = "as_str")]
	foo: Position,
}

//		StringToString															
#[derive(Serialize)]
struct StringToString {
	#[serde(serialize_with = "to_string")]
	foo: String,
}

//		IntToString																
#[derive(Serialize)]
struct IntToString {
	#[serde(serialize_with = "to_string")]
	foo: u32,
}

//		FloatToString															
#[derive(Serialize)]
struct FloatToString {
	#[serde(serialize_with = "to_string")]
	foo: f32,
}

//		BoolToString															
#[derive(Serialize)]
struct BoolToString {
	#[serde(serialize_with = "to_string")]
	foo: bool,
}

//		PosToString																
#[derive(Serialize)]
struct PosToString {
	#[serde(serialize_with = "to_string")]
	foo: Position,
}

//		StringFromStr															
#[derive(Deserialize)]
struct StringFromStr {
	#[serde(deserialize_with = "from_str")]
	foo: String,
}

//		IntFromStr																
#[derive(Deserialize)]
struct IntFromStr {
	#[serde(deserialize_with = "from_str")]
	foo: u32,
}

//		FloatFromStr															
#[derive(Deserialize)]
struct FloatFromStr {
	#[serde(deserialize_with = "from_str")]
	foo: f32,
}

//		BoolFromStr																
#[derive(Deserialize)]
struct BoolFromStr {
	#[serde(deserialize_with = "from_str")]
	foo: bool,
}

//		PosFromStr																
#[derive(Deserialize)]
struct PosFromStr {
	#[serde(deserialize_with = "from_str")]
	foo: Position,
}

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

//		PosFromInt																
#[derive(Debug, Deserialize)]
struct PosFromInt {
	foo: PositionInfallible,
}

//		PosFromString															
#[derive(Debug, Deserialize)]
struct PosFromString {
	#[serde(deserialize_with = "from_string")]
	foo: PositionInfallible,
}

//		PosFromIntGeneric														
#[derive(Debug, Deserialize)]
struct PosFromIntGeneric {
	#[serde(deserialize_with = "from::<PositionInfallible, u8, __D>")]
	foo: PositionInfallible,
}

//		PosFromStringGeneric													
#[derive(Debug, Deserialize)]
struct PosFromStringGeneric {
	#[serde(deserialize_with = "from::<PositionInfallible, String, __D>")]
	foo: PositionInfallible,
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

//		as_str																	
#[test]
fn as_str__string_standard() {
	let test = StringStandard {
		foo: s!("Test"),
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Test"}"#);
}
#[test]
fn as_str__string_as_str() {
	let test = StringAsStr {
		foo: s!("Test"),
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Test"}"#);
}
#[test]
fn as_str__pos_as_string() {
	let test = PosAsStr {
		foo: Position::Two,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Two"}"#);
}

//		to_string																
#[test]
fn to_string__string() {
	let test = StringToString {
		foo: s!("Test"),
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Test"}"#);
}
#[test]
fn to_string__int() {
	let test = IntToString {
		foo: 1_234,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"1234"}"#);
}
#[test]
fn to_string__float() {
	let test = FloatToString {
		foo: 12.34,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"12.34"}"#);
}
#[test]
fn to_string__bool() {
	let test = BoolToString {
		foo: true,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"true"}"#);
}
#[test]
fn to_string__pos() {
	let test = PosToString {
		foo: Position::Two,
	};
	assert_eq!(serde_json::to_string(&test).unwrap(), r#"{"foo":"Position Two"}"#);
}

//		from_str																
#[test]
fn from_str__string() {
	let test: StringFromStr = serde_json::from_str(r#"{"foo":"Test"}"#).unwrap();
	assert_eq!(test.foo, s!("Test"));
}
#[test]
fn from_str__int() {
	let test: IntFromStr = serde_json::from_str(r#"{"foo":"1234"}"#).unwrap();
	assert_eq!(test.foo, 1234);
}
#[test]
fn from_str__float() {
	let test: FloatFromStr = serde_json::from_str(r#"{"foo":"12.34"}"#).unwrap();
	assert_eq!(test.foo, 12.34);
}
#[test]
fn from_str__bool() {
	let test: BoolFromStr = serde_json::from_str(r#"{"foo":"true"}"#).unwrap();
	assert_eq!(test.foo, true);
}
#[test]
fn from_str__pos() {
	let test: PosFromStr = serde_json::from_str(r#"{"foo":"Two"}"#).unwrap();
	assert_eq!(test.foo, Position::Two);
}

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

//		from_string																
#[test]
fn from_string__int() {
	//	This tests the default behaviour, i.e. without deserialize_with applied
	let test: PosFromInt = serde_json::from_str(r#"{"foo":1}"#).unwrap();
	assert_eq!(test.foo, PositionInfallible::One);
}
#[test]
fn from_string__string() {
	let test: PosFromString = serde_json::from_str(r#"{"foo":"Two"}"#).unwrap();
	assert_eq!(test.foo, PositionInfallible::Two);
}
#[test]
fn from_string__absent() {
	let test: Result<PosFromString, _> = serde_json::from_str(r#"{"foo":"Three"}"#);
	assert_ok!(&test);
	assert_eq!(test.unwrap().foo, PositionInfallible::Zero);
}

//		from__string_generic													
#[test]
fn from__int_present() {
	let test: PosFromIntGeneric = serde_json::from_str(r#"{"foo":2}"#).unwrap();
	assert_eq!(test.foo, PositionInfallible::Two);
}
#[test]
fn from__string_present() {
	let test: PosFromStringGeneric = serde_json::from_str(r#"{"foo":"One"}"#).unwrap();
	assert_eq!(test.foo, PositionInfallible::One);
}
#[test]
fn from__int_absent() {
	let test: Result<PosFromIntGeneric, _> = serde_json::from_str(r#"{"foo":3}"#);
	assert_ok!(&test);
	assert_eq!(test.unwrap().foo, PositionInfallible::Zero);
}
#[test]
fn from__string_absent() {
	let test: Result<PosFromStringGeneric, _> = serde_json::from_str(r#"{"foo":"Three"}"#);
	assert_ok!(&test);
	assert_eq!(test.unwrap().foo, PositionInfallible::Zero);
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


