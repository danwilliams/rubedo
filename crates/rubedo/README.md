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

The [`std`](https://docs.rs/rubedo/latest/rubedo/std/index.html) module provides
extensions to the [Rust standard library](https://doc.rust-lang.org/std/).

#### AsStr

The [`AsStr`](https://docs.rs/rubedo/latest/rubedo/std/trait.AsStr.html) trait
is essentially a marker trait, indicating the presence of an `as_str()` method,
primarily for use with the [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html)
module, most usefully with enums:

  - [`as_str()`](https://docs.rs/rubedo/latest/rubedo/std/trait.AsStr.html#tymethod.as_str) -
    Provides a string slice representation of the type.

#### FromIntWithScale and ToIntWithScale

The [`FromIntWithScale`](https://docs.rs/rubedo/latest/rubedo/std/trait.FromIntWithScale.html)
and [`ToIntWithScale`](https://docs.rs/rubedo/latest/rubedo/std/trait.ToIntWithScale.html)
traits are used to convert between integers and floating-point numbers using
specified scales, i.e. different numbers of decimal places, primarily for use
with the [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html)
module, most usefully with currency values:

  - [`from_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/std/trait.FromIntWithScale.html#tymethod.from_int_with_scale) -
    Converts from an integer to a floating-point number with a specified scale.

  - [`to_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/std/trait.ToIntWithScale.html#tymethod.to_int_with_scale) -
    Converts from a floating-point number to an integer with a specified scale.

#### Iterator

The [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait
is extended with (technically, complemented by) the following methods:

  - [`limit()`](https://docs.rs/rubedo/latest/rubedo/std/trait.IteratorExt.html#tymethod.limit) -
    Limits the number of items returned by an iterator, similar to
    [`take()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take),
    but accepts an `Option`.

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

The [`chrono`](https://docs.rs/rubedo/latest/rubedo/chrono/index.html) module
provides extensions to the [Chrono](https://crates.io/crates/chrono) crate.

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

The [`http`](https://docs.rs/rubedo/latest/rubedo/http/index.html) module
provides extensions to the [HTTP](https://crates.io/crates/http), [Hyper](https://crates.io/crates/hyper),
and [Axum](https://crates.io/crates/axum) crates.

#### Response

The [`Response`](https://docs.rs/http/latest/http/response/struct.Response.html)
struct is extended with the following methods:

  - [`unpack()`](https://docs.rs/rubedo/latest/rubedo/http/trait.ResponseExt.html#tymethod.unpack) -
    Unpacks the response and provides the headers and body in a more accessible
    form, to allow it to be checked, compared, and printed easily.

### serde

The [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html) module
provides conversion utility functions for use with [Serde](https://crates.io/crates/serde).

#### Functions

##### Serialisation

  - [`as_str()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.as_str.html) -
    Returns a string representation of a type from a string slice.

  - [`into()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.into.html) -
    Returns a serialised representation of a type.

  - [`into_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.into_string.html) -
    Returns a string representation of a type.

  - [`to_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_string.html) -
    Returns a string copy of a type.

##### Deserialisation

  - [`from()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from.html) -
    Returns a type from a string or other serialised representation.
    
  - [`from_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_string.html) -
    Returns a type from a string representation.

  - [`from_str()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_str.html) -
    Returns a type from a string slice representation.
    
  - [`try_from()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from.html) -
    Returns a type from a string or other serialised representation.

  - [`try_from_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_string.html) -
    Returns a type from a string representation.

##### Decimal helpers

  - [`from_cents()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_cents.html) -
    Converts an integer to a [`Decimal`](https://crates.io/crates/rust_decimal)
    to 2 decimal places.

  - [`from_pence()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_pence.html) -
    Converts an integer to a [`Decimal`](https://crates.io/crates/rust_decimal)
    to 2 decimal places.

  - [`to_cents()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_cents.html) -
    Converts a [`Decimal`](https://crates.io/crates/rust_decimal) to an integer
    to 2 decimal places.

  - [`to_pence()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_pence.html) -
    Converts a [`Decimal`](https://crates.io/crates/rust_decimal) to an integer
    to 2 decimal places.

  - [`try_from_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_with_scale.html) -
    Converts an integer to a floating-point number with scale.

  - [`try_from_int_1dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_1dp.html) -
    Converts an integer to a floating-point number to 1 decimal place.

  - [`try_from_int_2dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_2dp.html) -
    Converts an integer to a floating-point number to 2 decimal places.

  - [`try_from_int_3dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_3dp.html) -
    Converts an integer to a floating-point number to 3 decimal places.

  - [`try_from_int_4dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_4dp.html) -
    Converts an integer to a floating-point number to 4 decimal places.

  - [`try_to_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_with_scale.html) -
    Converts a floating-point number to an integer with scale.

  - [`try_to_int_1dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_1dp.html) -
    Converts a floating-point number to an integer to 1 decimal place.

  - [`try_to_int_2dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_2dp.html) -
    Converts a floating-point number to an integer to 2 decimal places.

  - [`try_to_int_3dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_3dp.html) -
    Converts a floating-point number to an integer to 3 decimal places.

  - [`try_to_int_4dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_4dp.html) -
    Converts a floating-point number to an integer to 4 decimal places.

### Macros

Some macros are provided to provide syntactic sugar for common operations.

  - [`ip!`](https://docs.rs/rubedo-macros/latest/rubedo_macros/macro.ip.html) -
    Builds an IP address from a range of input types. Note that this macro is
    provided by the [`rubedo-macros`](https://crates.io/crates/rubedo-macros)
    crate.

  - [`s!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.s.html) -
    Converts a `str` string literal to an owned `String`, saving having to do
    `"foo".to_owned()` or `String::from("foo")`. It will also convert any other
    type that implements the `ToString` trait to a `String`.

  - [`variants!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.variants.html) -
    Allows shorthand for referring to multiple variants of the same enum,
    producing a `Vec` of the variants.

  - [`variants_hashset!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.variants_hashset.html) -
    Allows shorthand for referring to multiple variants of the same enum,
    producing a `HashSet` of the variants.

  - [`vv!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.vv.html) -
    Abbreviated form of [`variants!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.variants.html).

  - [`vh!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.vh.html) -
    Abbreviated form of [`variants_hashset!`](https://docs.rs/rubedo/latest/rubedo/sugar/macro.variants_hashset.html).


