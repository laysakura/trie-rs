use std::fmt::Debug;
use std::iter::FromIterator;

/// Try to collect from an iterator that may fail.
///
/// # Motivation
///
/// I really wanted to be able to turn a `Iterator<Item = u8>` into a String
/// more easily.
///
/// ```
/// use trie_rs::try_collect::*;
/// let bytes: Vec<u8> = vec![72, 105];
/// let s: String = bytes.into_iter().try_collect().unwrap();
/// assert_eq!(s, "Hi");
/// ```
pub trait TryCollect: Iterator {
    /// Use this iterator as a prefix for a frayed iterator with many postfixes.
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
    type Error: Debug;
    fn try_from_iter<T>(iter: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: IntoIterator<Item = A>;
}

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
