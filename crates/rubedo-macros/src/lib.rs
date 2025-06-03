//! This library provides macros that add syntactic sugar for common operations.
//! 
//! Specifically, the macros implemented in this crate are proc macros, which
//! have to live separately from other code. They are part of the [`rubedo`](https://crates.io/crates/rubedo)
//! ecosystem.



//		Global configuration																							

//	Customisations of the standard linting configuration
#![allow(clippy::absolute_paths,          reason = "Needed in the re-exported docs")]
#![allow(clippy::expect_used,             reason = "Okay in a proc macro")]
#![allow(clippy::items_after_test_module, reason = "Not needed with separated tests")]
#![allow(clippy::panic,                   reason = "Also okay in a proc macro")]



//		Modules																											

/// List of crates used only in integration tests.
#[cfg(test)]
mod integration_tests {
	use trybuild as _;
}



//		Packages																										

use proc_macro::{TokenStream, TokenTree};
use quote::quote;



//		Macros																											

//		ip!																		
/// Builds an IP address from a range of input types.
/// 
/// This macro provides syntactic sugar to build an [`IpAddr`](core::net::IpAddr)
/// from a range of input types.
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
/// [`IpAddr`](core::net::IpAddr) instance, just like [`IpAddr::from()`](core::net::IpAddr::from()).
/// This macro is intended to be used with hard-coded addresses. If you need to
/// handle errors, use the standard [`IpAddr::from_str()`](core::str::FromStr::from_str())
/// method instead.
/// 
/// # Examples
/// 
/// ```
/// use rubedo_macros::ip;
/// use core::net::IpAddr;
/// 
/// assert_eq!(ip!(1.2.3.4), IpAddr::from([1, 2, 3, 4]));
/// assert_eq!(ip!("1.2.3.4"), IpAddr::from([1, 2, 3, 4]));
/// assert_eq!(ip!(1, 2, 3, 4), IpAddr::from([1, 2, 3, 4]));
/// ```
/// 
/// # See also
/// 
/// * [`IpAddr`](core::net::IpAddr)
/// 
#[proc_macro]
pub fn ip(input: TokenStream) -> TokenStream {
	//	We should not use things from proc_macro::TokenStream too deeply. Here's
	//	the reason why:
	//	
	//	https://fasterthanli.me/articles/proc-macro-support-in-rust-analyzer-for-nightly-rustc-versions
	//	
	//	Not only does attempting to use proc_macro::bridge give an error saying
	//	"use of unstable library feature 'proc_macro_internals'", but also even
	//	if flags were added to access the internals, it seems they are subject
	//	to change without warning. This means that we cannot access things like
	//	LitKind (the kind property of a TokenStream token of type Literal, e.g.
	//	Literal.kind = LitKind::Float) and therefore have to rely upon the other
	//	properties and do some rudimentary and naive type detection.
	//	
	//	https://github.com/rust-lang/rust/blob/master/library/proc_macro/src/bridge/mod.rs
	//	
	//	Unfortunately, this means that as we cannot look at the Literal.kind
	//	property to get an authoritative statement about the type, the guesswork
	//	process we are forced to use means we cannot tell the difference between
	//	code that is written as 1.2.3.4 versus "1.2"."3.4". Both cases will lead
	//	to a literal, followed by punctuation, followed by another literal. The
	//	first one will be two Literals of kind Float, and the second will be two
	//	Literals of type Str. We would ideally like to consider as technically
	//	invalid, but are bound to accept it due to this lack of differentiation
	//	ability. Hopefully this will change in future.
	//	
	//	Other than that, it is not advantageous to use proc_macro2, as it does
	//	not provide anything we cannot already access or do, and using syn is a
	//	problem for two reasons: firstly and most importantly that it enforces
	//	Rust grammar and is not merely a method of accessing tokens, meaning
	//	that it will complain about our custom syntax; and secondly, it is quite
	//	a heavy package and if we don't need it then it's nice to avoid the
	//	compilation overhead and increased binary size. (Note: It would probably
	//	be possible to use peek() in some way to adjust the grammar checking,
	//	and indeed it looks like the grammar checking could potentially be
	//	controlled or circumvented by defining a custom struct to use in the
	//	parse_macro_input!(input as T) call, but this seems like too much to do
	//	for something so simple, and on balance an undesirable approach.)
	let mut str = String::new();
	for (count, token) in input.into_iter().enumerate() {
		//	If there's only one token, it should be a string literal to be valid. We
		//	cannot confirm what kind of Literal it is, but we can check that it's a
		//	Literal and then try to parse it into an IpAddr.
		//	
		//	If there are three tokens, this should correspond to 1.2.3.4 syntax.
		//	
		//	If there are seven tokens, this should correspond to 1, 2, 3, 4 syntax.
		if count == 0 || count == 2 || count == 4 || count == 6 {
			match token {
				TokenTree::Literal(lit) => str.push_str(&lit.to_string().replace('"', "")),
				TokenTree::Group(_)     |
				TokenTree::Ident(_)     |
				TokenTree::Punct(_)     => panic!("Invalid IP address"),
			}
		} else if count == 1 || count == 3 || count == 5 {
			match token {
				TokenTree::Punct(punct) => {
					match punct.to_string().chars().next().expect("Invalid IP address") {
						'.'             => str.push('.'),
						','             => str.push(','),
						_               => panic!("Invalid IP address")
					}
				},
				TokenTree::Group(_)   |
				TokenTree::Ident(_)   |
				TokenTree::Literal(_) => panic!("Invalid IP address"),
			}
		} else {
			panic!("Invalid IP address")
		}
	}
	if str.is_empty() {
		return quote! {
			core::net::IpAddr::from([0, 0, 0, 0])
		}.into();
	}
	//	We should not have more than 3 dots, more than 3 commas, or a mixture of
	//	dots and commas.
	let count_dots   = str.matches('.').count();
	let count_commas = str.matches(',').count();
	assert!((count_dots == 3 && count_commas == 0) || (count_dots == 0 && count_commas == 3), "Invalid IP address");
	let ip_addr      = str.replace(',', ".").parse::<core::net::IpAddr>().expect("Invalid IP address").to_string();
	quote! {
		#ip_addr.parse::<core::net::IpAddr>().unwrap()
	}.into()
}


