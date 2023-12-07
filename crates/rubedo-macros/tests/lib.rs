#![allow(non_snake_case)]

//		Tests

//		ip!																		
#[cfg(test)]
mod ip {
	use rubedo_macros::*;
	use std::net::Ipv4Addr;
	
	//		ip!																	
	#[test]
	fn ip__empty_1() {
		assert_eq!(ip!(), Ipv4Addr::new(0, 0, 0, 0));
	}
	#[test]
	fn ip__empty_2() {
		assert_eq!(ip!(""), Ipv4Addr::new(0, 0, 0, 0));
	}
	#[test]
	fn ip__str_valid() {
		assert_eq!(ip!("1.2.3.4"), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	fn ip__args_valid() {
		assert_eq!(ip!(1, 2, 3, 4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	fn ip__raw_valid_1() {
		assert_eq!(ip!(1.2.3.4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	fn ip__raw_valid_2() {
		assert_eq!(ip!(1 . 2 . 3 . 4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	fn ip__mixed_invalid() {
		//	We don't really want this, but it's a side-effect of the parser.
		assert_eq!(ip!("1.2".3.4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	fn compile_fail() {
		trybuild::TestCases::new().compile_fail("tests/compile_fail/lib.rs");
	}
}


