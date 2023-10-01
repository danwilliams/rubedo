#![allow(non_snake_case)]

//		Tests

//§		DurationExt																
#[cfg(test)]
mod duration_ext {
	use super::super::*;
	
	//		humanize															
	#[test]
	fn humanize() {
		let duration = Duration::seconds(0);
		assert_eq!(duration.humanize(), "0 seconds");
		
		let duration = Duration::seconds(1);
		assert_eq!(duration.humanize(), "1 second");
		
		let duration = Duration::seconds(2);
		assert_eq!(duration.humanize(), "2 seconds");
		
		let duration = Duration::seconds(59);
		assert_eq!(duration.humanize(), "59 seconds");
		
		let duration = Duration::seconds(60);
		assert_eq!(duration.humanize(), "1 minute");
		
		let duration = Duration::seconds(61);
		assert_eq!(duration.num_seconds(), 61);
		assert_eq!(duration.humanize(), "1 minute");
		
		let duration = Duration::seconds(130);
		assert_eq!(duration.humanize(), "2 minutes");
		
		let duration = Duration::seconds(3599);
		assert_eq!(duration.humanize(), "59 minutes");
		
		let duration = Duration::seconds(3600);
		assert_eq!(duration.humanize(), "1 hour");
		
		let duration = Duration::seconds(3601);
		assert_eq!(duration.humanize(), "1 hour");
		
		let duration = Duration::seconds(7200);
		assert_eq!(duration.humanize(), "2 hours");
		
		let duration = Duration::seconds(86399);
		assert_eq!(duration.humanize(), "23 hours");
		
		let duration = Duration::seconds(86400);
		assert_eq!(duration.humanize(), "1 day");
		
		let duration = Duration::seconds(86401);
		assert_eq!(duration.humanize(), "1 day");
		
		let duration = Duration::seconds(172800);
		assert_eq!(duration.humanize(), "2 days");
		
		let duration = Duration::seconds(604799);
		assert_eq!(duration.humanize(), "6 days");
		
		let duration = Duration::seconds(604800);
		assert_eq!(duration.humanize(), "1 week");
		
		let duration = Duration::seconds(604801);
		assert_eq!(duration.humanize(), "1 week");
		
		let duration = Duration::seconds(1209600);
		assert_eq!(duration.humanize(), "2 weeks");
		
		let duration = Duration::seconds(2591999);
		assert_eq!(duration.humanize(), "4 weeks");
		
		let duration = Duration::seconds(2592000);
		assert_eq!(duration.humanize(), "1 month");
		
		let duration = Duration::seconds(2592001);
		assert_eq!(duration.humanize(), "1 month");
		
		let duration = Duration::seconds(5184000);
		assert_eq!(duration.humanize(), "2 months");
		
		let duration = Duration::seconds(31535999);
		assert_eq!(duration.humanize(), "12 months");
		
		let duration = Duration::seconds(31536000);
		assert_eq!(duration.humanize(), "1 year");
		
		let duration = Duration::seconds(31536001);
		assert_eq!(duration.humanize(), "1 year");
		
		let duration = Duration::seconds(63072000);
		assert_eq!(duration.humanize(), "2 years");
		
		let duration = Duration::seconds(315360000);
		assert_eq!(duration.humanize(), "10 years");
		
		let duration = Duration::seconds(3153600000);
		assert_eq!(duration.humanize(), "100 years");
		
		let duration = Duration::seconds(31536000000);
		assert_eq!(duration.humanize(), "1000 years");
	}
}

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
		assert!(NaiveDate::end_of_month_opt(2000000, 1).is_none());
		assert!(NaiveDate::end_of_month_opt(2000000, 13).is_none());
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


