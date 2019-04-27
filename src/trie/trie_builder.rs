use super::TrieBuilder;
use crate::Trie;

impl<T: Eq + Ord> TrieBuilder<T> {
    pub fn new() -> Self {
        TrieBuilder { words: vec![] }
    }

    pub fn push<U: Into<T>>(&mut self, word: U) {}

    pub fn build(&self) -> Trie<T> {
        Trie { container: vec![] }
    }
}
