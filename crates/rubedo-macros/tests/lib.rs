#![allow(non_snake_case)]

//		Tests

//		ip!																		
#[cfg(test)]
mod ip {
	use rubedo_macros::*;
	use std::net::{IpAddr, Ipv4Addr};
	
	//		ip!																	
	#[test]
	fn ip__empty() {
		assert_eq!(ip!(), Ipv4Addr::new(0, 0, 0, 0));
	}
	#[test]
	fn ip__str() {
		assert_eq!(ip!("1.2.3.4"), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	#[should_panic]
	fn ip__str_invalid_1() {
		ip!("1.2.3");
	}
	#[test]
	#[should_panic]
	fn ip__str_invalid_2() {
		ip!("1.2.3.4.5");
	}
	#[test]
	#[should_panic]
	fn ip__str_invalid_3() {
		ip!("1.2.3.999");
	}
	#[test]
	fn ip__args() {
		//	There are no tests for this style failing, as the compiler will
		//	enforce the correct number of arguments and that each is a valid u8.
		assert_eq!(ip!(1, 2, 3, 4), Ipv4Addr::new(1, 2, 3, 4));
	}
}


