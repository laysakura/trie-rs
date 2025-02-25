#![forbid(missing_docs)]
#![doc(html_root_url = "https://docs.rs/trie-rs/0.4.2")]
#![doc = include_str!("../README.md")]

pub mod inc_search;
mod internal;
pub mod iter;
pub mod label;
pub mod map;
mod trie;
pub mod try_collect;
pub use trie::{Trie, TrieBuilder};
