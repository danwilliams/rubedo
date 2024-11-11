# Rubedo

![Rust](https://img.shields.io/badge/Rust-1.81%2B-b7410e?style=flat&logo=rust&logoColor=white&labelColor=b7410e)
[![Crate version](https://img.shields.io/crates/v/rubedo?style=flat)](https://crates.io/crates/rubedo)
[![CI](https://img.shields.io/github/actions/workflow/status/danwilliams/rubedo/ci.yml?style=flat&logo=github&logoColor=white&label=build%2Ftest)](https://github.com/danwilliams/rubedo/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/docsrs/rubedo?style=flat&logo=docs.rs&logoColor=white)](https://docs.rs/crate/rubedo/latest)
![License](https://img.shields.io/github/license/danwilliams/rubedo?style=flat)

The Rubedo crate is a library of useful functionality, some being extensions of
standard library entities; some extensions of other, popular crates; and some
being completely new functionality.

It is named after the alchemical process of turning base metals into gold. The
word "rubedo" is Latin for "redness", and is the final stage of the alchemical
process. It is also the name of the final stage of the
[Magnum Opus](https://en.wikipedia.org/wiki/Magnum_opus_(alchemy)). The Magnum
Opus is the process of creating the philosopher's stone, which is supposed to be
able to turn base metals into gold. The philosopher's stone is also said to be
able to grant immortality, and is the main goal of alchemy. The philosopher's
stone is also known as the "red stone" or the "red powder". The reasons for the
choice of name are firstly that the "redness" is tangentially related to Rust
through colour, and secondly that this crate will aspirationally help to turn
your Rust code into gold... well, maybe... or at least make it easier to write.

The modules provided are:

  - [std](docs/std.md)
  - [chrono](docs/chrono.md)
  - [crypto](docs/crypto.md)
  - [http](docs/http.md)
  - [serde](docs/serde.md)
  - [sugar](docs/sugar.md)

Note, each module is behind a feature flag, in order to keep the crate size down
for those who don't need all the functionality.

At present the default is for all features to be enabled, for
backwards-compatibility. This will change to a smaller set of default features
in the next minor version. The defaults can be overridden in your `Cargo.toml`
file by setting the `default-features` key to `false` and then enabling the
features you want.


