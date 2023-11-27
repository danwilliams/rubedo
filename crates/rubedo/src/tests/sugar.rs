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
		//	This tests that although the macro is intended for use with str slice
		//	literals, it will also work with other types providing they have a
		//	to_owned() function - the outcome will not be a String, which will not
		//	match expectations, but it will still work. Really, the macro should not
		//	be used in this way, as it is only designed for use with str literals.
		assert_eq!(s!(42), 42);
	}
	#[test]
	fn s__types() {
		assert_ne!(type_of(&"foo"),     type_of(&String::from("foo")));
		assert_eq!(type_of(&s!("foo")), type_of(&String::from("foo")));
	}
}


