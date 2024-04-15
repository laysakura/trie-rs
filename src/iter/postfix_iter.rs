use crate::map::Trie;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the postfixes of a matching query.
pub struct PostfixIter<'a, Label, Value, C, M> {
    trie: &'a Trie<Label, Value>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Label>,
    value: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label: Ord, Value, C, M> PostfixIter<'a, Label, Value, C, M>
where
    C: TryFromIterator<Label, M>,
{
    #[inline]
    pub(crate) fn new(trie: &'a Trie<Label, Value>, root: LoudsNodeNum) -> Self {
        let mut children: Vec<_> = trie.children_node_nums(root).map(|n| (0, n)).collect();
        children.reverse();
        Self {
            trie,
            queue: children,
            buffer: Vec::new(),
            value: None,
            col: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn empty(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            buffer: Vec::new(),
            value: None,
            col: PhantomData,
        }
    }
}

impl<'a, Label: Ord + Clone, Value, C, M> Iterator for PostfixIter<'a, Label, Value, C, M>
where
    C: TryFromIterator<Label, M>,
{
    type Item = (C, &'a Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering;
        while self.value.is_none() {
            if let Some((depth, node)) = self.queue.pop() {
                let children = self.trie.children_node_nums(node);
                self.queue
                    .extend(children.rev().map(|child| (depth + 1, child)));
                match depth.cmp(&self.buffer.len()) {
                    Ordering::Equal => {
                        self.buffer.push(self.trie.label(node));
                    }
                    Ordering::Less => {
                        let _ = self.buffer.drain(depth + 1..);
                        self.buffer[depth] = self.trie.label(node);
                    }
                    Ordering::Greater => {
                        panic!("depth > buffer.len()");
                    }
                }
                self.value = self.trie.value(node);
            } else {
                break;
            }
        }
        if let Some(v) = self.value.take() {
            Some((
                self.buffer
                    .iter()
                    .cloned()
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

// impl<Label: Ord, V, C, M> Value<V> for PostfixIter<'_, Label, V, C, M> {
//     fn value(&self) -> Option<&V> {
//         self.value
//     }
// }
