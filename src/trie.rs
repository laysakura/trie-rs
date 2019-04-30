use crate::internal_data_structure::naive_trie::NaiveTrie;

pub mod trie;
pub mod trie_builder;

pub struct Trie<Label> {
    // louds: Louds,
    /// LoudsNodeNum -> Option<Label>
    ///
    /// 0 -> None
    /// 1 -> None
    /// 2.. -> Some
    labels: Vec<Option<Label>>,
}

pub struct TrieBuilder<Label> {
    naive_trie: NaiveTrie<Label>,
}
