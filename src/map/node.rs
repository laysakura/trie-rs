use std::hint::unreachable_unchecked;

use louds_rs::LoudsNodeNum;

use crate::{iter::PostfixIter, label::LabelKind, try_collect::TryFromIterator};

use super::Trie;

/// A reference to a trie node.
pub struct NodeRef<'t, Token, Value> {
    pub(super) trie: &'t Trie<Token, Value>,
    pub(super) node_num: LoudsNodeNum,
}

impl<'t, Token: Ord, Value> NodeRef<'t, Token, Value> {
    /// Returns the postfixes and values of all children of this node.
    pub fn iter_postfix<C, M>(&self) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        PostfixIter::new(&self.trie, self.node_num)
    }

    /// Returns the kind of the node's label.
    pub fn kind(&self) -> LabelKind {
        match (self.is_prefix(), self.is_exact()) {
            (true, false) => LabelKind::Prefix,
            (false, true) => LabelKind::Match,
            (true, true) => LabelKind::PrefixAndMatch,
            // SAFETY: Since we already have the node, it must at least be a prefix or exact match.
            _ => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns `true`` if the node's label is an exact match.
    pub fn is_exact(&self) -> bool {
        self.trie.value(self.node_num).is_some()
    }

    /// Returns `true`` if the node's label is a prefix match.
    pub fn is_prefix(&self) -> bool {
        self.trie.has_children_node_nums(self.node_num)
    }

    /// Returns `Some(&Value)` if the node's label is an exact match.
    pub fn value(&self) -> Option<&Value> {
        self.trie.value(self.node_num)
    }

    /// Returns the token of the node.
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

/// A mutable reference to a trie node.
pub struct NodeMut<'t, Token, Value> {
    pub(super) trie: &'t mut Trie<Token, Value>,
    pub(super) node_num: LoudsNodeNum,
}

impl<'t, Token: Ord, Value> NodeMut<'t, Token, Value> {
    /// Converts to an immutable node reference.
    pub fn as_ref(&self) -> NodeRef<'_, Token, Value> {
        NodeRef {
            trie: self.trie,
            node_num: self.node_num,
        }
    }

    /// Returns the postfixes and values of all children of this node.
    pub fn iter_postfix<C, M>(&self) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        PostfixIter::new(&self.trie, self.node_num)
    }

    /// Returns the kind of the node's label.
    pub fn kind(&self) -> LabelKind {
        match (self.is_prefix(), self.is_match()) {
            (true, false) => LabelKind::Prefix,
            (false, true) => LabelKind::Match,
            (true, true) => LabelKind::PrefixAndMatch,
            // SAFETY: Since we already have the node, it must at least be a prefix or exact match.
            _ => unsafe { unreachable_unchecked() },
        }
    }

    /// Returns `true`` if the node's label is a match.
    pub fn is_match(&self) -> bool {
        self.trie.value(self.node_num).is_some()
    }

    /// Returns `true`` if the node's label is a prefix.
    pub fn is_prefix(&self) -> bool {
        self.trie.has_children_node_nums(self.node_num)
    }

    /// Returns `Some(&Value)` if the node's label is an exact match.
    pub fn value(&self) -> Option<&Value> {
        self.trie.value(self.node_num)
    }

    /// Returns `Some(&mut Value)` if the node's label is an exact match.
    pub fn value_mut(&mut self) -> Option<&mut Value> {
        self.trie.value_mut(self.node_num)
    }

    /// Returns the token of the node.
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
