//! KV capable prefix trie library based on LOUDS.
//!
//! This is a fork of https://laysakura.github.io/trie-rs/trie_rs.
//!
//! # Quickstart
//!
//! To use trie-rs, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! trie-rs = "0.1"  # NOTE: Replace to latest minor version.
//! ```
//!
//! ## Usage Overview
//! ```rust
//! use std::str;
//! use trie_rs::TrieBuilder;
//!
//! let mut builder = TrieBuilder::new();  // Inferred `TrieBuilder<u8>` automatically
//! builder.push("すし", 1);
//! builder.push("すしや", 2);
//! builder.push("すしだね", 3);
//! builder.push("すしづめ", 4);
//! builder.push("すしめし", 5);
//! builder.push("すしをにぎる", 6);
//! builder.push("すし", 7);  // Word `push`ed twice is just ignored.
//! builder.push("🍣", 8);
//!
//! let mut trie = builder.build();
//!
//! // exact_match(): Find a word exactly match to query.
//! assert_eq!(trie.exact_match("すし"), true);
//! assert_eq!(trie.exact_match("🍣"), true);
//! assert_eq!(trie.exact_match("🍜"), false);
//!
//! // predictive_search(): Find words which include `query` as their prefix.
//! let results_in_u8s: Vec<Vec<u8>> = trie.predictive_search("すし");
//! let results_in_str: Vec<&str> = results_in_u8s
//!     .iter()
//!     .map(|u8s| str::from_utf8(u8s).unwrap())
//!     .collect();
//! assert_eq!(
//!     results_in_str,
//!     vec![
//!         "すし",
//!         "すしだね",
//!         "すしづめ",
//!         "すしめし",
//!         "すしや",
//!         "すしをにぎる"
//!     ]  // Sorted by `Vec<u8>`'s order
//! );
//!
//! // common_prefix_search(): Find words which is included in `query`'s prefix.
//! let results_in_u8s: Vec<Vec<u8>> = trie.common_prefix_search("すしや");
//! let results_in_str: Vec<&str> = results_in_u8s
//!     .iter()
//!     .map(|u8s| str::from_utf8(u8s).unwrap())
//!     .collect();
//! assert_eq!(
//!     results_in_str,
//!     vec![
//!         "すし",
//!         "すしや",
//!     ]  // Sorted by `Vec<u8>`'s order
//! );
//!
//! // common_prefix_search_with_value(): Find words which is included in `query`'s prefix and return their values.
//! let results_in_u8s: Vec<(Vec<u8>, u8)> = trie.common_prefix_search_with_values("すしや");
//! let results_in_str: Vec<(&str, u8)> = results_in_u8s
//!    .iter()
//!    .map(|(u8s, v)| (str::from_utf8(u8s).unwrap(), *v))
//!    .collect();
//!
//!    assert_eq!(
//!    results_in_str,
//!    vec![
//!    ("すし", 1),
//!    ("すしや", 2),
//!    ]  // Sorted by `Vec<u8>`'s order
//!    );
//!
//! // get_value(): Get value of a word.
//! assert_eq!(trie.get("すし"), Some(&1));
//!
//! // set value in a built trie.
//! trie.set("すし", 9);
//! assert_eq!(trie.get("すし"), Some(&9));
//!
//! ```
//!
//! ## Using with Various Data Types
//! `TrieBuilder` is implemented using generic type like following:
//!
//! ```text
//! impl<Label: Ord + Clone> TrieBuilder<Label> {
//!     ...
//!     pub fn push<Arr: AsRef<[Label]>>(&mut self, word: Arr) { ... }
//!     ...
//! }
//! ```
//!
//! In the above `Usage Overview` example, we used `Label=u8, Arr=&str`.
//!
//! Here shows other `Label` and `Arr` type examples.
//!
//! ### `Label=&str, Arr=Vec<&str>`
//! Say `Label` is English words and `Arr` is English phrases.
//!
//! ```rust
//! use trie_rs::TrieBuilder;
//!
//! let mut builder = TrieBuilder::new();
//! builder.push(vec!["a", "woman"], 0 );
//! builder.push(vec!["a", "woman", "on", "the", "beach"], 1);
//! builder.push(vec!["a", "woman", "on", "the", "run"], 2);
//!
//! let trie = builder.build();
//!
//! assert_eq!(
//!     trie.exact_match(vec!["a", "woman", "on", "the", "beach"]),
//!     true
//! );
//! assert_eq!(
//!     trie.predictive_search(vec!["a", "woman", "on"]),
//!     vec![
//!         ["a", "woman", "on", "the", "beach"],
//!         ["a", "woman", "on", "the", "run"],
//!     ],
//! );
//! assert_eq!(
//!     trie.common_prefix_search(vec!["a", "woman", "on", "the", "beach"]),
//!     vec![vec!["a", "woman"], vec!["a", "woman", "on", "the", "beach"]],
//! );
//! ```
//!
//! ### `Label=u8, Arr=[u8; n]`
//! Say `Label` is a digit in Pi (= 3.14...) and Arr is a window to separate pi's digit by 10.
//!
//! ```rust
//! use trie_rs::TrieBuilder;
//!
//! let mut builder = TrieBuilder::<u8, u8>::new(); // Pi = 3.14...
//!
//! builder.push([1, 4, 1, 5, 9, 2, 6, 5, 3, 5], 1);
//! builder.push([8, 9, 7, 9, 3, 2, 3, 8, 4, 6], 2);
//! builder.push([2, 6, 4, 3, 3, 8, 3, 2, 7, 9], 3);
//! builder.push([6, 9, 3, 9, 9, 3, 7, 5, 1, 0], 4);
//! builder.push([5, 8, 2, 0, 9, 7, 4, 9, 4, 4], 5);
//! builder.push([5, 9, 2, 3, 0, 7, 8, 1, 6, 4], 6);
//! builder.push([0, 6, 2, 8, 6, 2, 0, 8, 9, 9], 7);
//! builder.push([8, 6, 2, 8, 0, 3, 4, 8, 2, 5], 8);
//! builder.push([3, 4, 2, 1, 1, 7, 0, 6, 7, 9], 9);
//! builder.push([8, 2, 1, 4, 8, 0, 8, 6, 5, 1], 10);
//! builder.push([3, 2, 8, 2, 3, 0, 6, 6, 4, 7], 11);
//! builder.push([0, 9, 3, 8, 4, 4, 6, 0, 9, 5], 12);
//! builder.push([5, 0, 5, 8, 2, 2, 3, 1, 7, 2], 13);
//! builder.push([5, 3, 5, 9, 4, 0, 8, 1, 2, 8], 14);
//!
//! let trie = builder.build();
//!
//! assert_eq!(trie.exact_match([5, 3, 5, 9, 4, 0, 8, 1, 2, 8]), true);
//! assert_eq!(
//!     trie.predictive_search([3]),
//!     vec![
//!         [3, 2, 8, 2, 3, 0, 6, 6, 4, 7],
//!         [3, 4, 2, 1, 1, 7, 0, 6, 7, 9],
//!     ],
//! );
//! assert_eq!(
//!     trie.common_prefix_search([1, 4, 1, 5, 9, 2, 6, 5, 3, 5]),
//!     vec![[1, 4, 1, 5, 9, 2, 6, 5, 3, 5]],
//! );
//! ```
//!
//! # Features
//! - **Generic type support**: As the above examples show, trie-rs can be used for searching not only UTF-8 string but also other data types.
//! - **Based on [louds-rs](https://crates.io/crates/louds-rs)**, which is fast, parallelized, and memory efficient.
//! - **Latest benchmark results are always accessible**: trie-rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/trie-rs/criterion/report/).
//!
//! # Acknowledgments
//! [`edict.furigana`](https://github.com/laysakura/trie-rs/blob/master/benches/edict.furigana) is used for benchmark.
//! This file is constructed in the following step:
//!
//! 1. Download `edict.gz` from [EDICT](http://www.edrdg.org/jmdict/edict.html).
//! 2. Convert it from original EUC into UTF-8.
//! 3. Translate it into CSV file with [edict-to-csv](https://pypi.org/project/edict-to-csv/).
//! 4. Extract field $1 for Hiragana/Katakana words, and field $3 for other (like Kanji) words.
//! 5. Translate Katakana into Hiragana with [kana2hira](https://github.com/ShellShoccar-jpn/misc-tools/blob/master/kata2hira).
//!
//! Many thanks for these dictionaries and tools.

pub use trie::Trie;
pub use trie::TrieBuilder;

mod internal_data_structure;
pub mod trie;
