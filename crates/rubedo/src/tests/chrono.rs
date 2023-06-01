#![allow(non_snake_case)]

//		Tests

//§		NaiveDateExt															
#[cfg(test)]
mod naivedate_ext {
	use super::super::*;
	
	//		today																
	#[test]
	fn today() {
		//	There is a very small possibility that this test will fail if the
		//	test is run at the exact moment that the date changes.
		let timestamp = Utc::now().timestamp();
		let datetime  = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
		let today     = NaiveDate::today();
		assert_eq!(today.year(),  datetime.year());
		assert_eq!(today.month(), datetime.month());
		assert_eq!(today.day(),   datetime.day());
	}
	
	//		days_in_month														
	#[test]
	fn days_in_month() {
		let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
		assert_eq!(date.days_in_month(), 31);
	}
	
	//		days_in_month_opt													
	#[test]
	fn days_in_month_opt__valid() {
		assert_eq!(NaiveDate::days_in_month_opt(2000, 1).unwrap(), 31);
	}
	#[test]
	fn days_in_month_opt__invalid() {
		assert!(NaiveDate::days_in_month_opt(2000, 13).is_none());
	}
	
	//		days_in_year														
	#[test]
	fn days_in_year__normal() {
		let date = NaiveDate::from_ymd_opt(2001, 1, 1).unwrap();
		assert_eq!(date.days_in_year(), 365);
	}
	#[test]
	fn days_in_year__leap() {
		let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
		assert_eq!(date.days_in_year(), 366);
	}
	
	//		days_in_year_opt													
	#[test]
	fn days_in_year_opt__normal() {
		assert_eq!(NaiveDate::days_in_year_opt(2001).unwrap(), 365);
	}
	#[test]
	fn days_in_year_opt__leap() {
		assert_eq!(NaiveDate::days_in_year_opt(2000).unwrap(), 366);
	}
	
	//		is_leap_year														
	#[test]
	fn is_leap_year() {
		let date = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
		assert_eq!(date.is_leap_year(), true);
		
		let date = NaiveDate::from_ymd_opt(2001, 1, 1).unwrap();
		assert_eq!(date.is_leap_year(), false);
	}
	
	//		is_leap_year_opt													
	#[test]
	fn is_leap_year_opt() {
		assert_eq!(NaiveDate::is_leap_year_opt(2000).unwrap(), true);
		assert_eq!(NaiveDate::is_leap_year_opt(2001).unwrap(), false);
	}
	
	//		start_of_month														
	#[test]
	fn start_of_month() {
		let date = NaiveDate::from_ymd_opt(2000, 1, 20).unwrap();
		assert_eq!(date.start_of_month(), NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
	}
	
	//		start_of_month_opt													
	#[test]
	fn start_of_month_opt__valid() {
		assert_eq!(NaiveDate::start_of_month_opt(2000, 1).unwrap(), NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
	}
	#[test]
	fn start_of_month_opt__invalid() {
		assert!(NaiveDate::start_of_month_opt(2000, 13).is_none());
	}
	
	//		end_of_month														
	#[test]
	fn end_of_month() {
		let date = NaiveDate::from_ymd_opt(2000, 1, 20).unwrap();
		assert_eq!(date.end_of_month(), NaiveDate::from_ymd_opt(2000, 1, 31).unwrap());
	}
	
	//		end_of_month_opt													
	#[test]
	fn end_of_month_opt__valid() {
		assert_eq!(NaiveDate::end_of_month_opt(2000, 1).unwrap(), NaiveDate::from_ymd_opt(2000, 1, 31).unwrap());
	}
	#[test]
	fn end_of_month_opt__invalid() {
		assert!(NaiveDate::end_of_month_opt(2000, 13).is_none());
	}
	
	//		start_of_year														
	#[test]
	fn start_of_year() {
		let date = NaiveDate::from_ymd_opt(2000, 2, 20).unwrap();
		assert_eq!(date.start_of_year(), NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
	}
	
	//		start_of_year_opt													
	#[test]
	fn start_of_year_opt() {
		assert_eq!(NaiveDate::start_of_year_opt(2000).unwrap(), NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
	}
	
	//		end_of_year														
	#[test]
	fn end_of_year() {
		let date = NaiveDate::from_ymd_opt(2000, 2, 20).unwrap();
		assert_eq!(date.end_of_year(), NaiveDate::from_ymd_opt(2000, 12, 31).unwrap());
	}
	
	//		end_of_year_opt													
	#[test]
	fn end_of_year_opt() {
		assert_eq!(NaiveDate::end_of_year_opt(2000).unwrap(), NaiveDate::from_ymd_opt(2000, 12, 31).unwrap());
	}
}


