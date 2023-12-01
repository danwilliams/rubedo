//! This module provides conversion utility functions for use with [Serde](https://crates.io/crates/serde).
//! 
//! This module attempts to consider the common use cases for (de)serialisation,
//! and provide functions that are semantically appropriate for those use cases.
//! The functions are intended to be used with the
//! [`#[serde(serialize_with)]`](https://docs.serde.rs/serde/attr.serialize_with.html)
//! and [`#[serde(deserialize_with)]`](https://docs.serde.rs/serde/attr.deserialize_with.html)
//! attributes.
//! 
//! Of course, as this module is an *extension* of standard Serde functionality,
//! it does not attempt to reproduce what Serde already does by default. For
//! instance, if a struct has a field of type [`u32`], then Serde will already
//! know how to serialise and deserialise that field. Equally, if a struct has a
//! field that is an enum, then Serde is able to serialise and deserialise that
//! field according to the available enum variants, and the chosen internal
//! representation of the enum.
//! 
//! Where this module provides particular value is when an alternative
//! serialised representation is required for certain struct members. For
//! example, it is common to have an enum that naturally serialises to and from
//! an integer, but also has a string representation. Or equally, it could be
//! that the required serialised form is not the default. This module allows
//! easy specification of alternative forms for serialisation and
//! deserialisation, while working with the existing Serde derive macros.
//! 
//! As a general statement, the intention of this module is to provide this
//! functionality for data types such as structs and enums, where the Serde
//! derive macros would be used, and there is no obvious application for
//! primitive types such as integers, floats, and booleans, or string types such
//! as [`String`] and [`str`].
//! 
//! # Naming conventions
//! 
//! Generally in Rust, the naming of functions carries semantic meaning:
//! 
//!   - `to_` prefix: This implies a conversion that does not necessarily
//!     consume the original value. It's often seen in methods that return a new
//!     value based on the original, without consuming the original.
//! 
//!   - `into_` prefix: This indicates that the function consumes the original
//!     value and transforms it into another. It's commonly used with Rust's
//!     ownership system, signifying that the original value will no longer be
//!     usable after the conversion.
//! 
//!   - `as_` prefix: This is typically used for cheap reference conversions
//!     that don't involve any data processing. It suggests a view or
//!     representation of the original value, not a conversion or
//!     transformation.
//! 
//! # `From` and `Into`
//! 
//! The first case considered is general conversion using [`Into`] and [`From`].
//! In a situation where a type implements [`Into<T>`](Into) and either
//! [`From<T>`](From) or [`TryFrom<T>`](TryFrom), then it seems natural and
//! appropriate to be able to use those implementations for serialisation and
//! deserialisation. Indeed, Serde does allow this, and it is possible to use
//! the [`#[serde(into)]`](https://serde.rs/container-attrs.html#into),
//! and [`#[serde(from)]`](https://serde.rs/container-attrs.html#from), and
//! and [`#[serde(try_from)]`](https://serde.rs/container-attrs.html#try_from)
//! attributes to specify the desired primary types. However, these apply at the
//! container level, and there are no equivalent attributes for specifying the
//! same behaviour at the field level. Instead, the [`#[serde(with)]`](https://serde.rs/field-attrs.html#with)
//! attribute can be used, but this requires the implementation of a custom
//! serialiser and/or deserialiser. That's where this module comes in.
//! 
//! The [`into()`], [`from()`], and [`try_from()`] functions can be used to
//! specify the desired behaviour at the field level, matching the behaviour of
//! the Serde container-level attributes, without the need to implement custom
//! serialisers and deserialisers. This allows for variations other than the
//! default to be easily specified. Additionally, ease-of-use functions for
//! [`String`] conversion are provided in the form of [`into_string()`],
//! [`from_string()`], and [`try_from_string()`]. Note that these functions
//! expect to work on a full [`String`], not a [`str`] slice, due to their
//! context.
//! 
//! The end result is that it becomes trivial to specify alternate conversions
//! for any type that implements the common conversion traits.
//! 
//! # `as_str()` representation
//! 
//! The second case considered is the [`as_str()`] function. This function is
//! intended to be used with any type that implements the [`AsStr`] trait, which
//! is a marker trait used to indicate that a type has an `as_str()` method.
//! This function is primarily intended to be used with enums, where it is
//! common to have variants that naturally serialise to and from integers, but
//! also have a string representation. In such cases, the enums will typically
//! be created with `static &str` values for such representation, in which case
//! it is desirable to use and propagate those actual values by reference
//! instead of making unnecessary copies. This is the purpose of the
//! [`as_str()`] function.
//! 
//! In keeping with Rust naming conventions and idioms, the concept of
//! representation is considered to be distinct from the concept of conversion,
//! with this function providing an unmodified, uncopied "view" onto a value
//! provided by the type for this purpose.
//! 



//		Modules

#[cfg(test)]
#[path = "tests/serde.rs"]
mod tests;



//		Packages

use core::fmt::Display;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};



//		Functions

//		into																	
/// Returns a serialised representation of a type.
/// 
/// This can be used with any type that implements [`Into<T>`](Into), which
/// comes for free when implementing [`From<T>`](From), but it is perhaps most
/// useful when applied to enums. Note that **both** `Into<U> for T` **AND**
/// `Into<U> for &T` need to be implemented (usually via `From<T> for U` and
/// `From<&T> for U`), due to the way Serde handles generic cases like this.
/// 
/// It is a fairly common pattern to have an enum that naturally serialises to
/// and from an integer, but also has a string representation. Or equally, it
/// could be that the required serialised form is not the default, which would
/// be to use the variant name. This function allows the correct string
/// representation to be obtained from the enum value. However, because it
/// relies upon [`Into`], it can be used with any type that implements that
/// trait and has a serialised representation.
/// 
/// It is different from the [`Display`] implementation in that it returns a
/// serialised representation of the enum (or indeed other type), suitable for
/// exchanging via serialised data, rather than a human-readable representation.
/// 
/// This function is intended for use by [`serde`] to serialise the enum or
/// other type of interest, e.g. when using [`#[serde(serialize_with)]`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "into::<Foo, String, __S>")]
/// ```
/// 
/// Note that the example above will specify [`String`] as the concrete type
/// that the serialiser will attempt to serialise into. Any type that [`serde`]
/// supports can be used here, but it must be specified explicitly. Because
/// converting to a [`String`] is such a common use case, there is a convenience
/// function, [`into_string()`], that can be used instead. This achieves the
/// same result as the example above, but is more concise:
/// 
/// ```ignore
/// #[serde(serialize_with = "into_string")]
/// ```
/// 
/// In the [`into()`] example, `Foo` is the type that is being serialised, which
/// will result in a conversion to [`String`]. It needs to be specified because
/// the output type requires an annotation, as it cannot be inferred, and there
/// is no way to specify the output type without also specifying the input type.
/// This is another reason why [`into_string()`] is more concise, as it does not
/// require the input type to be specified.
/// 
/// So in summary:
/// 
///   - [`Display`] is for human-readable representations. This also implements
///     [`ToString`], which provides the [`to_string()`](ToString::to_string())
///     method. The semantic purpose is conversion *to* a string. This concept
///     of conversion signifies that the resulting string is not "the thing",
///     but a description of it.
/// 
///   - [`Into<String>`](Into) is for attempting an infallible conversion. It
///     should be possible to convert *to* a [`String`] without failing. The
///     semantic purpose is representation *as* a string, but achieved through
///     conversion here. This concept signifies that the resulting string *is*
///     "the thing", just in a different form, which can be used to recreate the
///     original and is for all intents and purposes equivalent to it.
/// 
/// Conversion to a [`String`] has a number of semantic possibilities. However,
/// there is a general premise that there is generally only one correct
/// serialised string representation of a given value, and that other string
/// representations are not representative of the serialised form. This is not
/// the case for general conversion to a [`String`], where there is no such
/// assumption. For example, the [`Display`] implementation for an enum may
/// return a string representation that is not the same as the serialised form,
/// as it is intended for a different purpose. However, the function written to
/// implement [`Into`] should output the string representation that is
/// considered to be authoritative.
/// 
/// Note that, unlike with deserialisation using [`try_from()`], there is only
/// one serialised type possibility supported by this function, which is `U`. If
/// there is a need to serialise to other types, then the [`Serialize`] trait
/// should be implemented directly.
/// 
/// # Parameters
///
/// * `value`      - The value to serialise.
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// This function will return an error if the value cannot be serialised to a
/// [`String`]. The error will be a [`Serializer::Error`], which is passed
/// through from the [`serde`] crate.
/// 
/// Note that the actual conversion of `T` to `U` is infallible, but the
/// serialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`into_string()`]
/// * [`try_from()`]
/// 
pub fn into<T, U, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: Into<U>,
	U: Serialize + for<'a > From<&'a T>,
	S: Serializer,
{
	let converted: U = value.into();
	converted.serialize(serializer)
}

//		into_string																
/// Returns string representation of a type.
/// 
/// This is a convenience function that can be used instead of [`into()`] when
/// when the output type is [`String`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "into_string")]
/// ```
/// 
/// It is equivalent to the following:
/// 
/// ```ignore
/// #[serde(serialize_with = "into::<T, String, __S>")]
/// ```
/// 
/// It can be used with any type that implements [`Into<String>`](Into), which
/// comes for free when implementing [`From<T>`](From) for [`String`], but it is
/// perhaps most useful when applied to enums. Note that, unlike with the
/// generic [`into()`] function, only `Into<String> for &T` needs to be
/// implemented (usually via `From<&T> for String`), and not `Into<String> for
/// T` as well.
/// 
/// Note also that, unlike with deserialisation using [`try_from_string()`],
/// there is only one serialised type supported by this function, which is
/// [`String`]. If there is a need to serialise to other types, then the
/// [`Serialize`] trait should be implemented directly.
/// 
/// For more information, see the documentation for [`into()`].
/// 
/// # Parameters
/// 
/// * `value`      - The value to serialise.
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// This function will return an error if the value cannot be serialised to a
/// [`String`]. The error will be a [`Serializer::Error`], which is passed
/// through from the [`serde`] crate.
/// 
/// Note that the actual conversion of `T` to a [`String`] is infallible, but
/// the serialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`into()`]
/// * [`try_from_string()`]
/// 
pub fn into_string<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: Into<String>,
	S: Serializer,
{
	serializer.serialize_str(&value.into())
}

//		try_from																
/// Returns a type from a string or other serialised representation.
/// 
/// This can be used with any type that implements [`TryFrom<String>`](TryFrom),
/// but it is perhaps most useful when applied to enums.
/// 
/// It is a fairly common pattern to have an enum that naturally serialises to
/// and from an integer, but also has a string representation. Or equally, it
/// could be that the required serialised form is not the default, which would
/// be to use the variant name. This function allows the correct string
/// representation to be used to obtain the appropriate enum value. However,
/// because it relies upon [`TryFrom`], it can be used with any type that
/// implements that trait and has a serialised representation.
/// 
/// This function is intended for use by [`serde`] to deserialise the enum or
/// other type of interest, e.g. when using [`#[serde(deserialize_with)]`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from::<Foo, String, __D>")]
/// ```
/// 
/// Note that the example above will specify [`String`] as the concrete type
/// that the deserialiser will attempt to deserialise from. Any type that
/// [`serde`] supports can be used here, but it must be specified explicitly.
/// Because converting from a [`String`] is such a common use case, there is a
/// convenience function, [`try_from_string()`], that can be used instead. This
/// achieves the same result as the example above, but is more concise:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from_string")]
/// ```
/// 
/// In the [`try_from()`] example, `Foo` is the type that is being deserialised,
/// which will be the result of the conversion. It needs to be specified because
/// the input type requires an annotation, as it cannot be inferred, and there
/// is no way to specify the input type without also specifying the output type.
/// This is another reason why [`try_from_string()`] is more concise, as it does
/// not require the output type to be specified.
/// 
/// Conversion from a [`String`] does not share the same number of semantic
/// possibilities as conversion to a [`String`]. There is a general premise that
/// there is generally only one correct serialised string representation of a
/// given value, and that other string representations are not representative of
/// the serialised form. This is not the case for conversion to a [`String`],
/// where there is no such assumption. For example, the [`Display`]
/// implementation for an enum may return a string representation that is not
/// the same as the serialised form, as it is intended for a different purpose.
/// However, the function written to implement [`TryFrom`] can of course do
/// whatever it likes, and can support any number of string representations.
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// This function will return an error if the deserialised value cannot be
/// converted to the required type. The error will be a [`DeError`], which is
/// passed through from the [`serde`] crate.
/// 
/// It will also return an error if the conversion from `U` to `T` fails. The
/// nature of this error will be specific to the type being converted to.
/// 
/// # See also
/// 
/// * [`into()`]
/// * [`try_from_string()`]
/// 
pub fn try_from<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T:        TryFrom<U>,
	U:        Deserialize<'de>,
	T::Error: Display,
	D:        Deserializer<'de>,
{
	U::deserialize(deserializer).and_then(|value| T::try_from(value).map_err(DeError::custom))
}

//		try_from_string															
/// Returns a type from a string representation.
/// 
/// This is a convenience function that can be used instead of [`try_from()`]
/// when the input type is [`String`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from_string")]
/// ```
/// 
/// It is equivalent to the following:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from::<T, String, __D>")]
/// ```
/// 
/// For more information, see the documentation for [`try_from()`].
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// This function will return an error if the deserialised value cannot be
/// converted to the required type. The error will be a [`DeError`], which is
/// passed through from the [`serde`] crate.
/// 
/// It will also return an error if the conversion from the [`String`] to `T`
/// fails. The nature of this error will be specific to the type being converted
/// to.
/// 
/// # See also
/// 
/// * [`into_string()`]
/// * [`try_from()`]
/// 
pub fn try_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T:        TryFrom<String>,
	T::Error: Display,
	D:        Deserializer<'de>,
{
	String::deserialize(deserializer).and_then(|value| T::try_from(value).map_err(DeError::custom))
}


