#![allow(non_snake_case)]
#![allow(unused_imports)]

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
	fn ip__args_valid() {
		assert_eq!(ip!(1, 2, 3, 4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	#[should_panic]
	fn ip__args_invalid_1() {
		ip!(1, 2, 3);
	}
	#[test]
	#[should_panic]
	fn ip__args_invalid_2() {
		ip!(1, 2, 3, 4, 5);
	}
	#[test]
	#[should_panic]
	fn ip__args_invalid_3() {
		ip!(1, 2, 3, 999);
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
	#[should_panic]
	fn ip__raw_invalid_1() {
		ip!(1.2.3);
	}
	#[test]
	#[should_panic]
	fn ip__raw_invalid_2() {
		ip!(1.2.3.4.5);
	}
	#[test]
	#[should_panic]
	fn ip__raw_invalid_3() {
		ip!(1.2.3.999);
	}
	#[test]
	fn ip__mixed_invalid() {
		//	We don't really want this, but it's a side-effect of the parser.
		assert_eq!(ip!("1.2".3.4), Ipv4Addr::new(1, 2, 3, 4));
	}
	#[test]
	#[should_panic]
	fn ip__mixed_invalid_1() {
		ip!(1.2,3.4);
	}
	#[test]
	#[should_panic]
	fn ip__mixed_invalid_2() {
		ip!(1,2.3.4);
	}
}


