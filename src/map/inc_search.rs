use crate::map::Trie;
use louds_rs::LoudsNodeNum;

pub struct IncSearch<'a, Label, Value>
{
    trie: &'a Trie<Label, Value>,
    node: LoudsNodeNum,
}

#[derive(Debug, PartialEq, Eq)]
struct Answer {
    is_prefix: bool,
    is_match: bool,
}

impl Answer {
    const NO_MATCH: Answer = Answer { is_prefix: false, is_match: false };
    const PREFIX: Answer = Answer { is_prefix: true, is_match: false };
    const MATCH: Answer = Answer { is_prefix: false, is_match: true };
    const PREFIX_AND_MATCH: Answer = Answer { is_prefix: true, is_match: true };

    // pub fn new(is_prefix: bool, is_match: bool) -> Self {
    //     Answer { is_prefix, is_match }
    // }
}

impl<'a, Label: Ord, Value> IncSearch<'a, Label, Value> {
    pub fn new(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1)
        }
    }

    pub fn peek<L>(&self, chr: L) -> Answer
        where Label: PartialOrd<L> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node)
                                             .collect();
        let res = self.trie.bin_search_by_children_labels(&chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                let node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(node);
                let is_match = self.trie.value(node).is_some();
                Answer { is_prefix, is_match }
            }
            Err(_) => return Answer { is_prefix: false, is_match: false },
        }
    }

    pub fn query<L>(&mut self, chr: L) -> Answer
        where Label: PartialOrd<L> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node)
                                             .collect();
        let res = self.trie.bin_search_by_children_labels(&chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                self.node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(self.node);
                let is_match = self.trie.value(self.node).is_some();
                Answer { is_prefix, is_match }
            }
            Err(_) => return Answer::NO_MATCH,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.trie.value(self.node)
    }

    pub fn value_mut<'b>(&self, trie: &'b mut Trie<Label, Value>) -> Option<&'b mut Value> {
        trie.value_mut(self.node)
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;
    use crate::map::{Trie, TrieBuilder};

    fn build_trie() -> Trie<u8, u8> {
        let mut builder = TrieBuilder::new();
        builder.push("a", 0);
        builder.push("app", 1);
        builder.push("apple", 2);
        builder.push("better", 3);
        builder.push("application", 4);
        builder.push("アップル🍎", 5);
        builder.build()
    }

    #[test]
    fn inc_search() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(Answer::NO_MATCH, search.query(b'z'));
        assert_eq!(Answer::PREFIX_AND_MATCH, search.query(b'a'));
        assert_eq!(Answer::PREFIX, search.query(b'p'));
        assert_eq!(Answer::PREFIX_AND_MATCH, search.query(b'p'));
        assert_eq!(Answer::PREFIX, search.query(b'l'));
        assert_eq!(Answer::MATCH, search.query(b'e'));
    }
}
