//		Packages

#[allow(unused_imports)]
use std::net::IpAddr;



//		Macros

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
/// ```ignore
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


