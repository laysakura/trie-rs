use crate::map;
use crate::try_collect::{TryFromIterator};

use derive_deref::{Deref, DerefMut};
use std::clone::Clone;

#[derive(Deref, DerefMut)]
pub struct Trie<Label>(pub map::Trie<Label, ()>);
pub struct TrieBuilder<Label>(map::TrieBuilder<Label, ()>);

impl<Label: Ord> Trie<Label> {
    /// Return true if `query` is an exact match.
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`, cloned.
    pub fn common_prefix_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0
            .common_prefix_search(query)
            .map(|x: (C, &())| x.0)
            .collect()
    }

    /// Return all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M> + Clone,
        Label: Clone,
    {
        self.0
            .predictive_search(query)
            .map(|x: (C, &())| x.0)
            .collect()
    }
    /// Return the postfixes of all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0
            .postfix_search(query)
            .map(|x: (C, &())| x.0)
            .collect()
    }
}

impl<Label: Ord + Clone> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord + Clone> TrieBuilder<Label> {
    pub fn new() -> Self {
        Self {
            0: map::TrieBuilder::new(),
        }
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
        Trie { 0: self.0.build() }
    }
}
