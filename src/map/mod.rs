//! A trie that maps `Label`s (sequences of `Token`s) to a `Value`.

mod node;
mod trie;
mod trie_builder;

pub use node::*;
pub use trie::*;
pub use trie_builder::*;
