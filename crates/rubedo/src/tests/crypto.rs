//		Packages

use super::*;
use crate::sugar::s;
use assert_json_diff::assert_json_eq;
use claims::{assert_err, assert_err_eq, assert_ok_eq};
use rand::rngs::OsRng;
use serde_json::json;
use std::hash::DefaultHasher;



//		Constants

const HASH_INPUT:      &str     = "This is a test";
const EMPTY_256_HASH:  [u8; 32] = [0; 32];
const EMPTY_512_HASH:  [u8; 64] = [0; 64];
const TEST_256_HASH:   [u8; 32] = [
	0xc7, 0xbe, 0x1e, 0xd9, 0x02, 0xfb, 0x8d, 0xd4, 0xd4, 0x89, 0x97, 0xc6, 0x45, 0x2f, 0x5d, 0x7e,
	0x50, 0x9f, 0xbc, 0xdb, 0xe2, 0x80, 0x8b, 0x16, 0xbc, 0xf4, 0xed, 0xce, 0x4c, 0x07, 0xd1, 0x4e,
];
const TEST_512_HASH:   [u8; 64] = [
	0xa0, 0x28, 0xd4, 0xf7, 0x4b, 0x60, 0x2b, 0xa4, 0x5e, 0xb0, 0xa9, 0x3c, 0x9a, 0x46, 0x77, 0x24,
	0x0d, 0xcf, 0x28, 0x1a, 0x1a, 0x93, 0x22, 0xf1, 0x83, 0xbd, 0x32, 0xf0, 0xbe, 0xd8, 0x2e, 0xc7,
	0x2d, 0xe9, 0xc3, 0x95, 0x7b, 0x2f, 0x4c, 0x9a, 0x1c, 0xcf, 0x7e, 0xd1, 0x4f, 0x85, 0xd7, 0x34,
	0x98, 0xdf, 0x38, 0x01, 0x7e, 0x70, 0x3d, 0x47, 0xeb, 0xb9, 0xf0, 0xb3, 0xbf, 0x11, 0x6f, 0x69,
];
const TEST_PRVKEY:     [u8; 32] = [
	0xbe, 0xef, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0x0d, 0x1e, 0x2f, 0x3a, 0x4b,
	0x5c, 0x6d, 0x7e, 0x8f, 0x9a, 0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f,
];
const TEST_PUBKEY:     [u8; 32] = [
	0x9f, 0xd7, 0xb9, 0xe7, 0x28, 0xde, 0x47, 0xab, 0x7d, 0x9d, 0x81, 0x6e, 0x70, 0x57, 0x60, 0x6d,
	0xd3, 0x02, 0xf3, 0x8d, 0xde, 0xe6, 0x42, 0x72, 0xe0, 0xed, 0x93, 0x3f, 0x08, 0x96, 0xbc, 0x8e,
];
const TEST_256_HEX:    &str     = "c7be1ed902fb8dd4d48997c6452f5d7e509fbcdbe2808b16bcf4edce4c07d14e";
const TEST_512_HEX:    &str     = "a028d4f74b602ba45eb0a93c9a4677240dcf281a1a9322f183bd32f0bed82ec7\
                                   2de9c3957b2f4c9a1ccf7ed14f85d73498df38017e703d47ebb9f0b3bf116f69";
const TEST_PRVKEY_HEX: &str     = "beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f";
const TEST_PUBKEY_HEX: &str     = "9fd7b9e728de47ab7d9d816e7057606dd302f38ddee64272e0ed933f0896bc8e";
const TEST_256_BASE64: &str     = "x74e2QL7jdTUiZfGRS9dflCfvNvigIsWvPTtzkwH0U4=";
const TEST_512_BASE64: &str     = "oCjU90tgK6ResKk8mkZ3JA3PKBoakyLxg70y8L7YLsct\
                                   6cOVey9MmhzPftFPhdc0mN84AX5wPUfrufCzvxFvaQ==";
const TEST_PRVKEY_B64: &str     = "vu8aKzxNXm96i5wNHi86S1xtfo+aCxwtPk9aa3yNng8=";
const TEST_PUBKEY_B64: &str     = "n9e55yjeR6t9nYFucFdgbdMC843e5kJy4O2TPwiWvI4=";



//		Tests

//		Sha256Hash																
#[cfg(test)]
mod sha256_hash__struct {
	use super::*;
	
	//		new																	
	#[allow(clippy::needless_borrows_for_generic_args)]
	#[test]
	fn new() {
		let hash1 = Sha256Hash::new(TEST_256_HASH);
		assert_eq!(hash1, Sha256Hash { hash: TEST_256_HASH });
		
		let hash2 = Sha256Hash::new(&TEST_256_HASH);
		assert_eq!(hash2, Sha256Hash { hash: TEST_256_HASH });
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
		let hash1 = Sha256Hash::from_base64(TEST_256_BASE64).unwrap();
		assert_eq!(hash1.hash, TEST_256_HASH);
		
		let hash2 = Sha256Hash::from_base64("").unwrap();
		assert_eq!(hash2.hash, EMPTY_256_HASH);
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
		let hash1 = Sha256Hash::from_hex(TEST_256_HEX).unwrap();
		assert_eq!(hash1.hash, TEST_256_HASH);
		
		let hash2 = Sha256Hash::from_hex("").unwrap();
		assert_eq!(hash2.hash, EMPTY_256_HASH);
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
	#[allow(clippy::clone_on_copy)]
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
		assert_eq!(format!("{hash:?}"), TEST_256_HEX);
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
		assert_eq!(format!("{hash}"), TEST_256_HEX);
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
		let hash1 = Sha256Hash::force_from(&TEST_256_HASH[..]);
		assert_eq!(hash1, Sha256Hash { hash: TEST_256_HASH });
		
		let hash2 = Sha256Hash::force_from(&TEST_256_HASH[..31]);
		assert_ne!(hash2, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn force_from__vec_u8() {
		let hash1 = Sha256Hash::force_from(TEST_256_HASH.to_vec());
		assert_eq!(hash1, Sha256Hash { hash: TEST_256_HASH });
		
		let hash2 = Sha256Hash::force_from(TEST_256_HASH[..31].to_vec());
		assert_ne!(hash2, Sha256Hash { hash: TEST_256_HASH });
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let hash1 = Sha256Hash::force_from(&TEST_256_HASH.to_vec());
		assert_eq!(hash1, Sha256Hash { hash: TEST_256_HASH });
		
		let hash2 = Sha256Hash::force_from(&TEST_256_HASH[..31].to_vec());
		assert_ne!(hash2, Sha256Hash { hash: TEST_256_HASH });
	}
	
	//		hashed																
	#[test]
	fn from_digest() {
		let mut hasher = Sha256::new();
		hasher.update(HASH_INPUT);
		assert_eq!(Sha256Hash::from_digest(hasher.finalize()), Sha256Hash { hash: TEST_256_HASH });
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
		let json = format!(r#""{TEST_256_HEX}""#);
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
		let hash = Sha256Hash::try_from("c7be1ed902fb8dd4d48997c6452f5d7e509fbcdbe2808b16bcf4edce4c07d14e");
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

//		Sha512Hash																
#[cfg(test)]
mod sha512_hash__struct {
	use super::*;
	
	//		new																	
	#[allow(clippy::needless_borrows_for_generic_args)]
	#[test]
	fn new() {
		let hash1 = Sha512Hash::new(TEST_512_HASH);
		assert_eq!(hash1, Sha512Hash { hash: TEST_512_HASH });
		
		let hash2 = Sha512Hash::new(&TEST_512_HASH);
		assert_eq!(hash2, Sha512Hash { hash: TEST_512_HASH });
	}
}

#[cfg(test)]
mod sha512_hash__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let hash       = Sha512Hash { hash: TEST_512_HASH };
		let byte_slice = hash.as_bytes();
		
		//	Ensure the byte slice matches the original hash's bytes.
		assert_eq!(*byte_slice, TEST_512_HASH);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original hash.
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		as_mut_bytes														
	#[test]
	fn as_mut_bytes() {
		let mut hash   = Sha512Hash { hash: TEST_512_HASH };
		let byte_array = hash.as_mut_bytes();
		
		//	Ensure the byte array matches the original hash's bytes.
		assert_eq!(*byte_array, TEST_512_HASH);
		
		// We can modify the byte array.
		byte_array[10] = 84;
		assert_ne!(*byte_array, TEST_512_HASH);
		
		//	as_mut_bytes() doesn't consume the original hash, but modifying
		//	the returned array will have affected the hash's contents.
		assert_ne!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		into_bytes															
	#[test]
	fn into_bytes() {
		let hash           = Sha512Hash { hash: TEST_512_HASH };
		let mut byte_array = hash.into_bytes();
		
		//	Ensure the byte array matches the original hash's bytes.
		assert_eq!(byte_array, TEST_512_HASH);
		
		// We can modify the byte array.
		byte_array[10]     = 84;
		assert_ne!(byte_array, TEST_512_HASH);
		
		//	We can't use the original hash after calling into_bytes(), because it
		//	has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(hash, Sha512Hash { hash: TEST_HASH });
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let hash           = Sha512Hash { hash: TEST_512_HASH };
		let mut byte_clone = hash.to_bytes();
		
		//	Ensure the clone matches the original hash's bytes.
		assert_eq!(byte_clone, TEST_512_HASH);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_512_HASH);
		
		//	to_bytes() doesn't consume or affect the original hash.
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let hash = Sha512Hash::from_bytes(TEST_512_HASH);
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash.to_string(), TEST_512_HEX);
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash.to_base64(), TEST_512_BASE64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let hash1 = Sha512Hash::from_base64(TEST_512_BASE64).unwrap();
		assert_eq!(hash1.hash, TEST_512_HASH);
		
		let hash2 = Sha512Hash::from_base64("").unwrap();
		assert_eq!(hash2.hash, EMPTY_512_HASH);
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(Sha512Hash::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash.to_hex(), TEST_512_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let hash1 = Sha512Hash::from_hex(TEST_512_HEX).unwrap();
		assert_eq!(hash1.hash, TEST_512_HASH);
		
		let hash2 = Sha512Hash::from_hex("").unwrap();
		assert_eq!(hash2.hash, EMPTY_512_HASH);
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(Sha512Hash::from_hex("invalid@@hex"));
	}
	
	//		into_vec															
	#[test]
	fn into_vec() {
		let hash         = Sha512Hash { hash: TEST_512_HASH };
		let mut byte_vec = hash.into_vec();
		
		//	Ensure the byte vector matches the original hash's vec.
		assert_eq!(byte_vec, TEST_512_HASH.to_vec());
		
		// We can modify the byte vector.
		byte_vec[10]     = 84;
		assert_ne!(byte_vec, TEST_512_HASH.to_vec());
		
		//	We can't use the original hash after calling into_vec(), because it
		//	has been consumed.
		//	Uncommenting the line below would cause a compilation error:
		// assert_eq!(hash, Sha512Hash { hash: TEST_HASH });
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let hash           = Sha512Hash { hash: TEST_512_HASH };
		let mut byte_clone = hash.to_vec();
		
		//	Ensure the clone matches the original hash's vec.
		assert_eq!(byte_clone, TEST_512_HASH.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_512_HASH.to_vec());
		
		//	to_vec() doesn't consume or affect the original hash.
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
}

#[cfg(test)]
mod sha512_hash__traits {
	use super::*;
	
	//		as_mut																
	#[test]
	fn as_mut() {
		//	Same tests as for as_mut_bytes().
		let mut hash   = Sha512Hash { hash: TEST_512_HASH };
		let byte_array = hash.as_mut();
		assert_eq!(*byte_array, TEST_512_HASH);
		
		byte_array[10] = 84;
		assert_ne!(*byte_array, TEST_512_HASH);
		assert_ne!(hash,        Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let hash       = Sha512Hash { hash: TEST_512_HASH };
		let byte_slice = hash.as_ref();
		assert_eq!(*byte_slice, TEST_512_HASH);
		assert_eq!(hash,        Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		clone																
	#[allow(clippy::clone_on_copy)]
	#[test]
	fn clone() {
		let mut hash   = Sha512Hash { hash: TEST_512_HASH };
		let clone      = hash.clone();
		assert_eq!(clone, Sha512Hash { hash: TEST_512_HASH });
		
		let byte_array = hash.as_mut();
		byte_array[10] = 84;
		assert_ne!(hash,  Sha512Hash { hash: TEST_512_HASH });
		assert_eq!(clone, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let hash      = Sha512Hash { hash: TEST_512_HASH };
		let mut clone = Sha512Hash { hash: EMPTY_512_HASH };
		clone.clone_from(&hash);
		assert_eq!(hash,  Sha512Hash { hash: TEST_512_HASH });
		assert_eq!(clone, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		debug																
	#[test]
	fn debug() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(format!("{hash:?}"), TEST_512_HEX);
	}
	
	//		default																
	#[test]
	fn default() {
		let hash = Sha512Hash::default();
		assert_eq!(hash, Sha512Hash { hash: EMPTY_512_HASH });
	}
	
	//		display																
	#[test]
	fn display() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(format!("{hash}"), TEST_512_HEX);
	}
	
	//		from																
	#[test]
	fn from__fixed_length_byte_array() {
		let hash       = Sha512Hash::from(TEST_512_HASH);
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn from__fixed_length_byte_slice() {
		let hash       = Sha512Hash::from(&TEST_512_HASH);
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn from__generic_array() {
		let array = GenericArray::from(TEST_512_HASH);
		let hash  = Sha512Hash::from(array);
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn from__generic_array_ref() {
		let array = GenericArray::from(TEST_512_HASH);
		let hash  = Sha512Hash::from(&array);
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_ok_eq!(Sha512Hash::from_str(TEST_512_HEX), Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn from_str__err_decoding() {
		let err = Sha512Hash::from_str("invalid@@hex");
		assert_err_eq!(err, ByteSizedError::InvalidHexString);
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is not in valid hexadecimal format"));
	}
	#[test]
	fn from_str__err_too_long() {
		let err = Sha512Hash::from_str("0102030405060708091011121314151617181920212223242526272829303132\
		                                333435363738394041424344454647484950515253545556575859606162636465");
		assert_err_eq!(err, ByteSizedError::DataTooLong(64));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 64 bytes"));
	}
	#[test]
	fn from_str__err_too_short() {
		let err = Sha512Hash::from_str("0102030405060708091011121314151617181920212223242526272829303132\
		                                33343536373839404142434445464748495051525354555657585960616263");
		assert_err_eq!(err, ByteSizedError::DataTooShort(64));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 64 bytes"));
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let hash1 = Sha512Hash::force_from(&TEST_512_HASH[..]);
		assert_eq!(hash1, Sha512Hash { hash: TEST_512_HASH });
		
		let hash2 = Sha512Hash::force_from(&TEST_512_HASH[..31]);
		assert_ne!(hash2, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn force_from__vec_u8() {
		let hash1 = Sha512Hash::force_from(TEST_512_HASH.to_vec());
		assert_eq!(hash1, Sha512Hash { hash: TEST_512_HASH });
		
		let hash2 = Sha512Hash::force_from(TEST_512_HASH[..31].to_vec());
		assert_ne!(hash2, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let hash1 = Sha512Hash::force_from(&TEST_512_HASH.to_vec());
		assert_eq!(hash1, Sha512Hash { hash: TEST_512_HASH });
		
		let hash2 = Sha512Hash::force_from(&TEST_512_HASH[..31].to_vec());
		assert_ne!(hash2, Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		hashed																
	#[test]
	fn from_digest() {
		let mut hasher = Sha512::new();
		hasher.update(HASH_INPUT);
		assert_eq!(Sha512Hash::from_digest(hasher.finalize()), Sha512Hash { hash: TEST_512_HASH });
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
		assert_ne!(hash, Sha512Hash { hash: EMPTY_512_HASH });
	}
	#[test]
	fn partial_eq__fixed_length_byte_array() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash, TEST_512_HASH);
		assert_ne!(hash, EMPTY_512_HASH);
	}
	#[test]
	fn partial_eq__fixed_length_byte_slice() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_eq!(hash, &TEST_512_HASH);
		assert_ne!(hash, &EMPTY_512_HASH);
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let hash = Sha512Hash { hash: TEST_512_HASH };
		let json = json!(TEST_512_HEX);
		assert_json_eq!(json!(hash), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let json = format!(r#""{TEST_512_HEX}""#);
		let hash = Sha512Hash { hash: TEST_512_HASH };
		assert_ok_eq!(serde_json::from_str::<Sha512Hash>(&json), hash);
	}
	
	//		try_from															
	#[test]
	fn try_from__byte_slice() {
		let hash = Sha512Hash::try_from(&TEST_512_HASH[..]);
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__byte_slice__err_too_long() {
		let array: [u8; 65] = [0; 65];
		let err = Sha512Hash::try_from(&array[..]);
		assert_err_eq!(err, ByteSizedError::DataTooLong(64));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 64 bytes"));
	}
	#[test]
	fn try_from__byte_slice__err_too_short() {
		let err = Sha512Hash::try_from(&TEST_512_HASH[..31]);
		assert_err_eq!(err, ByteSizedError::DataTooShort(64));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 64 bytes"));
	}
	#[test]
	fn try_from__str() {
		let hash = Sha512Hash::try_from("a028d4f74b602ba45eb0a93c9a4677240dcf281a1a9322f183bd32f0bed82ec7\
                                         2de9c3957b2f4c9a1ccf7ed14f85d73498df38017e703d47ebb9f0b3bf116f69");
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__str_ref() {
		let hash = Sha512Hash::try_from(TEST_512_HEX);
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__string() {
		let hash = Sha512Hash::try_from(TEST_512_HEX.to_owned());
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__string_ref() {
		let hash = Sha512Hash::try_from(&TEST_512_HEX.to_owned());
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__box_str() {
		let box_str = TEST_512_HEX.to_owned().into_boxed_str();
		let hash    = Sha512Hash::try_from(box_str);
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed(TEST_512_HEX);
		let hash              = Sha512Hash::try_from(cow);
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(TEST_512_HEX.to_owned());
		let hash              = Sha512Hash::try_from(cow);
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__vec_u8() {
		let hash = Sha512Hash::try_from(TEST_512_HASH.to_vec());
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
	#[test]
	fn try_from__vec_u8_ref() {
		let hash = Sha512Hash::try_from(&TEST_512_HASH.to_vec());
		assert_ok_eq!(hash, Sha512Hash { hash: TEST_512_HASH });
	}
}

//		SigningKey																
#[cfg(test)]
mod signing_key__struct {
	use super::*;
	
	//		generate															
	#[test]
	fn generate() {
		let mut csprng      = OsRng;
		let key: SigningKey = SigningKey::generate(&mut csprng);
		assert_ne!(key,            SigningKey::from_bytes(EMPTY_256_HASH));
		assert_ne!(key.as_bytes(), &EMPTY_256_HASH);
	}
	
	//		into_inner															
	#[test]
	fn into_inner() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.into_inner(), RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	
	//		verifying_key														
	#[test]
	fn verifying_key() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.verifying_key(),            VerifyingKey::from_bytes(TEST_PUBKEY));
		assert_eq!(key.verifying_key().as_bytes(), &TEST_PUBKEY);
	}
}

#[cfg(test)]
mod signing_key__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let key        = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let byte_slice = key.as_bytes();
		
		//	Ensure the byte slice matches the original key's bytes.
		assert_eq!(*byte_slice, TEST_PRVKEY);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original key.
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let key            = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let mut byte_clone = key.to_bytes();
		
		//	Ensure the clone matches the original key's bytes.
		assert_eq!(byte_clone, TEST_PRVKEY);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PRVKEY);
		
		//	to_bytes() doesn't consume or affect the original key.
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let key = SigningKey::from_bytes(TEST_PRVKEY);
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.to_string(), TEST_PRVKEY_HEX);
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.to_base64(), TEST_PRVKEY_B64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let key1 = SigningKey::from_base64(TEST_PRVKEY_B64).unwrap();
		assert_eq!(key1.key, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = SigningKey::from_base64("").unwrap();
		assert_eq!(key2.key, RealSigningKey::from_bytes(&EMPTY_256_HASH));
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(SigningKey::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.to_hex(), TEST_PRVKEY_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let key1 = SigningKey::from_hex(TEST_PRVKEY_HEX).unwrap();
		assert_eq!(key1.key, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = SigningKey::from_hex("").unwrap();
		assert_eq!(key2.key, RealSigningKey::from_bytes(&EMPTY_256_HASH));
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(SigningKey::from_hex("invalid@@hex"));
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let key            = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let mut byte_clone = key.to_vec();
		
		//	Ensure the clone matches the original key's vec.
		assert_eq!(byte_clone, TEST_PRVKEY.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PRVKEY.to_vec());
		
		//	to_vec() doesn't consume or affect the original key.
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
}

#[cfg(test)]
mod signing_key__traits {
	use super::*;
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let key        = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let byte_slice = key.as_ref();
		assert_eq!(*byte_slice, TEST_PRVKEY);
		assert_eq!(key,         SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		clone																
	#[allow(clippy::redundant_clone)]
	#[test]
	fn clone() {
		let key   = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let clone = key.clone();
		assert_eq!(clone, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let key       = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let mut clone = SigningKey { key: RealSigningKey::from_bytes(&EMPTY_256_HASH) };
		clone.clone_from(&key);
		assert_eq!(key,   SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
		assert_eq!(clone, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		debug																
	#[test]
	fn debug() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(format!("{key:?}"), TEST_PRVKEY_HEX);
	}
	
	//		default																
	#[test]
	fn default() {
		let key = SigningKey::default();
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&EMPTY_256_HASH) });
	}
	
	//		deref																
	#[allow(clippy::explicit_deref_methods)]
	#[test]
	fn deref() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key.deref(), &RealSigningKey::from_bytes(&TEST_PRVKEY));
		assert_eq!(&*key,       &RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	#[test]
	fn deref__to_keypair_bytes() {
		let key      = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let mut pair = vec![];
		pair.extend_from_slice(&TEST_PRVKEY);
		pair.extend_from_slice(&TEST_PUBKEY);
		assert_eq!(key.to_keypair_bytes(), RealSigningKey::from_bytes(&TEST_PRVKEY).to_keypair_bytes());
		assert_eq!(key.to_keypair_bytes(), &*pair);
	}
	
	//		display																
	#[test]
	fn display() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(format!("{key}"), TEST_PRVKEY_HEX);
	}
	
	//		from																
	#[test]
	fn from__real_signing_key() {
		let key = SigningKey::from(RealSigningKey::from_bytes(&TEST_PRVKEY));
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn from__real_signing_key_ref() {
		let key = SigningKey::from(&RealSigningKey::from_bytes(&TEST_PRVKEY));
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn from__fixed_length_byte_array() {
		let key = SigningKey::from(TEST_PRVKEY);
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn from__fixed_length_byte_slice() {
		let key = SigningKey::from(&TEST_PRVKEY);
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_ok_eq!(SigningKey::from_str(TEST_PRVKEY_HEX), SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn from_str__err_decoding() {
		let err = SigningKey::from_str("invalid@@hex");
		assert_err_eq!(err, ByteSizedError::InvalidHexString);
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is not in valid hexadecimal format"));
	}
	#[test]
	fn from_str__err_too_long() {
		let err = SigningKey::from_str("010203040506070809101112131415161718192021222324252627282930313233");
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn from_str__err_too_short() {
		let err = SigningKey::from_str("01020304050607080910111213141516171819202122232425262728293031");
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key1 = SigningKey::force_from(&TEST_PRVKEY[..]);
		assert_eq!(key1, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
		
		let key2 = SigningKey::force_from(&TEST_PRVKEY[..31]);
		assert_ne!(key2, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn force_from__vec_u8() {
		let key1 = SigningKey::force_from(TEST_PRVKEY.to_vec());
		assert_eq!(key1, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
		
		let key2 = SigningKey::force_from(TEST_PRVKEY[..31].to_vec());
		assert_ne!(key2, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let key1 = SigningKey::force_from(&TEST_PRVKEY.to_vec());
		assert_eq!(key1, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
		
		let key2 = SigningKey::force_from(&TEST_PRVKEY[..31].to_vec());
		assert_ne!(key2, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	
	//		hash																
	#[test]
	fn hash() {
		let key1        = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let key2        = SigningKey { key: RealSigningKey::from_bytes(&EMPTY_256_HASH) };
		let mut hasher1 = DefaultHasher::new();
		let mut hasher2 = DefaultHasher::new();
		key1.hash(&mut hasher1);
		key2.hash(&mut hasher2);
		assert_ne!(hasher1.finish(), hasher2.finish());
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let key = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
		assert_ne!(key, SigningKey { key: RealSigningKey::from_bytes(&EMPTY_256_HASH) });
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let key  = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		let json = json!(TEST_PRVKEY_HEX);
		assert_json_eq!(json!(key), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let json = format!(r#""{TEST_PRVKEY_HEX}""#);
		let key  = SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) };
		assert_ok_eq!(serde_json::from_str::<SigningKey>(&json), key);
	}
	
	//		try_from															
	#[test]
	fn try_from__byte_slice() {
		let key = SigningKey::try_from(&TEST_PRVKEY[..]);
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__byte_slice__err_too_long() {
		let array: [u8; 33] = [0; 33];
		let err = SigningKey::try_from(&array[..]);
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn try_from__byte_slice__err_too_short() {
		let err = SigningKey::try_from(&TEST_PRVKEY[..31]);
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	#[test]
	fn try_from__str() {
		let key = SigningKey::try_from("beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f");
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__str_ref() {
		let key = SigningKey::try_from(TEST_PRVKEY_HEX);
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__string() {
		let key = SigningKey::try_from(TEST_PRVKEY_HEX.to_owned());
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__string_ref() {
		let key = SigningKey::try_from(&TEST_PRVKEY_HEX.to_owned());
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__box_str() {
		let box_str = TEST_PRVKEY_HEX.to_owned().into_boxed_str();
		let key     = SigningKey::try_from(box_str);
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed(TEST_PRVKEY_HEX);
		let key               = SigningKey::try_from(cow);
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(TEST_PRVKEY_HEX.to_owned());
		let key               = SigningKey::try_from(cow);
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__vec_u8() {
		let key = SigningKey::try_from(TEST_PRVKEY.to_vec());
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
	#[test]
	fn try_from__vec_u8_ref() {
		let key = SigningKey::try_from(&TEST_PRVKEY.to_vec());
		assert_ok_eq!(key, SigningKey { key: RealSigningKey::from_bytes(&TEST_PRVKEY) });
	}
}

//§		SigningKeyExt															
#[cfg(test)]
mod signing_key_ext__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let key        = RealSigningKey::from_bytes(&TEST_PRVKEY);
		let byte_slice = ByteSized::as_bytes(&key);
		
		//	Ensure the byte slice matches the original key's bytes.
		assert_eq!(*byte_slice, TEST_PRVKEY);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original key.
		assert_eq!(key, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let key            = RealSigningKey::from_bytes(&TEST_PRVKEY);
		let mut byte_clone = ByteSized::to_bytes(&key);
		
		//	Ensure the clone matches the original key's bytes.
		assert_eq!(byte_clone, TEST_PRVKEY);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PRVKEY);
		
		//	to_bytes() doesn't consume or affect the original key.
		assert_eq!(key, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let key = <RealSigningKey as ByteSized<32>>::from_bytes(TEST_PRVKEY);
		assert_eq!(key, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let key = RealSigningKey::from_bytes(&TEST_PRVKEY);
		assert_eq!(key.to_base64(), TEST_PRVKEY_B64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let key1 = RealSigningKey::from_base64(TEST_PRVKEY_B64).unwrap();
		assert_eq!(key1, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = RealSigningKey::from_base64("").unwrap();
		assert_eq!(key2, RealSigningKey::from_bytes(&EMPTY_256_HASH));
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(RealSigningKey::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let key = RealSigningKey::from_bytes(&TEST_PRVKEY);
		assert_eq!(key.to_hex(), TEST_PRVKEY_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let key1 = RealSigningKey::from_hex(TEST_PRVKEY_HEX).unwrap();
		assert_eq!(key1, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = RealSigningKey::from_hex("").unwrap();
		assert_eq!(key2, RealSigningKey::from_bytes(&EMPTY_256_HASH));
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(RealSigningKey::from_hex("invalid@@hex"));
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let key            = RealSigningKey::from_bytes(&TEST_PRVKEY);
		let mut byte_clone = key.to_vec();
		
		//	Ensure the clone matches the original key's vec.
		assert_eq!(byte_clone, TEST_PRVKEY.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PRVKEY.to_vec());
		
		//	to_vec() doesn't consume or affect the original key.
		assert_eq!(key, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
}

#[cfg(test)]
mod signing_key_ext__traits {
	use super::*;
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key1 = RealSigningKey::force_from(&TEST_PRVKEY[..]);
		assert_eq!(key1, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = RealSigningKey::force_from(&TEST_PRVKEY[..31]);
		assert_ne!(key2, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	#[test]
	fn force_from__vec_u8() {
		let key1 = RealSigningKey::force_from(TEST_PRVKEY.to_vec());
		assert_eq!(key1, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = RealSigningKey::force_from(TEST_PRVKEY[..31].to_vec());
		assert_ne!(key2, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let key1 = RealSigningKey::force_from(&TEST_PRVKEY.to_vec());
		assert_eq!(key1, RealSigningKey::from_bytes(&TEST_PRVKEY));
		
		let key2 = RealSigningKey::force_from(&TEST_PRVKEY[..31].to_vec());
		assert_ne!(key2, RealSigningKey::from_bytes(&TEST_PRVKEY));
	}
}

//		VerifyingKey															
#[cfg(test)]
mod verifying_key__struct {
	use super::*;
	
	//		into_inner															
	#[test]
	fn into_inner() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key.into_inner(), RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
}

#[cfg(test)]
mod verifying_key__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let key        = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let byte_slice = key.as_bytes();
		
		//	Ensure the byte slice matches the original key's bytes.
		assert_eq!(*byte_slice, TEST_PUBKEY);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original key.
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let key            = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let mut byte_clone = key.to_bytes();
		
		//	Ensure the clone matches the original key's bytes.
		assert_eq!(byte_clone, TEST_PUBKEY);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PUBKEY);
		
		//	to_bytes() doesn't consume or affect the original key.
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let key = VerifyingKey::from_bytes(TEST_PUBKEY);
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		to_string															
	#[test]
	fn to_string() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key.to_string(), TEST_PUBKEY_HEX);
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key.to_base64(), TEST_PUBKEY_B64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let key1 = VerifyingKey::from_base64(TEST_PUBKEY_B64).unwrap();
		assert_eq!(key1.key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = VerifyingKey::from_base64("").unwrap();
		assert_eq!(key2.key, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(VerifyingKey::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key.to_hex(), TEST_PUBKEY_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let key1 = VerifyingKey::from_hex(TEST_PUBKEY_HEX).unwrap();
		assert_eq!(key1.key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = VerifyingKey::from_hex("").unwrap();
		assert_eq!(key2.key, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(VerifyingKey::from_hex("invalid@@hex"));
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let key            = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let mut byte_clone = key.to_vec();
		
		//	Ensure the clone matches the original key's vec.
		assert_eq!(byte_clone, TEST_PUBKEY.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PUBKEY.to_vec());
		
		//	to_vec() doesn't consume or affect the original key.
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
}

#[cfg(test)]
mod verifying_key__traits {
	use super::*;
	
	//		as_ref																
	#[test]
	fn as_ref() {
		//	Same tests as for as_bytes().
		let key        = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let byte_slice = key.as_ref();
		assert_eq!(*byte_slice, TEST_PUBKEY);
		assert_eq!(key,         VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		clone																
	#[allow(clippy::clone_on_copy)]
	#[test]
	fn clone() {
		let key   = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let clone = key.clone();
		assert_eq!(clone, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		clone_from															
	#[test]
	fn clone_from() {
		let key       = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let mut clone = VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() };
		clone.clone_from(&key);
		assert_eq!(key,   VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		assert_eq!(clone, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		debug																
	#[test]
	fn debug() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(format!("{key:?}"), TEST_PUBKEY_HEX);
	}
	
	//		default																
	#[test]
	fn default() {
		let key = VerifyingKey::default();
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() });
	}
	
	//		deref																
	#[allow(clippy::explicit_deref_methods)]
	#[test]
	fn deref() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key.deref(), &RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(&*key,       &RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
	
	//		display																
	#[test]
	fn display() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(format!("{key}"), TEST_PUBKEY_HEX);
	}
	
	//		from																
	#[test]
	fn from__real_verifying_key() {
		let key = VerifyingKey::from(RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn from__real_verifying_key_ref() {
		let key = VerifyingKey::from(&RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn from__fixed_length_byte_array() {
		let key = VerifyingKey::from(TEST_PUBKEY);
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn from__fixed_length_byte_slice() {
		let key = VerifyingKey::from(&TEST_PUBKEY);
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	
	//		from_str															
	#[test]
	fn from_str() {
		assert_ok_eq!(VerifyingKey::from_str(TEST_PUBKEY_HEX), VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn from_str__err_decoding() {
		let err = VerifyingKey::from_str("invalid@@hex");
		assert_err_eq!(err, ByteSizedError::InvalidHexString);
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is not in valid hexadecimal format"));
	}
	#[test]
	fn from_str__err_too_long() {
		let err = VerifyingKey::from_str("010203040506070809101112131415161718192021222324252627282930313233");
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn from_str__err_too_short() {
		let err = VerifyingKey::from_str("01020304050607080910111213141516171819202122232425262728293031");
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key1 = VerifyingKey::force_from(&TEST_PUBKEY[..]);
		assert_eq!(key1, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		
		let key2 = VerifyingKey::force_from(&TEST_PUBKEY[..31]);
		assert_ne!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		assert_eq!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() });
	}
	#[test]
	fn force_from__vec_u8() {
		let key1 = VerifyingKey::force_from(TEST_PUBKEY.to_vec());
		assert_eq!(key1, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		
		let key2 = VerifyingKey::force_from(TEST_PUBKEY[..31].to_vec());
		assert_ne!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		assert_eq!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() });
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let key1 = VerifyingKey::force_from(&TEST_PUBKEY.to_vec());
		assert_eq!(key1, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		
		let key2 = VerifyingKey::force_from(&TEST_PUBKEY[..31].to_vec());
		assert_ne!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		assert_eq!(key2, VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() });
	}
	
	//		hash																
	#[test]
	fn hash() {
		let key1        = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let key2        = VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() };
		let mut hasher1 = DefaultHasher::new();
		let mut hasher2 = DefaultHasher::new();
		key1.hash(&mut hasher1);
		key2.hash(&mut hasher2);
		assert_ne!(hasher1.finish(), hasher2.finish());
	}
	
	//		partial_eq															
	#[test]
	fn partial_eq() {
		let key = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
		assert_ne!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap() });
	}
	
	//		serialize															
	#[test]
	fn serialize() {
		let key  = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		let json = json!(TEST_PUBKEY_HEX);
		assert_json_eq!(json!(key), json);
	}
	
	//		deserialize															
	#[test]
	fn deserialize() {
		let json = format!(r#""{TEST_PUBKEY_HEX}""#);
		let key  = VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() };
		assert_ok_eq!(serde_json::from_str::<VerifyingKey>(&json), key);
	}
	
	//		try_from															
	#[test]
	fn try_from__byte_slice() {
		let key = VerifyingKey::try_from(&TEST_PUBKEY[..]);
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__byte_slice__err_too_long() {
		let array: [u8; 33] = [0; 33];
		let err = VerifyingKey::try_from(&array[..]);
		assert_err_eq!(err, ByteSizedError::DataTooLong(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is longer than 32 bytes"));
	}
	#[test]
	fn try_from__byte_slice__err_too_short() {
		let err = VerifyingKey::try_from(&TEST_PUBKEY[..31]);
		assert_err_eq!(err, ByteSizedError::DataTooShort(32));
		assert_eq!(err.unwrap_err().to_string(), s!("The supplied data is shorter than 32 bytes"));
	}
	#[test]
	fn try_from__str() {
		let key = VerifyingKey::try_from("9fd7b9e728de47ab7d9d816e7057606dd302f38ddee64272e0ed933f0896bc8e");
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__str_ref() {
		let key = VerifyingKey::try_from(TEST_PUBKEY_HEX);
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__string() {
		let key = VerifyingKey::try_from(TEST_PUBKEY_HEX.to_owned());
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__string_ref() {
		let key = VerifyingKey::try_from(&TEST_PUBKEY_HEX.to_owned());
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__box_str() {
		let box_str = TEST_PUBKEY_HEX.to_owned().into_boxed_str();
		let key     = VerifyingKey::try_from(box_str);
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__cow_borrowed() {
		let cow: Cow<'_, str> = Cow::Borrowed(TEST_PUBKEY_HEX);
		let key               = VerifyingKey::try_from(cow);
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__cow_owned() {
		let cow: Cow<'_, str> = Cow::Owned(TEST_PUBKEY_HEX.to_owned());
		let key               = VerifyingKey::try_from(cow);
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__vec_u8() {
		let key = VerifyingKey::try_from(TEST_PUBKEY.to_vec());
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
	#[test]
	fn try_from__vec_u8_ref() {
		let key = VerifyingKey::try_from(&TEST_PUBKEY.to_vec());
		assert_ok_eq!(key, VerifyingKey { key: RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap() });
	}
}

//§		VerifyingKeyExt															
#[cfg(test)]
mod verifying_key_ext__bytesized {
	use super::*;
	
	//		as_bytes															
	#[test]
	fn as_bytes() {
		let key        = RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap();
		let byte_slice = ByteSized::as_bytes(&key);
		
		//	Ensure the byte slice matches the original key's bytes.
		assert_eq!(*byte_slice, TEST_PUBKEY);
		
		//	We can't modify the byte slice due to immutability.
		//	Uncommenting the line below would cause a compilation error:
		//byte_slice[10] = 84;
		
		//	as_bytes() doesn't consume the original key.
		assert_eq!(key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
	
	//		to_bytes															
	#[test]
	fn to_bytes() {
		let key            = RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap();
		let mut byte_clone = ByteSized::to_bytes(&key);
		
		//	Ensure the clone matches the original key's bytes.
		assert_eq!(byte_clone, TEST_PUBKEY);
		
		//	We can modify the cloned byte array.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PUBKEY);
		
		//	to_bytes() doesn't consume or affect the original key.
		assert_eq!(key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
	
	//		from_bytes															
	#[test]
	fn from_bytes() {
		let key = <RealVerifyingKey as ByteSized<32>>::from_bytes(TEST_PUBKEY);
		assert_eq!(key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let key = RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap();
		assert_eq!(key.to_base64(), TEST_PUBKEY_B64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let key1 = RealVerifyingKey::from_base64(TEST_PUBKEY_B64).unwrap();
		assert_eq!(key1, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = RealVerifyingKey::from_base64("").unwrap();
		assert_eq!(key2, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn from_base64__invalid() {
		assert_err!(RealVerifyingKey::from_base64("invalid@@base64"));
	}
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let key = RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap();
		assert_eq!(key.to_hex(), TEST_PUBKEY_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let key1 = RealVerifyingKey::from_hex(TEST_PUBKEY_HEX).unwrap();
		assert_eq!(key1, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = RealVerifyingKey::from_hex("").unwrap();
		assert_eq!(key2, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn from_hex__invalid() {
		assert_err!(RealVerifyingKey::from_hex("invalid@@hex"));
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		let key            = RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap();
		let mut byte_clone = key.to_vec();
		
		//	Ensure the clone matches the original key's vec.
		assert_eq!(byte_clone, TEST_PUBKEY.to_vec());
		
		//	We can modify the cloned byte vector.
		byte_clone[10]     = 84;
		assert_ne!(byte_clone, TEST_PUBKEY.to_vec());
		
		//	to_vec() doesn't consume or affect the original key.
		assert_eq!(key, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
	}
}

#[cfg(test)]
mod verifying_key_ext__traits {
	use super::*;
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key1 = RealVerifyingKey::force_from(&TEST_PUBKEY[..]);
		assert_eq!(key1, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = RealVerifyingKey::force_from(&TEST_PUBKEY[..31]);
		assert_ne!(key2, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(key2, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn force_from__vec_u8() {
		let key1 = RealVerifyingKey::force_from(TEST_PUBKEY.to_vec());
		assert_eq!(key1, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = RealVerifyingKey::force_from(TEST_PUBKEY[..31].to_vec());
		assert_ne!(key2, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(key2, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
	#[test]
	fn force_from__vec_u8_ref() {
		let key1 = RealVerifyingKey::force_from(&TEST_PUBKEY.to_vec());
		assert_eq!(key1, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		
		let key2 = RealVerifyingKey::force_from(&TEST_PUBKEY[..31].to_vec());
		assert_ne!(key2, RealVerifyingKey::from_bytes(&TEST_PUBKEY).unwrap());
		assert_eq!(key2, RealVerifyingKey::from_bytes(&EMPTY_256_HASH).unwrap());
	}
}


