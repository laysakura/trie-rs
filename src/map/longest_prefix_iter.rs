use crate::map::Trie;
use louds_rs::LoudsNodeNum;

pub struct LongestPrefixIter<'a, Label, Value> {
    trie: &'a Trie<Label, Value>,
    query: Vec<Label>,
    node: LoudsNodeNum,
    index: usize,
}

impl<'a, Label: Ord, Value> LongestPrefixIter<'a, Label, Value> {
    #[inline]
    pub fn new(trie: &'a Trie<Label, Value>, query: Vec<Label>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
            query,
            index: 0,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
            query: Vec::new(),
            index: 0,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.trie.value(self.node)
    }
}

impl<'a, Label: Ord, Value> Iterator for LongestPrefixIter<'a, Label, Value> {
    type Item = &'a Label;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(chr) = self.query.get(self.index) {
            let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
            let res = self
                .trie
                .bin_search_by_children_labels(&chr, &children_node_nums[..]);
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
        } else {
            if self.trie.is_terminal(self.node) {
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
}
