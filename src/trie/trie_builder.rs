use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::trie::TrieLabel;
use crate::{Trie, TrieBuilder};
use louds_rs::Louds;

impl<K: Ord + Clone, V: Clone> TrieBuilder<K, V> {
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    pub fn push<Key: AsRef<[K]>>(&mut self, key: Key, value: V) {
        self.naive_trie.push(key, value);
    }

    pub fn build(&self) -> Trie<K, V> {
        let mut louds_bits: Vec<bool> = vec![true, false];
        let mut trie_labels: Vec<TrieLabel<K, V>> = vec![];
        for node in self.naive_trie.bf_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(_) => {
                    louds_bits.push(true);
                    trie_labels.push(TrieLabel {
                        key: node.label().clone(),
                        value: node.value().clone(),
                        is_terminal: node.is_terminal(),
                    });
                }
                NaiveTrie::PhantomSibling => {
                    louds_bits.push(false);
                }
            }
        }
        let louds = Louds::from(&louds_bits[..]);

        Trie { louds, trie_labels }
    }
}
