use crate::Trie;
use louds_rs::LoudsNodeNum;

pub struct PostfixIter<'a, Label>
{
    inner: RefCell<PostfixInner<'a, Label>>,
    index: Cell<usize>,
}

impl<'a, Label> Iterator for PostfixIter<'a, Label> {
    type Item = &'a Label;

    fn next(&mut self) -> Option<&'a Label> {
        self.inner.borrow_mut().next();
    }
}

pub struct PostfixInner<'a, Label>
{
    trie: &'a Trie<Label>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Label>,
    consume: Option<usize>,
    defer: Option<(usize, LoudsNodeNum)>,
    done: bool,
    top_group: usize,
}

impl<'a, Label> PostfixInner<'a, Label>
{
    #[inline]
    pub fn new(trie: &'a Trie<Label>, root: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![(0, root)],
            buffer: Vec::new(),
            consume: None,
            defer: None,
            done: false,
        }
    }

    #[inline]
    pub fn empty(trie: &'a Trie<Label>) -> Self {
        Self {
            trie,
            queue: vec![],
            buffer: Vec::new(),
            consume: None,
            defer: None,
            done: false,
        }
    }
}

impl<'a, Label: Ord + Clone> Iterator for PostfixInner<'a, Label>
{
    type Item = &'a Label;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.consume.is_none() {
            if let Some((depth, node)) = self.queue.pop() {
                // eprintln!("depth {}", depth);
                let children = self.trie.children_node_nums(node);
                self.queue.extend(children.map(|child| (depth + 1, child)));
                if depth == self.buffer.len() {
                    self.buffer.push(self.trie.label(node));
                } else if depth < self.buffer.len() {
                    self.consume = Some(1);
                    self.defer = Some((depth, node));
                } else {
                    panic!("depth > buffer.len()");
                }
            } else {
                if self.done {
                    return None;
                } else {
                    self.consume = Some(1);
                    self.done = true;
                    // eprintln!("break");
                    break;
                }
            }
        }
        if let Some(i) = self.consume.take() {
            // eprintln!("consume {}", i);
            if i >= self.buffer.len() {
                if let Some((depth, node)) = self.defer.take() {
                    let _ = self.buffer.drain(depth+1..);
                    self.buffer[depth] = self.trie.label(node);
                }
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
