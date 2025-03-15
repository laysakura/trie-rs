//! Token streams.

mod into;
mod kind;

pub use into::*;
pub use kind::*;

/// Used to convert labels to token streams.
pub trait Label<Token> {
    /// Token iterator that is produced.
    type IntoTokens: Iterator<Item = Token>;

    /// Creates an iterator that produces tokens.
    fn into_tokens(self) -> Self::IntoTokens;
}

impl<'i, T: Clone> Label<T> for &'i [T] {
    type IntoTokens = core::iter::Cloned<core::slice::Iter<'i, T>>;

    fn into_tokens(self) -> Self::IntoTokens {
        self.iter().cloned()
    }
}

impl<const N: usize, T> Label<T> for [T; N] {
    type IntoTokens = core::array::IntoIter<T, N>;

    fn into_tokens(self) -> Self::IntoTokens {
        self.into_iter()
    }
}

impl<T> Label<T> for Vec<T> {
    type IntoTokens = std::vec::IntoIter<T>;

    fn into_tokens(self) -> Self::IntoTokens {
        self.into_iter()
    }
}

impl<'i> Label<u8> for &'i str {
    type IntoTokens = core::iter::Copied<core::slice::Iter<'i, u8>>;

    fn into_tokens(self) -> Self::IntoTokens {
        self.as_bytes().iter().copied()
    }
}

impl<'i> Label<char> for &'i str {
    type IntoTokens = std::str::Chars<'i>;

    fn into_tokens(self) -> Self::IntoTokens {
        self.chars()
    }
}

impl Label<u8> for char {
    type IntoTokens = CharBytes;

    fn into_tokens(self) -> Self::IntoTokens {
        CharBytes::new(self)
    }
}

/// An iterator over the bytes in a `char`.
pub struct CharBytes {
    bytes: [u8; 4],
    idx: usize,
    len: usize,
}

impl CharBytes {
    fn new(c: char) -> Self {
        let mut bytes = [0; 4];
        let len = c.encode_utf8(&mut bytes).len();
        Self { bytes, idx: 0, len }
    }
}

impl Iterator for CharBytes {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.len {
            return None;
        }

        let byte = self.bytes[self.idx];
        self.idx += 1;
        Some(byte)
    }
}
