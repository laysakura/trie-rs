//! Memory efficient trie (prefix tree) library based on LOUDS.
//!
//! [Master API Docs](https://laysakura.github.io/trie-rs/trie_rs/)
//! |
//! [Released API Docs](https://docs.rs/crate/trie-rs)
//! |
//! [Benchmark Results](https://laysakura.github.io/trie-rs/criterion/report/)
//! |
//! [Changelog](https://github.com/laysakura/trie-rs/blob/master/CHANGELOG.md)
//!
//! [![Build Status](https://travis-ci.com/laysakura/trie-rs.svg?branch=master)](https://travis-ci.com/laysakura/trie-rs)
//! [![Crates.io Version](https://img.shields.io/crates/v/trie-rs.svg)](https://crates.io/crates/trie-rs)
//! [![Crates.io Downloads](https://img.shields.io/crates/d/trie-rs.svg)](https://crates.io/crates/trie-rs)
//! [![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/trie-rs#rust-version-supports)
//! [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-MIT)
//! [![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-APACHE)
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
//! (TBD)
//!
//! # Features
//! (TBD)
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
