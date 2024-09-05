#![allow(non_snake_case)]
#![allow(unused_crate_dependencies)]

//		Packages

use chrono::{NaiveDate, Utc};
use rubedo::chrono::NaiveDateExt;



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


