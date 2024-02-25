use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

pub mod trie;
pub mod trie_builder;
mod postfix_iter;

/// A trie for sequences of the type `Label`.
pub struct Trie<Label> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label>>,
}

/// A trie builder for [Trie].
pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}

struct TrieLabel<Label> {
    label: Label,
    is_terminal: bool,
}
