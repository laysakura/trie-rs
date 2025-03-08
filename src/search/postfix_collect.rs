use std::marker::PhantomData;

use louds_rs::LoudsNodeNum;

use crate::{label::Label, map::Trie, try_from::TryFromTokens};

#[derive(Debug, Clone)]
/// Iterates through all the postfixes of a matching query.
pub struct PostfixCollect<'a, Token, Value, L> {
    trie: &'a Trie<Token, Value>,
    queue: Vec<(usize, LoudsNodeNum)>,
    buffer: Vec<&'a Token>,
    start: usize,
    _collector: PhantomData<L>,
}

impl<'a, Token: Ord, Value, L> PostfixCollect<'a, Token, Value, L>
where
    L: TryFromTokens<Token>,
{
    #[inline]
    pub(crate) fn starts_with(trie: &'a Trie<Token, Value>, label: impl Label<Token>) -> Self {
        let mut root = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space
        let mut buffer = vec![];
        let mut len = 0;

        for token in label.into_tokens() {
            len += 1;
            children_node_nums.clear();
            children_node_nums.extend(trie.children_node_nums(root));
            let res = trie.bin_search_by_children_labels(&token, &children_node_nums[..]);

            let Ok(j) = res else {
                return Self::empty(trie);
            };

            root = children_node_nums[j];
            buffer.push(trie.token(root));
        }

        Self {
            trie,
            queue: vec![(0, root)],
            buffer,
            start: len,
            _collector: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn suffixes_of(trie: &'a Trie<Token, Value>, start: LoudsNodeNum) -> Self {
        let mut queue: Vec<_> = trie.children_node_nums(start).map(|n| (0, n)).collect();
        queue.reverse();
        Self {
            trie,
            queue,
            buffer: Vec::new(),
            start: start.0 as usize + 1,
            _collector: PhantomData,
        }
    }

    #[inline]
    pub(crate) fn empty(trie: &'a Trie<Token, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            buffer: Vec::new(),
            _collector: PhantomData,
            start: 1,
        }
    }
}

impl<'t, Token: Ord + Clone, Value, L> Iterator for PostfixCollect<'t, Token, Value, L>
where
    L: TryFromTokens<Token>,
{
    type Item = L::Zip<&'t Value>;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        use std::cmp::Ordering;

        let value = loop {
            let (depth, node) = self.queue.pop()?;

            let children = self.trie.children_node_nums(node);
            self.queue
                .extend(children.rev().map(|child| (depth + 1, child)));

            match depth.cmp(&(self.buffer.len() - self.start + 1)) {
                Ordering::Equal => {
                    self.buffer.push(self.trie.token(node));
                }
                Ordering::Less => {
                    let _ = self.buffer.drain(depth + self.start..);
                    self.buffer[depth + self.start - 1] = self.trie.token(node);
                }
                Ordering::Greater => {
                    panic!("depth > buffer.len()");
                }
            }

            if let Some(value) = self.trie.value(node) {
                break value;
            }
        };

        let tokens = self.buffer.iter().cloned().cloned();
        let label = L::try_from_tokens(tokens);
        Some(L::zip(label, value))
    }
}
