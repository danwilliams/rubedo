# Rubedo macros

![Rust](https://img.shields.io/badge/Rust-1.81%2B-b7410e?style=flat&logo=rust&logoColor=white&labelColor=b7410e)
[![Crate version](https://img.shields.io/crates/v/rubedo-macros?style=flat)](https://crates.io/crates/rubedo-macros)
[![CI](https://img.shields.io/github/actions/workflow/status/danwilliams/rubedo/ci.yml?style=flat&logo=github&logoColor=white&label=build%2Ftest)](https://github.com/danwilliams/rubedo/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/docsrs/rubedo-macros?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/crate/rubedo-macros/latest)
![License](https://img.shields.io/github/license/danwilliams/rubedo?style=flat)

This crate provides proc macros for the main [`rubedo`](../rubedo/README.md)
crate. It is not intended to be used directly, but rather to be used with the
Rubedo crate, which re-exports its functionality.

It exists because it is not currently possible to have proc macros in the same
crate as the code that uses them. This is because the compiler needs to compile
the proc macros before it can use them, but it can't compile them until it has
compiled the code that uses them. This is a chicken-and-egg problem, and the
solution is to put the proc macros in a separate crate.

## Features

### Macros

The macros are provided to provide syntactic sugar for common operations.

  - [`ip!`](https://docs.rs/rubedo-macros/latest/rubedo_macros/macro.ip.html)
    Builds an IP address from a range of input types.


