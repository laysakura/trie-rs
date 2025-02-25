use crate::map::Trie;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::LoudsNodeNum;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
/// Iterates through all the postfixes of a matching query.
pub struct PostfixIter<'a, Token, Value, C, M> {
    trie: &'a Trie<Token, Value>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Token>,
    value: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Token: Ord, Value, C, M> PostfixIter<'a, Token, Value, C, M>
where
    C: TryFromIterator<Token, M>,
{
    #[inline]
    pub(crate) fn new(trie: &'a Trie<Token, Value>, root: LoudsNodeNum) -> Self {
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
    pub(crate) fn empty(trie: &'a Trie<Token, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            buffer: Vec::new(),
            value: None,
            col: PhantomData,
        }
    }
}

impl<'a, Token: Ord + Clone, Value, C, M> Iterator for PostfixIter<'a, Token, Value, C, M>
where
    C: TryFromIterator<Token, M>,
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
                        self.buffer.push(self.trie.token(node));
                    }
                    Ordering::Less => {
                        let _ = self.buffer.drain(depth + 1..);
                        self.buffer[depth] = self.trie.token(node);
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

// impl<Token: Ord, V, C, M> Value<V> for PostfixIter<'_, Token, V, C, M> {
//     fn value(&self) -> Option<&V> {
//         self.value
//     }
// }
