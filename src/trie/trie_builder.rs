use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::{Trie, TrieBuilder};

impl<Label: Ord + Clone> TrieBuilder<Label> {
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    pub fn push<Arr: AsRef<[Label]>>(&mut self, word: Arr) {
        self.naive_trie.push(word);
    }

    pub fn build(&self) -> Trie<Label> {
        Trie { labels: vec![] }
    }
}
