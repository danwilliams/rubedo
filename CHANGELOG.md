# Changelog

[Axum]:                https://crates.io/crates/axum
[Chrono]:              https://crates.io/crates/chrono
[Hyper]:               https://crates.io/crates/hyper
[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


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
    for `http::UnpackedResponseBody` to be content-type aware.

  - Updated `http::UnpackedResponseBody::new()` to accept any types that `From`
    has been implemented for.

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


