use super::Trie;
use crate::{label::Label, map};

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Token>(map::TrieBuilder<Token, ()>);

impl<Token: Ord> TrieBuilder<Token> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        Self(map::TrieBuilder::new())
    }

    /// Add an entry.
    pub fn insert(&mut self, label: impl Label<Token>) {
        self.0.insert(label, ());
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Token> {
        Trie(self.0.build())
    }
}

impl<Token: Ord> Default for TrieBuilder<Token> {
    fn default() -> Self {
        Self::new()
    }
}
