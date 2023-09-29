//! This module provides extensions to the [Chrono](https://crates.io/crates/chrono)
//! crate.



//		Modules

#[cfg(test)]
#[path = "tests/chrono.rs"]
mod tests;



//		Packages

use crate::sugar::s;
use chrono::{prelude::*, Duration, NaiveDate, Utc};



//		Traits

//§		DurationExt																
/// This trait provides additional functionality to [`Duration`].
pub trait DurationExt {
	//		humanize															
	/// Returns a human-readable string representation of the duration.
	/// 
	/// This will indicate the duration as an expression of the largest unit
	/// available. For example, if the duration is 1 year, 2 months, 3 weeks,
	/// 4 days, 5 hours, 6 minutes, and 7 seconds, it will return "1 year".
	fn humanize(&self) -> String;
}

impl DurationExt for Duration {
	//		humanize															
	fn humanize(&self) -> String {
		let seconds = self.num_seconds();
		let units   = vec![
			(31536000, "year"),    //  60 * 60 * 24 * 365		
			(2592000,  "month"),   //  60 * 60 * 24 * 30		
			(604800,   "week"),    //  60 * 60 * 24 * 7		
			(86400,    "day"),     //  60 * 60 * 24			
			(3600,     "hour"),    //  60 * 60					
			(60,       "minute"),  //  60						
			(1,        "second"),  //  1						
		];
		for (unit, name) in units {
			if seconds >= unit {
				let count = seconds / unit;
				return format!("{} {}{}", count, name, if count == 1 { "" } else { "s" });
			}
		}
		s!("0 seconds")
	}
}

//§		NaiveDateExt															
/// This trait provides additional functionality to [`NaiveDate`].
pub trait NaiveDateExt {
	//		today																
	/// Returns the current date.
	/// 
	/// Although this is a static method, it does not return an [`Option`] as it
	/// cannot fail.
	fn today() -> NaiveDate;
	
	//		days_in_month														
	/// Returns the number of days in the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`days_in_month_opt()`](NaiveDateExt::days_in_month_opt()).
	fn days_in_month(&self) -> u32;
	
	//		days_in_month_opt														
	/// Returns the number of days in the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`days_in_month()`](NaiveDateExt::days_in_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, `None` is returned.
	fn days_in_month_opt(year: i32, month: u32) -> Option<u32>;
	
	//		days_in_year														
	/// Returns the number of days in the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`days_in_year_opt()`](NaiveDateExt::days_in_year_opt()).
	fn days_in_year(&self) -> u32;
	
	//		days_in_year_opt														
	/// Returns the number of days in the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`days_in_year()`](NaiveDateExt::days_in_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// `None` is returned.
	fn days_in_year_opt(year: i32) -> Option<u32>;
	
	//		is_leap_year														
	/// Returns `true` if the date's year is a leap year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`is_leap_year_opt()`](NaiveDateExt::is_leap_year_opt()).
	fn is_leap_year(&self) -> bool;
	
	//		is_leap_year_opt														
	/// Returns `true` if the given year is a leap year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`is_leap_year()`](NaiveDateExt::is_leap_year()).
	/// 
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// `None` is returned.
	fn is_leap_year_opt(year: i32) -> Option<bool>;
	
	//		start_of_month														
	/// Returns the date of the first day of the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt()).
	fn start_of_month(&self) -> NaiveDate;
	
	//		start_of_month_opt														
	/// Returns the date of the first day of the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`start_of_month()`](NaiveDateExt::start_of_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, `None` is returned.
	fn start_of_month_opt(year: i32, month: u32) -> Option<NaiveDate>;
	
	//		end_of_month														
	/// Returns the date of the last day of the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt()).
	fn end_of_month(&self) -> NaiveDate;
	
	//		end_of_month_opt														
	/// Returns the date of the last day of the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`end_of_month()`](NaiveDateExt::end_of_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, `None` is returned.
	fn end_of_month_opt(year: i32, month: u32) -> Option<NaiveDate>;
	
	//		start_of_year														
	/// Returns the date of the first day of the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt()).
	fn start_of_year(&self) -> NaiveDate;
	
	//		start_of_year_opt														
	/// Returns the date of the first day of the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`start_of_year()`](NaiveDateExt::start_of_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// `None` is returned.
	fn start_of_year_opt(year: i32) -> Option<NaiveDate>;
	
	//		end_of_year															
	/// Returns the date of the last day of the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt()).
	fn end_of_year(&self) -> NaiveDate;
	
	//		end_of_year_opt															
	/// Returns the date of the last day of the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`end_of_year()`](NaiveDateExt::end_of_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// `None` is returned.
	fn end_of_year_opt(year: i32) -> Option<NaiveDate>;
}

impl NaiveDateExt for NaiveDate {
	//		today																
	fn today() -> Self {
		Utc::now().date_naive()
	}
	
	//		days_in_month														
	fn days_in_month(&self) -> u32 {
		Self::days_in_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		days_in_month_opt													
	fn days_in_month_opt(year: i32, month: u32) -> Option<u32> {
		Self::end_of_month_opt(year, month).map(|date| date.day())
	}
	
	//		days_in_year														
	fn days_in_year(&self) -> u32 {
		Self::days_in_year_opt(self.year()).unwrap()
	}
	
	//		days_in_year_opt													
	fn days_in_year_opt(year: i32) -> Option<u32> {
		Self::from_ymd_opt(year, 2, 1).map(|_| if Self::from_ymd_opt(year, 2, 29).is_some() { 366 } else { 365 })
	}
	
	//		is_leap_year														
	fn is_leap_year(&self) -> bool {
		Self::is_leap_year_opt(self.year()).unwrap()
	}
	
	//		is_leap_year_opt													
	fn is_leap_year_opt(year: i32) -> Option<bool> {
		Self::from_ymd_opt(year, 2, 1).map(|_| Self::from_ymd_opt(year, 2, 29).is_some())
	}
	
	//		start_of_month														
	fn start_of_month(&self) -> Self {
		Self::start_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		start_of_month_opt													
	fn start_of_month_opt(year: i32, month: u32) -> Option<Self> {
		Self::from_ymd_opt(year, month, 1)
	}
	
	//		end_of_month														
	fn end_of_month(&self) -> Self {
		Self::end_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		end_of_month_opt													
	fn end_of_month_opt(year: i32, month: u32) -> Option<Self> {
		match Self::from_ymd_opt(year, month, 1) {
			Some(_)         => match Self::from_ymd_opt(year, month + 1, 1).or(Self::from_ymd_opt(year + 1, 1, 1)) {
				Some(date) => date.pred_opt(),
				None       => None,
			},
			None           => None,
		}
	}
	
	//		start_of_year														
	fn start_of_year(&self) -> Self {
		Self::start_of_year_opt(self.year()).unwrap()
	}
	
	//		start_of_year_opt													
	fn start_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 1, 1)
	}
	
	//		end_of_year															
	fn end_of_year(&self) -> Self {
		Self::end_of_year_opt(self.year()).unwrap()
	}
	
	//		end_of_year_opt														
	fn end_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 12, 31)
	}
}


