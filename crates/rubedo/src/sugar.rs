//! This module provides macros that add syntactic sugar for common operations.
//! 
//! Note that some macros are implemented as proc macros, which can be found in
//! the [`rubedo-macros`](https://crates.io/crates/rubedo-macros) crate.



//		Modules

#[cfg(test)]
#[path = "tests/sugar.rs"]
mod tests;



//		Packages

pub use rubedo_macros::ip;



//		Macros

//		s!																		
/// Converts a [`str`] string literal to an owned [`String`].
/// 
/// This macro provides syntactic sugar to convert static [`str`] instances to 
/// [`String`]s - this saves having to do `"foo".to_owned()` or
/// `String::from("foo")`.
/// 
/// It will also convert any other type that implements the [`ToString`] trait
/// to a [`String`] - providing that it is passed in as a variable or some kind
/// of expression, and not as a literal. That's because, at present, there is no
/// way for the macro to tell the difference between [`str`] literals and other
/// literals such as numbers, and as a result it will call `to_owned()` on them
/// and give a non-[`String`] result, which will not match expectations. For
/// this reason, this ability is not as useful as the [`str`] behaviour, and it
/// does not provide a consistent interface for converting to [`String`]s (and
/// nor is it intended to).
/// 
/// When converting from other types it is likely best to use the standard
/// conversion functions directly, to avoid confusion, and so the recommendation
/// is to only use this macro as shorthand for converting [`str`] instances.
/// 
/// The inspiration for this macro comes from the [`velcro`](https://crates.io/crates/velcro)
/// crate, which provides a range of macros for creating collections, building
/// on the built-in [`vec!`] macro.
/// 
/// # Examples
/// 
/// ```
/// use rubedo::s;
/// 
/// assert_eq!(s!("foo"), "foo");
/// assert_eq!(s!("foo"), "foo".to_owned());
/// assert_eq!(s!("foo"), "foo".to_string());
/// assert_eq!(s!("foo"), String::from("foo"));
/// ```
/// 
/// # See also
/// 
/// * [`str`]
/// * [`String`]
/// * [`ToString`]
/// * [`vec!`]
/// * [`velcro`](https://crates.io/crates/velcro)
/// 
#[macro_export]
macro_rules! s {
	//	Convert a str string literal to a String
	($s:literal) => {
		$s.to_owned()
	};
	//	General expression
	($s:expr) => {
		$s.to_string()
	};
	//	Empty expression
	() => {
		String::new()
	};
}

#[allow(unused_imports)]
pub use s;


