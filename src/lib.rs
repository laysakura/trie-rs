//! High performance LOUDS (Level-Order Unary Degree Sequence) library.
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
//! [![Crates.io](https://img.shields.io/crates/v/trie-rs.svg)](https://crates.io/crates/trie-rs)
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

pub use trie::Trie;
pub use trie::TrieBuilder;

mod internal_data_structure;
pub mod trie;
