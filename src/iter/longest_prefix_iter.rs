use crate::map::Trie;
use louds_rs::LoudsNodeNum;

/// An iterator to find the longest shared prefix. This is useful in cases where
/// one wants to offer tab-completion.
pub struct LongestPrefixIter<'a, Label, Value, Query> {
    trie: &'a Trie<Label, Value>,
    query: Query,
    node: LoudsNodeNum,
    index: usize,
}

impl<'a, Label: Ord, Value, Query: AsRef<[Label]>> LongestPrefixIter<'a, Label, Value, Query> {
    #[inline]
    pub(crate) fn new(trie: &'a Trie<Label, Value>, query: Query) -> Self {
        Self {
            trie,
            query,
            node: LoudsNodeNum(1),
            index: 0,
        }
    }
}

impl<'a, Label: Ord, Value, Query: AsRef<[Label]>> Iterator
    for LongestPrefixIter<'a, Label, Value, Query>
{
    type Item = &'a Label;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chr) = self.query.as_ref().get(self.index) {
            let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
            let res = self
                .trie
                .bin_search_by_children_labels(chr, &children_node_nums[..]);
            self.index += 1;
            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    self.node = child_node_num;
                    // if self.trie.is_terminal(child_node_num) {
                    //     None
                    // } else {
                    Some(self.trie.label(child_node_num))
                    // }
                }
                Err(_) => None,
            }
        } else if self.trie.is_terminal(self.node) {
            None
        } else {
            let mut iter = self.trie.children_node_nums(self.node);
            let first = iter.next();
            let second = iter.next();
            match (first, second) {
                (Some(child_node_num), None) => {
                    self.node = child_node_num;
                    Some(self.trie.label(child_node_num))
                }
                _ => None,
            }
        }
    }
}
