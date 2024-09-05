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



//		Tests

//		ip!																		
#[cfg(test)]
mod ip {
	use rubedo_macros::*;
	use core::net::Ipv4Addr;
	
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


