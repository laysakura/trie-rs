use crate::iter::{NodeIter, PairIter};
use crate::map::Trie;
use louds_rs::LoudsNodeNum;

/// Iterates through all the postfixes of a matching label.
#[derive(Debug, Clone)]
pub struct PostfixIter<'a, Token, Value> {
    trie: &'a Trie<Token, Value>,
    queue: Vec<LoudsNodeNum>,
    start: LoudsNodeNum,
}

impl<'a, Token: Ord, Value> PostfixIter<'a, Token, Value> {
    #[inline]
    pub(crate) fn starts_with(trie: &'a Trie<Token, Value>, start: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![start],
            start: LoudsNodeNum(1),
        }
    }

    #[inline]
    pub(crate) fn suffixes_of(trie: &'a Trie<Token, Value>, start: LoudsNodeNum) -> Self {
        let mut queue: Vec<_> = trie.children_node_nums(start).collect();
        queue.reverse();
        Self { trie, queue, start }
    }

    #[inline]
    pub(crate) fn empty(trie: &'a Trie<Token, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            start: LoudsNodeNum(1),
        }
    }

    /// Convert node iterators to `(label, value)` pairs.
    pub fn pairs<L>(self) -> PairIter<Self, L>
    where
        Self: Sized,
    {
        PairIter::new(self)
    }
}

impl<'t, Token: Ord + Clone, Value> Iterator for PostfixIter<'t, Token, Value> {
    type Item = NodeIter<'t, Token, Value>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut end: Option<LoudsNodeNum> = None;

        while end.is_none() {
            let Some(node) = self.queue.pop() else {
                break;
            };

            if node.0 == 1 {
                continue;
            }

            let children = self.trie.children_node_nums(node);
            self.queue.extend(children.rev());

            if self.trie.value(node).is_some() {
                end = Some(node);
            }
        }

        end.map(|end| NodeIter {
            trie: &self.trie,
            start: self.start,
            end,
        })
    }
}
