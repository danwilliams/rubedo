#![allow(non_snake_case)]

//		Packages

use super::*;
use std::collections::HashSet;



//		Tests

//		s!																		
#[cfg(test)]
mod s {
	use super::*;
	
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

//		variants!																
#[cfg(test)]
mod variants {
	use super::*;
	
	//		Position															
	#[derive(Copy, Clone, Debug, Eq, PartialEq)]
	#[repr(u8)]
	enum Position {
		Zero = 0,
		One  = 1,
		Two  = 2,
	}
	
	//		variants!															
	#[test]
	#[cfg_attr(    feature = "reasons",  allow(trivial_casts,
		reason = "Trivial cast here is needed for the test"
	))]
	#[cfg_attr(not(feature = "reasons"), allow(trivial_casts))]
	fn variants__empty() {
		assert_eq!(variants![] as Vec<Position>, vec![]);
		assert_eq!(variants!() as Vec<Position>, vec![]);
		assert_eq!(variants!{} as Vec<Position>, vec![]);
		assert_eq!(variants![ Position: ], vec![]);
		assert_eq!(variants!( Position: ), vec![]);
		assert_eq!(variants!{ Position: }, vec![]);
	}
	#[test]
	fn variants__single() {
		assert_eq!(variants![ Position: One  ], vec![ Position::One ]);
		assert_eq!(variants![ Position: One, ], vec![ Position::One ]);
		assert_eq!(variants!( Position: One  ), vec![ Position::One ]);
		assert_eq!(variants!{ Position: One, }, vec![ Position::One ]);
	}
	#[test]
	fn variants__multiple() {
		assert_eq!(variants![ Position: Zero, One      ], vec![ Position::Zero, Position::One ]);
		assert_eq!(variants![ Position: Zero, One,     ], vec![ Position::Zero, Position::One ]);
		assert_eq!(variants![ Position: Zero, One, Two ], vec![ Position::Zero, Position::One, Position::Two ]);
		assert_eq!(variants!( Position: Zero, One, Two ), vec![ Position::Zero, Position::One, Position::Two ]);
		assert_eq!(variants!{ Position: Zero, One, Two }, vec![ Position::Zero, Position::One, Position::Two ]);
	}
}

//		variants_hashset!														
#[cfg(test)]
mod variants_hashset {
	use super::*;
	
	//		Position															
	#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
	#[repr(u8)]
	enum Position {
		Zero = 0,
		One  = 1,
		Two  = 2,
	}
	
	//		variants_hashset!													
	#[test]
	#[cfg_attr(    feature = "reasons",  allow(trivial_casts,
		reason = "Trivial cast here is needed for the test"
	))]
	#[cfg_attr(not(feature = "reasons"), allow(trivial_casts))]
	fn variants__empty() {
		assert_eq!(variants_hashset![] as HashSet<Position>, HashSet::new());
		assert_eq!(variants_hashset!() as HashSet<Position>, HashSet::new());
		assert_eq!(variants_hashset!{} as HashSet<Position>, HashSet::new());
		assert_eq!(variants_hashset![ Position: ], HashSet::new());
		assert_eq!(variants_hashset!( Position: ), HashSet::new());
		assert_eq!(variants_hashset!{ Position: }, HashSet::new());
	}
	#[test]
	fn variants__single() {
		assert_eq!(variants_hashset![ Position: One  ], HashSet::from([ Position::One ]));
		assert_eq!(variants_hashset![ Position: One, ], HashSet::from([ Position::One ]));
		assert_eq!(variants_hashset!( Position: One  ), HashSet::from([ Position::One ]));
		assert_eq!(variants_hashset!{ Position: One, }, HashSet::from([ Position::One ]));
	}
	#[test]
	fn variants__multiple() {
		assert_eq!(variants_hashset![ Position: Zero, One      ], HashSet::from([ Position::Zero, Position::One ]));
		assert_eq!(variants_hashset![ Position: Zero, One,     ], HashSet::from([ Position::Zero, Position::One ]));
		assert_eq!(variants_hashset![ Position: Zero, One, Two ], HashSet::from([ Position::Zero, Position::One, Position::Two ]));
		assert_eq!(variants_hashset!( Position: Zero, One, Two ), HashSet::from([ Position::Zero, Position::One, Position::Two ]));
		assert_eq!(variants_hashset!{ Position: Zero, One, Two }, HashSet::from([ Position::Zero, Position::One, Position::Two ]));
	}
}


