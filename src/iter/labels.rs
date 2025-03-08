use std::marker::PhantomData;

use crate::try_from::TryFromTokens;

/// Returns labels from `(label, value)` iterators.
pub struct Labels<I, L, Token>(I, PhantomData<L>, PhantomData<Token>);

impl<I, L, Token> Labels<I, L, Token> {
    pub(crate) fn new(iter: I) -> Self {
        Self(iter, PhantomData, PhantomData)
    }
}

impl<I, L, Token> Iterator for Labels<I, L, Token>
where
    L: TryFromTokens<Token>,
    I: Iterator<Item = L::Zip<&'static ()>>,
{
    type Item = L::Result;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(L::unzip)
    }
}
