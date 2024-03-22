use std::iter::FromIterator;

/// Try to create an object from an iterator.
pub trait TryFromIterator<A, Marker> {
    type Error: std::fmt::Debug;
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
