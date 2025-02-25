use crate::label::Label;
use crate::map::Trie;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the common prefixes of a given query.
pub struct PrefixIter<'a, Token, Value, C, M> {
    trie: &'a Trie<Token, Value>,
    query: Vec<Token>,
    index: usize,
    node: LoudsNodeNum,
    buffer: Vec<&'a Token>,
    consume: Option<&'a Value>,
    col: PhantomData<(C, M)>,
    children_node_nums: Vec<LoudsNodeNum>, // reuse vec across iterations
}

impl<'a, Token: Ord + Clone, Value, C, M> PrefixIter<'a, Token, Value, C, M> {
    #[inline]
    pub(crate) fn new(trie: &'a Trie<Token, Value>, query: impl Label<Token>) -> Self {
        Self {
            trie,
            query: query.into_tokens().collect(),
            index: 0,
            node: LoudsNodeNum(1),
            buffer: Vec::new(),
            consume: None,
            col: PhantomData,
            children_node_nums: Vec::new(),
        }
    }
}

impl<'a, Token: Ord + Clone, Value, C, M> Iterator for PrefixIter<'a, Token, Value, C, M>
where
    C: TryFromIterator<Token, M>,
{
    type Item = (C, &'a Value);
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some(chr) = self.query.get(self.index) {
                self.children_node_nums.clear();
                self.children_node_nums
                    .extend(self.trie.children_node_nums(self.node));
                let res = self
                    .trie
                    .bin_search_by_children_labels(chr, &self.children_node_nums[..]);
                match res {
                    Ok(j) => {
                        let child_node_num = self.children_node_nums[j];
                        self.buffer.push(self.trie.token(child_node_num));
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
