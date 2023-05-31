//		Modules

#[cfg(test)]
#[path = "tests/sugar.rs"]
mod tests;



//		Packages

#[allow(unused_imports)]
use std::net::IpAddr;



//		Macros

//		s!																		
/// Converts a `str` string literal to an owned `String`.
/// 
/// This macro provides syntactic sugar to convert static [`str`] instances to 
/// [`String`]s - this saves having to do `"foo".to_owned()` or
/// `String::from("foo")`.
/// 
/// It will also convert any other type that implements the [`ToString`] trait
/// to a `String`. This is perhaps not as useful as the `str` conversion, but it
/// does provide a consistent interface for converting to `String`s. When
/// converting from other types it is likely best to use the standard conversion
/// functions directly, to avoid confusion, and so the recommendation is to only
/// use this macro as shorthand for converting `str` instances.
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
/// # See Also
/// 
/// * [`str`]
/// * [`String`]
/// * [`ToString`]
/// * [`vec!`]
/// * [`velcro`](https://crates.io/crates/velcro)
/// 
#[macro_export]
macro_rules! s {
	($s:expr) => {
		$s.to_string()
	};
	() => {
		String::new()
	};
}

//		ip!																		
/// Builds an IP address from a range of input types.
/// 
/// This macro provides syntactic sugar to build an [`IpAddr`] from a range of
/// input types.
/// 
/// If left empty, it will default to `0.0.0.0`.
/// 
/// At present only IPv4 is supported, as this is the most common and the
/// clearest use of this shorthand syntax. IPv6 support might be added in the
/// future.
/// 
/// # Panics
/// 
/// Note that this macro will panic if it fails to parse a string literal as an
/// IP address. This is by design, so that all variants of IP creation return an
/// `IpAddr` instance, just like [`IpAddr::from()`]. This macro is intended to
/// be used with hard-coded addresses. If you need to handle errors, use the
/// standard [`IpAddr::from_str()`](IpAddr) method instead.
/// 
/// # Examples
/// 
/// ```
/// use rubedo::ip;
/// use std::net::IpAddr;
/// 
/// assert_eq!(ip!("1.2.3.4"), IpAddr::from([1, 2, 3, 4]));
/// assert_eq!(ip!(1, 2, 3, 4), IpAddr::from([1, 2, 3, 4]));
/// ```
/// 
/// # See Also
/// 
/// * [`IpAddr`]
/// 
#[macro_export]
macro_rules! ip {
	($s:expr) => {
		$s.parse::<IpAddr>().unwrap()
	};
	($a:expr, $b:expr, $c:expr, $d:expr) => {
		IpAddr::from([$a, $b, $c, $d])
	};
	() => {
		IpAddr::from([0, 0, 0, 0])
	};
}

#[allow(unused_imports)]
pub use ip;
pub use s;


