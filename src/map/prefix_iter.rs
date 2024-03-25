use crate::map::Trie;
use louds_rs::LoudsNodeNum;
use crate::try_collect::{TryFromIterator, TryCollect};
use std::marker::PhantomData;

pub struct PrefixIter<'a, Label, Value, Query, C, M> {
    trie: &'a Trie<Label, Value>,
    query: Query,
    index: usize,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord, Value, Query, C, M> PrefixIter<'a, Label, Value, Query, C, M>
where
    Query: AsRef<[Label]>,
{
    #[inline]
    // pub fn new(trie: &'a Trie<Label, Value>, query: &'b [Label]) -> Self {
    pub fn new(trie: &'a Trie<Label, Value>, query: Query) -> Self {
        Self {
            trie,
            query,
            index: 0,
            node: LoudsNodeNum(1),
            buffer: Vec::new(),
            consume: None,
            col: PhantomData,
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

}

impl<'a, Label: Ord + Clone, Value, Query, C, M> Iterator for PrefixIter<'a, Label, Value, Query, C, M>
where
    C: TryFromIterator<Label, M>,
    Query: AsRef<[Label]>,
{
    type Item = (C, &'a Value);
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
                        self.consume = self.trie.value(child_node_num);
                        self.node = child_node_num;
                    }
                    Err(_) => break,
                }
            } else {
                return None;
            }
            self.index += 1;
        }
        if let Some(v) = self.consume.take() {
            let col = self.buffer.clone();
            Some((col.into_iter().cloned().try_collect().expect("Could not collect"), v))
        } else {
            None
        }
    }
}

// impl<'a, Label: Ord, V, Q> Value<V> for frayed::defray::Group<'a, PrefixIter<'_, Label, V, Q>>
// where
//     Q: AsRef<[Label]>,
// {
//     fn value(&self) -> Option<&V> {
//         self.parent.iter_ref().value()
//     }
// }
