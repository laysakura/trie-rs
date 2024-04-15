use super::Trie;
use crate::map;

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Label>(map::TrieBuilder<Label, ()>);

impl<Label: Ord> TrieBuilder<Label> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        Self(map::TrieBuilder::new())
    }

    /// Add a cloneable entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr)
    where
        Label: Clone,
    {
        self.0.push(entry, ());
    }

    /// Add an entry.
    pub fn insert<Arr: IntoIterator<Item = Label>>(&mut self, entry: Arr) {
        self.0.insert(entry, ());
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label> {
        Trie(self.0.build())
    }
}

impl<Label: Ord> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}
