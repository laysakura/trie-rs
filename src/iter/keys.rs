use crate::try_from::TryFromTokens;

use super::{KeyIter, NodeIter, TokenIter};

#[derive(Debug, Clone)]
/// Retains keys and strips off `Value`s from a [crate::iter] iterator.
pub struct Keys<I>(I);

impl<'t, I, Token: 't> Keys<I>
where
    I: Iterator<Item = NodeIter<'t, Token, ()>>,
{
    /// Creates a new `Keys` iterator.
    pub fn new(iter: I) -> Self {
        Self(iter)
    }

    /// Convert key iterators to labels.
    pub fn labels<L>(self) -> impl Iterator<Item = Result<L, L::Error>>
    where
        L: TryFromTokens<Token>,
        Token: Clone,
    {
        self.0
            .map(|iter| L::try_from_reverse_tokens(TokenIter::new(iter)))
    }
}

// TODO: This is generic for V, which is a stand-in for the Value, but in a
// `map::Trie<K,V>`, its iterators will actually reurn `(C, &V)`. Hopefully that
// won't matter.
impl<'t, I, Token: 't> Iterator for Keys<I>
where
    I: Iterator<Item = NodeIter<'t, Token, ()>>,
{
    type Item = KeyIter<'t, Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(KeyIter)
    }
}

/// Strip an iterator items `(K, V)` to only have `K`.
pub trait KeysExt<'t, Token: 't>: Iterator<Item = NodeIter<'t, Token, ()>> {
    /// Retain keys and strip values from a [crate::iter] iterator.
    fn keys(self) -> Keys<Self>
    where
        Self: Sized,
    {
        Keys::new(self)
    }
}

impl<'t, T, Token: 't> KeysExt<'t, Token> for T where
    T: Iterator<Item = NodeIter<'t, Token, ()>> + ?Sized
{
}
