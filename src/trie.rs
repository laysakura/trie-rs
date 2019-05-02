use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

pub mod trie;
pub mod trie_builder;

pub struct Trie<Label> {
    louds: Louds,

    /// LoudsNodeNum -> Option<TrieLabel>
    ///
    /// 0 -> None
    /// 1 -> None
    /// 2.. -> Some(trie_label)
    trie_labels: Vec<Option<TrieLabel<Label>>>,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}

struct TrieLabel<Label> {
    label: Label,
    is_terminal: bool,
}
