#![forbid(missing_docs)]
#![doc(html_root_url = "https://docs.rs/trie-rs/0.4.1")]
#![doc = include_str!("../README.md")]

pub mod inc_search;
mod internal_data_structure;
pub mod iter;
pub mod map;
mod trie;
pub mod try_collect;
pub use trie::{Trie, TrieBuilder};
