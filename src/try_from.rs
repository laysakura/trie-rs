//! Try to collect a label from tokens.

use std::convert::Infallible;

/// Used to create labels from tokens.
pub trait TryFromTokens<Token> {
    /// Error type for creating label from tokens.
    type Error;

    /// Create a new label from the tokens.
    ///
    /// `use_reverse_insertion` indicates whether reverse insertion should be attempted.
    /// In such a case, the iterator should be reversed and the collection reversed afterwards (some collections might be better off ignoring this).
    ///
    /// Note: `NodeIter` benefits from reverse order insertion since iterating over nodes by taking the parent is faster than searching for the child.
    fn try_from_tokens<T>(tokens: T, use_reverse_insertion: bool) -> Result<Self, Self::Error>
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator;
}

impl<Token> TryFromTokens<Token> for Vec<Token> {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T, use_reverse_insertion: bool) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        if use_reverse_insertion {
            let mut c = Vec::from_iter(tokens.rev());
            c.reverse();
            Ok(c)
        } else {
            Ok(Vec::from_iter(tokens))
        }
    }
}

impl<Token> TryFromTokens<Token> for Box<[Token]> {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T, use_reverse_insertion: bool) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        if use_reverse_insertion {
            let mut c = Box::from_iter(tokens.rev());
            c.reverse();
            Ok(c)
        } else {
            Ok(Box::from_iter(tokens))
        }
    }
}

impl TryFromTokens<char> for String {
    type Error = Infallible;

    fn try_from_tokens<T>(tokens: T, _use_reverse_insertion: bool) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = char> + DoubleEndedIterator,
    {
        Ok(String::from_iter(tokens))
    }
}

impl TryFromTokens<u8> for String {
    type Error = std::string::FromUtf8Error;

    fn try_from_tokens<T>(tokens: T, _use_reverse_insertion: bool) -> Result<Self, Self::Error>
    where
        T: Iterator<Item = u8> + DoubleEndedIterator,
    {
        String::from_utf8(Vec::from_iter(tokens))
    }
}
