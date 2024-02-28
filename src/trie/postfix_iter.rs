use crate::Trie;
use louds_rs::LoudsNodeNum;
use std::iter::Peekable;

pub struct UnfusedPrefix<I,J: Iterator>
{
    prefix: I,
    iter: Peekable<J>,
    consume: Option<I>,
}

impl<I,J> UnfusedPrefix<I,J>
    where I:Iterator + Clone,
    J:Iterator<Item = I::Item>
{
    pub fn new(prefix: I, iter: J) -> Self {
        let mut peekable = iter.peekable();
        let consume = peekable.peek().is_some().then(|| prefix.clone());
        UnfusedPrefix {
            iter: peekable,
            consume,
            prefix,
        }
    }

    /// Will provide prefix even if the underlying iterator is empty().
    pub fn require_prefix(mut self) -> Self {
        if self.consume.is_none() {
            self.consume = Some(self.prefix.clone());
        }
        self
    }

    fn step(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            None => {
                // eprintln!("cloning prefix with {} len", self.prefix.clone().count());
                self.consume = self.iter.peek().is_some().then(|| self.prefix.clone());
                None
            }
            x => x
        }
    }
}

impl<I,J> Iterator for UnfusedPrefix<I,J>
    where I:Iterator + Clone,
    J:Iterator<Item = I::Item>
{
    type Item=I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match self.consume {
            Some(ref mut pre) => {
                match pre.next() {
                    None => {
                        self.consume = None;
                        self.step()
                    },
                    x => x
                }
            },
            None => self.step()

        }
    }
}

pub struct PostfixIter<'a, Label>
{
    trie: &'a Trie<Label>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
    done: bool,
}

enum State {
    Consume(usize),
    Prepend(usize),
    Pop,
}

impl<'a, Label> PostfixIter<'a, Label>
{
    #[inline]
    pub fn new(trie: &'a Trie<Label>, root: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![(0, root)],
            buffer: Vec::new(),
            consume: None,
            done: false,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            buffer: Vec::new(),
            consume: None,
            done: false,
        }
    }
}

impl<'a, Label: Ord + Clone> Iterator for PostfixIter<'a, Label>
{
    type Item = &'a Label;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some((depth, node)) = self.queue.pop() {
                // eprintln!("depth {}", depth);
                let children = self.trie.children_node_nums(node);
                self.queue.extend(children.rev().map(|child| (depth + 1, child)));
                if depth == self.buffer.len() {
                    self.buffer.push(self.trie.label(node));
                } else if depth < self.buffer.len() {
                    let _ = self.buffer.drain(depth+1..);
                    self.buffer[depth] = self.trie.label(node);
                    // self.defer = Some((depth, node));
                } else {
                    panic!("depth > buffer.len()");
                }
                if self.trie.is_terminal(node) {
                    self.consume = Some(0);
                }
            } else {
                if self.done {
                    return None;
                } else {
                    // self.consume = Some(1);
                    self.done = true;
                    // eprintln!("break");
                    break;
                }
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
