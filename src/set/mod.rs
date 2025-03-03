//! A trie that sets `Label`s (sequences of `Token`s).

mod key_ref;
mod trie;
mod trie_builder;

pub use key_ref::*;
pub use trie::*;
pub use trie_builder::*;
