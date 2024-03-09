#![allow(non_snake_case)]

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



//		Tests

//		SigningKey																
#[cfg(test)]
mod signing_key__bytesized {
	use super::*;
	use rubedo::crypto::SigningKey;
	use rubedo::std::ByteSized;
	
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
	use rubedo::std::ForceFrom;
	
	//		deref																
	#[test]
	fn deref__to_keypair_bytes() {
		let key = SigningKey::force_from(&TEST_256_HASH[..]);
		let mut pair = vec![];
		pair.extend_from_slice(&TEST_256_HASH);
		pair.extend_from_slice(&TEST_PUBKEY);
		assert_eq!(key.to_keypair_bytes(), &pair[..]);
	}
	
	//		force_from															
	#[test]
	fn force_from__byte_slice() {
		let key = SigningKey::force_from(&TEST_256_HASH[..]);
		assert_eq!(key.as_bytes(), &TEST_256_HASH);
		
		let key = SigningKey::force_from(&TEST_256_HASH[..31]);
		assert_ne!(key.as_bytes(), &TEST_256_HASH);
	}
}


