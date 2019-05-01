use crate::internal_data_structure::naive_trie::NaiveTrie;
use crate::traits::trie_methods::TrieMethods;
use crate::trie::TrieLabel;
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
        // create louds & trie_labels first.
        let mut louds_bits: Vec<bool> = vec![true];
        let mut trie_labels: Vec<Option<TrieLabel<Label>>> = vec![None, None];
        for node in self.naive_trie.bf_iter() {
            match node {
                NaiveTrie::Root(_) => {}
                NaiveTrie::IntermOrLeaf(_) => {
                    louds_bits.push(true);
                    trie_labels.push(Some(TrieLabel {
                        label: node.label(),
                        is_terminal: node.is_terminal(),
                    }));
                }
                NaiveTrie::PhantomSibling => {
                    louds_bits.push(false);
                }
            }
        }
        let louds = Rc::new(Louds::from(&louds_bits[..]));
        let trie_labels = Rc::new(trie_labels);

        // create tries.
        // let mut tries: Vec<Option<Box<Trie<Label>>>> = vec![None, None];
        // let mut current_node_num = 2u64;
        // let dummy_tries: Rc<Vec<Option<Box<Trie<Label>>>>> = Rc::new(vec![]);
        // for node in self.naive_trie.bf_iter() {
        //     match node {
        //         NaiveTrie::IntermOrLeaf(_) => {
        //             let trie = Box::new(Trie {
        //                 current_node_num: LoudsNodeNum::new(current_node_num),
        //                 louds: louds.clone(),
        //                 trie_labels: trie_labels.clone(),
        //                 tries: dummy_tries.clone(),
        //             });
        //             tries.push(Some(trie));
        //             current_node_num += 1;
        //         }
        //         _ => {}
        //     }
        // }
        // for trie in tries {
        //     if let Some(t) = trie {
        //         t.tries = tries;
        //     }
        // }

        // returns root trie.
        Trie {
            current_node_num: LoudsNodeNum::new(1),
            louds: louds.clone(),
            trie_labels: trie_labels.clone(),
            // tries: tries.clone(),
        }
    }
}
