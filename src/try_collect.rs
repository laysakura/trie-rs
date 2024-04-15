//! Try to collect from an iterator; operation may fail.
//!
//! Any type can that be `collect()`ed can be `try_collect()`ed without fail.
//!
//! # Usage
//!
//! The simplest usage is like this.
//!
//! ```
//! use trie_rs::try_collect::*;
//! let bytes: Vec<u8> = vec![72, 105];
//! let s: String = bytes.into_iter().try_collect().unwrap();
//! assert_eq!(s, "Hi");
//! ```
//!
//! # Motivation
//!
//! I really wanted to be able to turn a `Iterator<Item = u8>` into a String
//! more easily, so that one could accumulate trie entries as `Vec<u8>`s or as
//! `String`s. This is made complicated by the fact that [String] does not have
//! a `FromIterator<u8>` implementation, and the method it does have
//! `from_utf8()` is fallible; it returns a `Result`.
//!
//! Thus [TryFromIterator] is simply a fallible version of
//! [std::iter::FromIterator]. And `try_collect()` is `collect()` fallible
//! cousin as well.
//!
//! # Technical Note
//!
//! `TryFromIterator<A, M>` accepts a generic type `M` marker parameter. In
//! general usage, the caller will simply pass along a generic `M` type.
//!
//! The reason it exists is so we can specify a blanket implementation of
//! [TryFromIterator] for all [std::iter::FromIterator]s, and we can also
//! specify one for [String].
//!
//! Without this marker type, it's not possible to have a blanket and
//! specialized implementation of the trait.
//!
use std::fmt::Debug;
use std::iter::FromIterator;

/// Try to collect from an iterator; operation may fail.
pub trait TryCollect: Iterator {
    /// Use this iterator to collect into a container `C`, may fail.
    fn try_collect<C, M>(self) -> Result<C, C::Error>
    where
        C: TryFromIterator<Self::Item, M>,
        Self: Sized,
    {
        C::try_from_iter(self)
    }
}

impl<T> TryCollect for T where T: Iterator + ?Sized {}

/// Try to create an object from an iterator.
pub trait TryFromIterator<A, Marker> {
    /// Error type of [TryFromIterator::try_from_iter].
    type Error: Debug;
    /// Try to turn the given iterator into `Self`.
    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = A>;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Marker type for blanket [TryFromIterator] implementation.
#[doc(hidden)]
pub struct Collect;

impl<S, A> TryFromIterator<A, Collect> for S
where
    S: FromIterator<A>,
{
    type Error = ();
    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = A>,
    {
        Ok(FromIterator::from_iter(iter))
    }
}

#[derive(Debug, Clone)]
/// Marker type for String [TryFromIterator] implementation.
#[doc(hidden)]
pub struct StringCollect;

impl TryFromIterator<u8, StringCollect> for String {
    type Error = std::string::FromUtf8Error;
    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = u8>,
    {
        String::from_utf8(iter.into_iter().collect())
    }
}
