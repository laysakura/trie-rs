// use crate::map::Trie;
// use crate::try_collect::TryFromIterator;
// use louds_rs::LoudsNodeNum;
// use std::marker::PhantomData;

// pub struct LongestPrefixIter<'a, Label, Value, Query, Collection,M> {
//     trie: &'a Trie<Label, Value>,
//     query: Query,
//     node: LoudsNodeNum,
//     index: usize,
//     col: PhantomData<(Collection, M)>,
// }

// impl<'a, Label: Ord, Value, Query: AsRef<[Label]>, C, M> LongestPrefixIter<'a, Label, Value, Query, C, M>
// where C: TryFromIterator<Label, M> {
//     #[inline]
//     pub fn new(trie: &'a Trie<Label, Value>, query: Query) -> Self {
//         Self {
//             trie,
//             query,
//             node: LoudsNodeNum(1),
//             index: 0,
//             col: PhantomData
//         }
//     }

//     pub fn value(&self) -> Option<&'a Value> {
//         self.trie.value(self.node)
//     }
// }

// impl<'a, Label: Ord, Value, Query: AsRef<[Label]>, C, M> Iterator
//     for LongestPrefixIter<'a, Label, Value, Query, C, M>
// where C: TryFromIterator<Label, M>, Label: Clone {
//     type Item = C;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(chr) = self.query.as_ref().get(self.index) {
//             let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
//             let res = self
//                 .trie
//                 .bin_search_by_children_labels(chr, &children_node_nums[..]);
//             self.index += 1;
//             match res {
//                 Ok(j) => {
//                     let child_node_num = children_node_nums[j];
//                     self.node = child_node_num;
//                     // if self.trie.is_terminal(child_node_num) {
//                     //     None
//                     // } else {
//                     Some(self.trie.label(child_node_num))
//                     // }
//                 }
//                 Err(_) => None,
//             }
//         } else if self.trie.is_terminal(self.node) {
//             None
//         } else {
//             let mut iter = self.trie.children_node_nums(self.node);
//             let first = iter.next();
//             let second = iter.next();
//             match (first, second) {
//                 (Some(child_node_num), None) => {
//                     self.node = child_node_num;
//                     Some(self.trie.label(child_node_num))
//                 }
//                 _ => None,
//             }
//         }
//     }
// }
