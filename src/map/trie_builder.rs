use crate::internal::naive_trie::NaiveTrie;
use crate::map::TrieLabel;
use crate::map::{Trie, TrieBuilder};
use louds_rs::Louds;

impl<Label: Ord, Value> Default for TrieBuilder<Label, Value> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord, Value> TrieBuilder<Label, Value> {
    /// Return a [TrieBuilder].
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    /// Add a cloneable entry and value.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr, value: Value)
    where
        Label: Clone,
    {
        self.naive_trie.push(entry.as_ref().iter().cloned(), value);
    }

    /// Add an entry and value.
    pub fn insert<Arr: IntoIterator<Item = Label>>(&mut self, entry: Arr, value: Value) {
        self.naive_trie.push(entry.into_iter(), value);
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label, Value> {
        let mut louds_bits: Vec<bool> = vec![true, false];
        let mut trie_labels: Vec<TrieLabel<Label, Value>> = vec![];
        for node in self.naive_trie.into_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(n) => {
                    louds_bits.push(true);
                    trie_labels.push(TrieLabel {
                        label: n.label,
                        value: n.value,
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
