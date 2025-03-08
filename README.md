# trie-rs

Memory efficient trie (prefix tree) and map library based on LOUDS.

[Master API Docs](https://laysakura.github.io/trie-rs/trie_rs/)
|
[Released API Docs](https://docs.rs/crate/trie-rs)
|
[Benchmark Results](https://laysakura.github.io/trie-rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/trie-rs/blob/master/CHANGELOG.md)

[![GitHub Actions Status](https://github.com/laysakura/trie-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/laysakura/trie-rs/actions)
[![Crates.io Version](https://img.shields.io/crates/v/trie-rs.svg)](https://crates.io/crates/trie-rs)
[![Crates.io Downloads](https://img.shields.io/crates/d/trie-rs.svg)](https://crates.io/crates/trie-rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.56+-lightgray.svg)](https://github.com/laysakura/trie-rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-APACHE)

## Quickstart

To use trie-rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
trie-rs = "0.4.2"
```

### Usage Overview
```rust
use std::str;
use trie_rs::set::TrieBuilder;

let mut builder = TrieBuilder::<u8>::new();
builder.insert("すし");
builder.insert("すしや");
builder.insert("すしだね");
builder.insert("すしづめ");
builder.insert("すしめし");
builder.insert("すしをにぎる");
builder.insert("すし");  // Word `push`ed twice is just ignored.
builder.insert("🍣");

let trie = builder.build();

// is_exact(): Find a node that exactly matches a label.
assert_eq!(trie.is_exact("すし"), true);
assert_eq!(trie.is_exact("🍣"), true);
assert_eq!(trie.is_exact("🍜"), false);

// start_with(): Find words which include the label as their prefix.
let results_in_u8s: Vec<_> = trie
    .starts_with("すし")
    .labels::<Vec<_>>()
    .collect();
let results_in_str: Vec<_> = trie
    .starts_with("すし")
    .labels::<String>()
    .filter_map(Result::ok)
    .collect();
assert_eq!(
    results_in_str,
    vec![
        "すし",
        "すしだね",
        "すしづめ",
        "すしめし",
        "すしや",
        "すしをにぎる"
    ]  // Sorted by `Vec<u8>`'s order
);

// prefixes_of(): Find words that are included in a label's prefix.
let results_in_u8s: Vec<_> = trie
    .prefixes_of("すしや")
    .labels::<Vec<_>>()
    .collect();
let results_in_str: Vec<_> = trie
    .prefixes_of("すしや")
    .labels::<String>()
    .filter_map(Result::ok)
    .collect();
assert_eq!(
    results_in_str,
    vec![
        "すし",
        "すしや",
    ]  // Sorted by `Vec<u8>`'s order
);
```

### Using with Various Data Types
`TrieBuilder` is implemented using generic type like following:

```ignore
impl<Token: Ord> TrieBuilder<Token> {
    ...
    pub fn insert(&mut self, label: impl Label<Token>) { ... }
    ...
}
```

In the above `Usage Overview` example, we used `Token=u8, Label=&str`.

Here are some other `Token` and `Label` type examples.

#### `Token=&str, Label=Vec<&str>`
Say `Token` is English words and `Label` is English phrases.

```rust
use trie_rs::set::TrieBuilder;

let mut builder = TrieBuilder::new();
builder.insert(vec!["a", "woman"]);
builder.insert(vec!["a", "woman", "on", "the", "beach"]);
builder.insert(vec!["a", "woman", "on", "the", "run"]);

let trie = builder.build();

assert_eq!(
    trie.is_exact(vec!["a", "woman", "on", "the", "beach"]),
    true
);

let r: Vec<_> = trie.starts_with(vec!["a", "woman", "on"])
    .labels::<Vec<_>>()
    .collect();
assert_eq!(
    r,
    vec![
        ["a", "woman", "on", "the", "beach"],
        ["a", "woman", "on", "the", "run"],
    ],
);

let s: Vec<_> = trie
    .prefixes_of(vec!["a", "woman", "on", "the", "beach"])
    .labels::<Vec<_>>()
    .collect();
assert_eq!(
    s,
    vec![vec!["a", "woman"], vec!["a", "woman", "on", "the", "beach"]],
);
```

#### `Label=u8, Arr=[u8; n]`
Say `Label` is a digit in Pi (= 3.14...) and Arr is a window to separate pi's digit by 10.

```rust
use trie_rs::set::TrieBuilder;

let mut builder = TrieBuilder::<u8>::new(); // Pi = 3.14...

builder.insert([1, 4, 1, 5, 9, 2, 6, 5, 3, 5]);
builder.insert([8, 9, 7, 9, 3, 2, 3, 8, 4, 6]);
builder.insert([2, 6, 4, 3, 3, 8, 3, 2, 7, 9]);
builder.insert([6, 9, 3, 9, 9, 3, 7, 5, 1, 0]);
builder.insert([5, 8, 2, 0, 9, 7, 4, 9, 4, 4]);
builder.insert([5, 9, 2, 3, 0, 7, 8, 1, 6, 4]);
builder.insert([0, 6, 2, 8, 6, 2, 0, 8, 9, 9]);
builder.insert([8, 6, 2, 8, 0, 3, 4, 8, 2, 5]);
builder.insert([3, 4, 2, 1, 1, 7, 0, 6, 7, 9]);
builder.insert([8, 2, 1, 4, 8, 0, 8, 6, 5, 1]);
builder.insert([3, 2, 8, 2, 3, 0, 6, 6, 4, 7]);
builder.insert([0, 9, 3, 8, 4, 4, 6, 0, 9, 5]);
builder.insert([5, 0, 5, 8, 2, 2, 3, 1, 7, 2]);
builder.insert([5, 3, 5, 9, 4, 0, 8, 1, 2, 8]);

let trie = builder.build();

assert_eq!(trie.is_exact([5, 3, 5, 9, 4, 0, 8, 1, 2, 8]), true);

let t: Vec<_> = trie
    .starts_with([3])
    .labels::<Vec<_>>()
    .collect();
assert_eq!(
    t,
    vec![
        [3, 2, 8, 2, 3, 0, 6, 6, 4, 7],
        [3, 4, 2, 1, 1, 7, 0, 6, 7, 9],
    ],
);

let u: Vec<_> = trie
    .prefixes_of([1, 4, 1, 5, 9, 2, 6, 5, 3, 5])
    .labels::<Vec<_>>()
    .collect();
assert_eq!(
    u,
    vec![[1, 4, 1, 5, 9, 2, 6, 5, 3, 5]],
);
```

### Trie Map Usage

To store a value with each word, use `trie_rs::map::{Trie, TrieBuilder}`.

```rust
use std::str;
use trie_rs::map::TrieBuilder;

let mut builder = TrieBuilder::<u8, u8>::new();
builder.insert("すし", 0);
builder.insert("すしや", 1);
builder.insert("すしだね", 2);
builder.insert("すしづめ", 3);
builder.insert("すしめし", 4);
builder.insert("すしをにぎる", 5);
builder.insert("すし", 6);  // Word `push`ed twice uses last value.
builder.insert("🍣", 7);

let mut trie = builder.build();

// get_value(): Find the value for an exact match.
assert_eq!(trie.get_value("すし"), Some(&6));
assert_eq!(trie.get_value("🍣"), Some(&7));
assert_eq!(trie.get_value("🍜"), None);

// Values can be modified.
let v = trie.get_value_mut("🍣").unwrap();
*v = 8;
assert_eq!(trie.get_value("🍣"), Some(&8));
```

### Incremental Search

For interactive applications, one can use an incremental search to get the
best performance. See [IncSearch][crate::inc_search::IncSearch].

```rust
use std::str;
use trie_rs::{set::TrieBuilder, label::LabelKind};

let mut builder = TrieBuilder::new();  // Inferred `TrieBuilder<u8, u8>` automatically
builder.insert("ab");
builder.insert("すし");
builder.insert("すしや");
builder.insert("すしだね");
builder.insert("すしづめ");
builder.insert("すしめし");
builder.insert("すしをにぎる");
let trie = builder.build();
let mut search = trie.inc_search();

// Query by the byte with `.next()` or `.next_kind()`.
assert_eq!(search.next_kind(&b'a'), Some(LabelKind::Prefix));
assert_eq!(search.next_kind(&b'c'), None);
assert_eq!(search.next_kind(&b'b'), Some(LabelKind::Exact));

// Reset the query to go again.
search.reset();

// For unicode its easier to use `.next_label()` or `.next_label_kind()`.
assert_eq!(search.next_label_kind('す'), Ok(LabelKind::Prefix));
assert_eq!(search.next_label_kind('し'), Ok(LabelKind::PrefixAndExact));
assert_eq!(search.next_label_kind('や'), Ok(LabelKind::Exact));
assert_eq!(search.next_kind(&b'a'), None);
assert_eq!(search.next_label_kind('a'), Err(0));

search.reset();
assert_eq!(search.next_label_kind("ab-NO-MATCH-"), Err(2)); // No match on byte at index 2.
```

## Features
- **Generic type support**: As the above examples show, trie-rs can be used for searching not only UTF-8 string but also other data types.
- **Based on [louds-rs](https://crates.io/crates/louds-rs)**, which is fast, parallelized, and memory efficient.
- **Latest benchmark results are always accessible**: trie-rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/trie-rs/criterion/report/).
- `map::Trie` associates a `Value` with each entry.
- `Value` does not require any traits.
- `Token: Clone` not required to create `Trie<Token>` but useful for many reifying search operations like `predictive_search()`.
- Many search operations are implemented via iterators which are lazy, require less memory, and can be short circuited.
- Incremental search available for "online" applications, i.e., searching one `Label` at a time.

## Cargo features

- "rayon"

Enables [rayon](https://crates.io/crates/rayon) a data parallelism library.

- "mem_dbg"

Can determine the size in bytes of nested data structures like the trie itself.

- "serde"

Can serialize and deserialize the trie.

## Acknowledgments
[`edict.furigana`](https://github.com/laysakura/trie-rs/blob/master/benches/edict.furigana) is used for benchmark.
This file is constructed in the following step:

1. Download `edict.gz` from [EDICT](http://www.edrdg.org/jmdict/edict.html).
2. Convert it from original EUC into UTF-8.
3. Translate it into CSV file with [edict-to-csv](https://pypi.org/project/edict-to-csv/).
4. Extract field $1 for Hiragana/Katakana words, and field $3 for other (like Kanji) words.
5. Translate Katakana into Hiragana with [kana2hira](https://github.com/ShellShoccar-jpn/misc-tools/blob/master/kata2hira).

Many thanks for these dictionaries and tools.

## Versions
trie-rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

## Rust Version Supports

trie-rs is continuously tested with these Rust versions in with the github CI:

- 1.75.0 with all features
- 1.67.0 with no features
- Latest stable version

So it is expected to work with Rust 1.75.0 and any newer versions.

Older versions may also work but are not tested or guaranteed.

### Earlier Rust Verion Supports

If support for Rust prior to 1.67.0 is required, trie-rs 0.2.0 supports Rust 1.33.0 and later.

## Contributing

Any kind of pull requests are appreciated.

## License

MIT OR Apache-2.0
