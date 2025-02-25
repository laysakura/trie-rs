//! Token streams.

/// Used to convert labels to token streams.
pub trait Label<Token> {
    /// Creates an iterator that produces tokens.
    fn into_tokens(self) -> impl Iterator<Item = Token>;
}

impl<T: Clone> Label<T> for &[T] {
    fn into_tokens(self) -> impl Iterator<Item = T> {
        self.iter().cloned()
    }
}

impl<const N: usize, T> Label<T> for [T; N] {
    fn into_tokens(self) -> impl Iterator<Item = T> {
        self.into_iter()
    }
}

impl<T> Label<T> for Vec<T> {
    fn into_tokens(self) -> impl Iterator<Item = T> {
        self.into_iter()
    }
}

impl Label<u8> for &str {
    fn into_tokens(self) -> impl Iterator<Item = u8> {
        self.as_bytes().iter().copied()
    }
}

impl Label<char> for &str {
    fn into_tokens(self) -> impl Iterator<Item = char> {
        self.chars()
    }
}

impl Label<u8> for char {
    fn into_tokens(self) -> impl Iterator<Item = u8> {
        /// An iterator over the bytes in a char.
        struct CharBytes {
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

        CharBytes::new(self)
    }
}

/// Used to convert a token iterator into a `Label<Token>` type.
pub trait ToLabel<Token>: Iterator<Item = Token> {
    /// Wraps the iterator so it can be used as a label.
    fn to_label(self) -> LabelIter<Token, Self>
    where
        Self: Sized;
}

impl<Token, T: Iterator<Item = Token>> ToLabel<Token> for T {
    fn to_label(self) -> LabelIter<Token, Self>
    where
        Self: Sized,
    {
        LabelIter(self)
    }
}

/// Newtype for any token iterators.
pub struct LabelIter<Token, Iter: Iterator<Item = Token>>(Iter);

impl<Token, Iter: Iterator<Item = Token>> Label<Token> for LabelIter<Token, Iter> {
    fn into_tokens(self) -> impl Iterator<Item = Token> {
        self.0
    }
}

#[cfg(test)]
mod label_tests {
    use super::{Label, ToLabel};

    #[test]
    fn generic_iter() {
        // let's turn a regular char iterator into a label
        let chars = "hello".chars();
        let label = chars.to_label();
        let _tokens = label.into_tokens();
    }
}
