//! A trie that maps sequence of `Label`s to a `Value`.
use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

mod trie;
mod trie_builder;

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

impl<Label, Value> Clone for TrieLabel<Label, Value>
where
    Label: Clone,
    Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            value: self.value.clone(),
        }
    }
}

impl<Label, Value> Clone for Trie<Label, Value>
where
    Label: Clone,
    Value: Clone,
{
    fn clone(&self) -> Self {
        Self {
            louds: self.louds.clone(),
            trie_labels: self.trie_labels.clone(),
        }
    }
}
