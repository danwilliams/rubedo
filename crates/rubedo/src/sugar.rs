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
/// use rubedo::sugar::s;
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

pub use s;

//		variants!																
/// Allows shorthand for referring to multiple variants of the same enum.
/// 
/// This macro provides syntactic sugar to specify multiple enum variants using
/// reduced syntax, making such usage more concise, and easier to read.
/// 
/// It supports lists of variants separated by commas. It would be nice to also
/// use the boolean OR operator for use in matches, but this is not possible at
/// present, as match arms must be explicit, and cannot rely on expressions.
/// 
/// This macro returns the variants as a [`Vec`], and there is an alternative
/// macro, [`variants_hashset!`], which returns a [`HashSet`](std::collections::HashSet)
/// instead.
/// 
/// It is is exported as `vv!` (meaning "variants vector") as well as
/// `variants!`, to allow for more concise usage.
/// 
/// # Examples
/// 
/// ```
/// use rubedo::sugar::vv;
/// 
/// #[derive(Copy, Clone, Debug, Eq, PartialEq)]
/// enum Foo {
///     Bar,
///     Baz,
///     Qux,
/// }
/// 
/// assert_eq!(vv![Foo: Bar, Baz, Qux], vec![Foo::Bar, Foo::Baz, Foo::Qux]);
/// assert_eq!(vv![Foo: ], vec![]);
/// ```
/// 
/// # See also
/// 
/// * [`variants_hashset!`]
/// 
#[macro_export]
macro_rules! variants {
	//	Comma-separated list
	($enum:ident: $variant:ident $(, $other_variants:ident)*) => {
		vec![$enum::$variant $(, $enum::$other_variants )*]
	};
	//	Comma-separated list with trailing comma
	($enum:ident: $variant:ident $(, $other_variants:ident)*,) => {
		vec![$enum::$variant $(, $enum::$other_variants )*]
	};
	//	Empty list
	($enum:ident:) => {
		Vec::<$enum>::new()
	};
	//	Empty expression
	() => {
		vec![]
	};
}

pub use variants;
pub use variants as vv;

//		variants_hashset!														
/// Allows shorthand for referring to multiple variants of the same enum.
/// 
/// This macro is the same as [`variants!`], but returns a [`HashSet`](std::collections::HashSet)
/// instead of a [`Vec`]. For more information, see the documentation for
/// [`variants!`].
/// 
/// It is is exported as `vh!` (meaning "variants hashset") as well as
/// `variants_hashset!`, to allow for more concise usage.
/// 
/// # Examples
/// 
/// ```
/// use rubedo::sugar::vh;
/// use std::collections::HashSet;
/// 
/// #[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
/// enum Foo {
///     Bar,
///     Baz,
///     Qux,
/// }
/// 
/// assert_eq!(vh![Foo: Bar, Baz, Qux], HashSet::from([Foo::Bar, Foo::Baz, Foo::Qux]));
/// assert_eq!(vh![Foo: ], HashSet::from([]));
/// ```
/// 
/// # See also
/// 
/// * [`variants!`]
/// 
#[macro_export]
macro_rules! variants_hashset {
	//	Comma-separated list
	($enum:ident: $variant:ident $(, $other_variants:ident)*) => {
		std::collections::HashSet::from([$enum::$variant $(, $enum::$other_variants )*])
	};
	//	Comma-separated list with trailing comma
	($enum:ident: $variant:ident $(, $other_variants:ident)*,) => {
		std::collections::HashSet::from([$enum::$variant $(, $enum::$other_variants )*])
	};
	//	Empty list
	($enum:ident:) => {
		std::collections::HashSet::<$enum>::new()
	};
	//	Empty expression
	() => {
		std::collections::HashSet::new()
	};
}

pub use variants_hashset;
pub use variants_hashset as vh;


