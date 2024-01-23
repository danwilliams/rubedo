# Changelog

[Axum]:                https://crates.io/crates/axum
[Chrono]:              https://crates.io/crates/chrono
[Hyper]:               https://crates.io/crates/hyper
[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


## 0.4.3 (23 January 2024)

### Added

  - Extended the `chrono` module
      - Added `MonthsExt` trait
          - Added `Months::MAX_MONTHS`
          - Added `Months::MAX_YEARS`
          - Added `Months.num_months()`
          - Added `Months.num_years()`
      - Added `MAX_*` and `MIN_*` constants to `DurationExt`
          - Added `Duration::MAX_NANOSECONDS`
          - Added `Duration::MAX_MICROSECONDS`
          - Added `Duration::MAX_MILLISECONDS`
          - Added `Duration::MAX_SECONDS`
          - Added `Duration::MAX_MINUTES`
          - Added `Duration::MAX_HOURS`
          - Added `Duration::MAX_DAYS`
          - Added `Duration::MAX_WEEKS`
          - Added `Duration::MIN_NANOSECONDS`
          - Added `Duration::MIN_MICROSECONDS`
          - Added `Duration::MIN_MILLISECONDS`
          - Added `Duration::MIN_SECONDS`
          - Added `Duration::MIN_MINUTES`
          - Added `Duration::MIN_HOURS`
          - Added `Duration::MIN_DAYS`
          - Added `Duration::MIN_WEEKS`
      - Added constants and methods to `DurationExt` for full range
          - Added `Duration.num_nanoseconds_full()`
          - Added `Duration.num_microseconds_full()`
          - Added `Duration::MAX_NANOSECONDS_FULL`
          - Added `Duration::MAX_MICROSECONDS_FULL`
          - Added `Duration::MIN_NANOSECONDS_FULL`
          - Added `Duration::MIN_MICROSECONDS_FULL`

### Changed

  - Updated `NaiveDateExt::MAX_YEAR` and `MIN_YEAR` for Chrono 0.4.32
  - Updated crate dependencies


## 0.4.2 (09 December 2023)

### Added

  - Enhanced `variants!()` macro to type-hint empty lists


## 0.4.1 (08 December 2023)

### Fixed

  - Fixed wrong links in README documentation

### Changed

  - Restructured README documentation


## 0.4.0 (07 December 2023)

### Added

  - Added `serde` module to provide useful conversions when (de)serialising
      - Added general type conversion functions
          - Added `as_str()`
          - Added `from()`
          - Added `from_str()`
          - Added `from_string()`
          - Added `into()`
          - Added `into_string()`
          - Added `to_string()`
          - Added `try_from()`
          - Added `try_from_string()`
      - Added helpers for scaling integers to/from floating-point numbers
          - Added `from_cents()`
          - Added `from_pence()`
          - Added `to_cents()`
          - Added `to_pence()`
          - Added `try_from_int_1dp()`
          - Added `try_from_int_2dp()`
          - Added `try_from_int_3dp()`
          - Added `try_from_int_4dp()`
          - Added `try_from_int_with_scale()`
          - Added `try_to_int_1dp()`
          - Added `try_to_int_2dp()`
          - Added `try_to_int_3dp()`
          - Added `try_to_int_4dp()`
          - Added `try_to_int_with_scale()`
  - Extended the `chrono` module
      - Added `MAX_YEAR` and `MIN_YEAR` constants to `NaiveDateExt`
  - Extended the `std` module
      - Added `AsStr` trait
      - Added `FromIntWithScale` trait
      - Added `ToIntWithScale` trait
      - Added implementations of `FromIntWithScale::from_int_with_scale()` and
        `ToIntWithScale::to_int_with_scale()` to/from `f32`/`f64`/`Decimal` and
        all integer types
  - Extended the `sugar` module
      - Added macros for enum variant shorthand
          - Added `variants!()`
          - Added `variants_hashset!()`
          - Added `vv!()` alias
          - Added `vh!()` alias
      - Re-exported `rubedo_macros::ip` as `rubedo::sugar::ip`
  - Added some integration tests
  - Added standard linting configuration

### Fixed

  - Improved `sugar::s!()` macro to detect string literals

### Changed

  - In the `http` module
      - Improved `ResponseError`, allowing any errors raised by `to_bytes()` to
        be captured
      - Amended `convert_response()` to deal with a borrowed body rather than
        taking ownership
      - Derived `Eq` for `ContentType`
      - Made `UnpackedResponseBody.content_type()` const
  - In the `std` module
      - Improved handling of `cwd` in `PathExt::normalize()`, to fall back to
        the filesystem root directory
      - Derived `Clone` and `Debug` for `LimitIterator`
  - General changes
      - Annotated non-exhaustive enums and structs as such
      - Applied `must_use` annotation to appropriate functions
      - Used `core` instead of `std` where possible
      - Refactored some functions
      - Improved documentation

### Removed

  - Removed support for using the `sugar::s!()` macro with non-string literals


## 0.3.6 (17 October 2023)

### Added

  - Added `std::Iterator.limit()`


## 0.3.5 (12 October 2023)

### Added

  - Implemented `From<HyperBody>` and `From<UnsyncBoxBody<Bytes, E>>` for
    `http::UnpackedResponseBody`

### Changed

  - Improved Rustdoc documentation


## 0.3.4 (07 October 2023)

### Added

  - Implemented `From<Json>` for `http::UnpackedResponseBody`
  - Implemented custom `Serialize` and `Deserialize` implementations for
    `http::UnpackedResponseBody`
  - Added `http::ContentType` enum to specify `http:UnpackedResponseBody`
    content type
  - Added `content_type()`, `set_content_type()`, `is_binary()`, `is_text()`,
    `to_base64()`, and `from_base64()` to `http::UnpackedResponseBody`

### Changed

  - Updated `Debug`, `Display`, `Serialize`, and `Deserialize` implementations
    for `http::UnpackedResponseBody` to be content-type aware
  - Updated `http::UnpackedResponseBody::new()` to accept any types that `From`
    has been implemented for
  - Changed `http::UnpackedResponseBody` from tuple struct to standard struct
  - Improved performance in `http::UnpackedResponseBody::from()`


## 0.3.3 (06 October 2023)

### Added

  - Added `push_char()` to `http::UnpackedResponseBody`
  - Implemented additional `Add` and `AddAssign` functionality for
    `http::UnpackedResponseBody`: `char`, `&char`, `String`, `&String`,
    `Box<str>`, `Cow<'a, str>`, `u8`, `Vec<u8>`, `&Vec<u8>`,
    `UnpackedResponseBody`, `&UnpackedResponseBody`
  - Implemented additional `From` conversions for `http::UnpackedResponseBody`:
    `&[u8]`, `&[u8; N]`, `&char`, `Vec<u8>`, `&Vec<u8>`

### Changed

  - Improved performance in `chrono::DurationExt.humanize()`
  - Improved performance in `http::UnpackedResponseBody::from<&String>()`


## 0.3.2 (05 October 2023)

### Added

  - Added `new()`, `clear()`, `empty()`, `is_empty()`, `len()`, `push()`,
    `push_bytes()`, `push_str()`, `as_bytes()`, `as_mut_bytes()`,
    `into_bytes()`, and `to_bytes()` to `http::UnpackedResponseBody`
  - Implemented `Add` and `AddAssign` functionality for
    `http::UnpackedResponseBody`: `&[u8]`, `&[u8; N]`, and `&str`
  - Implemented `AsRef`, `AsMut`, `Clone`, `Default`, and `Write` for
    `http::UnpackedResponseBody`
  - Implemented a range of `From` conversions for `http::UnpackedResponseBody`:
    `&str`, `&mut str`, `String`, `&String`, `Box<str>`, `Cow<'a, str>`, `char`,
    and `u8`


## 0.3.1 (04 October 2023)

### Added

  - Implemented `Serialize` and `Deserialize` for `http::UnpackedResponse`

### Changed

  - Changed `http::UnpackedResponse.body` to use a new `UnpackedResponseBody`
    struct


## 0.3.0 (02 October 2023)

### Added

  - Extended `http::ResponseExt` to cover [Axum][]

### Fixed

  - Made `http::UnpackedResponse` fields public

### Changed

  - Renamed `hyper` module to `http`


## 0.2.0 (01 October 2023)

### Added

  - Added `hyper` module to enhance [Hyper][] library
      - Added `hyper::Response.unpack()`


## 0.1.1 (29 September 2023)

### Changed

  - Improved README documentation
  - Improved Rustdoc documentation


## 0.1.0 (27 September 2023)

### Added

  - Added `std` module to enhance Rust std lib
      - Added `std::Path.append()`
      - Added `std::Path.is_subjective()`
      - Added `std::Path.normalize()`
      - Added `std::Path.restrict()`
      - Added `std::Path.strip_parentdirs()`
      - Added `std::Path.strip_root()`
  - Added `chrono` module to enhance [Chrono][] library
      - Added `chrono::Duration.humanize()`
      - Added `chrono::NaiveDate::today()`
      - Added `chrono::NaiveDate::days_in_month_opt()`
      - Added `chrono::NaiveDate::days_in_year_opt()`
      - Added `chrono::NaiveDate::is_leap_year_opt()`
      - Added `chrono::NaiveDate::start_of_month_opt()`
      - Added `chrono::NaiveDate::end_of_month_opt()`
      - Added `chrono::NaiveDate::start_of_year_opt()`
      - Added `chrono::NaiveDate::end_of_year_opt()`
      - Added `chrono::NaiveDate.days_in_month()`
      - Added `chrono::NaiveDate.days_in_year_opt()`
      - Added `chrono::NaiveDate.is_leap_year()`
      - Added `chrono::NaiveDate.start_of_month()`
      - Added `chrono::NaiveDate.end_of_month_opt()`
      - Added `chrono::NaiveDate.start_of_year()`
      - Added `chrono::NaiveDate.end_of_year_opt()`
  - Added macros for creation sugar
      - Added `ip!()` macro for IpAddr creation sugar
      - Added `s!()` macro for String creation sugar
  - Added `trybuild` to catch proc_macro test failures
  - Added README documentation


