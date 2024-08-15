use crate::map::Trie;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug)]
/// Iterates through all the common prefixes of a given query.
pub struct PrefixIter<'a, Label, I, Value, C, M> {
    trie: &'a Trie<Label, Value>,
    query: I,
    node: LoudsNodeNum,
    buffer: Vec<&'a Label>,
    consume: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, 'b, Label: Ord + Clone, Value, I: Iterator<Item = Label>, C, M>
    PrefixIter<'a, Label, I, Value, C, M>
{
    #[inline]
    pub(crate) fn new(
        trie: &'a Trie<Label, Value>,
        query: impl IntoIterator<IntoIter = I>,
    ) -> Self {
        Self {
            trie,
            query: query.into_iter(),
            node: LoudsNodeNum(1),
            buffer: Vec::new(),
            consume: None,
            col: PhantomData,
        }
    }
}

impl<'a, 'b, Label: Ord + Clone, Value, I: Iterator<Item = Label>, C, M> Iterator
    for PrefixIter<'a, Label, I, Value, C, M>
where
    C: TryFromIterator<Label, M>,
{
    type Item = (C, &'a Value);
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.next() {
                let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
                let res = self
                    .trie
                    .bin_search_by_children_labels(&chr, &children_node_nums[..]);
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
