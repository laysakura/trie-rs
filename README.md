# trie-rs

High performance LOUDS (Level-Order Unary Degree Sequence) library.

[Master API Docs](https://laysakura.github.io/trie-rs/trie_rs/)
|
[Released API Docs](https://docs.rs/crate/trie-rs)
|
[Benchmark Results](https://laysakura.github.io/trie-rs/criterion/report/)
|
[Changelog](https://github.com/laysakura/trie-rs/blob/master/CHANGELOG.md)

[![Build Status](https://travis-ci.com/laysakura/trie-rs.svg?branch=master)](https://travis-ci.com/laysakura/trie-rs)
[![Crates.io](https://img.shields.io/crates/v/trie-rs.svg)](https://crates.io/crates/trie-rs)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.33+-lightgray.svg)](https://github.com/laysakura/trie-rs#rust-version-supports)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-MIT)
[![License: Apache 2.0](https://img.shields.io/badge/license-Apache_2.0-blue.svg)](https://github.com/laysakura/trie-rs/blob/master/LICENSE-APACHE)

## Quickstart

To use trie-rs, add the following to your `Cargo.toml` file:

```toml
[dependencies]
trie-rs = "0.1"  # NOTE: Replace to latest minor version.
```

### Usage Overview
Say we want to hold the following tree structure in minimum length of bits.

```
(1)
 |
 |---+---+
 |   |   |
(2) (3) (4)
 |       |
 |       |---+-----+
 |       |   |     |
(5)     (6) (7)   (8)
             |     |
             |     |----+
             |     |    |
            (9)   (10) (11)
```

This tree has NodeNum (node number of 1-origin, assigned from left node to right & top to bottom) and edges.
With LOUDS, this tree is represented as the following LBS (LOUDS Bit String).

```
NodeNum       | 0 (virtual root) | 1          | 2    | 3 | 4          | 5 | 6 | 7    | 8       | 9 | 10 | 11 |
LBS           | 1  0             | 1  1  1  0 | 1  0 | 0 | 1  1  1  0 | 0 | 0 | 1  0 | 1  1  0 | 0 | 0  | 0  |
Child NodeNum | 1  -             | 2  3  4  - | 5  - | - | 6  7  8  - | - | - | 9  - | 10 11 - | - | -  | -  |
Index         | 0  1             | 2  3  4  5 | 6  7 | 8 | 9  10 11 12| 13| 14| 15 16| 17 18 19| 20| 21 | 22 |
```

The same tree is represented as follows using index.

```
<0>
 |
 |---+---+
 |   |   |
<2> <3> <4>
 |       |
 |       |---+-----+
 |       |   |     |
<6>     <9> <10>  <11>
             |     |
             |     |----+
             |     |    |
            <15>  <17> <18>
```

Then, create this tree structure with `Louds` and call operations to it.

```rust
use trie_rs::{Louds, LoudsIndex, LoudsNodeNum};

// Construct from LBS.
let s = "10_1110_10_0_1110_0_0_10_110_0_0_0";
let trie = Louds::from(s);

// LoudsNodeNum <-> LoudsIndex
let node8 = LoudsNodeNum::new(8);
let index11 = trie.node_num_to_index(&node8);
assert_eq!(trie.index_to_node_num(&index11), node8);

// Search for children.
assert_eq!(trie.parent_to_children(&node8), vec!(LoudsIndex::new(17), LoudsIndex::new(18)));

// Search for parent.
assert_eq!(trie.child_to_parent(&index11), LoudsNodeNum::new(4));
```

## Features
- **Arbitrary length support with minimum working memory**: trie-rs provides virtually _arbitrary size_ of LOUDS. It is carefully designed to use as small memory space as possible.
- **Based on [fid-rs](https://crates.io/crates/fid-rs)**, which is fast, parallelized, and memory efficient. It provides fast construction (`Louds::from()`).
- **Latest benchmark results are always accessible**: trie-rs is continuously benchmarked in Travis CI using [Criterion.rs](https://crates.io/crates/criterion). Graphical benchmark results are published [here](https://laysakura.github.io/trie-rs/criterion/report/).

### Complexity
When the number of nodes in the tree represented as LOUDS is _N_:

| Operation | Time-complexity | Space-complexity |
|-----------|-----------------|------------------|
| [Louds::from::<&str>()](https://laysakura.github.io/trie-rs/trie_rs/trie/struct.trie.html#implementations) | _O(N)_ | _N + o(N)_ |
| [node_num_to_index()](https://laysakura.github.io/trie-rs/trie_rs/struct.Louds.html#method.node_num_to_index) | _O()_ | _N + o(N)_ |
| [index_to_node_num()](https://laysakura.github.io/trie-rs/trie_rs/trie/struct.Louds.html#method.index_to_node_num) | _O(1)_ | _O(1)_ |
| [child_to_parent()](https://laysakura.github.io/trie-rs/trie_rs/trie/struct.Louds.html#method.child_to_parent) | _O(1)_ | _O(1)_ |
| [parent_to_children()](https://laysakura.github.io/trie-rs/trie_rs/trie/struct.Louds.html#method.parent_to_children) | _O( max(log N, <u>max num of children a node has</u>) )_ | _O( max(log N, <u>max num of children a node has</u>) )_ |

(`node_num_to_index()` and `child_to_parent()` use [Fid::select()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.select). `index_to_node_num()` and `parent_to_children()` use [rank()](https://laysakura.github.io/fid-rs/fid_rs/fid/struct.Fid.html#method.rank)).

## Versions
trie-rs uses [semantic versioning](http://semver.org/spec/v2.0.0.html).

Since current major version is _0_, minor version update might involve breaking public API change (although it is carefully avoided).

## Rust Version Supports

trie-rs is continuously tested with these Rust versions in Travis CI:

- 1.33.0
- Latest stable version

So it expectedly works with Rust 1.33.0 and any newer versions.

Older versions may also work, but are not tested or guaranteed.

## Contributing

Any kind of pull requests are appreciated.

### Guidelines

- `README.md` is generated from `$ cargo readme` command. Do not manually update `README.md` but edit `src/lib.rs` and then `$ cargo readme > README.md`.
- Travis CI automatically does the following commit & push to your pull-requests:
    - `$ cargo readme > README.md`
    - `$ cargo fmt --all`

## License

MIT OR Apache-2.0
