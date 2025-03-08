use std::marker::PhantomData;

use super::NodeIter;
use crate::try_from::TryFromTokens;

/// Iterator that converts node ranges to `(label, value)` pairs.
pub struct PairIter<I, L> {
    iter: I,
    _collector: PhantomData<L>,
}

impl<I, L> PairIter<I, L> {
    pub(crate) fn new(iter: I) -> Self {
        Self {
            iter,
            _collector: PhantomData,
        }
    }
}

impl<'t, Token: Clone + 't, Value: 't, I, L> Iterator for PairIter<I, L>
where
    I: Iterator<Item = NodeIter<'t, Token, Value>>,
    L: TryFromTokens<Token>,
{
    type Item = L::Zip<&'t Value>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| i.pair::<L>())
    }
}
