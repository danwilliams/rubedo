# Rubedo macros

This crate provides proc macros for the main [`rubedo`](../rubedo/README.md)
crate. It is not intended to be used directly, but rather to be used with the
Rubedo crate.

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


