use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::{Louds, LoudsNodeNum};
use std::rc::Rc;

pub mod trie;
pub mod trie_builder;

pub struct Trie<Label> {
    current_node_num: LoudsNodeNum,

    louds: Rc<Louds>,

    /// LoudsNodeNum -> Option<(Label, bool)>
    ///
    /// 0 -> None
    /// 1 -> None
    /// 2.. -> Some(label, is_terminal)
    label_terminal_vec: Rc<Vec<Option<(Label, bool)>>>,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}
