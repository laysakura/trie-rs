//! Try to collect a label from tokens.

/// Used to create labels from tokens.
pub trait TryFromTokens<Token> {
    /// The result returned when constructing the label.
    type Result;

    /// The result of zipping [`TryFromTokens::Result`] and `Other`.
    ///
    /// Used when returning a `(label, value)` pair.
    type Zip<Other>;

    /// Create a new label from the tokens.
    fn try_from_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = Token>;

    /// Create a new label from the tokens using reverse order insertion.
    ///
    /// The iterator should be reversed and the collection reversed afterwards (some collections might be better off ignoring this).
    ///
    /// Note: `NodeIter` benefits from reverse order insertion since iterating over nodes by taking the parent is faster than searching for the child.
    fn try_from_reverse_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator;

    /// Zips a value with the result.
    ///
    /// Meant to be used when returning a `(label, value)` pair that might be fallible.
    fn zip<Other>(this: Self::Result, other: Other) -> Self::Zip<Other>;

    /// Unzips a result from a pair.
    ///
    /// Meant to be used when returning a `label` from a pair that might be fallible.
    fn unzip<Other>(this: Self::Zip<Other>) -> Self::Result;
}

impl<Token> TryFromTokens<Token> for Vec<Token> {
    type Result = Self;

    type Zip<Other> = (Self, Other);

    fn try_from_tokens<T>(tokens: T) -> Self::Result
    where
        T: Iterator<Item = Token>,
    {
        Vec::from_iter(tokens)
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        let mut c = Vec::from_iter(tokens.rev());
        c.reverse();
        c
    }

    fn zip<Other>(this: Self::Result, other: Other) -> Self::Zip<Other> {
        (this, other)
    }

    fn unzip<Other>(this: Self::Zip<Other>) -> Self::Result {
        this.0
    }
}

impl<Token> TryFromTokens<Token> for Box<[Token]> {
    type Result = Self;

    type Zip<Other> = (Self, Other);

    fn try_from_tokens<T>(tokens: T) -> Self::Result
    where
        T: Iterator<Item = Token>,
    {
        Box::from_iter(tokens)
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = Token> + DoubleEndedIterator,
    {
        let mut c = Box::from_iter(tokens.rev());
        c.reverse();
        c
    }

    fn zip<Other>(this: Self::Result, other: Other) -> Self::Zip<Other> {
        (this, other)
    }

    fn unzip<Other>(this: Self::Zip<Other>) -> Self::Result {
        this.0
    }
}

impl TryFromTokens<char> for String {
    type Result = Self;

    type Zip<Other> = (Self, Other);

    fn try_from_tokens<T>(tokens: T) -> Self::Result
    where
        T: Iterator<Item = char>,
    {
        String::from_iter(tokens)
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = char> + DoubleEndedIterator,
    {
        let mut c = Box::<[_]>::from_iter(tokens.rev());
        c.reverse();
        String::from_iter(c.into_iter())
    }

    fn zip<Other>(this: Self::Result, other: Other) -> Self::Zip<Other> {
        (this, other)
    }

    fn unzip<Other>(this: Self::Zip<Other>) -> Self::Result {
        this.0
    }
}

impl TryFromTokens<u8> for String {
    type Result = Result<Self, std::string::FromUtf8Error>;

    type Zip<Other> = Result<(Self, Other), std::string::FromUtf8Error>;

    fn try_from_tokens<T>(tokens: T) -> Self::Result
    where
        T: Iterator<Item = u8>,
    {
        String::from_utf8(Vec::from_iter(tokens))
    }

    fn try_from_reverse_tokens<T>(tokens: T) -> Self::Result
    where
        Self: Sized,
        T: Iterator<Item = u8> + DoubleEndedIterator,
    {
        let mut c = Vec::from_iter(tokens.rev());
        c.reverse();
        String::from_utf8(c)
    }

    fn zip<Other>(this: Self::Result, other: Other) -> Self::Zip<Other> {
        this.map(|l| (l, other))
    }

    fn unzip<Other>(this: Self::Zip<Other>) -> Self::Result {
        this.map(|(l, _)| l)
    }
}
