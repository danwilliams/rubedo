#![allow(non_snake_case)]

//		Packages

use super::*;
use crate::sugar::s;
use assert_json_diff::assert_json_eq;
use claims::{assert_err, assert_err_eq, assert_ok_eq};
use serde_json::json;



//		Constants

const EMPTY_256_HASH:  [u8; 32] = [0; 32];
const TEST_256_HASH:   [u8; 32] = [
	0xbe, 0xef, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0x0d, 0x1e, 0x2f, 0x3a, 0x4b,
	0x5c, 0x6d, 0x7e, 0x8f, 0x9a, 0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f,
];
const TEST_256_HEX:    &str     = "beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f";
const TEST_256_BASE64: &str     = "vu8aKzxNXm96i5wNHi86S1xtfo+aCxwtPk9aa3yNng8=";



//		Tests

//		Sha256Hash																
#[cfg(test)]
mod sha256_hash__struct {
	use super::*;
	
	//		new																	
	#[test]
	fn new() {
		let hash = Sha256Hash::new(TEST_256_HASH);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
		
		let hash = Sha256Hash::new(&TEST_256_HASH);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
}

#[cfg(test)]
mod sha256_hash__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let hash       = Sha256Hash { hash: TEST_256_HASH };
		let byte_slice = hash.as_bytes();
		
		//	Ensure the byte slice matches the original hash's bytes.
		assert_eq!(*byte_slice, TEST_256_HASH);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original hash.
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		as_mut_bytes														
	#[test]
	fn as_mut_bytes() {
		let mut hash   = Sha256Hash { hash: TEST_256_HASH };
		let byte_array = hash.as_mut_bytes();
		
		//	Ensure the byte array matches the original hash's bytes.
		assert_eq!(*byte_array, TEST_256_HASH);
		
		// We can modify the byte array.
		byte_array[10] = 84;
		assert_ne!(*byte_array, TEST_256_HASH);
		
		//	as_mut_bytes() doesn't consume the original hash, but modifying
		//	the returned array will have affected the hash's contents.
		assert_ne!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		into_bytes															
	#[test]
	fn into_bytes() {
		let hash           = Sha256Hash { hash: TEST_256_HASH };
		let mut byte_array = hash.into_bytes();
		
		//	Ensure the byte array matches the original hash's bytes.
		assert_eq!(byte_array, TEST_256_HASH);
		
		// We can modify the byte array.
		byte_array[10]     = 84;
		assert_ne!(byte_array, TEST_256_HASH);
		
		//	We can't use the original hash after calling into_bytes(), because it
		//	has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(hash, Sha256Hash { hash: TEST_HASH });
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let hash           = Sha256Hash { hash: TEST_256_HASH };
		let mut byte_clone = hash.to_bytes();
		
		//	Ensure the clone matches the original hash's bytes.
		assert_eq!(byte_clone, TEST_256_HASH);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_256_HASH);
		
		//	to_bytes() doesn't consume or affect the original hash.
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let hash = Sha256Hash::from_bytes(TEST_256_HASH);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash.to_string(), TEST_256_HEX);
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash.to_base64(), TEST_256_BASE64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let hash = Sha256Hash::from_base64(TEST_256_BASE64).unwrap();
		assert_eq!(hash.hash, TEST_256_HASH);
		
		let hash = Sha256Hash::from_base64("").unwrap();
		assert_eq!(hash.hash, EMPTY_256_HASH);
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(Sha256Hash::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash.to_hex(), TEST_256_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let hash = Sha256Hash::from_hex(TEST_256_HEX).unwrap();
		assert_eq!(hash.hash, TEST_256_HASH);
		
		let hash = Sha256Hash::from_hex("").unwrap();
		assert_eq!(hash.hash, EMPTY_256_HASH);
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(Sha256Hash::from_hex("invalid@@hex"));
	}
	
	//		into_vec															
	#[test]
	fn into_vec() {
		let hash         = Sha256Hash { hash: TEST_256_HASH };
		let mut byte_vec = hash.into_vec();
		
		//	Ensure the byte vector matches the original hash's vec.
		assert_eq!(byte_vec, TEST_256_HASH.to_vec());
		
		// We can modify the byte vector.
		byte_vec[10]     = 84;
		assert_ne!(byte_vec, TEST_256_HASH.to_vec());
		
		//	We can't use the original hash after calling into_vec(), because it
		//	has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(hash, Sha256Hash { hash: TEST_HASH });
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let hash           = Sha256Hash { hash: TEST_256_HASH };
		let mut byte_clone = hash.to_vec();
		
		//	Ensure the clone matches the original hash's vec.
		assert_eq!(byte_clone, TEST_256_HASH.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_256_HASH.to_vec());
		
		//	to_vec() doesn't consume or affect the original hash.
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
}

#[cfg(test)]
mod sha256_hash__traits {
	use super::*;
	
	//		as_mut																
	#[test]
	fn as_mut() {
		//	Same tests as for as_mut_bytes().
		let mut hash   = Sha256Hash { hash: TEST_256_HASH };
		let byte_array = hash.as_mut();
		assert_eq!(*byte_array, TEST_256_HASH);
		
		byte_array[10] = 84;
		assert_ne!(*byte_array, TEST_256_HASH);
		assert_ne!(hash,        Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let hash       = Sha256Hash { hash: TEST_256_HASH };
		let byte_slice = hash.as_ref();
		assert_eq!(*byte_slice, TEST_256_HASH);
		assert_eq!(hash,        Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		clone																
	#[test]
	fn clone() {
		let mut hash   = Sha256Hash { hash: TEST_256_HASH };
		let clone      = hash.clone();
		assert_eq!(clone, Sha256Hash { hash: TEST_256_HASH });
		
		let byte_array = hash.as_mut();
		byte_array[10] = 84;
		assert_ne!(hash,  Sha256Hash { hash: TEST_256_HASH });
		assert_eq!(clone, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let hash      = Sha256Hash { hash: TEST_256_HASH };
		let mut clone = Sha256Hash { hash: EMPTY_256_HASH };
		clone.clone_from(&hash);
		assert_eq!(hash,  Sha256Hash { hash: TEST_256_HASH });
		assert_eq!(clone, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		debug																
	#[test]
	fn debug() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(format!("{:?}", hash), TEST_256_HEX);
	}
	
	//		default																
	#[test]
	fn default() {
		let hash = Sha256Hash::default();
		assert_eq!(hash, Sha256Hash { hash: EMPTY_256_HASH });
	}
	
	//		display																
	#[test]
	fn display() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(format!("{}", hash), TEST_256_HEX);
	}
	
	//		from																
	#[test]
	fn from__fixed_length_byte_array() {
		let hash       = Sha256Hash::from(TEST_256_HASH);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn from__fixed_length_byte_slice() {
		let hash       = Sha256Hash::from(&TEST_256_HASH);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn from__generic_array() {
		let array = GenericArray::from(TEST_256_HASH);
		let hash  = Sha256Hash::from(array);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn from__generic_array_ref() {
		let array = GenericArray::from(TEST_256_HASH);
		let hash  = Sha256Hash::from(&array);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_ok_eq!(Sha256Hash::from_str(TEST_256_HEX), Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn from_str__err_decoding() {
		let err = Sha256Hash::from_str("invalid@@hex");
		assert_err_eq!(err, ByteSizedError::InvalidHexString);
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is not in valid hexadecimal format"));
	}
	#[test]
	fn from_str__err_too_long() {
		let err = Sha256Hash::from_str("010203040506070809101112131415161718192021222324252627282930313233");
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn from_str__err_too_short() {
		let err = Sha256Hash::from_str("01020304050607080910111213141516171819202122232425262728293031");
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let hash = Sha256Hash::force_from(&TEST_256_HASH[..]);
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
		
		let hash = Sha256Hash::force_from(&TEST_256_HASH[..31]);
		assert_ne!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn force_from__vec_u8() {
		let hash = Sha256Hash::force_from(TEST_256_HASH.to_vec());
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
		
		let hash = Sha256Hash::force_from(TEST_256_HASH[..31].to_vec());
		assert_ne!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let hash = Sha256Hash::force_from(&TEST_256_HASH.to_vec());
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
		
		let hash = Sha256Hash::force_from(&TEST_256_HASH[..31].to_vec());
		assert_ne!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
		assert_ne!(hash, Sha256Hash { hash: EMPTY_256_HASH });
	}
	#[test]
	fn partial_eq__fixed_length_byte_array() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash, TEST_256_HASH);
		assert_ne!(hash, EMPTY_256_HASH);
	}
	#[test]
	fn partial_eq__fixed_length_byte_slice() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_eq!(hash, &TEST_256_HASH);
		assert_ne!(hash, &EMPTY_256_HASH);
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let hash = Sha256Hash { hash: TEST_256_HASH };
		let json = json!(TEST_256_HEX);
		assert_json_eq!(json!(hash), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let json = r#""beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f""#;
		let hash = Sha256Hash { hash: TEST_256_HASH };
		assert_ok_eq!(serde_json::from_str::<Sha256Hash>(&json), hash);
	}
	
	//		try_from															
	#[test]
	fn try_from__byte_slice() {
		let hash = Sha256Hash::try_from(&TEST_256_HASH[..]);
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__byte_slice__err_too_long() {
		let array: [u8; 33] = [0; 33];
		let err = Sha256Hash::try_from(&array[..]);
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn try_from__byte_slice__err_too_short() {
		let err = Sha256Hash::try_from(&TEST_256_HASH[..31]);
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	#[test]
	fn try_from__str() {
		let hash = Sha256Hash::try_from("beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f");
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__str_ref() {
		let hash = Sha256Hash::try_from(TEST_256_HEX);
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__string() {
		let hash = Sha256Hash::try_from(TEST_256_HEX.to_owned());
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__string_ref() {
		let hash = Sha256Hash::try_from(&TEST_256_HEX.to_owned());
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__box_str() {
		let box_str = TEST_256_HEX.to_owned().into_boxed_str();
		let hash    = Sha256Hash::try_from(box_str);
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed(TEST_256_HEX);
		let hash              = Sha256Hash::try_from(cow);
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(TEST_256_HEX.to_owned());
		let hash              = Sha256Hash::try_from(cow);
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__vec_u8() {
		let hash = Sha256Hash::try_from(TEST_256_HASH.to_vec());
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn try_from__vec_u8_ref() {
		let hash = Sha256Hash::try_from(&TEST_256_HASH.to_vec());
		assert_ok_eq!(hash, Sha256Hash { hash: TEST_256_HASH });
	}
}


