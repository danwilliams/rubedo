//! This module provides conversion utility functions for use with [Serde](https://crates.io/crates/serde).



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


