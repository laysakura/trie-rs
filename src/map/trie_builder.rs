use crate::internal::naive_trie::NaiveTrie;
use crate::label::Label;
use crate::map::TrieToken;
use crate::map::{Trie, TrieBuilder};
use louds_rs::Louds;

impl<Token: Ord, Value> Default for TrieBuilder<Token, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Token: Ord, Value> TrieBuilder<Token, Value> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    /// Insert a value for a given label.
    pub fn insert(&mut self, label: impl Label<Token>, value: Value) {
        self.naive_trie.insert(label.into_tokens(), value);
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Token, Value> {
        let mut louds_bits: Vec<bool> = vec![true, false];
        let mut trie_tokens: Vec<TrieToken<Token, Value>> = vec![];
        for node in self.naive_trie.into_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(n) => {
                    louds_bits.push(true);
                    trie_tokens.push(TrieToken {
                        token: n.token,
                        value: n.value,
                    });
                }
                NaiveTrie::PhantomSibling => {
                    louds_bits.push(false);
                }
            }
        }
        let louds = Louds::from(&louds_bits[..]);

        Trie { louds, trie_tokens }
    }
}
