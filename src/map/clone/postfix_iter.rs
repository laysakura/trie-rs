use std::marker::PhantomData;
use louds_rs::LoudsNodeNum;
use crate::try_collect::{TryFromIterator, TryCollect};
use crate::map::{Trie, Value};

pub struct PostfixIter<'a, Label, Value, C, M> {
    trie: &'a Trie<Label, Value>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
    value: Option<&'a Value>,
    col: PhantomData<(C, M)>,
}

impl<'a, Label, Value, C, M> PostfixIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> {
    #[inline]
    pub fn new(trie: &'a Trie<Label, Value>, root: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![(0, root)],
            buffer: Vec::new(),
            consume: None,
            value: None,
            col: PhantomData,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            buffer: Vec::new(),
            consume: None,
            value: None,
            col: PhantomData,
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.value
    }
}

impl<'a, Label: Ord + Clone, Value: Clone, C, M> Iterator for PostfixIter<'a, Label, Value, C, M>
where C: TryFromIterator<Label, M> {
    type Item = (C, Value);
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering;
        while self.consume.is_none() {
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
                if self.value.is_some() {
                    self.consume = Some(0);
                }
            } else if self.value.is_some() {
                return None;
            } else {
                self.value = None;
                break;
            }
        }
        if let Some(i) = self.consume.take() {
            Some((self.buffer[i..].iter().cloned().cloned().try_collect().expect("Could not collect"), self.value.cloned().expect("No value at terminal")))
        } else {
            None
        }
    }
}

impl<Label: Ord, V, C, M> Value<V> for PostfixIter<'_, Label, V, C, M> {
    fn value(&self) -> Option<&V> {
        self.value
    }
}
