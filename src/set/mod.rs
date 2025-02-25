//! A trie that sets `Label`s (sequences of `Token`s).

mod trie_builder;
mod trie_impl;

pub use trie_builder::TrieBuilder;
pub use trie_impl::Trie;
