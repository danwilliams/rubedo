//! The Rubedo crate is a library of useful functionality, some being extensions
//! of standard library entities; some extensions of other, popular crates; and
//! some being completely new functionality.
//! 
//! Note that some macros are implemented as proc macros, which can be found in
//! the [`rubedo-macros`](https://crates.io/crates/rubedo-macros) crate.



//		Global configuration

#![cfg_attr(feature = "reasons", feature(lint_reasons))]

//	Customisations of the standard linting configuration
#![cfg_attr(    feature = "reasons",  allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::multiple_crate_versions))]



//		Modules

pub mod chrono;
pub mod crypto;
pub mod http;
pub mod serde;
pub mod std;
pub mod sugar;


