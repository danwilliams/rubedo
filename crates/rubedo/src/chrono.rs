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
	/// The units used by [`humanize()`](DurationExt::humanize()). These
	/// determine the units that will be used to represent a duration, with the
	/// largest possible unit being used.
	const UNITS:   [(i64, &'static str); 7] = [
		(31_536_000, "year"),    //  60 * 60 * 24 * 365
		( 2_592_000, "month"),   //  60 * 60 * 24 * 30
		(   604_800, "week"),    //  60 * 60 * 24 * 7
		(    86_400, "day"),     //  60 * 60 * 24
		(     3_600, "hour"),    //  60 * 60
		(        60, "minute"),  //  60
		(         1, "second"),  //  1
	];
	
	//		humanize															
	/// Returns a human-readable string representation of a [`Duration`].
	/// 
	/// This will indicate the [`Duration`] as an expression of the largest unit
	/// available. For example, if the duration is 1 year, 2 months, 3 weeks,
	/// 4 days, 5 hours, 6 minutes, and 7 seconds, it will return "1 year".
	/// 
	fn humanize(&self) -> String;
}

impl DurationExt for Duration {
	//		humanize															
	fn humanize(&self) -> String {
		let seconds = self.num_seconds();
		for &(unit, name) in &Self::UNITS {
			if seconds >= unit {
				#[cfg_attr(    feature = "reasons",  allow(clippy::arithmetic_side_effects,
					reason = "Precision is not needed here, and unit cannot be zero"
				))]
				#[cfg_attr(not(feature = "reasons"), allow(clippy::arithmetic_side_effects))]
				#[cfg_attr(    feature = "reasons",  allow(clippy::integer_division,
					reason = "Precision is not needed here"
				))]
				#[cfg_attr(not(feature = "reasons"), allow(clippy::integer_division))]
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
	/// The maximum year that can be represented by a [`NaiveDate`].
	const MAX_YEAR: i32 = i32::MAX >> 13_i32;
	/// The minimum year that can be represented by a [`NaiveDate`].
	const MIN_YEAR: i32 = i32::MIN >> 13_i32;
	
	//		today																
	/// Returns the current date.
	/// 
	/// Although this is a static method, it does not return an [`Option`] as it
	/// cannot fail.
	/// 
	fn today() -> NaiveDate;
	
	//		days_in_month														
	/// Returns the number of days in the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`days_in_month_opt()`](NaiveDateExt::days_in_month_opt()).
	/// 
	/// # See also
	/// 
	/// * [`days_in_month_opt()`](NaiveDateExt::days_in_month_opt())
	/// * [`days_in_year()`](NaiveDateExt::days_in_year())
	/// 
	fn days_in_month(&self) -> u32;
	
	//		days_in_month_opt														
	/// Returns the number of days in the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`days_in_month()`](NaiveDateExt::days_in_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`days_in_month()`](NaiveDateExt::days_in_month())
	/// * [`days_in_year_opt()`](NaiveDateExt::days_in_year_opt())
	/// 
	fn days_in_month_opt(year: i32, month: u32) -> Option<u32>;
	
	//		days_in_year														
	/// Returns the number of days in the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`days_in_year_opt()`](NaiveDateExt::days_in_year_opt()).
	/// 
	/// # See also
	/// 
	/// * [`days_in_month()`](NaiveDateExt::days_in_month())
	/// * [`days_in_year_opt()`](NaiveDateExt::days_in_year_opt())
	/// 
	fn days_in_year(&self) -> u32;
	
	//		days_in_year_opt														
	/// Returns the number of days in the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`days_in_year()`](NaiveDateExt::days_in_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`days_in_month_opt()`](NaiveDateExt::days_in_month_opt())
	/// * [`days_in_year()`](NaiveDateExt::days_in_year())
	/// 
	fn days_in_year_opt(year: i32) -> Option<u32>;
	
	//		is_leap_year														
	/// Returns `true` if the date's year is a leap year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`is_leap_year_opt()`](NaiveDateExt::is_leap_year_opt()).
	/// 
	/// # See also
	/// 
	/// * [`is_leap_year_opt()`](NaiveDateExt::is_leap_year_opt())
	/// 
	fn is_leap_year(&self) -> bool;
	
	//		is_leap_year_opt														
	/// Returns `true` if the given year is a leap year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`is_leap_year()`](NaiveDateExt::is_leap_year()).
	/// 
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`is_leap_year()`](NaiveDateExt::is_leap_year())
	/// 
	fn is_leap_year_opt(year: i32) -> Option<bool>;
	
	//		start_of_month														
	/// Returns the date of the first day of the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt()).
	/// 
	/// # See also
	/// 
	/// * [`end_of_month()`](NaiveDateExt::end_of_month())
	/// * [`end_of_year()`](NaiveDateExt::end_of_year())
	/// * [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt())
	/// * [`start_of_year()`](NaiveDateExt::start_of_year())
	/// 
	fn start_of_month(&self) -> NaiveDate;
	
	//		start_of_month_opt														
	/// Returns the date of the first day of the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`start_of_month()`](NaiveDateExt::start_of_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt())
	/// * [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt())
	/// * [`start_of_month()`](NaiveDateExt::start_of_month())
	/// * [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt())
	/// 
	fn start_of_month_opt(year: i32, month: u32) -> Option<NaiveDate>;
	
	//		end_of_month														
	/// Returns the date of the last day of the date's month.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt()).
	/// 
	/// # See also
	/// 
	/// * [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt())
	/// * [`end_of_year()`](NaiveDateExt::end_of_year())
	/// * [`start_of_month()`](NaiveDateExt::start_of_month())
	/// * [`start_of_year()`](NaiveDateExt::start_of_year())
	/// 
	fn end_of_month(&self) -> NaiveDate;
	
	//		end_of_month_opt														
	/// Returns the date of the last day of the given month.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`end_of_month()`](NaiveDateExt::end_of_month()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year or month is
	/// invalid, [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`end_of_month()`](NaiveDateExt::end_of_month())
	/// * [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt())
	/// * [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt())
	/// * [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt())
	/// 
	fn end_of_month_opt(year: i32, month: u32) -> Option<NaiveDate>;
	
	//		start_of_year														
	/// Returns the date of the first day of the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt()).
	/// 
	/// # See also
	/// 
	/// * [`end_of_month()`](NaiveDateExt::end_of_month())
	/// * [`end_of_year()`](NaiveDateExt::end_of_year())
	/// * [`start_of_month()`](NaiveDateExt::start_of_month())
	/// * [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt())
	/// 
	fn start_of_year(&self) -> NaiveDate;
	
	//		start_of_year_opt														
	/// Returns the date of the first day of the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`start_of_year()`](NaiveDateExt::start_of_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt())
	/// * [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt())
	/// * [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt())
	/// * [`start_of_year()`](NaiveDateExt::start_of_year())
	/// 
	fn start_of_year_opt(year: i32) -> Option<NaiveDate>;
	
	//		end_of_year															
	/// Returns the date of the last day of the date's year.
	/// 
	/// This method operates on `&self`. For the equivalent method that operates
	/// without an instance, see [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt()).
	/// 
	/// # See also
	/// 
	/// * [`end_of_month()`](NaiveDateExt::end_of_month())
	/// * [`end_of_year_opt()`](NaiveDateExt::end_of_year_opt())
	/// * [`start_of_month()`](NaiveDateExt::start_of_month())
	/// * [`start_of_year()`](NaiveDateExt::start_of_year())
	/// 
	fn end_of_year(&self) -> NaiveDate;
	
	//		end_of_year_opt															
	/// Returns the date of the last day of the given year.
	/// 
	/// This method operates without an instance. For the equivalent method that
	/// operates on `&self`, see [`end_of_year()`](NaiveDateExt::end_of_year()).
	///
	/// The outcome is wrapped in an [`Option`]. If the given year is invalid,
	/// [`None`] is returned.
	/// 
	/// # See also
	/// 
	/// * [`end_of_month_opt()`](NaiveDateExt::end_of_month_opt())
	/// * [`end_of_year()`](NaiveDateExt::end_of_year())
	/// * [`start_of_month_opt()`](NaiveDateExt::start_of_month_opt())
	/// * [`start_of_year_opt()`](NaiveDateExt::start_of_year_opt())
	/// 
	fn end_of_year_opt(year: i32) -> Option<NaiveDate>;
}

impl NaiveDateExt for NaiveDate {
	//		today																
	fn today() -> Self {
		Utc::now().date_naive()
	}
	
	//		days_in_month														
	fn days_in_month(&self) -> u32 {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::days_in_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		days_in_month_opt													
	fn days_in_month_opt(year: i32, month: u32) -> Option<u32> {
		Self::end_of_month_opt(year, month).map(|date| date.day())
	}
	
	//		days_in_year														
	fn days_in_year(&self) -> u32 {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::days_in_year_opt(self.year()).unwrap()
	}
	
	//		days_in_year_opt													
	fn days_in_year_opt(year: i32) -> Option<u32> {
		Self::from_ymd_opt(year, 2, 1).map(|_| if Self::from_ymd_opt(year, 2, 29).is_some() { 366 } else { 365 })
	}
	
	//		is_leap_year														
	fn is_leap_year(&self) -> bool {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::is_leap_year_opt(self.year()).unwrap()
	}
	
	//		is_leap_year_opt													
	fn is_leap_year_opt(year: i32) -> Option<bool> {
		Self::from_ymd_opt(year, 2, 1).map(|_| Self::from_ymd_opt(year, 2, 29).is_some())
	}
	
	//		start_of_month														
	fn start_of_month(&self) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::start_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		start_of_month_opt													
	fn start_of_month_opt(year: i32, month: u32) -> Option<Self> {
		Self::from_ymd_opt(year, month, 1)
	}
	
	//		end_of_month														
	fn end_of_month(&self) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::end_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		end_of_month_opt													
	fn end_of_month_opt(year: i32, month: u32) -> Option<Self> {
		_ = Self::from_ymd_opt(year, month, 1)?;
		//	The range of years is controlled by having already validated the date
		//	by attempting to create it above. This is well within the range of a u32.
		//	The same applies to the month.
		#[cfg_attr(    feature = "reasons",  allow(clippy::arithmetic_side_effects, reason = "Range is controlled"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::arithmetic_side_effects))]
		Self::from_ymd_opt(
			if month == 12 { year + 1 } else { year      },
			if month == 12 { month    } else { month + 1 },
			1,
		)?.pred_opt()
	}
	
	//		start_of_year														
	fn start_of_year(&self) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::start_of_year_opt(self.year()).unwrap()
	}
	
	//		start_of_year_opt													
	fn start_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 1, 1)
	}
	
	//		end_of_year															
	fn end_of_year(&self) -> Self {
		#[cfg_attr(    feature = "reasons",  allow(clippy::unwrap_used, reason = "Infallible"))]
		#[cfg_attr(not(feature = "reasons"), allow(clippy::unwrap_used))]
		Self::end_of_year_opt(self.year()).unwrap()
	}
	
	//		end_of_year_opt														
	fn end_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 12, 31)
	}
}


