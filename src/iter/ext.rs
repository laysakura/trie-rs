use super::TokenIter;
use crate::{
    map::{NodeMut, NodeRef},
    set::KeyRef,
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
