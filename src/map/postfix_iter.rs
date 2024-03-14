use crate::map::Trie;
use louds_rs::LoudsNodeNum;

pub struct PostfixIter<'a, Label, Value> {
    trie: &'a Trie<Label, Value>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
    value: Option<&'a Value>,
}

impl<'a, Label, Value> PostfixIter<'a, Label, Value> {
    #[inline]
    pub fn new(trie: &'a Trie<Label, Value>, root: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![(0, root)],
            buffer: Vec::new(),
            consume: None,
            value: None,
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
        }
    }

    pub fn value(&self) -> Option<&'a Value> {
        self.value
    }
}

impl<'a, Label: Ord, Value> Iterator for PostfixIter<'a, Label, Value> {
    type Item = &'a Label;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering;
        while self.consume.is_none() {
            if let Some((depth, node)) = self.queue.pop() {
                // eprintln!("depth {}", depth);
                let children = self.trie.children_node_nums(node);
                self.queue
                    .extend(children.rev().map(|child| (depth + 1, child)));
                match depth.cmp(&self.buffer.len()) {
                    Ordering::Equal => {
                        self.buffer.push(self.trie.label(node));
                    },
                    Ordering::Less => {
                        let _ = self.buffer.drain(depth + 1..);
                        self.buffer[depth] = self.trie.label(node);
                        // self.defer = Some((depth, node));
                    },
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
                // self.consume = Some(1);
                self.value = None;
                // eprintln!("break");
                break;
            }
        }
        if let Some(i) = self.consume.take() {
            // eprintln!("consume {}", i);
            if i >= self.buffer.len() {
                None
            } else {
                self.consume = Some(i + 1);
                Some(self.buffer[i])
            }
        } else {
            None
        }
    }
}
