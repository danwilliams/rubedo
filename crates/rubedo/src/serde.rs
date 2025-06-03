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
//! # `Display` and `ToString`, and `FromStr`
//! 
//! Implementing [`Display`] for a type adds a free implementation of
//! [`ToString`] as well, which provides the [`to_string()`](ToString::to_string())
//! method. This is intended to be used for human-readable representations of a
//! type, and provides a [`String`] copy of the converted type. This is not
//! necessarily the same as the serialised representation, which is intended for
//! machine-readable uses. However, for cases where the [`Display`]
//! implementation is the same as the serialised representation, it is possible
//! to use the [`to_string()`] function to provide the desired behaviour.
//! 
//! Notably, this is conceptually a subset of the [`Into<String>`](Into) use
//! case, as [`Into<String>`](Into) is intended to be used for any type that can
//! be converted to a [`String`], and [`ToString`] does that as well, albeit
//! with a different semantic purpose, and via copy versus consumption. Although
//! it is *advised* to use the [`into_string()`] or [`as_str()`] functions
//! instead (as appropriate), the [`to_string()`] Serde helper function is
//! provided for completeness and for such cases where a [`Display`]
//! implementation may exist and is the same as the serialised form, in which
//! case it would be onerous to also implement another function just for the
//! sake of it.
//! 
//! The other side of the [`ToString`] coin is [`FromStr`], which provides the
//! [`from_str()`](FromStr::from_str()) method. This is intended to be used for
//! parsing a [`String`] into a type, and is the counterpart to [`to_string()`].
//! For this purpose, the [`from_str()`] function is provided, which is
//! basically equivalent to [`from()`], but for [`String`] types. In this way,
//! it serves the same essential purpose as [`from_string()`], but for types
//! that implement [`FromStr`] instead of [`TryFrom<String>`](TryFrom). That is
//! the only difference, and the choice of which to use is entirely down to the
//! implementation of the type in question.
//! 
//! # `AsStr`
//! 
//! The second case considered is representation using [`AsStr`]. The
//! [`as_str()`] function is intended to be used with any type that implements
//! the [`AsStr`] trait, which provides an [`as_str()`](AsStr::as_str()) method.
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

use crate::std::{AsStr, FromIntWithScale, ToIntWithScale};
use core::{
	fmt::Display,
	str::FromStr,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError, ser::Error as SerError};



//		Functions																										

//		as_str																	
/// Returns a string representation of a type from a string slice.
/// 
/// This can be used with any type that implements [`AsStr`], but it is perhaps
/// most useful when applied to enums. There is no `AsStr` trait in core Rust at
/// present, but as the focus of this function is representation as [`str`], it
/// seems best and most intuitive to name it `as_str()`. It is distinct in
/// purpose from the [`Display`] trait, which is intended for human-readable
/// representations, and provides [`ToString`].
/// 
/// It is a fairly common pattern to have an enum that naturally serialises to
/// and from an integer, but also has a string representation. Or equally, it
/// could be that the required serialised form is not the default, which would
/// be to use the variant name. This function allows the correct string
/// representation to be obtained from the enum value.
/// 
/// It is different from the [`Display`] implementation in that it returns a
/// serialised representation of the enum (or indeed other type), suitable for
/// exchanging via serialised data, rather than a human-readable representation.
/// 
/// This function is intended for use by [`serde`] to serialise the enum or
/// other type of interest, e.g. when using [`#[serde(serialize_with)]`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "as_str")]
/// ```
/// 
/// So in summary:
/// 
///   - [`Display`] is for human-readable representations. This also implements
///     [`ToString`], which provides the [`to_string()`](ToString::to_string())
///     method. The semantic purpose is *conversion to* a string. This concept
///     of conversion signifies that the resulting string is not "the thing",
///     but a description of it, and reversing the process is not necessarily
///     guaranteed to be possible, and also may not have a 1:1 relationship.
/// 
///   - [`Into<String>`](Into) is for attempting an infallible conversion that
///     takes, transforms, and consumes the original value. It may be different
///     in intent and purpose from the [`Display`] implementation, but the
///     technical difference is simply that `to_` functions convert by cloning,
///     and `into_` functions convert by consuming. The semantic purpose is
///     *transformation into* a string. This concept in this current context
///     signifies that the resulting string is *equivalent to* "the thing", just
///     in a different form, which can be used to recreate the original and is
///     for all intents and purposes equivalent to it, but involves a process of
///     fundamental type conversion rather than a presentation of something
///     already present.
/// 
///   - [`AsStr`] is different in intent and purpose from both [`Display`] and
///     [`Into<String>`](Into). It is intended to be used with types that have a
///     method that returns a [`str`] slice, usually built-in as a static part
///     of themselves, and therefore providing another "view" of the type. The
///     semantic purpose is *representation as* a string. This concept signifies
///     that the resulting string *is* "the thing", just viewed in a different
///     way, which can be used to recreate the original and did not involve any
///     conversion in order to provide.
/// 
/// The [`AsStr`] implementation is therefore typically a "lightweight" method
/// of getting type representation, and the [`Into<String>`](Into)
/// implementation is a more "heavyweight" approach.
/// 
/// Note that there may be cause to implement both [`Into<String>`](Into) and
/// `AsStr` for a type, such as an enum, but the latter may well simply call the
/// former.
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
/// Note that the actual provision of `T` as a [`str`] is infallible, but the
/// serialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`into_string()`]
/// * [`to_string()`]
/// 
pub fn as_str<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: AsStr,
	S: Serializer,
{
	serializer.serialize_str(value.as_str())
}

//		to_string																
/// Returns a string copy of a type.
/// 
/// This can be used with any type that implements [`ToString`], which is
/// usually achieved by implementing [`Display`]. Although this is typically
/// intended for human-readable representations, using it for serialisation can
/// be useful in cases where this matches the serialised representation.
/// 
/// This function is intended for use by [`serde`] to serialise the type of
/// interest, e.g. when using [`#[serde(serialize_with)]`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "to_string")]
/// ```
/// 
/// The equivalent opposite of this function is [`from_str()`], which is
/// intended for use with [`FromStr`].
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
/// Note that the actual provision of `T` as a [`str`] is infallible, but the
/// serialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`as_str()`]
/// * [`from_str()`]
/// * [`into_string()`]
/// 
pub fn to_string<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: ToString,
	S: Serializer,
{
	serializer.serialize_str(&value.to_string())
}

//		from_str																
/// Returns a type from a string slice representation.
///
/// This can be used with any type that implements [`FromStr`]. It is basically
/// the opposite of [`to_string()`], and is intended to be used for parsing a
/// [`String`] into a type, and is essentially equivalent to [`from_string()`],
/// but for types that implement [`FromStr`] instead of [`TryFrom<String>`](TryFrom).
/// 
/// This function is intended for use by [`serde`] to deserialise the type of
/// interest, e.g. when using [`#[serde(deserialize_with)]`]:
///
/// ```ignore
/// #[serde(deserialize_with = "from_str")]
/// ```
///
/// The equivalent opposite of this function is [`to_string()`], which is
/// intended for use with [`ToString`].
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
/// Note that the actual conversion of the [`String`] to `T` is infallible, but
/// the deserialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`try_from()`]
/// * [`from_str()`]
/// * [`into_string()`]
/// * [`try_from_string()`]
/// 
pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T:      FromStr,
	T::Err: Display,
	D:      Deserializer<'de>,
{
	T::from_str(&String::deserialize(deserializer)?).map_err(DeError::custom)
}

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
///     method. The semantic purpose is *conversion to* a string. This concept
///     of conversion signifies that the resulting string is not "the thing",
///     but a description of it, and reversing the process is not necessarily
///     guaranteed to be possible, and also may not have a 1:1 relationship.
///
///   - [`Into<String>`](Into) is for attempting an infallible conversion that
///     takes, transforms, and consumes the original value. It may be different
///     in intent and purpose from the [`Display`] implementation, but the
///     technical difference is simply that `to_` functions convert by cloning,
///     and `into_` functions convert by consuming. The semantic purpose is
///     *transformation into* a string. This concept in this current context
///     signifies that the resulting string is *equivalent to* "the thing", just
///     in a different form, which can be used to recreate the original and is
///     for all intents and purposes equivalent to it, but involves a process of
///     fundamental type conversion rather than a presentation of something
///     already present.
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
/// * [`from()`]
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
/// For a lightweight alternative to provide representation rather than
/// conversion, see [`as_str()`].
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
/// * [`as_str()`]
/// * [`from()`]
/// * [`into()`]
/// * [`to_string()`]
/// * [`try_from_string()`]
/// 
pub fn into_string<T, S>(value: T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: Into<String>,
	S: Serializer,
{
	serializer.serialize_str(&value.into())
}

//		from																	
/// Returns a type from a string or other serialised representation.
/// 
/// This can be used with any type that implements [`From<String>`](From), but
/// it is perhaps most useful when applied to enums.
/// 
/// It is a fairly common pattern to have an enum that naturally serialises to
/// and from an integer, but also has a string representation. Or equally, it
/// could be that the required serialised form is not the default, which would
/// be to use the variant name. This function allows the correct string
/// representation to be used to obtain the appropriate enum value. However,
/// because it relies upon [`From`], it can be used with any type that
/// implements that trait and has a serialised representation.
/// 
/// This function is intended for use by [`serde`] to deserialise the enum or
/// other type of interest, e.g. when using [`#[serde(deserialize_with)]`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "from::<Foo, String, __D>")]
/// ```
/// 
/// Note that the example above will specify [`String`] as the concrete type
/// that the deserialiser will attempt to deserialise from. Any type that
/// [`serde`] supports can be used here, but it must be specified explicitly.
/// Because converting from a [`String`] is such a common use case, there is a
/// convenience function, [`from_string()`], that can be used instead. This
/// achieves the same result as the example above, but is more concise:
/// 
/// ```ignore
/// #[serde(deserialize_with = "from_string")]
/// ```
/// 
/// In the [`from()`] example, `Foo` is the type that is being deserialised,
/// which will be the result of the conversion. It needs to be specified because
/// the input type requires an annotation, as it cannot be inferred, and there
/// is no way to specify the input type without also specifying the output type.
/// This is another reason why [`from_string()`] is more concise, as it does not
/// require the output type to be specified.
/// 
/// Conversion from a [`String`] does not share the same number of semantic
/// possibilities as conversion to a [`String`]. There is a general premise that
/// there is generally only one correct serialised string representation of a
/// given value, and that other string representations are not representative of
/// the serialised form. This is not the case for conversion to a [`String`],
/// where there is no such assumption. For example, the [`Display`]
/// implementation for an enum may return a string representation that is not
/// the same as the serialised form, as it is intended for a different purpose.
/// However, the function written to implement [`From`] can of course do
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
/// Note that the actual conversion of `U` to `T` is infallible, but the
/// deserialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`into()`]
/// * [`from_str()`]
/// * [`from_string()`]
/// * [`try_from()`]
/// 
pub fn from<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T: From<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	U::deserialize(deserializer).map(T::from)
}

//		from_string																
/// Returns a type from a string representation.
/// 
/// This is a convenience function that can be used instead of [`from()`] when
/// the input type is [`String`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "from_string")]
/// ```
/// 
/// It is equivalent to the following:
/// 
/// ```ignore
/// #[serde(deserialize_with = "from::<T, String, __D>")]
/// ```
/// 
/// For more information, see the documentation for [`from()`].
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
/// Note that the actual conversion of the [`String`] to `T` is infallible, but
/// the deserialisation process may experience an error.
/// 
/// # See also
/// 
/// * [`from()`]
/// * [`from_str()`]
/// * [`into_string()`]
/// * [`try_from_string()`]
/// 
pub fn from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T: From<String>,
	D: Deserializer<'de>,
{
	String::deserialize(deserializer).map(T::from)
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
/// * [`from()`]
/// * [`from_str()`]
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
	T::try_from(U::deserialize(deserializer)?).map_err(DeError::custom)
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
/// * [`from_str()`]
/// * [`from_string()`]
/// * [`into_string()`]
/// * [`try_from()`]
/// 
pub fn try_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
	T:        TryFrom<String>,
	T::Error: Display,
	D:        Deserializer<'de>,
{
	T::try_from(String::deserialize(deserializer)?).map_err(DeError::custom)
}

//		try_from_int_with_scale													
/// Converts an integer to a floating-point number with scale.
/// 
/// This function takes an integer which represents a decimal value to a
/// specified number of decimal places, and converts it to a floating-point
/// number. So for instance, the integer `12345` could be converted to the
/// floating-point number `123.45` if the scale is 2.
/// 
/// It is intended for use by [`serde`], indirectly when using the following
/// convenience functions with [`#[serde(deserialize_with)]`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from_int_1dp::<T, i64, __D>")]
/// #[serde(deserialize_with = "try_from_int_2dp::<T, i64, __D>")]
/// #[serde(deserialize_with = "try_from_int_3dp::<T, i64, __D>")]
/// #[serde(deserialize_with = "try_from_int_4dp::<T, i64, __D>")]
/// ```
/// 
/// Note that the examples above will specify [`i64`] as the concrete type that
/// the deserialiser will attempt to deserialise from. Any integer type can be
/// used here, or indeed any type recognised by an implementation of
/// [`FromIntWithScale`], but it must be specified explicitly.
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// * `scale`        - The scale factor, i.e. the number of decimal places. Note
///                    that this is essentially limited to a maximum of 19 DP of
///                    movement for an [`f32`] or [`f64`] without overflowing,
///                    and 28 DP for a [`Decimal`].
/// 
/// # Errors
/// 
/// This function will return an error if the deserialised value cannot be
/// converted to the required type. The error will be a [`DeError`], which is
/// passed through from the [`serde`] crate.
/// 
/// It will also return an error if the conversion from the integer to `T`
/// fails. The nature of this error will be specific to the type being converted
/// to.
/// 
/// # See also
/// 
/// * [`from_cents()`]
/// * [`from_pence()`]
/// * [`try_from_int_1dp()`]
/// * [`try_from_int_2dp()`]
/// * [`try_from_int_3dp()`]
/// * [`try_from_int_4dp()`]
/// * [`try_to_int_with_scale()`]
/// 
pub fn try_from_int_with_scale<'de, T, U, D>(deserializer: D, scale: u8) -> Result<T, D::Error>
where
	T: FromIntWithScale<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	T::from_int_with_scale(U::deserialize(deserializer)?, scale)
		.ok_or_else(|| DeError::custom("Failed to convert from integer with scale"))
}

//		try_from_int_1dp														
/// Converts an integer to a floating-point number to 1 decimal place.
/// 
/// This is a convenience function — see [`try_from_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// See [`try_from_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_2dp()`]
/// * [`try_from_int_3dp()`]
/// * [`try_from_int_4dp()`]
/// * [`try_to_int_1dp()`]
/// 
pub fn try_from_int_1dp<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromIntWithScale<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	try_from_int_with_scale(deserializer, 1)
}

//		try_from_int_2dp														
/// Converts an integer to a floating-point number to 2 decimal places.
/// 
/// This is a convenience function — see [`try_from_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// See [`try_from_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`from_cents()`]
/// * [`from_pence()`]
/// * [`try_from_int_1dp()`]
/// * [`try_from_int_3dp()`]
/// * [`try_from_int_4dp()`]
/// * [`try_to_int_2dp()`]
/// 
pub fn try_from_int_2dp<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromIntWithScale<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	try_from_int_with_scale(deserializer, 2)
}

//		try_from_int_3dp														
/// Converts an integer to a floating-point number to 3 decimal places.
/// 
/// This is a convenience function — see [`try_from_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// See [`try_from_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_1dp()`]
/// * [`try_from_int_2dp()`]
/// * [`try_from_int_4dp()`]
/// * [`try_to_int_3dp()`]
/// 
pub fn try_from_int_3dp<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromIntWithScale<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	try_from_int_with_scale(deserializer, 3)
}

//		try_from_int_4dp														
/// Converts an integer to a floating-point number to 4 decimal places.
/// 
/// This is a convenience function — see [`try_from_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// See [`try_from_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_1dp()`]
/// * [`try_from_int_2dp()`]
/// * [`try_from_int_3dp()`]
/// * [`try_to_int_4dp()`]
/// 
pub fn try_from_int_4dp<'de, T, U, D>(deserializer: D) -> Result<T, D::Error>
where
	T: FromIntWithScale<U>,
	U: Deserialize<'de>,
	D: Deserializer<'de>,
{
	try_from_int_with_scale(deserializer, 4)
}

//		try_to_int_with_scale													
/// Converts a floating-point number to an integer with scale.
/// 
/// This function takes a floating-point number and converts it to an integer
/// which represents a decimal value to a specified number of decimal places. So
/// for instance, the floating-point number `123.45` could be converted to the
/// integer `12345` if the scale is 2.
/// 
/// It is intended for use by [`serde`], indirectly when using the following
/// convenience functions with [`#[serde(serialize_with)]`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "try_to_int_1dp::<T, i64, __S>")]
/// #[serde(serialize_with = "try_to_int_2dp::<T, i64, __S>")]
/// #[serde(serialize_with = "try_to_int_3dp::<T, i64, __S>")]
/// #[serde(serialize_with = "try_to_int_4dp::<T, i64, __S>")]
/// ```
/// 
/// Note that the examples above will specify [`i64`] as the concrete type that
/// the serialiser will attempt to serialise to. Any integer type can be used
/// here, or indeed any type recognised by an implementation of
/// [`ToIntWithScale`], but it must be specified explicitly.
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// * `scale`      - The scale factor, i.e. the number of decimal places. Note
///                  that this is essentially limited to a maximum of 19 DP of
///                  movement without overflowing.
/// 
/// # Errors
/// 
/// This function will return an error if the value cannot be serialised to the
/// the required type. The error will be a [`DeError`], which is passed through
/// from the [`serde`] crate.
/// 
/// It will also return an error if the conversion from `T` to the integer type
/// fails. The nature of this error will be specific to the type being converted
/// from.
/// 
/// # See also
/// 
/// * [`to_cents()`]
/// * [`to_pence()`]
/// * [`try_from_int_with_scale()`]
/// * [`try_to_int_1dp()`]
/// * [`try_to_int_2dp()`]
/// * [`try_to_int_3dp()`]
/// * [`try_to_int_4dp()`]
/// 
pub fn try_to_int_with_scale<T, U, S>(value: &T, serializer: S, scale: u8) -> Result<S::Ok, S::Error>
where
	T: ToIntWithScale<U>,
	U: Serialize,
	S: Serializer,
{
	T::to_int_with_scale(value, scale)
		.ok_or_else(|| SerError::custom("Failed to convert to integer with scale"))?
		.serialize(serializer)
}

//		try_to_int_1dp															
/// Converts a floating-point number to an integer to 1 decimal place.
/// 
/// This is a convenience function — see [`try_to_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// See [`try_to_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_1dp()`]
/// * [`try_to_int_2dp()`]
/// * [`try_to_int_3dp()`]
/// * [`try_to_int_4dp()`]
/// 
pub fn try_to_int_1dp<T, U, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: ToIntWithScale<U>,
	U: Serialize,
	S: Serializer,
{
	try_to_int_with_scale(value, serializer, 1)
}

//		try_to_int_2dp															
/// Converts a floating-point number to an integer to 2 decimal places.
/// 
/// This is a convenience function — see [`try_to_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// See [`try_to_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`to_cents()`]
/// * [`to_pence()`]
/// * [`try_from_int_2dp()`]
/// * [`try_to_int_1dp()`]
/// * [`try_to_int_3dp()`]
/// * [`try_to_int_4dp()`]
/// 
pub fn try_to_int_2dp<T, U, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: ToIntWithScale<U>,
	U: Serialize,
	S: Serializer,
{
	try_to_int_with_scale(value, serializer, 2)
}

//		try_to_int_3dp															
/// Converts a floating-point number to an integer to 3 decimal places.
/// 
/// This is a convenience function — see [`try_to_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// See [`try_to_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_3dp()`]
/// * [`try_to_int_1dp()`]
/// * [`try_to_int_2dp()`]
/// * [`try_to_int_4dp()`]
/// 
pub fn try_to_int_3dp<T, U, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: ToIntWithScale<U>,
	U: Serialize,
	S: Serializer,
{
	try_to_int_with_scale(value, serializer, 3)
}

//		try_to_int_4dp															
/// Converts a floating-point number to an integer to 4 decimal places.
/// 
/// This is a convenience function — see [`try_to_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// See [`try_to_int_with_scale()`].
/// 
/// # See also
/// 
/// * [`try_from_int_4dp()`]
/// * [`try_to_int_1dp()`]
/// * [`try_to_int_2dp()`]
/// * [`try_to_int_3dp()`]
/// 
pub fn try_to_int_4dp<T, U, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
	T: ToIntWithScale<U>,
	U: Serialize,
	S: Serializer,
{
	try_to_int_with_scale(value, serializer, 4)
}

//		from_cents																
/// Converts an integer to a [`Decimal`] to 2 decimal places.
/// 
/// This is a convenience function that can be used instead of
/// [`try_from_int_2dp()`] when the input type is [`i64`] and the output type
/// is [`Decimal`]:
/// 
/// ```ignore
/// #[serde(deserialize_with = "from_cents")]
/// ```
/// 
/// It is equivalent to the following:
/// 
/// ```ignore
/// #[serde(deserialize_with = "try_from_int_2dp::<Decimal, i64, __D>")]
/// ```
/// 
/// This function takes an [`i64`] integer which represents a decimal value to 2
/// DP, and converts it to a [`Decimal`]. So for instance, the integer `12345`
/// would be converted to the decimal number `123.45`. The types were chosen
/// because [`Decimal`] is the safest to use for currency values (floats should
/// never be used as they are not safe), and [`i64`] is the most common integer
/// type in this context.
///
/// For more information, see the documentation for
/// [`try_from_int_with_scale()`].
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
/// It will also return an error if the conversion from the [`i64`] to
/// [`Decimal`] fails (which should never happen). The nature of this error will
/// be specific to the type being converted to.
/// 
/// # See also
/// 
/// * [`from_pence()`]
/// * [`to_cents()`]
/// * [`try_from_int_2dp()`]
/// 
pub fn from_cents<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
	D: Deserializer<'de>,
{
	try_from_int_2dp::<Decimal, i64, D>(deserializer)
}

//		to_cents																
/// Converts a [`Decimal`] to an integer to 2 decimal places.
/// 
/// This is a convenience function that can be used instead of
/// [`try_to_int_2dp()`] when the input type is [`Decimal`] and the output type
/// is [`i64`]:
/// 
/// ```ignore
/// #[serde(serialize_with = "to_cents")]
/// ```
/// 
/// It is equivalent to the following:
/// 
/// ```ignore
/// #[serde(serialize_with = "try_to_int_2dp::<Decimal, i64, __S>")]
/// ```
/// 
/// This function takes a [`Decimal`], and converts it to an integer which
/// represents a decimal value to 2 DP. So for instance, the decimal `123.45`
/// would be converted to the integer `12345`. The types were chosen because
/// [`Decimal`] is the safest to use for currency values (floats should never be
/// used as they are not safe), and [`i64`] is the most common integer type in
/// this context.
///
/// For more information, see the documentation for
/// [`try_to_int_with_scale()`].
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// This function will return an error if the value cannot be serialised to the
/// the required type. The error will be a [`DeError`], which is passed through
/// from the [`serde`] crate.
/// 
/// It will also return an error if the conversion from [`Decimal`] to [`i64`]
/// fails (for instance if the decimal is larger than can fit into the integer's
/// range). The nature of this error will be specific to the type being
/// converted from.
/// 
/// # See also
/// 
/// * [`from_cents()`]
/// * [`to_pence()`]
/// * [`try_to_int_2dp()`]
/// 
pub fn to_cents<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	try_to_int_2dp::<Decimal, i64, S>(value, serializer)
}

//		from_pence																
/// Converts an integer to a [`Decimal`] to 2 decimal places.
/// 
/// This is an alias of [`from_cents()`] for convenience.
/// 
/// # Parameters
/// 
/// * `deserializer` - The deserialiser to use.
/// 
/// # Errors
/// 
/// See [`from_cents()`].
/// 
/// # See also
/// 
/// * [`from_cents()`]
/// * [`to_pence()`]
/// * [`try_from_int_2dp()`]
/// 
pub fn from_pence<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
	D: Deserializer<'de>,
{
	from_cents(deserializer)
}

//		to_pence																
/// Converts a [`Decimal`] to an integer to 2 decimal places.
/// 
/// This is an alias of [`to_cents()`] for convenience.
/// 
/// # Parameters
/// 
/// * `serializer` - The serialiser to use.
/// 
/// # Errors
/// 
/// See [`to_cents()`].
/// 
/// # See also
/// 
/// * [`from_pence()`]
/// * [`to_cents()`]
/// * [`try_to_int_2dp()`]
/// 
pub fn to_pence<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
	S: Serializer,
{
	to_cents(value, serializer)
}


