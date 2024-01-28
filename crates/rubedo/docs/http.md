# http

The [`http`](https://docs.rs/rubedo/latest/rubedo/http/index.html) module
provides extensions to the [HTTP](https://crates.io/crates/http), [Hyper](https://crates.io/crates/hyper),
and [Axum](https://crates.io/crates/axum) crates.


## Response

The [`Response`](https://docs.rs/http/latest/http/response/struct.Response.html)
struct is extended with the following methods:

  - [`unpack()`](https://docs.rs/rubedo/latest/rubedo/http/trait.ResponseExt.html#tymethod.unpack) -
    Unpacks the response and provides the headers and body in a more accessible
    form, to allow it to be checked, compared, and printed easily.


