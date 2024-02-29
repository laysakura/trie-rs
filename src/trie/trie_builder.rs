use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::trie::TrieLabel;
use crate::{Trie, TrieBuilder};
use louds_rs::Louds;

impl<Label: Ord + Clone> TrieBuilder<Label> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    /// Add an entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr) -> &mut NaiveTrie<Label> {
        self.naive_trie.push(entry)
    }

    /// Build a [Trie].
    pub fn build(&self) -> Trie<Label> {
        let mut louds_bits: Vec<bool> = vec![true, false];
        let mut trie_labels: Vec<TrieLabel<Label>> = vec![];
        for node in self.naive_trie.bf_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(_) => {
                    louds_bits.push(true);
                    trie_labels.push(TrieLabel {
                        label: node.label(),
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
