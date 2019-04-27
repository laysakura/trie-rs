use super::TrieBuilder;
use crate::Trie;

impl<Elm: Eq + Ord + Clone> TrieBuilder<Elm> {
    pub fn new() -> Self {
        TrieBuilder { words: vec![] }
    }

    pub fn push<Arr: AsRef<[Elm]>>(&mut self, word: Arr) {}

    pub fn build(&self) -> Trie<Elm> {
        Trie { container: vec![] }
    }
}
