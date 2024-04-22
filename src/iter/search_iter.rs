use crate::iter::PostfixIter;
use crate::map::Trie;
use crate::try_collect::{Collect, TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the matches of a query.
pub struct SearchIter<'a, Label, Value, C, M> {
    prefix: Vec<Label>,
    first: Option<(C, &'a Value)>,
    postfix_iter: PostfixIter<'a, Label, Value, Vec<Label>, Collect>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord + Clone, Value, C, M> SearchIter<'a, Label, Value, C, M>
where
    C: TryFromIterator<Label, M> + Clone,
{
    pub(crate) fn new(trie: &'a Trie<Label, Value>, query: impl AsRef<[Label]>) -> Self {
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

    fn empty(trie: &'a Trie<Label, Value>) -> Self {
        SearchIter {
            prefix: Vec::new(),
            first: None,
            postfix_iter: PostfixIter::empty(trie),
            col: PhantomData,
        }
    }
}

impl<'a, Label: Ord + Clone, Value, C, M> Iterator for SearchIter<'a, Label, Value, C, M>
where
    C: TryFromIterator<Label, M> + Clone,
    Vec<Label>: TryFromIterator<Label, Collect>,
{
    type Item = (C, &'a Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self.first.take() {
            // None => None,
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

// impl<'a, Label: Ord + Clone, Value, C> Iterator for SearchIter<'a, Label, Value, C, Collect>
// where C: TryFromIterator<Label, Collect> + Extend<Label> + Clone,
// Vec<Label>: TryFromIterator<Label, Collect>
// {
//     type Item = (C, &'a Value);
//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.first.take() {
//             // None => None,
//             None => self.postfix_iter.next().map(|(postfix, v)| {
//                 let mut entry = self.prefix.clone();
//                 entry.extend(postfix.into_iter());
//                 (entry, v)
//             }),
//             x => x
//         }
//     }
// }

// impl<'a, Label: Ord, V> Value<V> for frayed::defray::Group<'a, SearchIter<'_, Label, V>> {
//     fn value(&self) -> Option<&V> {
//         self.parent.iter_ref().value()
//     }
// }
