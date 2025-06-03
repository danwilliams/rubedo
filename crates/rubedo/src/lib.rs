//! The Rubedo crate is a library of useful functionality, some being extensions
//! of standard library entities; some extensions of other, popular crates; and
//! some being completely new functionality.
//! 
//! Note that some macros are implemented as proc macros, which can be found in
//! the [`rubedo-macros`](https://crates.io/crates/rubedo-macros) crate.



//		Global configuration																							

//	Customisations of the standard linting configuration
#![allow(clippy::multiple_crate_versions, reason = "Cannot resolve all these")]
#![allow(clippy::items_after_test_module, reason = "Not needed with separated tests")]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::arithmetic_side_effects,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::too_many_lines,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in unit tests"
))]



//		Modules																											

#[cfg(feature = "chrono")]
pub mod chrono;
#[cfg(feature = "crypto")]
pub mod crypto;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "serde")]
pub mod serde;
#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "sugar")]
pub mod sugar;

/// List of crates used only in integration tests.
#[cfg(test)]
mod integration_tests {
	use rubedo as _;
}


