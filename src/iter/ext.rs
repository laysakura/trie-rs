use super::{KeyIter, NodeIter, PairIter, TokenIter};
use crate::{
    map::{NodeMut, NodeRef},
    set::KeyRef,
    try_from::TryFromTokens,
};

/// Extension trait for [NodeRef] iterators.
pub trait NodeRefExt<Token> {
    /// Get tokens of nodes.
    fn tokens(self) -> TokenIter<Token, Self>
    where
        Self: Sized;
}

impl<'t, Token: 't, Value: 't, I> NodeRefExt<Token> for I
where
    I: Iterator<Item = NodeRef<'t, Token, Value>>,
{
    fn tokens(self) -> TokenIter<Token, Self> {
        TokenIter::new(self)
    }
}

/// Extension trait for [NodeMut] iterators.
pub trait NodeMutExt<'t, Token: 't, Value: 't> {
    /// Convert [NodeMut]s to [NodeRef]s.
    fn as_ref(self) -> impl Iterator<Item = NodeRef<'t, Token, Value>>;

    /// Get tokens of nodes.
    fn tokens(self) -> TokenIter<Token, Self>
    where
        Self: Sized;
}

impl<'t, Token: 't, Value: 't, I> NodeMutExt<'t, Token, Value> for I
where
    I: Iterator<Item = NodeMut<'t, Token, Value>>,
{
    fn as_ref(self) -> impl Iterator<Item = NodeRef<'t, Token, Value>> {
        self.map(|n| NodeRef {
            trie: n.trie,
            node_num: n.node_num,
        })
    }

    fn tokens(self) -> TokenIter<Token, Self> {
        TokenIter::new(self)
    }
}

/// Extension trait for [KeyRef] iterators.
pub trait KeyRefExt<Token> {
    /// Get tokens of nodes.
    fn tokens(self) -> TokenIter<Token, Self>
    where
        Self: Sized;
}

impl<'t, Token: 't, I> KeyRefExt<Token> for I
where
    I: Iterator<Item = KeyRef<'t, Token>>,
{
    fn tokens(self) -> TokenIter<Token, Self> {
        TokenIter::new(self)
    }
}

/// Extension trait for [NodeIter] iterators.
pub trait NodeIterExt {
    /// Convert node iterators to `(label, value)` pairs.
    fn pairs<L>(self) -> PairIter<Self, L>
    where
        Self: Sized;
}

impl<'t, Token: 't, Value: 't, T> NodeIterExt for T
where
    T: Iterator<Item = NodeIter<'t, Token, Value>>,
{
    fn pairs<L>(self) -> PairIter<Self, L>
    where
        Self: Sized,
    {
        PairIter::new(self)
    }
}

/// Extension trait for [KeyIter] iterators.
pub trait KeyIterExt<Token> {
    /// Convert key iterators to labels.
    fn labels<L>(self) -> impl Iterator<Item = Result<L, L::Error>>
    where
        L: TryFromTokens<Token>;
}

impl<'t, Token: Clone + 't, T> KeyIterExt<Token> for T
where
    T: Iterator<Item = KeyIter<'t, Token>>,
{
    fn labels<L>(self) -> impl Iterator<Item = Result<L, L::Error>>
    where
        L: TryFromTokens<Token>,
    {
        self.map(|i| L::try_from_reverse_tokens(i.tokens()))
    }
}
