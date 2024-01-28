# chrono

[`Duration`]: https://docs.rs/chrono/latest/chrono/struct.Duration.html
[`Months`]:   https://docs.rs/chrono/latest/chrono/struct.Months.html

The [`chrono`](https://docs.rs/rubedo/latest/rubedo/chrono/index.html) module
provides extensions to the [Chrono](https://crates.io/crates/chrono) crate.

  - [`Duration`](#duration)
  - [`Months`](#months)
  - [`NaiveDate`](#naivedate)


## Duration

The [`Duration`][] struct is extended with the following methods:

  - [`humanize()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.humanize) -
    Returns a human-readable string representation of the duration, as an
    expression of the largest unit available.

  - [`microseconds_full()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.microseconds_full) -
    Make a new [`Duration`][] with the given number of microseconds.

  - [`nanoseconds_full()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.nanoseconds_full) -
    Make a new [`Duration`][] with the given number of nanoseconds.

  - [`num_microseconds_full()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.num_microseconds_full) -
    Returns the total number of microseconds in the [`Duration`][] instance.

  - [`num_nanoseconds_full()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.num_nanoseconds_full) -
    Returns the total number of nanoseconds in the [`Duration`][] instance.


## Months

The [`Months`][] struct is extended with the following methods:

  - [`months()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.MonthsExt.html#tymethod.months) -
    Make a new [`Months`][] with the given number of months.

  - [`num_months()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.num_months) -
    Returns the total number of months in the [`Months`][] instance.

  - [`num_years()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.num_years) -
    Returns the total number of whole years in the [`Months`][] instance.

  - [`years()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.MonthsExt.html#tymethod.years) -
    Make a new [`Months`][] with the given number of years.


## NaiveDate

The [`NaiveDate`](https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html)
struct is extended with the following methods:

  - [`today()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.today) -
    Returns the current date.

  - [`days_in_month()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.days_in_month) -
    Returns the number of days in the date's month.

  - [`days_in_month_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.days_in_month_opt) -
    Returns the number of days in the given month.

  - [`days_in_year()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.days_in_year) -
    Returns the number of days in the date's year.

  - [`days_in_year_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.days_in_year_opt) -
    Returns the number of days in the given year.

  - [`is_leap_year()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.is_leap_year) -
    Returns `true` if the date's year is a leap year.

  - [`is_leap_year_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.is_leap_year_opt) -
    Returns `true` if the given year is a leap year.

  - [`start_of_month()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.start_of_month) -
    Returns the date of the first day of the date's month.

  - [`start_of_month_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.start_of_month_opt) -
    Returns the date of the first day of the given month.

  - [`end_of_month()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.end_of_month) -
    Returns the date of the last day of the date's month.

  - [`end_of_month_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.end_of_month_opt) -
    Returns the date of the last day of the given month.

  - [`start_of_year()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.start_of_year) -
    Returns the date of the first day of the date's year.

  - [`start_of_year_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.start_of_year_opt) -
    Returns the date of the first day of the given year.

  - [`end_of_year()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.end_of_year) -
    Returns the date of the last day of the date's year.

  - [`end_of_year_opt()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.NaiveDateExt.html#tymethod.end_of_year_opt) -
    Returns the date of the last day of the given year.


