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
#![cfg_attr(    feature = "reasons",  allow(clippy::items_after_test_module, reason = "Not needed with separated tests"))]
#![cfg_attr(not(feature = "reasons"), allow(clippy::items_after_test_module))]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
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
	clippy::unwrap_in_result,
	clippy::unwrap_used,
))]



//		Modules

pub mod chrono;
pub mod crypto;
pub mod http;
pub mod serde;
pub mod std;
pub mod sugar;


