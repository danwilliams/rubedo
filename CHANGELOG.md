# Changelog

[Keep a Changelog]:    https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[Tera]:                https://crates.io/crates/tera

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog][], and this project adheres to
[Semantic Versioning][].


## 0.1.0 (27 September 2023)

### Added

  - Added std module to enhance Rust std lib
      - Added Path.append()
      - Added Path.is_subjective()
      - Added Path.normalize()
      - Added Path.restrict()
      - Added Path.strip_parentdirs()
      - Added Path.strip_root()
  - Added chrono module to enhance Chrono library
      - Added chrono::Duration.humanize()
      - Added chrono::NaiveDate.today()
      - Added chrono::NaiveDate.days_in_month()
      - Added chrono::NaiveDate.days_in_month_opt()
      - Added chrono::NaiveDate.days_in_year_opt()
      - Added chrono::NaiveDate.days_in_year_opt()
      - Added chrono::NaiveDate.is_leap_year()
      - Added chrono::NaiveDate.is_leap_year_opt()
      - Added chrono::NaiveDate.start_of_month()
      - Added chrono::NaiveDate.start_of_month_opt()
      - Added chrono::NaiveDate.end_of_month_opt()
      - Added chrono::NaiveDate.end_of_month_opt()
      - Added chrono::NaiveDate.start_of_year()
      - Added chrono::NaiveDate.start_of_year_opt()
      - Added chrono::NaiveDate.end_of_year_opt()
      - Added chrono::NaiveDate.end_of_year_opt()
  - Added macros for creation sugar
      - Added ip!() macro for IpAddr creation sugar
      - Added s!() macro for String creation sugar
  - Added trybuild to catch proc_macro test failures
  - Added README documentation


