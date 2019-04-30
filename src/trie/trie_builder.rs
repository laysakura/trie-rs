use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::traits::trie_methods::TrieMethods;
use crate::{Trie, TrieBuilder};
use louds_rs::{Louds, LoudsNodeNum};
use std::rc::Rc;

impl<Label: Ord + Clone> TrieBuilder<Label> {
    pub fn new() -> Self {
        let naive_trie = NaiveTrie::make_root();
        Self { naive_trie }
    }

    pub fn push<Arr: AsRef<[Label]>>(&mut self, word: Arr) {
        self.naive_trie.push(word);
    }

    pub fn build(&self) -> Trie<Label> {
        let mut louds_bits: Vec<bool> = vec![];
        let mut label_terminal_vec: Vec<Option<(Label, bool)>> = vec![None, None];

        for node in self.naive_trie.bf_iter() {
            match node {
                NaiveTrie::Root(_) => louds_bits.push(true),
                NaiveTrie::IntermOrLeaf(n) => {
                    louds_bits.push(true);
                    label_terminal_vec.push(Some((node.label(), node.is_terminal())));
                }
                NaiveTrie::PhantomSibling => louds_bits.push(false),
            }
        }

        Trie {
            current_node_num: LoudsNodeNum::new(1),
            louds: Rc::new(Louds::from(&louds_bits[..])),
            label_terminal_vec: Rc::new(label_terminal_vec),
        }
    }
}
