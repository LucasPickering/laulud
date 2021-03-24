mod auth;

pub use auth::*;

use std::{convert::TryInto, fmt::Debug};

/// Convert a collection length value to `i32`. Typically the input type is
/// `usize` or `i64`. Useful when going from vectors or Spotify values, which
/// use `usize` for length, to GraphQL return values, which use `i32` because
/// GraphQL only supports one integer type. This assumes that the conversion
/// will never actually fail, because we're never going to be handling more than
/// 2^31 values in a collection.
///
/// TODO make this return a result?
pub fn to_i32<T>(value: T) -> i32
where
    T: TryInto<i32>,
    T::Error: Debug,
{
    value
        .try_into()
        .expect("Cannot convert length to i32, out of bounds")
}
