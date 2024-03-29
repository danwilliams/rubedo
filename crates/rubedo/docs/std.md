# std

The [`std`](https://docs.rs/rubedo/latest/rubedo/std/index.html) module provides
extensions to the [Rust standard library](https://doc.rust-lang.org/std/).

  - [`AsStr`](#asstr)
  - [`ByteSized`, `ByteSizedFull`, and `ByteSizedMut`](#bytesized-bytesizedfull-and-bytesizedmut)
  - [`FileExt` and `AsyncFileExt`](#fileext-and-asyncfileext)
  - [`ForceFrom`](#forcefrom)
  - [`FromIntWithScale` and `ToIntWithScale`](#fromintwithscale-and-tointwithscale)
  - [`Iterator`](#iterator)
  - [`Path`](#path)


## AsStr

The [`AsStr`](https://docs.rs/rubedo/latest/rubedo/std/trait.AsStr.html) trait
is essentially a marker trait, indicating the presence of an `as_str()` method,
primarily for use with the [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html)
module, most usefully with enums:

  - [`as_str()`](https://docs.rs/rubedo/latest/rubedo/std/trait.AsStr.html#tymethod.as_str) -
    Provides a string slice representation of the type.


## ByteSized, ByteSizedFull, and ByteSizedMut

The [`ByteSized`](https://docs.rs/rubedo/latest/rubedo/std/trait.ByteSized.html)
trait formalises the behaviour of fixed-size byte arrays, for the purpose of
storage and conversion, primarily aimed at use with hashes and keys. For those
types that there is control over, [`ByteSizedFull`](https://docs.rs/rubedo/latest/rubedo/std/trait.ByteSizedFull.html)
and [`ByteSizedMut`](https://docs.rs/rubedo/latest/rubedo/std/trait.ByteSizedMut.html)
can be applied as well. For third-party types, inner mutation/consumption is not
possible, and neither is application of external traits due to the orphan rule,
hence the split.


## FileExt and AsyncFileExt

The [`File`](https://doc.rust-lang.org/std/file/struct.File.html) and
[`AsyncFile`](https://docs.rs/tokio/latest/tokio/fs/struct.File.html) structs
are extended with the following methods:

  - [`hash()`](https://docs.rs/rubedo/latest/rubedo/std/trait.FileExt.html#tymethod.hash) -
    Hashes the contents of a file and returns the specified [`Hashed`](https://docs.rs/rubedo/latest/rubedo/crypto/trait.Hashed.html)
    type.


## FromIntWithScale and ToIntWithScale

The [`FromIntWithScale`](https://docs.rs/rubedo/latest/rubedo/std/trait.FromIntWithScale.html)
and [`ToIntWithScale`](https://docs.rs/rubedo/latest/rubedo/std/trait.ToIntWithScale.html)
traits are used to convert between integers and floating-point numbers using
specified scales, i.e. different numbers of decimal places, primarily for use
with the [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html)
module, most usefully with currency values:

  - [`from_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/std/trait.FromIntWithScale.html#tymethod.from_int_with_scale) -
    Converts from an integer to a floating-point number with a specified scale.

  - [`to_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/std/trait.ToIntWithScale.html#tymethod.to_int_with_scale) -
    Converts from a floating-point number to an integer with a specified scale.


## ForceFrom

The [`ForceFrom`](https://docs.rs/rubedo/latest/rubedo/std/trait.ForceFrom.html)
trait is provided for situations where a lossy outcome from
[`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) is
acceptable for general use, but would not be appropriate to implement under
[`From`](https://doc.rust-lang.org/std/convert/trait.From.html). It's designed
to be implemented alongside [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html),
so that a path to convert and catch all errors can be given along with a path to
convert and ignore lossy errors.


## Iterator

The [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) trait
is extended with (technically, complemented by) the following methods:

  - [`limit()`](https://docs.rs/rubedo/latest/rubedo/std/trait.IteratorExt.html#tymethod.limit) -
    Limits the number of items returned by an iterator, similar to
    [`take()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take),
    but accepts an `Option`.


## Path

The [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) struct is
extended with the following methods:

  - [`append()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.append) -
    Adds a string to the end of a path, and returns the result as a new path,
    without creating a new path component.

  - [`is_subjective()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.is_subjective) -
    Checks if the path is specifically relative to the current directory, i.e.
    starts with a reference to the current directory, which can be `.` or `..`.

  - [`normalize()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.normalize) -
    Computes the canonicalized, absolute path of a file or directory, removing
    any `.` and `..` segments and returning the "real" path, without expanding
    symlinks or checking existence.

  - [`restrict()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.restrict) -
    Restricts the path to a given base path, normalising the path and not
    allowing it to go beyond the base path.

  - [`strip_parentdirs()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.strip_parentdirs) -
    Removes references to parent directories, i.e. `..`.

  - [`strip_root()`](https://docs.rs/rubedo/latest/rubedo/std/trait.PathExt.html#tymethod.strip_root) -
    Makes the path relative by removing the root and/or prefix components.


