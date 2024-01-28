## serde

[`Decimal`]: https://crates.io/crates/rust_decimal

The [`serde`](https://docs.rs/rubedo/latest/rubedo/serde/index.html) module
provides conversion utility functions for use with [Serde](https://crates.io/crates/serde).

### Serialisation

  - [`as_str()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.as_str.html) -
    Returns a string representation of a type from a string slice.

  - [`into()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.into.html) -
    Returns a serialised representation of a type.

  - [`into_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.into_string.html) -
    Returns a string representation of a type.

  - [`to_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_string.html) -
    Returns a string copy of a type.

### Deserialisation

  - [`from()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from.html) -
    Returns a type from a string or other serialised representation.
    
  - [`from_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_string.html) -
    Returns a type from a string representation.

  - [`from_str()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_str.html) -
    Returns a type from a string slice representation.
    
  - [`try_from()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from.html) -
    Returns a type from a string or other serialised representation.

  - [`try_from_string()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_string.html) -
    Returns a type from a string representation.

### Decimal helpers

  - [`from_cents()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_cents.html) -
    Converts an integer to a [`Decimal`][] to 2 decimal places.

  - [`from_pence()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.from_pence.html) -
    Converts an integer to a [`Decimal`][] to 2 decimal places.

  - [`to_cents()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_cents.html) -
    Converts a [`Decimal`][] to an integer to 2 decimal places.

  - [`to_pence()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.to_pence.html) -
    Converts a [`Decimal`][] to an integer to 2 decimal places.

  - [`try_from_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_with_scale.html) -
    Converts an integer to a floating-point number with scale.

  - [`try_from_int_1dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_1dp.html) -
    Converts an integer to a floating-point number to 1 decimal place.

  - [`try_from_int_2dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_2dp.html) -
    Converts an integer to a floating-point number to 2 decimal places.

  - [`try_from_int_3dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_3dp.html) -
    Converts an integer to a floating-point number to 3 decimal places.

  - [`try_from_int_4dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_from_int_4dp.html) -
    Converts an integer to a floating-point number to 4 decimal places.

  - [`try_to_int_with_scale()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_with_scale.html) -
    Converts a floating-point number to an integer with scale.

  - [`try_to_int_1dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_1dp.html) -
    Converts a floating-point number to an integer to 1 decimal place.

  - [`try_to_int_2dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_2dp.html) -
    Converts a floating-point number to an integer to 2 decimal places.

  - [`try_to_int_3dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_3dp.html) -
    Converts a floating-point number to an integer to 3 decimal places.

  - [`try_to_int_4dp()`](https://docs.rs/rubedo/latest/rubedo/serde/fn.try_to_int_4dp.html) -
    Converts a floating-point number to an integer to 4 decimal places.


