//! A trie that maps `Label`s (sequences of `Token`s) to a `Value`.

mod node_mut;
mod node_ref;
mod trie;
mod trie_builder;

pub use node_mut::*;
pub use node_ref::*;
pub use trie::*;
pub use trie_builder::*;
