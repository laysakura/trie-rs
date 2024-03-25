use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

mod longest_prefix_iter;
mod trie;
mod trie_builder;
pub mod postfix_iter;
pub mod search_iter;
pub mod prefix_iter;

/// A trie for sequences of the type `Label`; each sequence has an associated `Value`.
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
    value: Option<Value>,
}

