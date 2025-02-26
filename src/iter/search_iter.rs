use crate::iter::PostfixIter;
use crate::label::Label;
use crate::map::Trie;
use crate::try_collect::{Collect, TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the matches of a label.
pub struct SearchIter<'a, Token, Value, C, M> {
    prefix: Vec<Token>,
    first: Option<(C, &'a Value)>,
    postfix_iter: PostfixIter<'a, Token, Value, Vec<Token>, Collect>,
    col: PhantomData<(C, M)>,
}

impl<'a, Token: Ord + Clone, Value, C, M> SearchIter<'a, Token, Value, C, M>
where
    C: TryFromIterator<Token, M> + Clone,
{
    pub(crate) fn new(trie: &'a Trie<Token, Value>, label: impl Label<Token>) -> Self {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut prefix = Vec::new();
        let mut children_node_nums = Vec::new(); // reuse allocated space

        // Consumes label (prefix)
        for token in label.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(trie.children_node_nums(cur_node_num));
            let res = trie.bin_search_by_children_labels(&token, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => return Self::empty(trie),
            }
            prefix.push(trie.token(cur_node_num).clone());
        }
        // let prefix:  = prefix.into_iter().try_collect().expect("Could not collect");
        let first = trie.value(cur_node_num).map(|v| {
            (
                prefix
                    .clone()
                    .into_iter()
                    .try_collect()
                    .expect("Could not collect"),
                v,
            )
        });
        SearchIter {
            prefix,
            first,
            postfix_iter: PostfixIter::new(trie, cur_node_num),
            col: PhantomData,
        }
    }

    fn empty(trie: &'a Trie<Token, Value>) -> Self {
        SearchIter {
            prefix: Vec::new(),
            first: None,
            postfix_iter: PostfixIter::empty(trie),
            col: PhantomData,
        }
    }
}

impl<'a, Token: Ord + Clone, Value, C, M> Iterator for SearchIter<'a, Token, Value, C, M>
where
    C: TryFromIterator<Token, M> + Clone,
    Vec<Token>: TryFromIterator<Token, Collect>,
{
    type Item = (C, &'a Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.first.take() {
            None => self.postfix_iter.next().map(|(postfix, v)| {
                let entry = C::try_from_iter(self.prefix.clone().into_iter().chain(postfix))
                    .expect("Could not collect postfix");
                // let mut entry = self.prefix.clone();
                // let ext: C = postfix.into_iter().try_collect().expect("Could not collect postfix");
                // entry.extend([ext]);
                (entry, v)
            }),
            x => x,
        }
    }
}
