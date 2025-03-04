use core::fmt;

use louds_rs::LoudsNodeNum;

use crate::{
    iter::{NodeIter, NodeRefExt, TokenIter},
    label::LabelKind,
    search::{PostfixIter, PrefixIter},
    trie_ref::TrieRef,
    try_from::TryFromTokens,
};

use super::Trie;

/// A reference to a trie node.
#[derive(Clone, Copy)]
pub struct NodeRef<'t, Token, Value> {
    pub(crate) trie: &'t Trie<Token, Value>,
    pub(crate) node_num: LoudsNodeNum,
}

impl<'t, Token, Value> NodeRef<'t, Token, Value> {
    /// Returns the kind of this node's label.
    pub fn kind(&self) -> LabelKind {
        self.trie.kind(self.node_num)
    }

    /// Returns `true` if this node's label is an exact match.
    pub fn is_exact(&self) -> bool {
        self.trie.is_exact(self.node_num)
    }

    /// Returns `true` if this node's label is a prefix match.
    pub fn is_prefix(&self) -> bool {
        self.trie.is_prefix(self.node_num)
    }

    /// Iterate over this node's child nodes.
    pub fn children(&'t self) -> impl Iterator<Item = NodeRef<'t, Token, Value>> {
        self.trie
            .children_node_nums(self.node_num)
            .map(|node_num| NodeRef {
                trie: self.trie,
                node_num,
            })
    }

    /// Returns the token of this node.
    pub fn token(&self) -> &Token {
        self.trie.token(self.node_num)
    }

    /// Returns the range of this node.
    pub fn range(&self) -> NodeIter<'t, Token, Value> {
        NodeIter {
            trie: &self.trie,
            start: LoudsNodeNum(1),
            end: self.node_num,
        }
    }

    /// Returns the label of this node.
    pub fn label<L: TryFromTokens<Token>>(&self) -> Result<L, L::Error>
    where
        Token: Clone,
    {
        L::try_from_reverse_tokens(self.range().map(|n| n.token().clone()))
    }

    /// Returns the exact matches that come before this node.
    ///
    /// e.g. "apple" → "app"
    pub fn prefixes_of(
        &'t self,
    ) -> PrefixIter<'t, Token, Value, TokenIter<Token, NodeIter<'t, Token, Value>>>
    where
        Token: Clone + Ord,
    {
        PrefixIter::from_tokens(&self.trie, self.range().tokens())
    }

    /// Returns the exact matches as suffixes that follow after this node.
    ///
    /// e.g. "app" → "le" (as in "apple")
    ///
    /// Strips this node from the results; to include this node as a prefix, see [`Self::starts_with`].
    pub fn suffixes_of(&'t self) -> PostfixIter<'t, Token, Value>
    where
        Token: Clone + Ord,
    {
        PostfixIter::suffixes_of(self.trie, self.node_num)
    }

    /// Returns the exact matches that follow after this node.
    ///
    /// e.g. "app" → "apple"
    #[inline]
    pub fn starts_with(&self) -> PostfixIter<'t, Token, Value>
    where
        Token: Clone + Ord,
    {
        PostfixIter::starts_with(self.trie, self.node_num)
    }
}

impl<'t, Token, Value> NodeRef<'t, Token, Value> {
    /// Returns `Some(&Value)` if the node's label is an exact match.
    #[inline]
    pub fn value(&self) -> Option<&Value> {
        self.trie.value(self.node_num)
    }
}

impl<'t, Token, Value> TrieRef<'t, Token> for NodeRef<'t, Token, Value> {
    type Ref = NodeRef<'t, Token, Value>;

    type Range = NodeIter<'t, Token, Value>;

    type Prefixes
        = PrefixIter<'t, Token, Value, TokenIter<Token, Self::Range>>
    where
        Token: Clone;

    type Suffixes = PostfixIter<'t, Token, Value>;

    fn kind(&self) -> LabelKind {
        self.kind()
    }

    fn is_exact(&self) -> bool {
        self.is_exact()
    }

    fn is_prefix(&self) -> bool {
        self.is_prefix()
    }

    fn children(&'t self) -> impl Iterator<Item = Self::Ref> {
        self.children()
    }

    fn token(&self) -> &Token {
        self.token()
    }

    fn range(&self) -> Self::Range {
        self.range()
    }

    fn label<L: TryFromTokens<Token>>(&self) -> Result<L, L::Error>
    where
        Token: Clone,
    {
        self.label()
    }

    fn prefixes_of(&'t self) -> Self::Prefixes
    where
        Token: Clone + Ord,
    {
        self.prefixes_of()
    }

    fn suffixes_of(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord,
    {
        self.suffixes_of()
    }

    fn starts_with(&'t self) -> Self::Suffixes
    where
        Token: Clone + Ord,
    {
        self.starts_with()
    }
}

impl<Token, Value> PartialEq for NodeRef<'_, Token, Value> {
    fn eq(&self, other: &Self) -> bool {
        use std::ptr::from_ref;

        from_ref(self.trie) == from_ref(other.trie) && self.node_num == other.node_num
    }
}

impl<Token, Value> Eq for NodeRef<'_, Token, Value> {}

impl<Token: fmt::Debug, Value: fmt::Debug> fmt::Debug for NodeRef<'_, Token, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NodeRef")
            .field(self.token())
            .field(&self.value())
            .finish()
    }
}
