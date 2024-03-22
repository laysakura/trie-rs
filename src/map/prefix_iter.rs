use crate::map::{Trie, Value};
use louds_rs::LoudsNodeNum;

pub struct PrefixIter<'a, Label, Value> {
    trie: &'a Trie<Label, Value>,
    query: Vec<Label>,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
}

impl<'a, Label: Ord, Value> PrefixIter<'a, Label, Value> {
    #[inline]
    pub fn new(trie: &'a Trie<Label, Value>, mut query: Vec<Label>) -> Self {
        query.reverse();
        Self {
            trie,
            node: LoudsNodeNum(1),
            query,
            buffer: Vec::new(),
            consume: None,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
            query: Vec::new(),
            buffer: Vec::new(),
            consume: None,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.trie.value(self.node)
    }
}

impl<'a, Label: Ord, Value> Iterator for PrefixIter<'a, Label, Value>
{
    type Item = &'a Label;
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.pop() {
                let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
                let res = self
                    .trie
                    .bin_search_by_children_labels(&chr, &children_node_nums[..]);
                match res {
                    Ok(j) => {
                        let child_node_num = children_node_nums[j];
                        self.buffer.push(self.trie.label(child_node_num));
                        if self.trie.is_terminal(child_node_num) {
                            self.consume = Some(0);
                        };
                        self.node = child_node_num;
                    }
                    Err(_) => break,
                }
            } else {
                return None;
            }
        }
        if let Some(i) = self.consume.take() {
            if i >= self.buffer.len() {
                None
            } else {
                self.consume = Some(i + 1);
                Some(self.buffer[i])
            }
        } else {
            None
        }
    }
}

impl<'a, Label: Ord, V> Value<V> for frayed::defray::Group<'a, PrefixIter<'_, Label, V>> {
    fn value(&self) -> Option<&V> {
        self.parent.iter_ref().value()
    }
}
