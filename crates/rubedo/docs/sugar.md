## sugar

The [`sugar`](https://docs.rs/rubedo/latest/rubedo/sugar/index.html) module
provides macros that add syntactic sugar for common operations.

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


