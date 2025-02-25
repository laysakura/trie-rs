//! Incremental search
//!
//! # Motivation
//!
//! The motivation for this struct is for "online" or interactive use cases. One
//! often accumulates input to match against a trie. Using the standard
//! [`exact_match()`][crate::trie::Trie::exact_match] faculties which has a time
//! complexity of _O(m log n)_ where _m_ is the query string length and _n_ is
//! the number of entries in the trie. Consider this loop where we simulate
//! accumulating a query.
//!
//! ```rust
//! use trie_rs::set::Trie;
//!
//! let q = "appli"; // query string
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
//! let q = "appli"; // query string
//! let inc_search = trie.inc_search();
//! let mut is_match: bool;
//! for i = 0..q.len() {
//!     is_match = inc_search.query(q[i]).unwrap().is_match();
//! }
//! ```
//!
//! This means the above code restores the time complexity of _O(m log n)_ for
//! the loop.
use crate::{
    label::{Label, LabelKind},
    map::Trie,
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

impl<'a, Token: Ord, Value> IncSearch<'a, Token, Value> {
    /// Create a new incremental search for a trie.
    pub fn new(trie: &'a Trie<Token, Value>) -> Self {
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
    /// assert_eq!(inc_search.query_until("he"), Ok(LabelKind::Prefix));
    /// let position = LoudsNodeNum::from(inc_search);
    ///
    /// // inc_search is dropped.
    /// let mut inc_search2 = IncSearch::resume(&trie.0, position);
    /// assert_eq!(inc_search2.query_until("llo"), Ok(LabelKind::Match));
    ///
    /// ```
    pub fn resume(trie: &'a Trie<Token, Value>, position: Position) -> Self {
        Self {
            trie,
            node: position,
        }
    }

    /// Query but do not change the node we're looking at on the trie.
    pub fn peek(&self, chr: &Token) -> Option<LabelKind> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                let node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(node);
                let is_match = self.trie.value(node).is_some();
                LabelKind::new(is_prefix, is_match)
            }
            Err(_) => None,
        }
    }

    /// Query the trie and go to node if there is a match.
    pub fn query(&mut self, chr: &Token) -> Option<LabelKind> {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                self.node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(self.node);
                let is_match = self.trie.value(self.node).is_some();
                LabelKind::new(is_prefix, is_match)
            }
            Err(_) => None,
        }
    }

    /// Query the trie with a sequence. Will return `Err(index of query)` on
    /// first failure to match.
    pub fn query_until(&mut self, query: impl Label<Token>) -> Result<LabelKind, usize> {
        let mut result = None;
        let mut i = 0;
        for chr in query.into_tokens() {
            result = self.query(&chr);
            if result.is_none() {
                return Err(i);
            }
            i += 1;
        }
        result.ok_or(i)
    }

    /// Return the child nodes for the current prefix.
    pub fn children(&self) -> impl Iterator<Item = (&Token, Option<&Value>)> {
        self.trie
            .children_node_nums(self.node)
            .map(|node_num| (self.trie.token(node_num), self.trie.value(node_num)))
    }

    /// Return the value at current node. There should be one for any node where
    /// `answer.is_match()` is true.
    pub fn value(&self) -> Option<&'a Value> {
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
        // TODO: If PR for child_to_ancestors is accepted. Use the iterator and
        // remove `pub(crate)` from Trie.louds field. Also uncomment prefix()
        // above.

        self.trie.child_to_ancestors(self.node).count()

        // let mut node = self.node;
        // let mut count = 0;
        // while node.0 > 1 {
        //     let index = self.trie.louds.node_num_to_index(node);
        //     node = self.trie.louds.child_to_parent(index);
        //     count += 1;
        // }
        // count
    }

    // This isn't actually possible.
    // /// Return the mutable value at current node. There should be one for any
    // /// node where `answer.is_match()` is true.
    // ///
    // /// Note: Because [IncSearch] does not store a mutable reference to the
    // /// trie, a mutable reference must be provided.
    // pub fn value_mut<'b>(self, trie: &'b mut Trie<Token, Value>) -> Option<&'b mut Value> {
    //     trie.value_mut(self.node)
    // }

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
        assert_eq!(None, search.query(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(0, search.prefix_len());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(1, search.prefix_len());
        assert_eq!(LabelKind::Prefix, search.query(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(2, search.prefix_len());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(3, search.prefix_len());
        assert_eq!(LabelKind::Prefix, search.query(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(4, search.prefix_len());
        assert_eq!(LabelKind::Match, search.query(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(5, search.prefix_len());
    }

    #[test]
    fn inc_search_children() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(3, search.children().count());
        assert_eq!(None, search.query(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(3, search.children().count());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::Prefix, search.query(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(1, search.children().count());
        assert_eq!(LabelKind::Prefix, search.query(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(2, search.children().count());
        assert_eq!(
            vec![b'e', b'i'],
            search.children().map(|(c, _)| *c).collect::<Vec<_>>()
        );
        assert_eq!(LabelKind::Match, search.query(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(0, search.children().count());
    }

    #[test]
    fn inc_search_value() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(None, search.query(&b'z'));
        assert_eq!("", search.prefix::<String, _>());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'a').unwrap());
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(LabelKind::Prefix, search.query(&b'p').unwrap());
        assert_eq!("ap", search.prefix::<String, _>());
        assert_eq!(LabelKind::PrefixAndMatch, search.query(&b'p').unwrap());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(LabelKind::Prefix, search.query(&b'l').unwrap());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(LabelKind::Match, search.query(&b'e').unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(Some(&2), search.value());
    }

    #[test]
    fn inc_search_query_until() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(Err(0), search.query_until("zoo"));
        assert_eq!("", search.prefix::<String, _>());
        search.reset();
        assert_eq!(Err(1), search.query_until("blue"));
        assert_eq!("b", search.prefix::<String, _>());
        search.reset();
        assert_eq!(LabelKind::Match, search.query_until("apple").unwrap());
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
        assert_eq!(Ok(LabelKind::PrefixAndMatch), search.query_until("a"));
        assert_eq!("a", search.prefix::<String, _>());
        assert_eq!(Ok(2), search.goto_longest_prefix());
        assert_eq!("app", search.prefix::<String, _>());
        assert_eq!(Err(1), search.goto_longest_prefix());
        assert_eq!("appl", search.prefix::<String, _>());
        assert_eq!(Err(0), search.goto_longest_prefix());
        assert_eq!(Ok(LabelKind::Prefix), search.query_until("i"));
        assert_eq!(Ok(6), search.goto_longest_prefix());
        assert_eq!(Ok(0), search.goto_longest_prefix());
        assert_eq!("application", search.prefix::<String, _>());
        search.reset();
        assert_eq!(LabelKind::Match, search.query_until("apple").unwrap());
        assert_eq!("apple", search.prefix::<String, _>());
        assert_eq!(Some(&2), search.value());
    }

    // #[test]
    // fn inc_serach_value_mut() {
    //     let trie = build_trie();
    //     let mut search = trie.inc_search();
    //     assert_eq!(None, search.query(b'z'));
    //     assert_eq!(LabelKind::PrefixAndMatch, search.query(b'a').unwrap());
    //     assert_eq!(LabelKind::Prefix, search.query(b'p').unwrap());
    //     assert_eq!(LabelKind::PrefixAndMatch, search.query(b'p').unwrap());
    //     assert_eq!(LabelKind::Prefix, search.query(b'l').unwrap());
    //     assert_eq!(LabelKind::Match, search.query(b'e').unwrap());
    //     let mut v = search.value_mut(&mut trie);
    //     assert_eq!(Some(&2), v.as_deref())
    // }
}
