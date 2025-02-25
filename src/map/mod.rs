//! A trie that maps `Label`s (sequences of `Token`s) to a `Value`.
use crate::internal::naive_trie::NaiveTrie;
use louds_rs::Louds;

mod trie;
mod trie_builder;

#[cfg(feature = "mem_dbg")]
use mem_dbg::MemDbg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie for `Label`s (sequences of `Token`s); each sequence has an associated `Value`.
pub struct Trie<Token, Value> {
    louds: Louds,

    /// (LoudsNodeNum - 2) -> TrieToken
    trie_tokens: Vec<TrieToken<Token, Value>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie builder for [Trie].
pub struct TrieBuilder<Token, Value> {
    naive_trie: NaiveTrie<Token, Value>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TrieToken<Token, Value> {
    token: Token,
    value: Option<Value>,
}
