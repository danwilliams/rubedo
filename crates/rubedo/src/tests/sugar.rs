#![allow(non_snake_case)]

//		Tests

//		s!																		
#[cfg(test)]
mod s {
	use super::super::*;
	
	//		type_of																
	fn type_of<T>(_: &T) -> &'static str {
		std::any::type_name::<T>()
	}
	
	//		s!																	
	#[test]
	fn s__empty() {
		assert_eq!(s!(), "");
		assert_eq!(s!(), String::new());
	}
	#[test]
	fn s__str() {
		assert_eq!(s!("foo"), "foo");
		assert_eq!(s!("foo"), String::from("foo"));
	}
	#[test]
	fn s__string() {
		assert_eq!(s!(String::from("foo")), "foo");
		assert_eq!(s!(String::from("foo")), String::from("foo"));
	}
	#[test]
	fn s__int() {
		assert_eq!(s!(42), "42");
		assert_eq!(s!(42), String::from("42"));
	}
	#[test]
	fn s__types() {
		assert_ne!(type_of(&"foo"),     type_of(&String::from("foo")));
		assert_eq!(type_of(&s!("foo")), type_of(&String::from("foo")));
	}
}

//		ip!																		
#[cfg(test)]
mod ip {
	use super::super::*;
	use std::net::Ipv4Addr;
	
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


