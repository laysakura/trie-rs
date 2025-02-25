use core::fmt;

use louds_rs::LoudsNodeNum;

use crate::{iter::PostfixIter, label::LabelKind, try_collect::TryFromIterator};

use super::Trie;

/// A reference to a trie node.
pub struct NodeRef<'t, Token, Value> {
    pub(crate) trie: &'t Trie<Token, Value>,
    pub(crate) node_num: LoudsNodeNum,
}

impl<'t, Token, Value> NodeRef<'t, Token, Value> {
    /// Returns the kind of the node's label.
    #[inline]
    pub fn kind(&self) -> LabelKind {
        self.trie.kind(self.node_num)
    }

    /// Returns `true`` if the node's label is an exact match.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.trie.is_exact(self.node_num)
    }

    /// Returns `true`` if the node's label is a prefix match.
    #[inline]
    pub fn is_prefix(&self) -> bool {
        self.trie.is_prefix(self.node_num)
    }

    /// Returns `Some(&Value)` if the node's label is an exact match.
    #[inline]
    pub fn value(&self) -> Option<&Value> {
        self.trie.value(self.node_num)
    }

    /// Returns the token of the node.
    #[inline]
    pub fn token(&self) -> &Token {
        self.trie.token(self.node_num)
    }

    /// Iterate over child nodes.
    pub fn children(&self) -> impl Iterator<Item = NodeRef<'_, Token, Value>> {
        self.trie
            .children_node_nums(self.node_num)
            .map(|node_num| NodeRef {
                trie: self.trie,
                node_num,
            })
    }
}

impl<'t, Token: Ord, Value> NodeRef<'t, Token, Value> {
    /// Returns the postfixes and values of all children of this node.
    #[inline]
    pub fn iter_postfix<C, M>(&self) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        PostfixIter::new(&self.trie, self.node_num)
    }
}

impl<'t, Token, Value> PartialEq for NodeRef<'t, Token, Value> {
    fn eq(&self, other: &Self) -> bool {
        use std::ptr::from_ref;

        from_ref(self.trie) == from_ref(other.trie) && self.node_num == other.node_num
    }
}

impl<'t, Token, Value> Eq for NodeRef<'t, Token, Value> {}

impl<'t, Token: fmt::Debug, Value: fmt::Debug> fmt::Debug for NodeRef<'t, Token, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NodeRef")
            .field(self.token())
            .field(&self.value())
            .finish()
    }
}

/// A mutable reference to a trie node.
pub struct NodeMut<'t, Token, Value> {
    pub(crate) trie: &'t mut Trie<Token, Value>,
    pub(crate) node_num: LoudsNodeNum,
}

impl<'t, Token, Value> NodeMut<'t, Token, Value> {
    /// Converts to an immutable node reference.
    #[inline]
    pub fn as_ref(&self) -> NodeRef<'_, Token, Value> {
        NodeRef {
            trie: self.trie,
            node_num: self.node_num,
        }
    }
}

impl<'t, Token, Value> NodeMut<'t, Token, Value> {
    /// Returns the kind of the node's label.
    #[inline]
    pub fn kind(&self) -> LabelKind {
        self.trie.kind(self.node_num)
    }

    /// Returns `true`` if the node's label is a match.
    #[inline]
    pub fn is_exact(&self) -> bool {
        self.trie.is_exact(self.node_num)
    }

    /// Returns `true`` if the node's label is a prefix.
    #[inline]
    pub fn is_prefix(&self) -> bool {
        self.trie.is_prefix(self.node_num)
    }

    /// Returns `Some(&Value)` if the node's label is an exact match.
    #[inline]
    pub fn value(&self) -> Option<&Value> {
        self.trie.value(self.node_num)
    }

    /// Returns `Some(&mut Value)` if the node's label is an exact match.
    #[inline]
    pub fn value_mut(&mut self) -> Option<&mut Value> {
        self.trie.value_mut(self.node_num)
    }

    /// Returns the token of the node.
    #[inline]
    pub fn token(&self) -> &Token {
        self.trie.token(self.node_num)
    }

    /// Iterate over child nodes.
    pub fn children(&self) -> impl Iterator<Item = NodeRef<'_, Token, Value>> {
        self.trie
            .children_node_nums(self.node_num)
            .map(|node_num| NodeRef {
                trie: self.trie,
                node_num,
            })
    }
}

impl<'t, Token: Ord, Value> NodeMut<'t, Token, Value> {
    /// Returns the postfixes and values of all children of this node.
    pub fn iter_postfix<C, M>(&self) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        PostfixIter::new(&self.trie, self.node_num)
    }
}

impl<'t, Token, Value> PartialEq for NodeMut<'t, Token, Value> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref().eq(&other.as_ref())
    }
}

impl<'t, Token, Value> Eq for NodeMut<'t, Token, Value> {}

impl<'t, Token: fmt::Debug, Value: fmt::Debug> fmt::Debug for NodeMut<'t, Token, Value> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("NodeRef")
            .field(self.token())
            .field(&self.value())
            .finish()
    }
}
