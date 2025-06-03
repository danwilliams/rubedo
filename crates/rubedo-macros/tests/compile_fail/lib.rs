#![allow(non_snake_case)]
#![allow(unused_imports)]

//		Tests																											

fn main() {
	use rubedo_macros::*;
	use std::net::IpAddr;
	
	//		ip!																	
	//	These tests are here as it is is not possible to test a proc_macro in
	//	the same crate as it is defined.
	fn ip__str_invalid_1() {
		ip!("1.2.3");
	}
	fn ip__str_invalid_2() {
		ip!("1.2.3.4.5");
	}
	fn ip__str_invalid_3() {
		ip!("1.2.3.999");
	}
	fn ip__args_invalid_1() {
		ip!(1, 2, 3);
	}
	fn ip__args_invalid_2() {
		ip!(1, 2, 3, 4, 5);
	}
	fn ip__args_invalid_3() {
		ip!(1, 2, 3, 999);
	}
	fn ip__raw_invalid_1() {
		ip!(1.2.3);
	}
	fn ip__raw_invalid_2() {
		ip!(1.2.3.4.5);
	}
	fn ip__raw_invalid_3() {
		ip!(1.2.3.999);
	}
	fn ip__mixed_invalid_1() {
		ip!(1.2,3.4);
	}
	fn ip__mixed_invalid_2() {
		ip!(1,2.3.4);
	}
}


