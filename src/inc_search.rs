//! Incremental search
//!
//! # Motivation
//!
//! The motivation for this struct is for "online" or interactive use cases. One
//! often accumulates input to match against a trie. Using the standard
//! [`exact_match()`][crate::trie::Trie::exact_match] faculties which has a time
//! complexity of _O(m log n)_ where _m_ is the label length and _n_ is
//! the number of entries in the trie. Consider this loop where we simulate
//! accumulating a query.
//!
//! ```rust
//! use trie_rs::set::Trie;
//!
//! let q = "appli"; // label
//! let mut is_match: bool;
//! let trie = Trie::<u8>::from_iter(vec!["appli", "application"]);
//! for i in 0..q.len() - 1 {
//!     assert!(!trie.is_exact(&q[0..i]));
//! }
//! assert!(trie.is_exact(q));
//! ```
//!
//! Building the query one "character" at a time and `exact_match()`ing each
//! time, the loop has effectively complexity of _O(m<sup>2</sup> log n)_.
//!
//! Using the incremental search, the time complexity of each query is _O(log
//! n)_ which returns an [LabelKind] enum.
//!
//! ```ignore
//! let q = "appli"; // label
//! let inc_search = trie.inc_search();
//! let mut is_match: bool;
//! for i = 0..q.len() {
//!     is_match = inc_search.next_kind(q[i]).unwrap().is_match();
//! }
//! ```
//!
//! This means the above code restores the time complexity of _O(m log n)_ for
//! the loop.
use crate::{
    label::{Label, LabelKind},
    map::{NodeRef, Trie},
    try_collect::{TryCollect, TryFromIterator},
};
use louds_rs::LoudsNodeNum;

#[derive(Debug, Clone)]
/// An incremental search of the trie.
pub struct IncSearch<'a, Token, Value> {
    trie: &'a Trie<Token, Value>,
    node: LoudsNodeNum,
}

/// Search position in the trie.
///
/// # Why do this?
///
/// "Position" is more descriptive for incremental search purposes, and without
/// it a user would have to explicitly depend on `louds-rs`.
pub type Position = LoudsNodeNum;

/// Retrieve the position the search is on. Useful for hanging on to a search
/// without having to fight the borrow checker because its borrowing a trie.
impl<'a, T, V> From<IncSearch<'a, T, V>> for Position {
    fn from(inc_search: IncSearch<'a, T, V>) -> Self {
        inc_search.node
    }
}

impl<'t, Token: Ord, Value> IncSearch<'t, Token, Value> {
    /// Create a new incremental search for a trie.
    pub fn new(trie: &'t Trie<Token, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
        }
    }

    /// Resume an incremental search at a particular point.
    ///
    /// ```
    /// use trie_rs::{set::Trie, label::LabelKind, inc_search::IncSearch};
    /// use louds_rs::LoudsNodeNum;
    ///
    /// let trie: Trie<u8> = ["hello", "bye"].into_iter().collect();
    /// let mut inc_search = trie.inc_search();
    ///
    /// assert_eq!(inc_search.next_label_kind("he"), Ok(LabelKind::Prefix));
    /// let position = LoudsNodeNum::from(inc_search);
    ///
    /// // inc_search is dropped.
    /// let mut inc_search2 = IncSearch::resume(&trie.0, position);
    /// assert_eq!(inc_search2.next_label_kind("llo"), Ok(LabelKind::Exact));
    ///
    /// ```
    pub fn resume(trie: &'t Trie<Token, Value>, position: Position) -> Self {
        Self {
            trie,
            node: position,
        }
    }

    /// Query the trie with a token, but only peek at the result.
    pub fn peek(&self, token: &Token) -> Option<NodeRef<'t, Token, Value>> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(token, &children_node_nums[..]);
        match res {
            Ok(j) => {
                let node_num = children_node_nums[j];
                Some(NodeRef {
                    trie: self.trie,
                    node_num,
                })
            }
            Err(_) => None,
        }
    }

    /// Query the trie with a token, but only peek at the kind.
    pub fn peek_kind(&self, token: &Token) -> Option<LabelKind> {
        self.peek(token).map(|n| n.kind())
    }

    /// Query the trie with the next token and advance if found.
    pub fn next(&mut self, token: &Token) -> Option<NodeRef<'t, Token, Value>> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(token, &children_node_nums[..]);
        match res {
            Ok(j) => {
                self.node = children_node_nums[j];
                Some(NodeRef {
                    trie: self.trie,
                    node_num: self.node,
                })
            }
            Err(_) => None,
        }
    }

    /// Query the trie with the next token and advance if found, returning the kind.
    pub fn next_kind(&mut self, token: &Token) -> Option<LabelKind> {
        self.next(token).map(|n| n.kind())
    }

    /// Advance the trie with a label. Will return `Err(index of label)` on first failure to match.
    pub fn next_label(
        &mut self,
        label: impl Label<Token>,
    ) -> Result<NodeRef<'t, Token, Value>, usize> {
        let mut result = None;
        let mut i = 0;
        for token in label.into_tokens() {
            result = self.next(&token);
            if result.is_none() {
                return Err(i);
            }
            i += 1;
        }
        result.ok_or(i)
    }

    /// Advance the trie with a label, returning the kind. Will return `Err(index of label)` on first failure to match.
    pub fn next_label_kind(&mut self, label: impl Label<Token>) -> Result<LabelKind, usize> {
        self.next_label(label).map(|n| n.kind())
    }

    /// Return the child nodes for the current prefix.
    pub fn children(&self) -> impl Iterator<Item = (&Token, Option<&Value>)> {
        self.trie
            .children_node_nums(self.node)
            .map(|node_num| (self.trie.token(node_num), self.trie.value(node_num)))
    }

    /// Return the value at current node. There should be one for any node where `kind.is_exact()` is true.
    pub fn value(&self) -> Option<&'t Value> {
        self.trie.value(self.node)
    }

    /// Go to the longest shared prefix.
    pub fn goto_longest_prefix(&mut self) -> Result<usize, usize> {
        let mut count = 0;

        while count == 0 || !self.trie.is_exact(self.node) {
            let mut iter = self.trie.children_node_nums(self.node);
            let first = iter.next();
            let second = iter.next();
            match (first, second) {
                (Some(child_node_num), None) => {
                    self.node = child_node_num;
                    count += 1;
                }
                (None, _) => {
                    assert_eq!(count, 0);
                    return Ok(count);
                }
                _ => {
                    return Err(count);
                }
            }
        }
        Ok(count)
    }

    /// Return the current prefix for this search.
    pub fn prefix<C, M>(&self) -> C
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        let mut v: Vec<Token> = self
            .trie
            .child_to_ancestors(self.node)
            .map(|node| self.trie.token(node).clone())
            .collect();
        v.reverse();
        v.into_iter().try_collect().expect("Could not collect")
    }

    /// Returne the length of the current prefix for this search.
    pub fn prefix_len(&self) -> usize {
        self.trie.child_to_ancestors(self.node).count()
    }

    /// Reset the query.
    pub fn reset(&mut self) {
        self.node = LoudsNodeNum(1);
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;
    use crate::map::{Trie, TrieBuilder};

    fn build_trie() -> Trie<u8, u8> {
        let mut builder = TrieBuilder::new();
        builder.insert("a", 0);
        builder.insert("app", 1);
        builder.insert("apple", 2);
        builder.insert("better", 3);
        builder.insert("application", 4);
        builder.insert("アップル🍎", 5);
        builder.build()
    }

    #[test]
    fn inc_search() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(0, search.prefix_len());
        assert_eq!(None, search.next(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(0, search.prefix_len());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(1, search.prefix_len());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(2, search.prefix_len());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(3, search.prefix_len());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(4, search.prefix_len());
        assert_eq!(LabelKind::Exact, search.next_kind(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(5, search.prefix_len());
    }

    #[test]
    fn inc_search_children() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(3, search.children().count());
        assert_eq!(None, search.next_kind(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(3, search.children().count());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(2, search.children().count());
        assert_eq!(
            vec![b'e', b'i'],
            search.children().map(|(c, _)| *c).collect::<Vec<_>>()
        );
        assert_eq!(LabelKind::Exact, search.next_kind(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(0, search.children().count());
    }

    #[test]
    fn inc_search_value() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(None, search.next_kind(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(LabelKind::PrefixAndExact, search.next_kind(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(LabelKind::Prefix, search.next_kind(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(LabelKind::Exact, search.next_kind(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(Some(&2), search.value());
    }

    #[test]
    fn inc_search_label() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(Err(0), search.next_label("zoo"));
        assert_eq!("", search.prefix::<String, _>());
        search.reset();
        assert_eq!(Err(1), search.next_label("blue"));
        assert_eq!("b", search.prefix::<String, _>());
        search.reset();
        assert_eq!(LabelKind::Exact, search.next_label_kind("apple").unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(Some(&2), search.value());
    }

    #[test]
    fn inc_search_goto_longest_prefix() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(Err(0), search.goto_longest_prefix());
        assert_eq!("", search.prefix::<String, _>());
        search.reset();
        assert_eq!(Ok(LabelKind::PrefixAndExact), search.next_label_kind("a"));
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(Ok(2), search.goto_longest_prefix());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(Err(1), search.goto_longest_prefix());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(Err(0), search.goto_longest_prefix());
        assert_eq!(Ok(LabelKind::Prefix), search.next_label_kind("i"));
        assert_eq!(Ok(6), search.goto_longest_prefix());
        assert_eq!(Ok(0), search.goto_longest_prefix());
        assert_eq!("application", search.prefix::<String, _>());
        search.reset();
        assert_eq!(LabelKind::Exact, search.next_label_kind("apple").unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(Some(&2), search.value());
    }
}
