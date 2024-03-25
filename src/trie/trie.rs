use crate::iter::Entries;
use crate::inc_search::IncSearch;
use crate::map;
use crate::try_collect::TryFromIterator;
use std::clone::Clone;

/// A trie for sequences of the type `Label`.
pub struct Trie<Label>(pub map::Trie<Label, ()>);

/// A trie builder for [Trie].
pub struct TrieBuilder<Label>(map::TrieBuilder<Label, ()>);

impl<Label: Ord> Trie<Label> {
    /// Return true if `query` is an exact match.
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`.
    pub fn common_prefix_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        // self.0
        //     .common_prefix_search(query)
        //     .map(|x: (C, &())| x.0)
        //     .collect()
        Entries::new(self.0.common_prefix_search(query)).collect()
    }

    /// Return all entries that match `query`.
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
    /// Return the postfixes of all entries that match `query`.
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

    /// Create an incremental search. Useful for interactive applications. See
    /// [crate::inc_search] for details.
    pub fn inc_search(&self) -> IncSearch<'_, Label, ()> {
        IncSearch::new(&self.0)
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.is_prefix(query)
    }

    /// Return the longest shared prefix of `query`.
    pub fn longest_prefix<Query, C, M>(&self, query: Query) -> C
    where
        Query: AsRef<[Label]>,
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0.longest_prefix(query)
    }
}

impl<Label: Ord + Clone> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord + Clone> TrieBuilder<Label> {
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
