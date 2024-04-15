use crate::map::Trie;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the common prefixes of a given query.
pub struct PrefixIter<'a, Label, Value, C, M> {
    trie: &'a Trie<Label, Value>,
    query: Vec<Label>,
    index: usize,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord + Clone, Value, C, M> PrefixIter<'a, Label, Value, C, M> {
    #[inline]
    pub(crate) fn new(trie: &'a Trie<Label, Value>, query: impl AsRef<[Label]>) -> Self {
        Self {
            trie,
            query: query.as_ref().to_vec(),
            index: 0,
            node: LoudsNodeNum(1),
            buffer: Vec::new(),
            consume: None,
            col: PhantomData,
        }
    }
}

impl<'a, Label: Ord + Clone, Value, C, M> Iterator for PrefixIter<'a, Label, Value, C, M>
where
    C: TryFromIterator<Label, M>,
{
    type Item = (C, &'a Value);
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.get(self.index) {
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
            Some((
                col.into_iter()
                    .cloned()
                    .try_collect()
                    .expect("Could not collect"),
                v,
            ))
        } else {
            None
        }
    }
}
