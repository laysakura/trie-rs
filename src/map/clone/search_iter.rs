use std::marker::PhantomData;
use louds_rs::LoudsNodeNum;
use crate::try_collect::{TryFromIterator, TryCollect, Collect};
use crate::map::clone::postfix_iter::PostfixIter;
use crate::map::{Trie};

pub struct SearchIter<'a, Label, Value, C, M> {
    prefix: C,
    first: Option<(C, Value)>,
    postfix_iter: PostfixIter<'a, Label, Value, Vec<Label>, Collect>,
    value: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord + Clone, Value: Clone, C, M> SearchIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> + Extend<C> + Clone,
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
                Err(_) => return Self::empty(trie),
            }
            prefix.push(trie.label(cur_node_num).clone());
        }
        let prefix: C = prefix.into_iter().try_collect().expect("Could not collect");
        let first = trie.value(cur_node_num).map(|v| (prefix.clone(), v.clone()));
        SearchIter {
            prefix,
            first,
            value: None,
            postfix_iter: PostfixIter::new(trie, cur_node_num),
            col: PhantomData,
        }
    }

    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        let prefix = C::try_from_iter(std::iter::empty::<Label>()).expect("Could not make empty prefix");
        SearchIter {
            prefix,
            first: None,
            value: None,
            postfix_iter: PostfixIter::empty(trie),
            col: PhantomData,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.value
    }
}

impl<'a, Label: Ord + Clone, Value: Clone, C, M> Iterator for SearchIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> + Extend<C> + Clone,
Vec<Label>: TryFromIterator<Label, Collect>
{
    type Item = (C, Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.first.take() {
            // None => None,
            None => self.postfix_iter.next().map(|(postfix, v)| {
                let mut entry = self.prefix.clone();
                let ext: C = postfix.into_iter().try_collect().expect("Could not collect postfix");
                entry.extend([ext]);
                (entry, v)
            }),
            x => x
        }
    }
}

// impl<'a, Label: Ord, V> Value<V> for frayed::defray::Group<'a, SearchIter<'_, Label, V>> {
//     fn value(&self) -> Option<&V> {
//         self.parent.iter_ref().value()
//     }
// }
