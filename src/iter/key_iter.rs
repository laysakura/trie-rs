use std::iter::FusedIterator;

use crate::{set::KeyRef, try_from::TryFromTokens};

use super::NodeIter;

/// Iterator over nodes in a trie.
#[derive(Clone, Copy)]
pub struct KeyIter<'t, Token>(pub(crate) NodeIter<'t, Token, ()>);

impl<'t, Token> KeyIter<'t, Token> {
    /// Convert keys to a label.
    pub fn label<L: TryFromTokens<Token>>(self) -> L::Result
    where
        Token: Clone,
    {
        let tokens = self.map(|n| n.token().clone());
        L::try_from_reverse_tokens(tokens)
    }
}

impl<'t, Token> Iterator for KeyIter<'t, Token> {
    type Item = KeyRef<'t, Token>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(KeyRef)
    }
}

impl<'t, Token> DoubleEndedIterator for KeyIter<'t, Token> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(KeyRef)
    }
}

impl<Token> FusedIterator for KeyIter<'_, Token> {}
