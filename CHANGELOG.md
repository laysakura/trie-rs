# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
- Use iterators for search results

  Some benefits are that they're lazy, short-circuitable, and require less
  memory.

```
let a: Vec<Vec<u8>> = trie.predictive_search("ech").take(10).collect();
```

- Allow `Label` collection type to be specified

```
let a: Vec<Vec<u8>> = trie.predictive_search("ech").collect();
let b: Vec<String> = trie.predictive_search("ech").collect();
```

- Add incremental search
  
  If your search can be _O(log n)_ instead of _O(m log n)_, do that.

- Add `Trie::postfix_search()`
- Add `map::Trie::exact_match_mut()` to mutate map `Value`s
- Add `Trie::longest_prefix()`
- No longer panics on zero-length string queries

## [v0.2.0]

- feat: Add `trie_rs::map::{Trie, TrieBuilder}` ([#20](https://github.com/laysakura/trie-rs/pull/20))
- feat: Add `is_prefix()`.

## [v0.1.1]
Only internal data type change.

## [v0.1.0]
Initial release.

[Unreleased]: https://github.com/laysakura/trie-rs/compare/v0.2.0...HEAD
[v0.2.0]: <https://github.com/laysakura/trie-rs/compare/v0.1.1...v0.2.0>
[v0.1.1]: https://github.com/laysakura/trie-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/laysakura/trie-rs/compare/699e53d...v0.1.0
