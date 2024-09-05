//		Packages

use super::*;
use chrono::{DateTime, TimeZone};
use claims::{assert_none, assert_some_eq};



//		Tests

//§		DurationExt																
#[cfg(test)]
mod duration_ext {
	use super::*;
	
	//		MAX_NANOSECONDS														
	#[test]
	fn max_nanoseconds__max_allowed() {
		assert_eq!(Duration::nanoseconds(Duration::MAX_NANOSECONDS).num_nanoseconds(), Some(Duration::MAX_NANOSECONDS));
	}
	#[test]
	fn max_nanoseconds__overflow() {
		let duration = Duration::nanoseconds(Duration::MAX_NANOSECONDS) + Duration::nanoseconds(1);
		assert!(duration.num_nanoseconds().is_none());
	}
	
	//		MAX_NANOSECONDS_FULL												
	#[test]
	fn max_nanoseconds_full__max_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().num_nanoseconds_full(), Duration::MAX_NANOSECONDS_FULL);
	}
	#[test]
	fn max_nanoseconds_full__overflow() {
		assert!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().checked_add(&Duration::nanoseconds(1)).is_none());
	}
	
	//		MAX_MICROSECONDS													
	#[test]
	fn max_microseconds__max_allowed() {
		assert_eq!(Duration::microseconds(Duration::MAX_MICROSECONDS).num_microseconds(), Some(Duration::MAX_MICROSECONDS));
	}
	#[test]
	fn max_microseconds__overflow() {
		let duration = Duration::microseconds(Duration::MAX_MICROSECONDS) + Duration::microseconds(1);
		assert!(duration.num_microseconds().is_none());
	}
	
	//		MAX_MICROSECONDS_FULL												
	#[test]
	fn max_microseconds_full__max_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().num_microseconds_full(), Duration::MAX_MICROSECONDS_FULL);
	}
	#[test]
	fn max_microseconds_full__overflow() {
		assert!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().checked_add(&Duration::microseconds(1)).is_none());
	}
	
	//		MAX_MILLISECONDS													
	#[test]
	fn max_milliseconds__max_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().num_milliseconds(), Duration::MAX_MILLISECONDS);
	}
	#[test]
	fn max_milliseconds__overflow_addition() {
		assert!(Duration::try_milliseconds(Duration::MAX_MILLISECONDS).unwrap().checked_add(&Duration::try_milliseconds(1).unwrap()).is_none());
	}
	
	//		MAX_SECONDS															
	#[test]
	fn max_seconds__max_allowed() {
		assert_eq!(Duration::try_seconds(Duration::MAX_SECONDS).unwrap().num_seconds(), Duration::MAX_SECONDS);
	}
	#[test]
	fn max_seconds__overflow_construction() {
		assert_none!(Duration::try_seconds(Duration::MAX_SECONDS + 1));
	}
	#[test]
	fn max_seconds__overflow_addition() {
		assert!(Duration::try_seconds(Duration::MAX_SECONDS).unwrap().checked_add(&Duration::try_seconds(1).unwrap()).is_none());
	}
	
	//		MAX_MINUTES															
	#[test]
	fn max_minutes__max_allowed() {
		assert_eq!(Duration::try_minutes(Duration::MAX_MINUTES).unwrap().num_minutes(), Duration::MAX_MINUTES);
	}
	#[test]
	fn max_minutes__overflow_construction() {
		assert_none!(Duration::try_minutes(Duration::MAX_MINUTES + 1));
	}
	#[test]
	fn max_minutes__overflow_addition() {
		assert!(Duration::try_minutes(Duration::MAX_MINUTES).unwrap().checked_add(&Duration::try_minutes(1).unwrap()).is_none());
	}
	
	//		MAX_HOURS															
	#[test]
	fn max_hours__max_allowed() {
		assert_eq!(Duration::try_hours(Duration::MAX_HOURS).unwrap().num_hours(), Duration::MAX_HOURS);
	}
	#[test]
	fn max_hours__overflow_construction() {
		assert_none!(Duration::try_hours(Duration::MAX_HOURS + 1));
	}
	#[test]
	fn max_hours__overflow_addition() {
		assert!(Duration::try_hours(Duration::MAX_HOURS).unwrap().checked_add(&Duration::try_hours(1).unwrap()).is_none());
	}
	
	//		MAX_DAYS															
	#[test]
	fn max_days__max_allowed() {
		assert_eq!(Duration::try_days(Duration::MAX_DAYS).unwrap().num_days(), Duration::MAX_DAYS);
	}
	#[test]
	fn max_days__overflow_construction() {
		assert_none!(Duration::try_days(Duration::MAX_DAYS + 1));
	}
	#[test]
	fn max_days__overflow_addition() {
		assert!(Duration::try_days(Duration::MAX_DAYS).unwrap().checked_add(&Duration::try_days(1).unwrap()).is_none());
	}
	
	//		MAX_WEEKS															
	#[test]
	fn max_weeks__max_allowed() {
		assert_eq!(Duration::try_weeks(Duration::MAX_WEEKS).unwrap().num_weeks(), Duration::MAX_WEEKS);
	}
	#[test]
	fn max_weeks__overflow_construction() {
		assert_none!(Duration::try_weeks(Duration::MAX_WEEKS + 1));
	}
	#[test]
	fn max_weeks__overflow_addition() {
		assert!(Duration::try_weeks(Duration::MAX_WEEKS).unwrap().checked_add(&Duration::try_weeks(1).unwrap()).is_none());
	}
	
	//		MIN_NANOSECONDS														
	#[test]
	fn min_nanoseconds__min_allowed() {
		assert_eq!(Duration::nanoseconds(Duration::MIN_NANOSECONDS).num_nanoseconds(), Some(Duration::MIN_NANOSECONDS));
	}
	#[test]
	fn min_nanoseconds__overflow() {
		let duration = Duration::nanoseconds(Duration::MIN_NANOSECONDS) - Duration::nanoseconds(1);
		assert!(duration.num_nanoseconds().is_none());
	}
	
	//		MIN_NANOSECONDS_FULL												
	#[test]
	fn min_nanoseconds_full__min_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().num_nanoseconds_full(), Duration::MIN_NANOSECONDS_FULL);
	}
	#[test]
	fn min_nanoseconds_full__overflow() {
		assert!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().checked_sub(&Duration::nanoseconds(1)).is_none());
	}
	
	//		MIN_MICROSECONDS													
	#[test]
	fn min_microseconds__min_allowed() {
		assert_eq!(Duration::microseconds(Duration::MIN_MICROSECONDS).num_microseconds(), Some(Duration::MIN_MICROSECONDS));
	}
	#[test]
	fn min_microseconds__overflow() {
		let duration = Duration::microseconds(Duration::MIN_MICROSECONDS) - Duration::microseconds(1);
		assert!(duration.num_microseconds().is_none());
	}
	
	//		MIN_MICROSECONDS_FULL												
	#[test]
	fn min_microseconds_full__min_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().num_microseconds_full(), Duration::MIN_MICROSECONDS_FULL);
	}
	#[test]
	fn min_microseconds_full__overflow() {
		assert!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().checked_sub(&Duration::microseconds(1)).is_none());
	}
	
	//		MIN_MILLISECONDS													
	#[test]
	fn min_milliseconds__min_allowed() {
		assert_eq!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().num_milliseconds(), Duration::MIN_MILLISECONDS);
	}
	#[test]
	fn min_milliseconds__overflow_addition() {
		assert!(Duration::try_milliseconds(Duration::MIN_MILLISECONDS).unwrap().checked_sub(&Duration::try_milliseconds(1).unwrap()).is_none());
	}
	
	//		MIN_SECONDS															
	#[test]
	fn min_seconds__min_allowed() {
		assert_eq!(Duration::try_seconds(Duration::MIN_SECONDS).unwrap().num_seconds(), Duration::MIN_SECONDS);
	}
	#[test]
	fn min_seconds__overflow_construction() {
		assert_none!(Duration::try_seconds(Duration::MIN_SECONDS - 1));
	}
	#[test]
	fn min_seconds__overflow_addition() {
		assert!(Duration::try_seconds(Duration::MIN_SECONDS).unwrap().checked_sub(&Duration::try_seconds(1).unwrap()).is_none());
	}
	
	//		MIN_MINUTES															
	#[test]
	fn min_minutes__min_allowed() {
		assert_eq!(Duration::try_minutes(Duration::MIN_MINUTES).unwrap().num_minutes(), Duration::MIN_MINUTES);
	}
	#[test]
	fn min_minutes__overflow_construction() {
		assert_none!(Duration::try_minutes(Duration::MIN_MINUTES - 1));
	}
	#[test]
	fn min_minutes__overflow_addition() {
		assert!(Duration::try_minutes(Duration::MIN_MINUTES).unwrap().checked_sub(&Duration::try_minutes(1).unwrap()).is_none());
	}
	
	//		MIN_HOURS															
	#[test]
	fn min_hours__min_allowed() {
		assert_eq!(Duration::try_hours(Duration::MIN_HOURS).unwrap().num_hours(), Duration::MIN_HOURS);
	}
	#[test]
	fn min_hours__overflow_construction() {
		assert_none!(Duration::try_hours(Duration::MIN_HOURS - 1));
	}
	#[test]
	fn min_hours__overflow_addition() {
		assert!(Duration::try_hours(Duration::MIN_HOURS).unwrap().checked_sub(&Duration::try_hours(1).unwrap()).is_none());
	}
	
	//		MIN_DAYS															
	#[test]
	fn min_days__min_allowed() {
		assert_eq!(Duration::try_days(Duration::MIN_DAYS).unwrap().num_days(), Duration::MIN_DAYS);
	}
	#[test]
	fn min_days__overflow_construction() {
		assert_none!(Duration::try_days(Duration::MIN_DAYS - 1));
	}
	#[test]
	fn min_days__overflow_addition() {
		assert!(Duration::try_days(Duration::MIN_DAYS).unwrap().checked_sub(&Duration::try_days(1).unwrap()).is_none());
	}
	
	//		MIN_WEEKS															
	#[test]
	fn min_weeks__min_allowed() {
		assert_eq!(Duration::try_weeks(Duration::MIN_WEEKS).unwrap().num_weeks(), Duration::MIN_WEEKS);
	}
	#[test]
	fn min_weeks__overflow_construction() {
		assert_none!(Duration::try_weeks(Duration::MIN_WEEKS - 1));
	}
	#[test]
	fn min_weeks__overflow_addition() {
		assert!(Duration::try_weeks(Duration::MIN_WEEKS).unwrap().checked_sub(&Duration::try_weeks(1).unwrap()).is_none());
	}
	
	//		humanize															
	#[test]
	fn humanize() {
		assert_eq!(Duration::try_seconds(             0).unwrap().humanize(), "0 seconds");
		assert_eq!(Duration::try_seconds(             1).unwrap().humanize(), "1 second");
		assert_eq!(Duration::try_seconds(             2).unwrap().humanize(), "2 seconds");
		assert_eq!(Duration::try_seconds(            59).unwrap().humanize(), "59 seconds");
		assert_eq!(Duration::try_seconds(            60).unwrap().humanize(), "1 minute");
		assert_eq!(Duration::try_seconds(            61).unwrap().humanize(), "1 minute");
		assert_eq!(Duration::try_seconds(           130).unwrap().humanize(), "2 minutes");
		assert_eq!(Duration::try_seconds(         3_599).unwrap().humanize(), "59 minutes");
		assert_eq!(Duration::try_seconds(         3_600).unwrap().humanize(), "1 hour");
		assert_eq!(Duration::try_seconds(         3_601).unwrap().humanize(), "1 hour");
		assert_eq!(Duration::try_seconds(         7_200).unwrap().humanize(), "2 hours");
		assert_eq!(Duration::try_seconds(        86_399).unwrap().humanize(), "23 hours");
		assert_eq!(Duration::try_seconds(        86_400).unwrap().humanize(), "1 day");
		assert_eq!(Duration::try_seconds(        86_401).unwrap().humanize(), "1 day");
		assert_eq!(Duration::try_seconds(       172_800).unwrap().humanize(), "2 days");
		assert_eq!(Duration::try_seconds(       604_799).unwrap().humanize(), "6 days");
		assert_eq!(Duration::try_seconds(       604_800).unwrap().humanize(), "1 week");
		assert_eq!(Duration::try_seconds(       604_801).unwrap().humanize(), "1 week");
		assert_eq!(Duration::try_seconds(     1_209_600).unwrap().humanize(), "2 weeks");
		assert_eq!(Duration::try_seconds(     2_591_999).unwrap().humanize(), "4 weeks");
		assert_eq!(Duration::try_seconds(     2_592_000).unwrap().humanize(), "1 month");
		assert_eq!(Duration::try_seconds(     2_592_001).unwrap().humanize(), "1 month");
		assert_eq!(Duration::try_seconds(     5_184_000).unwrap().humanize(), "2 months");
		assert_eq!(Duration::try_seconds(    31_535_999).unwrap().humanize(), "12 months");
		assert_eq!(Duration::try_seconds(    31_536_000).unwrap().humanize(), "1 year");
		assert_eq!(Duration::try_seconds(    31_536_001).unwrap().humanize(), "1 year");
		assert_eq!(Duration::try_seconds(    63_072_000).unwrap().humanize(), "2 years");
		assert_eq!(Duration::try_seconds(   315_360_000).unwrap().humanize(), "10 years");
		assert_eq!(Duration::try_seconds( 3_153_600_000).unwrap().humanize(), "100 years");
		assert_eq!(Duration::try_seconds(31_536_000_000).unwrap().humanize(), "1000 years");
	}
	
	//		nanoseconds_full													
	#[test]
	fn nanoseconds_full() {
		assert_eq!(Duration::nanoseconds_full(             0).unwrap(), Duration::nanoseconds(             0));
		assert_eq!(Duration::nanoseconds_full(             1).unwrap(), Duration::nanoseconds(             1));
		assert_eq!(Duration::nanoseconds_full(            -1).unwrap(), Duration::nanoseconds(            -1));
		assert_eq!(Duration::nanoseconds_full( 1_000_000_000).unwrap(), Duration::nanoseconds( 1_000_000_000));
		assert_eq!(Duration::nanoseconds_full(-1_000_000_000).unwrap(), Duration::nanoseconds(-1_000_000_000));
		assert_eq!(Duration::nanoseconds_full( 1_234_567_890).unwrap(), Duration::nanoseconds( 1_234_567_890));
		assert_eq!(Duration::nanoseconds_full(-1_234_567_890).unwrap(), Duration::nanoseconds(-1_234_567_890));
	}
	#[test]
	fn nanoseconds_full__standard_limit_allowed() {
		assert_eq!(Duration::nanoseconds_full(Duration::MAX_NANOSECONDS as i128).unwrap(), Duration::nanoseconds(Duration::MAX_NANOSECONDS));
		assert_eq!(Duration::nanoseconds_full(Duration::MIN_NANOSECONDS as i128).unwrap(), Duration::nanoseconds(Duration::MIN_NANOSECONDS));
	}
	#[test]
	fn nanoseconds_full__standard_overflow() {
		assert_eq!(Duration::nanoseconds_full(Duration::MAX_NANOSECONDS as i128 + 1).unwrap().num_nanoseconds_full(), Duration::MAX_NANOSECONDS as i128 + 1);
		assert_eq!(Duration::nanoseconds_full(Duration::MIN_NANOSECONDS as i128 - 1).unwrap().num_nanoseconds_full(), Duration::MIN_NANOSECONDS as i128 - 1);
	}
	#[test]
	fn nanoseconds_full__full_limit_allowed() {
		assert_eq!(Duration::nanoseconds_full(Duration::MAX_NANOSECONDS_FULL).unwrap().num_nanoseconds_full(), Duration::MAX_NANOSECONDS_FULL);
		assert_eq!(Duration::nanoseconds_full(Duration::MIN_NANOSECONDS_FULL).unwrap().num_nanoseconds_full(), Duration::MIN_NANOSECONDS_FULL);
	}
	#[test]
	fn nanoseconds_full__full_overflow() {
		assert!(Duration::nanoseconds_full(Duration::MAX_NANOSECONDS_FULL + 1).is_none());
		assert!(Duration::nanoseconds_full(Duration::MIN_NANOSECONDS_FULL - 1).is_none());
	}
	
	//		microseconds_full													
	#[test]
	fn microseconds_full() {
		assert_eq!(Duration::microseconds_full(             0).unwrap(), Duration::microseconds(             0));
		assert_eq!(Duration::microseconds_full(             1).unwrap(), Duration::microseconds(             1));
		assert_eq!(Duration::microseconds_full(            -1).unwrap(), Duration::microseconds(            -1));
		assert_eq!(Duration::microseconds_full( 1_000_000_000).unwrap(), Duration::microseconds( 1_000_000_000));
		assert_eq!(Duration::microseconds_full(-1_000_000_000).unwrap(), Duration::microseconds(-1_000_000_000));
		assert_eq!(Duration::microseconds_full( 1_234_567_890).unwrap(), Duration::microseconds( 1_234_567_890));
		assert_eq!(Duration::microseconds_full(-1_234_567_890).unwrap(), Duration::microseconds(-1_234_567_890));
	}
	#[test]
	fn microseconds_full__standard_limit_allowed() {
		assert_eq!(Duration::microseconds_full(Duration::MAX_MICROSECONDS as i128).unwrap(), Duration::microseconds(Duration::MAX_MICROSECONDS));
		assert_eq!(Duration::microseconds_full(Duration::MIN_MICROSECONDS as i128).unwrap(), Duration::microseconds(Duration::MIN_MICROSECONDS));
	}
	#[test]
	fn microseconds_full__standard_overflow() {
		assert_eq!(Duration::microseconds_full(Duration::MAX_MICROSECONDS as i128 + 1).unwrap().num_microseconds_full(), Duration::MAX_MICROSECONDS as i128 + 1);
		assert_eq!(Duration::microseconds_full(Duration::MIN_MICROSECONDS as i128 - 1).unwrap().num_microseconds_full(), Duration::MIN_MICROSECONDS as i128 - 1);
	}
	#[test]
	fn microseconds_full__full_limit_allowed() {
		assert_eq!(Duration::microseconds_full(Duration::MAX_MICROSECONDS_FULL).unwrap().num_microseconds_full(), Duration::MAX_MICROSECONDS_FULL);
		assert_eq!(Duration::microseconds_full(Duration::MIN_MICROSECONDS_FULL).unwrap().num_microseconds_full(), Duration::MIN_MICROSECONDS_FULL);
	}
	#[test]
	fn microseconds_full__full_overflow() {
		assert!(Duration::microseconds_full(Duration::MAX_MICROSECONDS_FULL + 1).is_none());
		assert!(Duration::microseconds_full(Duration::MIN_MICROSECONDS_FULL - 1).is_none());
	}
	
	//		num_nanoseconds_full												
	#[test]
	fn num_nanoseconds_full() {
		assert_eq!(Duration::nanoseconds(             0).num_nanoseconds_full(),              0);
		assert_eq!(Duration::nanoseconds(             1).num_nanoseconds_full(),              1);
		assert_eq!(Duration::nanoseconds(            -1).num_nanoseconds_full(),             -1);
		assert_eq!(Duration::nanoseconds( 1_000_000_000).num_nanoseconds_full(),  1_000_000_000);
		assert_eq!(Duration::nanoseconds(-1_000_000_000).num_nanoseconds_full(), -1_000_000_000);
		assert_eq!(Duration::nanoseconds( 1_234_567_890).num_nanoseconds_full(),  1_234_567_890);
		assert_eq!(Duration::nanoseconds(-1_234_567_890).num_nanoseconds_full(), -1_234_567_890);
	}
	#[test]
	fn num_nanoseconds_full__beyond_normal_limits() {
		let duration1 = Duration::nanoseconds(Duration::MAX_NANOSECONDS)
			.checked_add(&Duration::nanoseconds(Duration::MAX_NANOSECONDS))
			.unwrap()
		;
		assert!(duration1.num_nanoseconds().is_none());
		assert_eq!(duration1.num_nanoseconds_full(), Duration::MAX_NANOSECONDS as i128 * 2);
		
		let duration2 = Duration::nanoseconds(Duration::MIN_NANOSECONDS)
			.checked_add(&Duration::nanoseconds(Duration::MIN_NANOSECONDS))
			.unwrap()
		;
		assert!(duration2.num_nanoseconds().is_none());
		assert_eq!(duration2.num_nanoseconds_full(), Duration::MIN_NANOSECONDS as i128 * 2);
	}
	
	//		num_microseconds_full												
	#[test]
	fn num_microseconds_full() {
		assert_eq!(Duration::microseconds(         0).num_microseconds_full(),          0);
		assert_eq!(Duration::microseconds(         1).num_microseconds_full(),          1);
		assert_eq!(Duration::microseconds(        -1).num_microseconds_full(),         -1);
		assert_eq!(Duration::microseconds( 1_000_000).num_microseconds_full(),  1_000_000);
		assert_eq!(Duration::microseconds(-1_000_000).num_microseconds_full(), -1_000_000);
		assert_eq!(Duration::microseconds( 1_234_567).num_microseconds_full(),  1_234_567);
		assert_eq!(Duration::microseconds(-1_234_567).num_microseconds_full(), -1_234_567);
	}
	#[test]
	fn num_microseconds_full__beyond_normal_limits() {
		let duration1 = Duration::microseconds(Duration::MAX_MICROSECONDS)
			.checked_add(&Duration::microseconds(Duration::MAX_MICROSECONDS))
			.unwrap()
		;
		assert!(duration1.num_microseconds().is_none());
		assert_eq!(duration1.num_microseconds_full(), Duration::MAX_MICROSECONDS as i128 * 2);
		
		let duration2 = Duration::microseconds(Duration::MAX_MICROSECONDS)
			.checked_add(&Duration::microseconds(Duration::MAX_MICROSECONDS))
			.unwrap()
		;
		assert!(duration2.num_microseconds().is_none());
		assert_eq!(duration2.num_microseconds_full(), Duration::MAX_MICROSECONDS as i128 * 2);
	}
}

//§		MonthsExt																
#[cfg(test)]
mod months_ext {
	use super::*;
	
	//		MAX_MONTHS															
	#[test]
	fn max_months__max_allowed() {
		assert_eq!(Months::new(Months::MAX_MONTHS).num_months(), Months::MAX_MONTHS);
	}
	
	//		MAX_YEARS															
	#[test]
	fn max_years__max_allowed() {
		assert_eq!(Months::new(Months::MAX_YEARS * 12).num_years(), Months::MAX_YEARS);
	}
	
	//		months																
	#[test]
	fn months() {
		assert_eq!(Months::months(0), Months::new(0));
		assert_eq!(Months::months(1), Months::new(1));
	}
	#[test]
	fn months__max_allowed() {
		assert_eq!(Months::months(Months::MAX_MONTHS).num_months(), Months::MAX_MONTHS);
		assert_eq!(Months::months(Months::MAX_MONTHS),              Months::new(Months::MAX_MONTHS));
	}
	
	//		years																
	#[test]
	fn years() {
		assert_eq!(Months::years(0).unwrap(), Months::new( 0));
		assert_eq!(Months::years(1).unwrap(), Months::new(12));
	}
	#[test]
	fn years__max_allowed() {
		assert_eq!(Months::years(Months::MAX_YEARS).unwrap().num_years(), Months::MAX_YEARS);
		assert_eq!(Months::years(Months::MAX_YEARS).unwrap(),             Months::new(Months::MAX_YEARS * 12));
	}
	#[test]
	fn years__overflow() {
		assert!(Months::years(Months::MAX_YEARS + 1).is_none());
	}
	
	//		num_months															
	#[test]
	fn num_months() {
		assert_eq!(Months::new(0).num_months(), 0);
		assert_eq!(Months::new(1).num_months(), 1);
		assert_eq!(Months::new(u32::MAX).num_months(), u32::MAX);
	}
	
	//		num_years															
	#[expect(clippy::integer_division, reason = "Acceptable for the test")]
	#[test]
	fn num_years() {
		assert_eq!(Months::new(0).num_years(), 0);
		assert_eq!(Months::new(1).num_years(), 0);
		assert_eq!(Months::new(11).num_years(), 0);
		assert_eq!(Months::new(12).num_years(), 1);
		assert_eq!(Months::new(23).num_years(), 1);
		assert_eq!(Months::new(24).num_years(), 2);
		assert_eq!(Months::new(u32::MAX).num_years(), u32::MAX / 12);
	}
}

//§		NaiveDateExt															
#[cfg(test)]
mod naivedate_ext {
	use super::*;
	
	//		MAX_YEAR															
	#[expect(clippy::decimal_literal_representation, reason = "Needed for clear formatting")]
	#[test]
	fn max_year__max_allowed() {
		assert_eq!(Utc.with_ymd_and_hms(NaiveDate::MAX_YEAR, 12, 31, 00, 00, 00).unwrap(), Utc.with_ymd_and_hms(262_142, 12, 31, 00, 00, 00).unwrap());
	}
	#[test]
	#[should_panic(expected = "No such local time")]
	fn max_year__overflow() {
		let _ = Utc.with_ymd_and_hms(NaiveDate::MAX_YEAR + 1, 12, 31, 00, 00, 00).unwrap();
	}
	
	//		MIN_YEAR															
	#[expect(clippy::decimal_literal_representation, reason = "Needed for clear formatting")]
	#[test]
	fn min_year__min_allowed() {
		assert_eq!(Utc.with_ymd_and_hms(NaiveDate::MIN_YEAR, 1, 1, 00, 00, 00).unwrap(), Utc.with_ymd_and_hms(-262_143, 1, 1, 00, 00, 00).unwrap());
	}
	#[test]
	#[should_panic(expected = "No such local time")]
	fn min_year__overflow() {
		let _ = Utc.with_ymd_and_hms(NaiveDate::MIN_YEAR - 1, 1, 1, 00, 00, 00).unwrap();
	}
	
	//		today																
	#[test]
	fn today() {
		//	There is a very small possibility that this test will fail if the
		//	test is run at the exact moment that the date changes.
		let timestamp = Utc::now().timestamp();
		let datetime  = DateTime::from_timestamp(timestamp, 0).unwrap();
		let today     = NaiveDate::today();
		assert_eq!(today.year(),  datetime.year());
		assert_eq!(today.month(), datetime.month());
		assert_eq!(today.day(),   datetime.day());
	}
	
	//		days_in_month														
	#[test]
	fn days_in_month() {
		let date = NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap();
		assert_eq!(date.days_in_month(), 31);
	}
	
	//		days_in_month_opt													
	#[test]
	fn days_in_month_opt__valid() {
		assert_some_eq!(NaiveDate::days_in_month_opt(2_000, 1), 31);
	}
	#[test]
	fn days_in_month_opt__invalid() {
		assert_none!(NaiveDate::days_in_month_opt(2_000, 13));
	}
	
	//		days_in_year														
	#[test]
	fn days_in_year__normal() {
		let date = NaiveDate::from_ymd_opt(2_001, 1, 1).unwrap();
		assert_eq!(date.days_in_year(), 365);
	}
	#[test]
	fn days_in_year__leap() {
		let date = NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap();
		assert_eq!(date.days_in_year(), 366);
	}
	
	//		days_in_year_opt													
	#[test]
	fn days_in_year_opt__normal() {
		assert_some_eq!(NaiveDate::days_in_year_opt(2_001), 365);
	}
	#[test]
	fn days_in_year_opt__leap() {
		assert_some_eq!(NaiveDate::days_in_year_opt(2_000), 366);
	}
	
	//		is_leap_year														
	#[test]
	fn is_leap_year() {
		let date1 = NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap();
		assert!( date1.is_leap_year());
		
		let date2 = NaiveDate::from_ymd_opt(2_001, 1, 1).unwrap();
		assert!(!date2.is_leap_year());
	}
	
	//		is_leap_year_opt													
	#[test]
	fn is_leap_year_opt() {
		assert_some_eq!(NaiveDate::is_leap_year_opt(2_000), true);
		assert_some_eq!(NaiveDate::is_leap_year_opt(2_001), false);
	}
	
	//		start_of_month														
	#[test]
	fn start_of_month() {
		let date = NaiveDate::from_ymd_opt(2_000, 1, 20).unwrap();
		assert_eq!(date.start_of_month(), NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap());
	}
	
	//		start_of_month_opt													
	#[test]
	fn start_of_month_opt__valid() {
		assert_some_eq!(NaiveDate::start_of_month_opt(2_000, 1), NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap());
	}
	#[test]
	fn start_of_month_opt__invalid() {
		assert_none!(NaiveDate::start_of_month_opt(2_000, 13));
	}
	
	//		end_of_month														
	#[test]
	fn end_of_month() {
		let date = NaiveDate::from_ymd_opt(2_000, 1, 20).unwrap();
		assert_eq!(date.end_of_month(), NaiveDate::from_ymd_opt(2_000, 1, 31).unwrap());
	}
	
	//		end_of_month_opt													
	#[test]
	fn end_of_month_opt__valid() {
		assert_some_eq!(NaiveDate::end_of_month_opt(2_000, 1), NaiveDate::from_ymd_opt(2_000, 1, 31).unwrap());
	}
	#[test]
	fn end_of_month_opt__invalid() {
		assert_none!(NaiveDate::end_of_month_opt(    2_000, 13));
		assert_none!(NaiveDate::end_of_month_opt(2_000_000,  1));
		assert_none!(NaiveDate::end_of_month_opt(2_000_000, 13));
	}
	
	//		start_of_year														
	#[test]
	fn start_of_year() {
		let date = NaiveDate::from_ymd_opt(2_000, 2, 20).unwrap();
		assert_eq!(date.start_of_year(), NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap());
	}
	
	//		start_of_year_opt													
	#[test]
	fn start_of_year_opt() {
		assert_some_eq!(NaiveDate::start_of_year_opt(2_000), NaiveDate::from_ymd_opt(2_000, 1, 1).unwrap());
	}
	
	//		end_of_year														
	#[test]
	fn end_of_year() {
		let date = NaiveDate::from_ymd_opt(2_000, 2, 20).unwrap();
		assert_eq!(date.end_of_year(), NaiveDate::from_ymd_opt(2_000, 12, 31).unwrap());
	}
	
	//		end_of_year_opt													
	#[test]
	fn end_of_year_opt() {
		assert_some_eq!(NaiveDate::end_of_year_opt(2_000), NaiveDate::from_ymd_opt(2_000, 12, 31).unwrap());
	}
}


