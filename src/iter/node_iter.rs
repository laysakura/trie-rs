use std::iter::FusedIterator;

use louds_rs::LoudsNodeNum;

use crate::{
    map::{NodeRef, Trie},
    try_from::TryFromTokens,
};

/// Iterator over nodes in a trie.
#[derive(Clone, Copy)]
pub struct NodeIter<'t, Token, Value> {
    pub(crate) trie: &'t Trie<Token, Value>,
    pub(crate) start: LoudsNodeNum,
    pub(crate) end: LoudsNodeNum,
}

impl<'t, Token, Value> NodeIter<'t, Token, Value> {
    /// Convert nodes to a `(label, value)` pair, where the last node provides the value.
    pub fn pair<L: TryFromTokens<Token>>(self) -> Result<(L, &'t Value), L::Error>
    where
        Token: Clone,
    {
        let value = self.trie.value(self.end).unwrap();

        let tokens = self.map(|n| n.token().clone());

        let label = L::try_from_reverse_tokens(tokens)?;

        Ok((label, value))
    }
}

impl<'t, Token, Value> Iterator for NodeIter<'t, Token, Value> {
    type Item = NodeRef<'t, Token, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == self.start {
            return None;
        }

        let mut curr = self.end;

        loop {
            let parent = self.trie.child_to_parent(curr);

            if parent == self.start {
                self.start = curr;
                break;
            }

            curr = parent;
        }

        self.start = curr;

        Some(NodeRef {
            trie: self.trie,
            node_num: self.start,
        })
    }
}

impl<'t, Token, Value> DoubleEndedIterator for NodeIter<'t, Token, Value> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end == self.start {
            return None;
        }

        let parent = self.trie.child_to_parent(self.end);
        let node_ref = NodeRef {
            trie: self.trie,
            node_num: self.end,
        };

        self.end = parent;

        return Some(node_ref);
    }
}

impl<Token, Value> FusedIterator for NodeIter<'_, Token, Value> {}
