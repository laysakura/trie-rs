use crate::internal_data_structure::naive_trie::NaiveTrie;
use louds_rs::Louds;

mod longest_prefix_iter;
mod postfix_iter;
mod prefix_iter;
mod search_iter;
mod trie;
mod trie_builder;

pub use longest_prefix_iter::LongestPrefixIter;
pub use postfix_iter::PostfixIter;
pub use prefix_iter::PrefixIter;
pub use search_iter::SearchIter;
pub mod clone;

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

/// This accesses a value if there is one. Useful on iterators.
pub trait Value<V> {
    fn value(&self) -> Option<&V>;
}
