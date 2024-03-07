
use super::map;
use super::map::{PostfixIter, SearchIter, PrefixIter, IncSearch};
use frayed::Chunk;

//
pub struct Trie<Label> { inner: map::Trie<Label, ()> }
pub struct TrieBuilder<Label> { inner: map::TrieBuilder<Label, ()> }

impl<Label: Ord> Trie<Label> {

    /// Return true if `query` is an exact match.
    pub fn exact_match<L>(&self, query: impl AsRef<[L]>) -> bool
    where Label: PartialOrd<L> {
        self.inner.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`.
    pub fn common_prefix_search_ref<L>(&self, query: impl AsRef<[L]>)
                                       -> Chunk<PrefixIter<'_, L, Label, ()>>
        where Label: PartialOrd<L>, L: Clone {
        self.inner.common_prefix_search_ref(query)
    }

    /// Return the common prefixes of `query`, cloned.
    pub fn common_prefix_search<L>(&self, query: impl AsRef<[L]>) -> Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone, L: Clone {
        self.inner.common_prefix_search_ref(query.as_ref().to_vec())
            .into_iter()
            .map(|v| v.into_iter().cloned().collect())
            .collect()
    }

    /// Return all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search_ref<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Chunk<SearchIter<'a, Label, ()>>
    where Label: PartialOrd<L>,
    {
        self.inner.predictive_search_ref(query)
    }

    /// Return all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone {
        let chunk = self.inner.predictive_search_ref(query);
        chunk
            .map(|v| v.cloned().collect())
            .into_iter()
            .collect()
    }

    /// Return the postfixes of all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search_ref<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Chunk<PostfixIter<'a, Label, ()>>
    where Label: PartialOrd<L>
    {
        self.inner.postfix_search_ref(query)
    }

    /// Return the postfixes of all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone {
        let chunk = self.inner.postfix_search_ref(query);
        chunk
            .map(|v| v.cloned().collect())
            .into_iter()
            .collect()
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix<L>(&self, query: impl AsRef<[L]>) -> bool
    where Label: PartialOrd<L> {
        self.inner.is_prefix(query)
    }

    pub fn inc_search(&self) -> IncSearch<'_, Label, ()> {
        IncSearch::new(&self.inner)
    }
}

impl<Label: Ord + Clone> TrieBuilder<Label> {

    pub fn new() -> Self {
        Self { inner: map::TrieBuilder::new() }
    }

    /// Add an entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr) {
        self.inner.push(entry, ());
    }

    /// Build a [Trie].
    pub fn build(&self) -> Trie<Label> {
        Trie { inner: self.inner.build() }
    }
}
