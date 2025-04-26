use std::{iter::FusedIterator, marker::PhantomData};

use crate::trie_ref::TrieRef;

/// Iterator over tokens in a trie range.
pub struct TokenIter<Token, I>(pub(crate) I, PhantomData<Token>);

impl<Token, I> TokenIter<Token, I> {
    pub(crate) fn new(iter: I) -> Self {
        Self(iter, PhantomData)
    }
}

impl<'t, Token: Clone + 't, I, R: TrieRef<'t, Token>> Iterator for TokenIter<Token, I>
where
    I: Iterator<Item = R>,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|n| n.token().clone())
    }
}

impl<'t, Token: Clone + 't, I, R: TrieRef<'t, Token>> DoubleEndedIterator for TokenIter<Token, I>
where
    I: DoubleEndedIterator<Item = R>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|n| n.token().clone())
    }
}

impl<'t, Token: Clone + 't, I, R: TrieRef<'t, Token>> FusedIterator for TokenIter<Token, I> where
    I: DoubleEndedIterator<Item = R>
{
}
