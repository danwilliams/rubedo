# sha2

The [`sha2`](https://docs.rs/rubedo/latest/rubedo/sha2/index.html) module
provides extensions to the [Sha2](https://crates.io/crates/sha2) crate.


## Sha256Hash

The [`Sha256Hash`](https://docs.rs/http/latest/sha2/sha256hash/struct.Sha256Hash.html)
struct is provided to formalise the handling of SHA256 hashes. It converts to
and from common formats, including serialisation and deserialisation, which
default to hexadecimal strings, although base64 is also supported.


