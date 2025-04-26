use std::iter::Peekable;

use crate::{iter::NodeIter, label::Label, map::Trie};
use louds_rs::LoudsNodeNum;

/// Iterates through all the common prefixes of a given label.
#[derive(Clone)]
pub struct PrefixIter<'t, Token, Value, Tokens: Iterator<Item = Token>> {
    trie: &'t Trie<Token, Value>,
    tokens: Peekable<Tokens>,
    curr: LoudsNodeNum,
    /// A temporary storage shared between iterations.
    children: Vec<LoudsNodeNum>,
}

impl<'t, Token, Value, Tokens: Iterator<Item = Token>> PrefixIter<'t, Token, Value, Tokens> {
    #[inline]
    pub(crate) fn new<L: Label<Token, IntoTokens = Tokens>>(
        trie: &'t Trie<Token, Value>,
        label: L,
    ) -> Self {
        Self {
            trie,
            tokens: label.into_tokens().peekable(),
            curr: LoudsNodeNum(1),
            children: Vec::new(),
        }
    }

    pub(crate) fn from_tokens(trie: &'t Trie<Token, Value>, tokens: Tokens) -> Self {
        Self {
            trie,
            tokens: tokens.peekable(),
            curr: LoudsNodeNum(1),
            children: Vec::new(),
        }
    }
}

impl<'t, Token, Value, Tokens> Iterator for PrefixIter<'t, Token, Value, Tokens>
where
    Token: Ord,
    Tokens: Iterator<Item = Token>,
{
    type Item = NodeIter<'t, Token, Value>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let token = self.tokens.next()?;

            self.children.clear();
            self.children
                .extend(self.trie.children_node_nums(self.curr));

            let idx = self
                .trie
                .bin_search_by_children_labels(&token, &self.children)
                .ok()?;
            self.curr = self.children[idx];

            if self.trie.value(self.curr).is_none() {
                continue;
            };

            return Some(NodeIter {
                trie: &self.trie,
                start: LoudsNodeNum(1),
                end: self.curr,
            });
        }
    }
}
