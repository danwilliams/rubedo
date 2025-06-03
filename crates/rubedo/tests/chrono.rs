//! Integration tests for the `chrono` module.

#![cfg(feature = "chrono")]
#![allow(unused_crate_dependencies, reason = "Creates a lot of noise")]

//	Lints specifically disabled for integration tests
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
	clippy::tests_outside_test_module,
	clippy::too_many_lines,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in integration tests"
))]



//		Packages

use chrono::{NaiveDate, Utc};
use rubedo::chrono::NaiveDateExt as _;



//		Tests

//		NaiveDateExt															
#[test]
fn naive_date_ext__today() {
	//	There is an extremely small chance that this test will fail if it is run
	//	just before midnight, but it is unlikely to be a problem.
	let today = NaiveDate::today();
	assert_eq!(today, Utc::now().date_naive());
}
#[test]
fn naive_date_ext__days_in_month() {
	let date = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
	assert_eq!(date.days_in_month(), 31);
}


