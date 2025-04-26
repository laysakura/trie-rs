use crate::iter::NodeIter;
use crate::map::Trie;
use crate::try_from::TryFromTokens;
use louds_rs::LoudsNodeNum;

use super::PostfixCollect;

/// Iterates through all the postfixes of a matching label.
#[derive(Debug, Clone)]
pub struct PostfixIter<'a, Token, Value> {
    pub(crate) trie: &'a Trie<Token, Value>,
    pub(crate) queue: Vec<LoudsNodeNum>,
    pub(crate) start: LoudsNodeNum,
    pub(crate) last: LoudsNodeNum,
}

impl<'t, Token: Ord, Value> PostfixIter<'t, Token, Value> {
    #[inline]
    pub(crate) fn starts_with(trie: &'t Trie<Token, Value>, last: LoudsNodeNum) -> Self {
        Self {
            trie,
            queue: vec![last],
            start: LoudsNodeNum(1),
            last,
        }
    }

    #[inline]
    pub(crate) fn suffixes_of(trie: &'t Trie<Token, Value>, last: LoudsNodeNum) -> Self {
        let mut queue: Vec<_> = trie.children_node_nums(last).collect();
        queue.reverse();
        Self { trie, queue, start: last, last }
    }

    #[inline]
    pub(crate) fn empty(trie: &'t Trie<Token, Value>) -> Self {
        Self {
            trie,
            queue: Vec::new(),
            start: LoudsNodeNum(1),
            last: LoudsNodeNum(1),
        }
    }

    /// Convert node iterators to `(label, value)` pairs.
    pub fn pairs<L: TryFromTokens<Token>>(self) -> PostfixCollect<'t, Token, Value, L>
    {
        PostfixCollect::from_iter(self)   
    }

    /// TODO: Docs
    pub fn suffixes(self) -> Self {
        let mut queue: Vec<_> = self.trie.children_node_nums(self.last).collect();
        queue.reverse();

        Self {
            trie: self.trie,
            queue,
            start: self.last,
            last: self.last,
        }
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
