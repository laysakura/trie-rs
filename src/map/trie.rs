//! A trie map stores a value with each word or key.
use super::Trie;
use crate::map::inc_search::IncSearch;
use crate::map::postfix_iter::PostfixIter;
use crate::map::prefix_iter::PrefixIter;
use crate::map::search_iter::SearchIter;
use frayed::Chunk;
use louds_rs::{self, ChildNodeIter, LoudsNodeNum};

impl<Label: Ord, Value> Trie<Label, Value> {
    /// Return `Some(&value)` if query is an exact match.
    pub fn exact_match<L>(&self, query: impl AsRef<[L]>) -> Option<&Value>
    where
        Label: PartialOrd<L>,
    {
        self.exact_match_node(query)
            .and_then(move |x| self.value(x))
    }

    /// Return `Node` if query is an exact match.
    #[inline]
    fn exact_match_node<L>(&self, query: impl AsRef<[L]>) -> Option<LoudsNodeNum>
    where
        Label: PartialOrd<L>,
    {
        let mut cur_node_num = LoudsNodeNum(1);

        for (i, chr) in query.as_ref().iter().enumerate() {
            let children_node_nums: Vec<LoudsNodeNum> =
                self.children_node_nums(cur_node_num).collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if i == query.as_ref().len() - 1 && self.is_terminal(child_node_num) {
                        return Some(child_node_num);
                    }
                    cur_node_num = child_node_num;
                }
                Err(_) => return None,
            }
        }
        None
    }

    /// Return `Some(&mut value)` if query is an exact match.
    pub fn exact_match_mut<L>(&mut self, query: impl AsRef<[L]>) -> Option<&mut Value>
    where
        Label: PartialOrd<L>,
    {
        self.exact_match_node(query)
            .and_then(move |x| self.value_mut(x))
    }

    /// Create an incremental search.
    pub fn inc_search(&self) -> IncSearch<'_, Label, Value> {
        IncSearch::new(self)
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix<L>(&self, query: impl AsRef<[L]>) -> bool
    where
        Label: PartialOrd<L>,
    {
        let mut cur_node_num = LoudsNodeNum(1);

        for chr in query.as_ref().iter() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num).collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(j) => cur_node_num = children_node_nums[j],
                Err(_) => return false,
            }
        }
        // Are there more nodes after our query?
        self.has_children_node_nums(cur_node_num)
    }

    /// Return all entries and their values that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<'a, L>(&'a self, query: impl AsRef<[L]>) -> Vec<(Vec<Label>, Value)>
    where
        Label: PartialOrd<L> + Clone,
        Value: Clone,
    {
        let chunk = self.predictive_search_ref(query);
        chunk
            .map(|v| {
                (
                    v.cloned().collect(),
                    chunk.iter_ref().value().cloned().unwrap(),
                )
            })
            .into_iter()
            .collect()
    }

    /// Return all entries and their values that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search_ref<'a, L>(
        &'a self,
        query: impl AsRef<[L]>,
    ) -> Chunk<SearchIter<'a, Label, Value>>
    where
        Label: PartialOrd<L>,
    {
        assert!(!query.as_ref().is_empty());
        let mut cur_node_num = LoudsNodeNum(1);
        let mut prefix = Vec::new();

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num).collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => return Chunk::new(SearchIter::empty(self)),
            }
            prefix.push(cur_node_num);
        }
        let _ = prefix.pop();
        Chunk::new(SearchIter::new(self, prefix, cur_node_num))
    }

    /// Return the postfixes and values of all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<'a, L>(&'a self, query: impl AsRef<[L]>) -> Vec<(Vec<Label>, Value)>
    where
        Label: PartialOrd<L> + Clone,
        Value: Clone,
    {
        let chunk = self.postfix_search_ref(query);
        chunk
            .map(|v| {
                (
                    v.cloned().collect(),
                    chunk.iter_ref().value().cloned().unwrap(),
                )
            })
            .into_iter()
            .collect()
    }

    /// Return the postfixes and values of all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search_ref<'a, L>(
        &'a self,
        query: impl AsRef<[L]>,
    ) -> Chunk<PostfixIter<'a, Label, Value>>
    where
        Label: PartialOrd<L>,
    {
        assert!(!query.as_ref().is_empty());
        let mut cur_node_num = LoudsNodeNum(1);

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num).collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    return Chunk::new(PostfixIter::empty(self));
                }
            }
        }
        Chunk::new(PostfixIter::new(self, cur_node_num))
    }

    /// Return the common prefixes of `query`, cloned.
    pub fn common_prefix_search<L>(&self, query: impl AsRef<[L]>) -> Vec<(Vec<Label>, Value)>
    where
        Label: PartialOrd<L> + Clone,
        L: Clone,
        Value: Clone,
    {
        let chunk = self.common_prefix_search_ref(query.as_ref().to_vec());
        chunk
            .map(|v| {
                (
                    v.cloned().collect(),
                    chunk.iter_ref().value().cloned().unwrap(),
                )
            })
            .into_iter()
            .collect()
    }

    /// Return the common prefixes and values of `query`.
    pub fn common_prefix_search_ref<L>(
        &self,
        query: impl AsRef<[L]>,
    ) -> Chunk<PrefixIter<'_, L, Label, Value>>
    where
        Label: PartialOrd<L>,
        L: Clone,
    {
        Chunk::new(PrefixIter::new(&self, query.as_ref().to_vec()))
    }

    pub(crate) fn has_children_node_nums(&self, node_num: LoudsNodeNum) -> bool {
        self.louds
            .parent_to_children_indices(node_num)
            .next()
            .is_some()
    }

    pub(crate) fn children_node_nums(&self, node_num: LoudsNodeNum) -> ChildNodeIter {
        self.louds.parent_to_children_nodes(node_num)
    }

    pub(crate) fn bin_search_by_children_labels<L>(
        &self,
        query: &L,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize>
    where
        Label: PartialOrd<L>,
    {
        children_node_nums.binary_search_by(|child_node_num| {
            self.label(*child_node_num).partial_cmp(query).unwrap()
        })
    }

    pub(crate) fn label(&self, node_num: LoudsNodeNum) -> &Label {
        &self.trie_labels[(node_num.0 - 2) as usize].label
    }

    pub(crate) fn is_terminal(&self, node_num: LoudsNodeNum) -> bool {
        self.trie_labels[(node_num.0 - 2) as usize]
            .value
            .is_some()
    }

    pub(crate) fn value(&self, node_num: LoudsNodeNum) -> Option<&Value> {
        self.trie_labels[(node_num.0 - 2) as usize]
            .value
            .as_ref()
    }

    pub(crate) fn value_mut(&mut self, node_num: LoudsNodeNum) -> Option<&mut Value> {
        self.trie_labels[(node_num.0 - 2) as usize]
            .value
            .as_mut()
    }
}
