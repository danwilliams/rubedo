//! Integration tests for the `crypto` module.

#![cfg(feature = "crypto")]
#![allow(unused_crate_dependencies, reason = "Creates a lot of noise")]

//	Lints specifically disabled for integration tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::arithmetic_side_effects,
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
	clippy::too_many_lines,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in integration tests"
))]



//		Constants

const TEST_256_HASH:   [u8; 32] = [
	0xbe, 0xef, 0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x7a, 0x8b, 0x9c, 0x0d, 0x1e, 0x2f, 0x3a, 0x4b,
	0x5c, 0x6d, 0x7e, 0x8f, 0x9a, 0x0b, 0x1c, 0x2d, 0x3e, 0x4f, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0x0f,
];
const TEST_PUBKEY:     [u8; 32] = [
	0x9f, 0xd7, 0xb9, 0xe7, 0x28, 0xde, 0x47, 0xab, 0x7d, 0x9d, 0x81, 0x6e, 0x70, 0x57, 0x60, 0x6d,
	0xd3, 0x02, 0xf3, 0x8d, 0xde, 0xe6, 0x42, 0x72, 0xe0, 0xed, 0x93, 0x3f, 0x08, 0x96, 0xbc, 0x8e,
];
const TEST_256_HEX:    &str     = "beef1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f";
const TEST_256_BASE64: &str     = "vu8aKzxNXm96i5wNHi86S1xtfo+aCxwtPk9aa3yNng8=";



//		Tests

//		SigningKey																
#[cfg(test)]
mod signing_key__bytesized {
	use super::*;
	use rubedo::crypto::SigningKey;
	use rubedo::std::ByteSized as _;
	
	//		to_hex																
	#[test]
	fn to_hex() {
		let key = SigningKey::from(TEST_256_HASH);
		assert_eq!(key.to_hex(), TEST_256_HEX);
	}
	
	//		from_hex															
	#[test]
	fn from_hex__valid() {
		let key = SigningKey::from_hex(TEST_256_HEX).unwrap();
		assert_eq!(key.as_bytes(), &TEST_256_HASH);
	}
}

#[cfg(test)]
mod signing_key__traits {
	use super::*;
	use rubedo::crypto::SigningKey;
	use rubedo::std::ForceFrom as _;
	
	//		deref																
	#[test]
	fn deref__to_keypair_bytes() {
		let key = SigningKey::force_from(&TEST_256_HASH[..]);
		let mut pair = vec![];
		pair.extend_from_slice(&TEST_256_HASH);
		pair.extend_from_slice(&TEST_PUBKEY);
		assert_eq!(key.to_keypair_bytes(), &*pair);
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key1 = SigningKey::force_from(&TEST_256_HASH[..]);
		assert_eq!(key1.as_bytes(), &TEST_256_HASH);
		
		let key2 = SigningKey::force_from(&TEST_256_HASH[..31]);
		assert_ne!(key2.as_bytes(), &TEST_256_HASH);
	}
}

//§		SigningKeyExt															
#[cfg(test)]
mod signing_key_ext {
	use super::*;
	use ed25519_dalek::SigningKey;
	use rubedo::std::ByteSized as _;
	
	//		to_base64															
	#[test]
	fn to_base64() {
		let key = SigningKey::from_bytes(&TEST_256_HASH);
		assert_eq!(key.to_base64(), TEST_256_BASE64);
	}
	
	//		from_base64															
	#[test]
	fn from_base64__valid() {
		let key = SigningKey::from_base64(TEST_256_BASE64).unwrap();
		assert_eq!(key, SigningKey::from_bytes(&TEST_256_HASH));
	}
}


