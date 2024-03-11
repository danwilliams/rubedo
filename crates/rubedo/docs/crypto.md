# crypto

The [`crypto`](https://docs.rs/rubedo/latest/rubedo/sha2/index.html) module
provides extensions to the [Rust Crypto](https://github.com/RustCrypto) set of
crates.

  - [`Hashed`](#hashed)
  - [`Sha256Hash`](#sha256hash)
  - [`Sha512Hash`](#sha512hash)
  - [`SigningKey`](#signingkey)
  - [`SigningKeyExt`](#signingkeyext)
  - [`VerifyingKey`](#verifyingkey)
  - [`VerifyingKeyExt`](#verifyingkeyext)


## Hashed

The [`Hashed`](https://docs.rs/rubedo/latest/rubedo/crypto/trait.Hashed.html)
trait provides a formal representation of actual hash values.


## Sha256Hash

The [`Sha256Hash`](https://docs.rs/rubedo/latest/rubedo/crypto/struct.Sha256Hash.html)
struct is provided to formalise the handling of SHA256 hashes. It converts to
and from common formats, including serialisation and deserialisation, which
default to hexadecimal strings, although base64 is also supported.


## Sha512Hash

The [`Sha512Hash`](https://docs.rs/rubedo/latest/rubedo/crypto/struct.Sha512Hash.html)
struct is provided to formalise the handling of SHA512 hashes. It converts to
and from common formats, including serialisation and deserialisation, which
default to hexadecimal strings, although base64 is also supported.


## SigningKey

The [`SigningKey`](https://docs.rs/rubedo/latest/rubedo/crypto/struct.SigningKey.html)
struct is a wrapper type provided to formalise the handling of [ed25519-dalek](https://crates.io/crates/ed25519-dalek)
signing keys. It converts to and from common formats, including serialisation
and deserialisation, which default to hexadecimal strings, although base64 is
also supported.


## SigningKeyExt

The [`SigningKeyExt`](https://docs.rs/rubedo/latest/rubedo/crypto/trait.SigningKeyExt.html)
trait is provided to enhance the [`ed25519-dalek::SigningKey`](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/struct.SigningKey.html)
struct when used directly.


## VerifyingKey

The [`VerifyingKey`](https://docs.rs/rubedo/latest/rubedo/crypto/struct.VerifyingKey.html)
struct is a wrapper type provided to formalise the handling of [ed25519-dalek](https://crates.io/crates/ed25519-dalek)
verifying keys. It converts to and from common formats, including
serialisation and deserialisation, which default to hexadecimal strings,
although base64 is also supported.


## VerifyingKeyExt

The [`VerifyingKeyExt`](https://docs.rs/rubedo/latest/rubedo/crypto/trait.VerifyingKeyExt.html)
trait is provided to enhance the [`ed25519-dalek::VerifyingKey`](https://docs.rs/ed25519-dalek/latest/ed25519_dalek/struct.VerifyingKey.html)
struct when used directly.


