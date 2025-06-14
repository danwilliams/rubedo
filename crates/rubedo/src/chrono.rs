//! This module provides extensions to the [Chrono](https://crates.io/crates/chrono)
//! crate.



//		Modules																											

#[cfg(test)]
#[path = "tests/chrono.rs"]
mod tests;



//		Packages																										

use crate::sugar::s;
use chrono::{Datelike as _, Duration, Months, NaiveDate, Utc};
use core::ops::Neg as _;



//		Traits																											

//§		DurationExt																
/// This trait provides additional functionality to [`Duration`].
pub trait DurationExt {
	/// The maximum number of nanoseconds that can be represented by a
	/// [`Duration`] when expressed alone. Note that this is not the maximum
	/// number of nanoseconds that can actually be stored by a [`Duration`], but
	/// the maximum number that can be expressed as an individual component. The
	/// [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow. The maximum number of nanoseconds that
	/// can be stored is therefore the maximum number of seconds multiplied by
	/// one billion. However, the expression of this full value as nanoseconds
	/// is not possible, and therefore this constant indicates the maximum
	/// number of nanoseconds that can be expressed with current Chrono
	/// functionality.
	const MAX_NANOSECONDS:       i64  = i64::MAX;
	
	/// The maximum number of nanoseconds that can be represented by a
	/// [`Duration`]. Note that this is the maximum number of nanoseconds that
	/// can actually be stored by a [`Duration`], which is more than can be
	/// expressed alone using standard Chrono functionality.
	const MAX_NANOSECONDS_FULL:  i128 = i64::MAX as i128 * 1_000 * 1_000;
	
	/// The maximum number of microseconds that can be represented by a
	/// [`Duration`] when expressed alone. Note that this is not the maximum
	/// number of microseconds that can actually be stored by a [`Duration`],
	/// but the maximum number that can be expressed as an individual component.
	/// The [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow. The maximum number of microseconds
	/// that can be stored is therefore the maximum number of seconds multiplied
	/// by one million. However, the expression of this full value as
	/// microseconds is not possible, and therefore this constant indicates the
	/// maximum number of microseconds that can be expressed with current Chrono
	/// functionality.
	const MAX_MICROSECONDS:      i64  = i64::MAX;
	
	/// The maximum number of microseconds that can be represented by a
	/// [`Duration`]. Note that this is the maximum number of microseconds that
	/// can actually be stored by a [`Duration`], which is more than can be
	/// expressed alone using standard Chrono functionality.
	const MAX_MICROSECONDS_FULL: i128 = i64::MAX as i128 * 1_000;
	
	/// The maximum number of milliseconds that can be represented by a
	/// [`Duration`]. The [`Duration`] struct stores its value as a number of
	/// seconds and nanoseconds, but artificially limits the number of seconds
	/// so that the milliseconds will never overflow. The maximum number of
	/// milliseconds that can be stored is therefore the maximum number of
	/// seconds multiplied by one thousand, and the expression of this full
	/// value as milliseconds is possible.
	const MAX_MILLISECONDS:      i64  = i64::MAX;
	
	/// The maximum number of seconds that can be represented by a [`Duration`].
	/// The [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow.
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_SECONDS:           i64  = i64::MAX / 1_000;
	
	/// The maximum number of minutes that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_MINUTES:           i64  = i64::MAX / 1_000 / 60;
	
	/// The maximum number of hours that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_HOURS:             i64  = i64::MAX / 1_000 / 60 / 60;
	
	/// The maximum number of days that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_DAYS:              i64  = i64::MAX / 1_000 / 60 / 60 / 24;
	
	/// The maximum number of weeks that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_WEEKS:             i64  = i64::MAX / 1_000 / 60 / 60 / 24 / 7;
	
	/// The minimum number of nanoseconds that can be represented by a
	/// [`Duration`] when expressed alone. Note that this is not the minimum
	/// number of nanoseconds that can actually be stored by a [`Duration`], but
	/// the minimum number that can be expressed as an individual component. The
	/// [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow. The minimum number of nanoseconds that
	/// can be stored is therefore the minimum number of seconds multiplied by
	/// one billion. However, the expression of this full value as nanoseconds
	/// is not possible, and therefore this constant indicates the minimum
	/// number of nanoseconds that can be expressed with current Chrono
	/// functionality.
	const MIN_NANOSECONDS:       i64  = i64::MIN;
	
	/// The minimum number of nanoseconds that can be represented by a
	/// [`Duration`]. Note that this is the minimum number of nanoseconds that
	/// can actually be stored by a [`Duration`], which is more than can be
	/// expressed alone using standard Chrono functionality. The range here
	/// notably is from `-i64::MAX` and not `i64::MIN`, due to changes in
	/// <https://github.com/chronotope/chrono/pull/1334>.
	const MIN_NANOSECONDS_FULL:  i128 = -i64::MAX as i128 * 1_000 * 1_000;
	
	/// The minimum number of microseconds that can be represented by a
	/// [`Duration`] when expressed alone. Note that this is not the minimum
	/// number of microseconds that can actually be stored by a [`Duration`],
	/// but the minimum number that can be expressed as an individual component.
	/// The [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow. The minimum number of microseconds
	/// that can be stored is therefore the minimum number of seconds multiplied
	/// by one million. However, the expression of this full value as
	/// microseconds is not possible, and therefore this constant indicates the
	/// minimum number of microseconds that can be expressed with current Chrono
	/// functionality.
	const MIN_MICROSECONDS:      i64  = i64::MIN;
	
	/// The minimum number of microseconds that can be represented by a
	/// [`Duration`]. Note that this is the minimum number of microseconds that
	/// can actually be stored by a [`Duration`], which is more than can be
	/// expressed alone using standard Chrono functionality. The range here
	/// notably is from `-i64::MAX` and not `i64::MIN`, due to changes in
	/// <https://github.com/chronotope/chrono/pull/1334>.
	const MIN_MICROSECONDS_FULL: i128 = -i64::MAX as i128 * 1_000;
	
	/// The minimum number of milliseconds that can be represented by a
	/// [`Duration`]. The [`Duration`] struct stores its value as a number of
	/// seconds and nanoseconds, but artificially limits the number of seconds
	/// so that the milliseconds will never overflow. The minimum number of
	/// milliseconds that can be stored is therefore the minimum number of
	/// seconds multiplied by one thousand, and the expression of this full
	/// value as milliseconds is possible. The range here notably is from
	/// `-i64::MAX` and not `i64::MIN`, due to changes in
	/// <https://github.com/chronotope/chrono/pull/1334>.
	const MIN_MILLISECONDS:      i64  = -i64::MAX;
	
	/// The minimum number of seconds that can be represented by a [`Duration`].
	/// The [`Duration`] struct stores its value as a number of seconds and
	/// nanoseconds, but artificially limits the number of seconds so that the
	/// milliseconds will never overflow.
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MIN_SECONDS:           i64  = i64::MIN / 1_000;
	
	/// The minimum number of minutes that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MIN_MINUTES:           i64  = i64::MIN / 1_000 / 60;
	
	/// The minimum number of hours that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MIN_HOURS:             i64  = i64::MIN / 1_000 / 60 / 60;
	
	/// The minimum number of days that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MIN_DAYS:              i64  = i64::MIN / 1_000 / 60 / 60 / 24;
	
	/// The minimum number of weeks that can be represented by a [`Duration`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MIN_WEEKS:             i64  = i64::MIN / 1_000 / 60 / 60 / 24 / 7;
	
	/// The units used by [`humanize()`](DurationExt::humanize()). These
	/// determine the units that will be used to represent a duration, with the
	/// largest possible unit being used.
	const UNITS: [(i64, &'static str); 7] = [
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
	
	//		nanoseconds_full												
	/// Make a new [`Duration`] with the given number of nanoseconds.
	/// 
	/// This function is necessary because although the [`Duration`] struct
	/// stores its value internally as seconds and nanoseconds, the
	/// [`nanoseconds()`](Duration::nanoseconds()) method only accepts the
	/// number of nanoseconds as an [`i64`], which is not large enough to
	/// express the full range of nanoseconds that can be stored by a
	/// [`Duration`] instance. This function therefore accepts the number of
	/// nanoseconds as an [`i128`], which is large enough to express the full
	/// range of nanoseconds that can be stored, and creates a [`Duration`]
	/// instance appropriately.
	/// 
	/// # Errors
	/// 
	/// This function will return [`None`] if the number of nanoseconds is
	/// greater than [`MAX_NANOSECONDS_FULL`](DurationExt::MAX_NANOSECONDS_FULL)
	/// or less than [`MIN_NANOSECONDS_FULL`](DurationExt::MIN_NANOSECONDS_FULL).
	/// This is not quite the same as Chrono's behaviour, which panics under
	/// similar conditions, but panics are undesirable in library code and hence
	/// this deviation seems justifiable.
	/// 
	fn nanoseconds_full(nanoseconds: i128) -> Option<Self> where Self: Sized;
	
	//		microseconds_full												
	/// Make a new [`Duration`] with the given number of microseconds.
	/// 
	/// This function is necessary because although the [`Duration`] struct
	/// stores its value internally as seconds and microseconds, the
	/// [`microseconds()`](Duration::microseconds()) method only accepts the
	/// number of microseconds as an [`i64`], which is not large enough to
	/// express the full range of microseconds that can be stored by a
	/// [`Duration`] instance. This function therefore accepts the number of
	/// microseconds as an [`i128`], which is large enough to express the full
	/// range of microseconds that can be stored, and creates a [`Duration`]
	/// instance appropriately.
	/// 
	/// # Errors
	/// 
	/// This function will return [`None`] if the number of microseconds is
	/// greater than [`MAX_MICROSECONDS_FULL`](DurationExt::MAX_MICROSECONDS_FULL)
	/// or less than [`MIN_MICROSECONDS_FULL`](DurationExt::MAX_MICROSECONDS_FULL).
	/// This is not quite the same as Chrono's behaviour, which panics under
	/// similar conditions, but panics are undesirable in library code and hence
	/// this deviation seems justifiable.
	/// 
	fn microseconds_full(microseconds: i128) -> Option<Self> where Self: Sized;
	
	//		num_nanoseconds_full												
	/// Returns the total number of nanoseconds in the [`Duration`] instance.
	/// 
	/// This function is necessary because although the [`Duration`] struct
	/// stores its value internally as seconds and nanoseconds, the
	/// [`num_nanoseconds()`](Duration::num_nanoseconds()) method returns the
	/// nanoseconds as an [`i64`], which is not large enough to express the full
	/// range of nanoseconds that can be stored by a [`Duration`] instance. This
	/// function therefore returns the nanoseconds as an [`i128`], which is
	/// large enough to express the full range of nanoseconds that can be
	/// stored.
	/// 
	fn num_nanoseconds_full(&self) -> i128;
	
	//		num_microseconds_full												
	/// Returns the total number of microseconds in the [`Duration`] instance.
	/// 
	/// This function is necessary because although the [`Duration`] struct
	/// stores its value internally as seconds and nanoseconds, the
	/// [`num_microseconds()`](Duration::num_microseconds()) method returns the
	/// microseconds as an [`i64`], which is not large enough to express the
	/// full range of microseconds that can be stored by a [`Duration`]
	/// instance. This function therefore returns the microseconds as an
	/// [`i128`], which is large enough to express the full range of
	/// microseconds that can be stored.
	/// 
	fn num_microseconds_full(&self) -> i128;
}

//󰭅		Duration																
impl DurationExt for Duration {
	//		humanize															
	fn humanize(&self) -> String {
		let seconds = self.num_seconds();
		for &(unit, name) in &Self::UNITS {
			if seconds >= unit {
				#[expect(clippy::arithmetic_side_effects, reason = "Precision is not needed here, and unit cannot be zero")]
				#[expect(clippy::integer_division,        reason = "Precision is not needed here")]
				let count = seconds / unit;
				return format!("{} {}{}", count, name, if count == 1 { "" } else { "s" });
			}
		}
		s!("0 seconds")
	}
	
	//		nanoseconds_full												
	fn nanoseconds_full(nanoseconds: i128) -> Option<Self> {
		if !(Self::MIN_NANOSECONDS_FULL..=Self::MAX_NANOSECONDS_FULL).contains(&nanoseconds) {
			return None;
		}
		#[expect(clippy::cast_possible_truncation, reason = "Range is controlled")]
		if (i128::from(Self::MIN_NANOSECONDS)..=i128::from(Self::MAX_NANOSECONDS)).contains(&nanoseconds) {
			Some(Self::nanoseconds(nanoseconds as i64))
		} else if nanoseconds < 0 {
			Self::try_seconds(     nanoseconds.abs().div_euclid(1_000_000_000_i128).neg() as i64)?.checked_sub(
				&Self::nanoseconds(nanoseconds.abs().rem_euclid(1_000_000_000_i128)       as i64)
			)
		} else {
			Self::try_seconds(     nanoseconds.div_euclid(1_000_000_000_i128) as i64)?.checked_add(
				&Self::nanoseconds(nanoseconds.rem_euclid(1_000_000_000_i128) as i64)
			)
		}
	}
	
	//		microseconds_full												
	fn microseconds_full(microseconds: i128) -> Option<Self> {
		if !(Self::MIN_MICROSECONDS_FULL..=Self::MAX_MICROSECONDS_FULL).contains(&microseconds) {
			return None;
		}
		#[expect(clippy::cast_possible_truncation, reason = "Range is controlled")]
		if (i128::from(Self::MIN_MICROSECONDS)..=i128::from(Self::MAX_MICROSECONDS)).contains(&microseconds) {
			Some(Self::microseconds(microseconds as i64))
		} else if microseconds < 0 {
			Self::try_seconds(      microseconds.abs().div_euclid(1_000_000_i128).neg() as i64)?.checked_sub(
				&Self::microseconds(microseconds.abs().rem_euclid(1_000_000_i128)       as i64)
			)
		} else {
			Self::try_seconds(      microseconds.div_euclid(1_000_000_i128) as i64)?.checked_add(
				&Self::microseconds(microseconds.rem_euclid(1_000_000_i128) as i64)
			)
		}
	}
	
	//		num_nanoseconds_full												
	fn num_nanoseconds_full(&self) -> i128 {
		//	This will actually never saturate, as Chrono uses i64 internally
		i128::from(self.num_seconds())
			.saturating_mul(1_000_000_000)
			.saturating_add(i128::from(self.subsec_nanos()))
	}
	
	//		num_microseconds_full												
	fn num_microseconds_full(&self) -> i128 {
		//	This will actually never saturate, as Chrono uses i64 internally
		i128::from(self.num_seconds())
			.saturating_mul(1_000_000)
			.saturating_add(i128::from(self.subsec_nanos()).saturating_div(1_000))
	}
}

//§		MonthsExt																
/// This trait provides additional functionality to [`Months`].
pub trait MonthsExt {
	/// The maximum number of months that can be represented by a [`Months`].
	const MAX_MONTHS: u32 = u32::MAX;
	
	/// The maximum number of years that can be represented by a [`Months`].
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	const MAX_YEARS:  u32 = u32::MAX / 12;
	
	//		months																
	/// Make a new [`Months`] with the given number of months.
	/// 
	/// This is a convenience function, to make [`Months`] construction fit more
	/// closely with the constructors available for [`Duration`].
	/// 
	/// It is actually a synonym for [`Months::new()`], but is provided for
	/// consistency with the general style of constructor functions available.
	/// 
	fn months(months: u32) -> Self;
	
	//		years																
	/// Make a new [`Months`] with the given number of years.
	/// 
	/// This is a convenience function, to make [`Months`] construction fit more
	/// closely with the constructors available for [`Duration`].
	/// 
	/// # Errors
	/// 
	/// This function will return [`None`] if the number of years is greater
	/// than [`MAX_YEARS`](MonthsExt::MAX_YEARS). This is not quite the same as
	/// Chrono's behaviour, which tends to panic under similar conditions, but
	/// panics are undesirable in library code and hence this deviation seems
	/// justifiable.
	/// 
	fn years(years: u32) -> Option<Self> where Self: Sized;
	
	//		num_months															
	/// Returns the total number of months in the [`Months`] instance.
	/// 
	/// This is a convenience function, to make the behaviour of [`Months`] fit
	/// more closely with the behaviour of [`Duration`].
	/// 
	fn num_months(&self) -> u32;
	
	//		num_years															
	/// Returns the total number of whole years in the [`Months`] instance.
	/// 
	/// This is a convenience function, to make the behaviour of [`Months`] fit
	/// more closely with the behaviour of [`Duration`].
	/// 
	fn num_years(&self) -> u32;
}

//󰭅		Months																	
impl MonthsExt for Months {
	//		months																
	fn months(months: u32) -> Self {
		Self::new(months)
	}
	
	//		years																
	fn years(years: u32) -> Option<Self> {
		if years > Self::MAX_YEARS {
			return None;
		}
		Some(Self::new(years.checked_mul(12)?))
	}
	
	//		num_months															
	fn num_months(&self) -> u32 {
		self.as_u32()
	}
	
	//		num_years															
	#[expect(clippy::integer_division, reason = "Precision is not needed here")]
	fn num_years(&self) -> u32 {
		self.as_u32() / 12
	}
}

//§		NaiveDateExt															
/// This trait provides additional functionality to [`NaiveDate`].
pub trait NaiveDateExt {
	/// The maximum year that can be represented by a [`NaiveDate`].
	const MAX_YEAR: i32 = (i32::MAX >> 13_i32) - 1_i32;
	
	/// The minimum year that can be represented by a [`NaiveDate`].
	const MIN_YEAR: i32 = (i32::MIN >> 13_i32) + 1_i32;
	
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

//󰭅		NaiveDate																
impl NaiveDateExt for NaiveDate {
	//		today																
	fn today() -> Self {
		Utc::now().date_naive()
	}
	
	//		days_in_month														
	fn days_in_month(&self) -> u32 {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::days_in_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		days_in_month_opt													
	fn days_in_month_opt(year: i32, month: u32) -> Option<u32> {
		Self::end_of_month_opt(year, month).map(|date| date.day())
	}
	
	//		days_in_year														
	fn days_in_year(&self) -> u32 {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::days_in_year_opt(self.year()).unwrap()
	}
	
	//		days_in_year_opt													
	fn days_in_year_opt(year: i32) -> Option<u32> {
		Self::from_ymd_opt(year, 2, 1).map(|_| if Self::from_ymd_opt(year, 2, 29).is_some() { 366 } else { 365 })
	}
	
	//		is_leap_year														
	fn is_leap_year(&self) -> bool {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::is_leap_year_opt(self.year()).unwrap()
	}
	
	//		is_leap_year_opt													
	fn is_leap_year_opt(year: i32) -> Option<bool> {
		Self::from_ymd_opt(year, 2, 1).map(|_| Self::from_ymd_opt(year, 2, 29).is_some())
	}
	
	//		start_of_month														
	fn start_of_month(&self) -> Self {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::start_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		start_of_month_opt													
	fn start_of_month_opt(year: i32, month: u32) -> Option<Self> {
		Self::from_ymd_opt(year, month, 1)
	}
	
	//		end_of_month														
	fn end_of_month(&self) -> Self {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::end_of_month_opt(self.year(), self.month()).unwrap()
	}
	
	//		end_of_month_opt													
	fn end_of_month_opt(year: i32, month: u32) -> Option<Self> {
		_ = Self::from_ymd_opt(year, month, 1)?;
		//	The range of years is controlled by having already validated the date
		//	by attempting to create it above. This is well within the range of a u32.
		//	The same applies to the month.
		#[expect(clippy::arithmetic_side_effects, reason = "Range is controlled")]
		Self::from_ymd_opt(
			if month == 12 { year + 1 } else { year      },
			if month == 12 { month    } else { month + 1 },
			1,
		)?.pred_opt()
	}
	
	//		start_of_year														
	fn start_of_year(&self) -> Self {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::start_of_year_opt(self.year()).unwrap()
	}
	
	//		start_of_year_opt													
	fn start_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 1, 1)
	}
	
	//		end_of_year															
	fn end_of_year(&self) -> Self {
		#[expect(clippy::unwrap_used, reason = "Infallible")]
		Self::end_of_year_opt(self.year()).unwrap()
	}
	
	//		end_of_year_opt														
	fn end_of_year_opt(year: i32) -> Option<Self> {
		Self::from_ymd_opt(year, 12, 31)
	}
}


