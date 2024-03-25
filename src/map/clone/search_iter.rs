use std::marker::PhantomData;
use louds_rs::LoudsNodeNum;
// use crate::try_collect::{TryFromIterator, TryCollect};
use crate::try_collect::*;
use crate::map::clone::postfix_iter::PostfixIter;
use crate::map::{Trie};

pub struct SearchIter<'a, Label, Value, C, M> {
    trie: &'a Trie<Label, Value>,
    prefix: C,
    index: usize,
    first: Option<&'a Label>,
    postfix_iter: PostfixIter<'a, Label, Value, Vec<Label>, Collect>,
    value: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord + Clone, Value, C, M> SearchIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> + Extend<Label> + Clone + Default,
{
    pub fn new(
        trie: &'a Trie<Label, Value>,
        query: impl AsRef<[Label]>,
    ) -> Self {

        let mut cur_node_num = LoudsNodeNum(1);
        let mut prefix = Vec::new();

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = trie.children_node_nums(cur_node_num).collect();
            let res = trie.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    prefix.clear();
                    break;
                }
            }
            prefix.push(trie.label(cur_node_num).clone());
        }
        let _ = prefix.pop();


        SearchIter {
            trie,
            prefix: prefix.into_iter().try_collect().expect("Could not collect"),
            index: 0,
            first: None,
            value: None,
            postfix_iter: PostfixIter::new(trie, cur_node_num),
            col: PhantomData,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.value
    }
}

impl<'a, Label: Ord + Clone, Value: Clone, C, M> Iterator for SearchIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> + Extend<Label> + Clone,
Vec<Label>: TryFromIterator<Label, Collect>
{
    type Item = (C, Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.postfix_iter.next().map(|(postfix, V)| {
            let mut entry = self.prefix.clone();
            entry.extend(postfix.into_iter());
            (entry, V)
        })
    }
}

// impl<'a, Label: Ord, V> Value<V> for frayed::defray::Group<'a, SearchIter<'_, Label, V>> {
//     fn value(&self) -> Option<&V> {
//         self.parent.iter_ref().value()
//     }
// }
