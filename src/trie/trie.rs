use std::iter::FromIterator;
use crate::inc_search::IncSearch;
use crate::iter::{Keys, KeysExt, PostfixIter, PrefixIter, SearchIter};
use crate::map;
use crate::try_collect::TryFromIterator;

/// A trie for sequences of the type `Label`.
pub struct Trie<Label>(pub(crate) map::Trie<Label, ()>);

impl<Label: Ord> Trie<Label> {
    /// Return true if `query` is an exact match.
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.0.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`.
    pub fn common_prefix_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<PrefixIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        // TODO: We could return Keys iterators instead of collecting.
        self.0.common_prefix_search(query).keys()
    }

    /// Return all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<SearchIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M> + Clone,
        Label: Clone,
    {
        self.0.predictive_search(query).keys()
    }
    /// Return the postfixes of all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<C, M>(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Keys<PostfixIter<'_, Label, (), C, M>>
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0.postfix_search(query).keys()
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
    pub fn longest_prefix<C, M>(&self, query: impl AsRef<[Label]>) -> C
    where
        C: TryFromIterator<Label, M>,
        Label: Clone,
    {
        self.0.longest_prefix(query)
    }
}

impl<Label, C> FromIterator<C> for Trie<Label>
where C: AsRef<[Label]>,
      Label: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        Self: Sized,
        T: IntoIterator<Item = C> {
        let mut builder = super::TrieBuilder::new();
        for k in iter {
            builder.push(k)
        }
        builder.build()
    }
}
