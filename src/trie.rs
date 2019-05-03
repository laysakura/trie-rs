use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

pub mod trie;
pub mod trie_builder;

pub struct Trie<Label> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label>>,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}

struct TrieLabel<Label> {
    label: Label,
    is_terminal: bool,
}
