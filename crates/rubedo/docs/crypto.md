# crypto

The [`crypto`](https://docs.rs/rubedo/latest/rubedo/sha2/index.html) module
provides extensions to the [Rust Crypto](https://github.com/RustCrypto) set of
crates.


## Sha256Hash

The [`Sha256Hash`](https://docs.rs/http/latest/sha2/sha256hash/struct.Sha256Hash.html)
struct is provided to formalise the handling of SHA256 hashes. It converts to
and from common formats, including serialisation and deserialisation, which
default to hexadecimal strings, although base64 is also supported.


## Sha512Hash

The [`Sha512Hash`](https://docs.rs/http/latest/sha2/sha512hash/struct.Sha512Hash.html)
struct is provided to formalise the handling of SHA512 hashes. It converts to
and from common formats, including serialisation and deserialisation, which
default to hexadecimal strings, although base64 is also supported.


