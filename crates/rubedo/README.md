# Rubedo

The Rubedo crate is a library of useful functionality, some being extensions of
standard library entities; some extensions of other, popular crates; and some
being completely new functionality.

It is named after the alchemical process of turning base metals into gold. The
word "rubedo" is Latin for "redness", and is the final stage of the alchemical
process. It is also the name of the final stage of the
[Magnum Opus](https://en.wikipedia.org/wiki/Magnum_opus_(alchemy)). The Magnum
Opus is the process of creating the philosopher's stone, which is supposed to be
able to turn base metals into gold. The philosopher's stone is also said to be
able to grant immortality, and is the main goal of alchemy. The philosopher's
stone is also known as the "red stone" or the "red powder". The reasons for the
choice of name are firstly that the "redness" is tangentially related to Rust
through colour, and secondly that this crate will aspirationally help to turn
your Rust code into gold... well, maybe... or at least make it easier to write.

## Features

### std

The `std` module provides extensions to the Rust standard library.

#### Path

The [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) struct is
extended with the following methods:

  - [`append()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.append) -
    Adds a string to the end of a path, and returns the result as a new path,
    without creating a new path component.

  - [`is_subjective()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.is_subjective) -
    Checks if the path is specifically relative to the current directory, i.e.
    starts with a reference to the current directory, which can be `.` or `..`.

  - [`normalize()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.normalize) -
    Computes the canonicalized, absolute path of a file or directory, removing
    any `.` and `..` segments and returning the "real" path, without expanding
    symlinks or checking existence.

  - [`restrict()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.restrict) -
    Restricts the path to a given base path, normalising the path and not
    allowing it to go beyond the base path.

  - [`strip_parentdirs()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.strip_parentdirs) -
    Removes references to parent directories, i.e. `..`.

  - [`strip_root()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.strip_root) -
    Makes the path relative by removing the root and/or prefix components.

### chrono

The `chrono` module provides extensions to the [Chrono](https://crates.io/crates/chrono)
crate.

#### Duration

The [`Duration`](https://docs.rs/chrono/latest/chrono/struct.Duration.html)
struct is extended with the following methods:

  - [`humanize()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.DurationExt.html#tymethod.humanize) -
    Returns a human-readable string representation of the duration, as an
    expression of the largest unit available.

#### NaiveDate

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

### http

The `http` module provides extensions to the [HTTP](https://crates.io/crates/http),
[Hyper](https://crates.io/crates/hyper), and [Axum](https://crates.io/crates/axum)
crates.

#### Response

The [`Response`](https://docs.rs/http/latest/http/response/struct.Response.html)
struct is extended with the following methods:

  - [`unpack()`](https://docs.rs/rubedo/latest/rubedo/chrono/trait.ResponseExt.html#tymethod.unpack) -
    Unpacks the response and provides the headers and body in a more accessible
    form, to allow it to be checked, compared, and printed easily.

### Macros

Some macros are provided to provide syntactic sugar for common operations.

  - [`ip!`](https://docs.rs/rubedo-macros/latest/rubedo_macros/macro.ip.html)
    Builds an IP address from a range of input types. Note that this macro is
    provided by the [`rubedo-macros`](https://crates.io/crates/rubedo-macros)
    crate.

  - [`s!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.s.html)
    Converts a `str` string literal to an owned `String`, saving having to do
    `"foo".to_owned()` or `String::from("foo")`. It will also convert any other
    type that implements the [`ToString`] trait to a `String`.

