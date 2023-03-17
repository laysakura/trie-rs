use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

pub mod trie;
pub mod trie_builder;

pub struct Trie<K, V> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<K, V>>,
}

#[derive(Debug, Clone)]
pub struct TrieBuilder<K, V> {
    naive_trie: NaiveTrie<K, V>,
}

struct TrieLabel<K, V> {
    key: K,
    value: V,
    is_terminal: bool,
}
