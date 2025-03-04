//! Try to collect a label from tokens.

use std::{convert::Infallible, fmt::Debug};

/// Used to create labels from tokens.
pub trait TryFromTokens<Token> {
    /// Error type for creating label from tokens.
    type Error: Debug;

    /// Create a new label from the tokens.
    fn try_from_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = Token>;

    /// Create a new label from the tokens using reverse order insertion.
    ///
    /// The iterator should be reversed and the collection reversed afterwards (some collections might be better off ignoring this).
    ///
    /// Note: `NodeIter` benefits from reverse order insertion since iterating over nodes by taking the parent is faster than searching for the child.
    fn try_from_reverse_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator;
}

impl<Token> TryFromTokens<Token> for Vec<Token> {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = Token>,
    {
        Ok(Vec::from_iter(tokens))
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        let mut c = Vec::from_iter(tokens.rev());
        c.reverse();
        Ok(c)
    }
}

impl<Token> TryFromTokens<Token> for Box<[Token]> {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = Token>,
    {
        Ok(Box::from_iter(tokens))
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        let mut c = Box::from_iter(tokens.rev());
        c.reverse();
        Ok(c)
    }
}

impl TryFromTokens<char> for String {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = char>,
    {
        Ok(String::from_iter(tokens))
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = char> + DoubleEndedIterator,
    {
        let mut c = Box::<[_]>::from_iter(tokens.rev());
        c.reverse();
        Ok(String::from_iter(c.into_iter()))
    }
}

impl TryFromTokens<u8> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = u8>,
    {
        String::from_utf8(Vec::from_iter(tokens))
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = u8> + DoubleEndedIterator,
    {
        let mut c = Vec::from_iter(tokens.rev());
        c.reverse();
        String::from_utf8(c)
    }
}
