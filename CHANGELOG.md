# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.3.1]
- Added derives across all data structures, including:
  - `Clone`
  - `Debug`
  - serde's `Serialize` & `Deserialize`, available under the feature `serde`.
  - [mem-dbg](https://github.com/zommiommy/mem_dbg-rs)'s `MemDbg` & `MemSize`, available under the feature `mem-dbg`.
- Moved rayon, which is optionally used in dependency crates, under the feature `rayon`.
- Switched from Travis-CI to GitHub Actions.
- Fixed several code smells as reported by clippy.
- Extended documentation & doctests.

## [v0.3.0]
- Use iterators for search results.

  Benefits being that they're lazy, short-circuitable, and require less memory.

```
let a: Vec<Vec<u8>> = trie.predictive_search("ech").take(10).collect();
```

- Allow `Label` collection type to be specified.

  This includes machinery in `crate::try_collect` to allow us to collect into
  `String` directly.

```
let a: Vec<Vec<u8>> = trie.predictive_search("ech").collect();
let b: Vec<String> = trie.predictive_search("ech").collect();
```

- Add incremental search.

  Lets the user build their query one label at a time.
```
let mut builder = TrieBuilder::new();
builder.push("a", 0);
builder.push("app", 1);
builder.push("apple", 2);
let trie = builder.build();
let mut search = trie.inc_search();
assert_eq!(None, search.query(&b'z'));
assert_eq!(Answer::PrefixAndMatch, search.query(&b'a').unwrap());
assert_eq!(Answer::Prefix, search.query(&b'p').unwrap());
assert_eq!(Answer::PrefixAndMatch, search.query(&b'p').unwrap());
assert_eq!(Answer::Prefix, search.query(&b'l').unwrap());
assert_eq!(Answer::Match, search.query(&b'e').unwrap());
```
  
  If your search can be _O(log n)_ instead of _O(m log n)_, do that.

- Add `Trie::postfix_search()`.
- Add `map::Trie::exact_match_mut()` to mutate `Value`s.
- Add `Trie::longest_prefix()`.
  
  Find the longest prefix. This is the kind of behavior one would want to implement tab completion for instance.

- No longer panics on zero-length string queries.

  Previously a zero-length query would instantiate the entirety of the trie
  essentially uncompressed. Now, however, an iterator only allocates one word at
  a time, and one can limit their search results to avoid whole trie collection.
  
```
let b: Vec<String> = trie.predictive_search("").take(100).collect();
```
- Make Trie cloneable.

## [v0.2.0]

- Add `trie_rs::map::{Trie, TrieBuilder}` ([#20](https://github.com/laysakura/trie-rs/pull/20))
- Add `is_prefix()`.

## [v0.1.1]
Only internal data type change.

## [v0.1.0]
Initial release.

[Unreleased]: https://github.com/laysakura/trie-rs/compare/v0.2.0...HEAD
[v0.2.0]: <https://github.com/laysakura/trie-rs/compare/v0.1.1...v0.2.0>
[v0.1.1]: https://github.com/laysakura/trie-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/laysakura/trie-rs/compare/699e53d...v0.1.0
