use super::TrieBuilder;
use crate::Trie;

impl<Label: Ord + Clone> TrieBuilder<Label> {
    pub fn new() -> Self {
        TrieBuilder { words: vec![] }
    }

    pub fn push<Arr: AsRef<[Label]>>(&mut self, word: Arr) {}

    pub fn build(&self) -> Trie<Label> {
        Trie { container: vec![] }
    }
}
