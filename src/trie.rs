
use super::map::Trie as TrieMap;
use super::map::TrieBuilder as TrieMapBuilder;
use super::map::PostfixIter;
use super::map::SearchIter;
use super::map::PrefixIter;
use frayed::Chunk;
//
pub struct Trie<Label>(TrieMap<Label, ()>);
pub struct TrieBuilder<Label>(TrieMapBuilder<Label, ()>);

impl<Label: Ord> Trie<Label> {

    pub fn exact_match<L>(&self, query: impl AsRef<[L]>) -> bool
    where Label: PartialOrd<L> {
        self.0.exact_match(query).is_some()
    }

    pub fn common_prefix_search<L>(&self, query: impl AsRef<[L]>) -> Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone, L: Clone {
        self.0.common_prefix_search_ref(query.as_ref().to_vec())
            .into_iter()
            .map(|v| v.into_iter().cloned().collect())
            .collect()
    }

    pub fn common_prefix_search_ref<L>(&self, query: impl AsRef<[L]>)
                                       -> Chunk<PrefixIter<'_, L, Label, ()>>
        where Label: PartialOrd<L>, L: Clone {
        self.0.common_prefix_search_ref(query)
    }

    pub fn predictive_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone {
        let chunk = self.0.predictive_search_ref(query);
        chunk
            .map(|v| v.cloned().collect())
            .into_iter()
            .collect()
    }

    pub fn predictive_search_ref<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Chunk<SearchIter<'a, Label, ()>>
    where Label: PartialOrd<L>,
    {
        self.0.predictive_search_ref(query)
    }

    pub fn postfix_search_ref<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Chunk<PostfixIter<'a, Label, ()>>
    where Label: PartialOrd<L>
    {
        self.0.postfix_search_ref(query)
    }

    pub fn postfix_search<'a, L>(&'a self, query: impl AsRef<[L]>) ->
        Vec<Vec<Label>>
    where Label: PartialOrd<L> + Clone {
        let chunk = self.0.postfix_search_ref(query);
        chunk
            .map(|v| v.cloned().collect())
            .into_iter()
            .collect()
    }

    pub fn is_prefix<L>(&self, query: impl AsRef<[L]>) -> bool
    where Label: PartialOrd<L> {
        self.0.is_prefix(query)
    }

    // pub fn exact_match<Arr: AsRef<[K]>>(&self, query: Arr) -> Option<&V> {
}

impl<Label: Ord + Clone> TrieBuilder<Label> {

    pub fn new() -> Self {
        Self(TrieMapBuilder::new())
    }

    /// Add an entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr) {
        self.0.push(entry, ());
    }

    /// Build a [Trie].
    pub fn build(&self) -> Trie<Label> {
        Trie(self.0.build())
    }
}
