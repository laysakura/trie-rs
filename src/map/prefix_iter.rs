use crate::map::{Trie, Value};
use louds_rs::LoudsNodeNum;

pub struct PrefixIter<'a, Label, Value, Query> {
    trie: &'a Trie<Label, Value>,
    query: Query,
    index: usize,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
}

impl<'a, Label: Ord, Value, Query> PrefixIter<'a, Label, Value, Query>
where
    Query: AsRef<[Label]>,
{
    #[inline]
    // pub fn new(trie: &'a Trie<Label, Value>, query: &'b [Label]) -> Self {
    pub fn new(trie: &'a Trie<Label, Value>, query: Query) -> Self {
        let mut v = Vec::new();
        for x in query.as_ref().iter() {
            v.push(x);
        }
        Self {
            trie,
            // query: query.as_ref().into_iter().collect(),
            query,
            index: 0,
            node: LoudsNodeNum(1),
            buffer: Vec::new(),
            consume: None,
        }
    }

    // #[inline]
    // pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
    //     Self {
    //         trie,
    //         query: Vec::new(),
    //         index: 0,
    //         node: LoudsNodeNum(1),
    //         buffer: Vec::new(),
    //         consume: None,
    //     }
    // }

    pub fn value(&self) -> Option<&'a Value> {
        self.trie.value(self.node)
    }
}

impl<'a, Label: Ord, Value, Query> Iterator for PrefixIter<'a, Label, Value, Query>
where
    Query: AsRef<[Label]>,
{
    type Item = &'a Label;
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.as_ref().get(self.index) {
                let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
                let res = self
                    .trie
                    .bin_search_by_children_labels(chr, &children_node_nums[..]);
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
            self.index += 1;
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

impl<'a, Label: Ord, V, Q> Value<V> for frayed::defray::Group<'a, PrefixIter<'_, Label, V, Q>>
where
    Q: AsRef<[Label]>,
{
    fn value(&self) -> Option<&V> {
        self.parent.iter_ref().value()
    }
}
