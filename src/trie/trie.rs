use crate::iter::Entries;
use crate::inc_search::IncSearch;
use crate::map;
use crate::try_collect::TryFromIterator;
use std::clone::Clone;

/// A trie for sequences of the type `Label`.
pub struct Trie<Label>(pub map::Trie<Label, ()>);

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
        // TODO: We could return Entries iterators instead of collecting.
        Entries::new(self.0.common_prefix_search(query))
            .collect()
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
        Entries::new(self.0.predictive_search(query))
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
        Entries::new(self.0.postfix_search(query))
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

