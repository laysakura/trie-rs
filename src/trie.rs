use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

pub mod trie;
pub mod trie_builder;
mod postfix_iter;
mod prefix_iter;

/// A trie for sequences of the type `Label`.
pub struct Trie<Label, Value> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label, Value>>,
}

/// A trie builder for [Trie].
pub struct TrieBuilder<Label, Value> {
    naive_trie: NaiveTrie<Label, Value>,
}

struct TrieLabel<Label, Value> {
    label: Label,
    is_terminal: Option<Value>,
}
