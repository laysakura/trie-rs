//! A trie that maps sequence of `Label`s to a `Value`.
use crate::internal::naive_trie::NaiveTrie;
use louds_rs::Louds;

mod trie;
mod trie_builder;

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie for sequences of the type `Label`; each sequence has an associated `Value`.
pub struct Trie<Label, Value> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieLabel
    trie_labels: Vec<TrieLabel<Label, Value>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Label, Value> {
    naive_trie: NaiveTrie<Label, Value>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TrieLabel<Label, Value> {
    label: Label,
    value: Option<Value>,
}
