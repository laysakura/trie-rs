//! A trie that sets `Label`s (sequences of `Token`s).

mod key;
mod trie;
mod trie_builder;

pub use key::KeyRef;
pub use trie::Trie;
pub use trie_builder::TrieBuilder;
