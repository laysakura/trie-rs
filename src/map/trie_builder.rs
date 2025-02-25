use super::trie::*;
use crate::internal::naive_trie::NaiveTrie;
use crate::label::Label;
use louds_rs::Louds;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Token, Value> {
    naive_trie: NaiveTrie<Token, Value>,
}

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
        let mut nodes: Vec<Node<Token, Value>> = vec![];
        for naive_node in self.naive_trie.into_iter() {
            match naive_node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(n) => {
                    louds_bits.push(true);
                    nodes.push(Node {
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
        let nodes = nodes.into_boxed_slice();

        Trie { louds, nodes }
    }
}
