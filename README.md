# Rubedo

![Rust](https://img.shields.io/badge/Rust-1.81%2B-b7410e?style=flat&logo=rust&logoColor=white&labelColor=b7410e)
[![Crate version](https://img.shields.io/crates/v/rubedo?style=flat)](https://crates.io/crates/rubedo)
[![CI](https://img.shields.io/github/actions/workflow/status/danwilliams/rubedo/ci.yml?style=flat&logo=github&logoColor=white&label=build%2Ftest)](https://github.com/danwilliams/rubedo/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/docsrs/rubedo?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/crate/rubedo/latest)
![License](https://img.shields.io/github/license/danwilliams/rubedo?style=flat)

Rubedo is a set of library crates that provide useful utilities, helpers, and
other functionality for Rust projects.

The main crate is [`rubedo`](crates/rubedo/README.md), and the other crates
rely on it or provide secondary functionality in some way. This repository as
a whole shares a name with the main crate. You can read about the naming and
purpose of each crate in their respective README files.


## Crates

  - [`rubedo`](crates/rubedo/README.md) - The main crate, providing extensions
    to the standard library and other popular crates, as well as some new
    functionality.

  - [`rubedo-macros`](crates/rubedo-macros/README.md) - Proc macros for the
    Rubedo crate. There is generally no need to use this directly, as its
    functionality is re-exported by the main Rubedo crate.


