use std::marker::PhantomData;

use louds_rs::LoudsNodeNum;

use crate::{label::Label, map::Trie, try_from::TryFromTokens};

#[derive(Debug, Clone)]
/// Iterates through all the common prefixes of a given query.
pub struct PrefixCollect<'t, Token, Value, L> {
    trie: &'t Trie<Token, Value>,
    tokens: Box<[Token]>,
    index: usize,
    node: LoudsNodeNum,
    /// Temp storage used across iterations.
    children: Vec<LoudsNodeNum>,
    _collector: PhantomData<L>,
}

impl<'t, Token: Ord + Clone, Value, L> PrefixCollect<'t, Token, Value, L> {
    #[inline]
    pub(crate) fn new(trie: &'t Trie<Token, Value>, label: impl Label<Token>) -> Self {
        Self {
            trie,
            tokens: label.into_tokens().collect(),
            index: 0,
            node: LoudsNodeNum(1),
            children: Vec::new(),
            _collector: PhantomData,
        }
    }
}

impl<'t, Token: Ord + Clone, Value, L> Iterator for PrefixCollect<'t, Token, Value, L>
where
    L: TryFromTokens<Token>,
{
    type Item = (L, &'t Value);
    fn next(&mut self) -> Option<Self::Item> {
        let value = loop {
            let token = self.tokens.get(self.index)?;

            self.children.clear();
            self.children
                .extend(self.trie.children_node_nums(self.node));

            let j = self
                .trie
                .bin_search_by_children_labels(token, &self.children[..])
                .ok()?;

            self.node = self.children[j];
            self.index += 1;

            if let Some(value) = self.trie.value(self.node) {
                break value;
            }
        };

        let tokens = &mut self.tokens[..self.index].into_iter().cloned();
        let label = L::try_from_tokens(tokens).unwrap();
        Some((label, value))
    }
}
