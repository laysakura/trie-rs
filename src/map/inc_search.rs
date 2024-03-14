use crate::map::Trie;
use louds_rs::LoudsNodeNum;

/// An incremental search of the trie.
///
/// # Motivation
///
/// The motivation for this struct is for "online" or interactive use cases. One
/// often accumulates input to match against a trie. Using the standard
/// [`exact_match()`] faculties which has a time complexity of _O(m log n)_
/// where _m_ is the query string length and _n_ is the number of entries in the
/// trie. Consider this loop where we simulate accumulating a query.
///
/// ```ignore
/// let q = "appli"; // query string
/// let mut is_match: bool;
/// for i = 0..q.len() {
///     is_match = trie.exact_match(q[0..i]);
/// }
/// ```
///
/// Building the query one "character" at a time and `exact_match()`ing each
/// time, the loop has effectively complexity of _O(m<sup>2</sup> log n)_.
///
/// Using the incremental search, the time complexity of each query is _O(log
/// n)_ which returns an [Answer] enum.
///
/// ```ignore
/// let q = "appli"; // query string
/// let inc_search = trie.inc_search();
/// let mut is_match: bool;
/// for i = 0..q.len() {
///     is_match = inc_search.query(q[i]).unwrap().is_match();
/// }
/// ```
///
/// This means the above code restores the time complexity of _O(m log n)_ for
/// the loop.
pub struct IncSearch<'a, Label, Value> {
    trie: &'a Trie<Label, Value>,
    node: LoudsNodeNum,
}

/// A "matching" answer to an incremental search on a partial query.
#[derive(Debug, PartialEq, Eq)]
pub enum Answer {
    /// There is a prefix here.
    Prefix,
    /// There is an exact match here.
    Match,
    /// There is a prefix and an exact match here.
    PrefixAndMatch,
}

impl Answer {
    /// Is query answer a prefix?
    pub fn is_prefix(&self) -> bool {
        match self {
            Answer::Prefix | Answer::PrefixAndMatch => true,
            _ => false,
        }
    }

    /// Is query answer an exact match?
    pub fn is_match(&self) -> bool {
        match self {
            Answer::Match | Answer::PrefixAndMatch => true,
            _ => false,
        }
    }

    fn new(is_prefix: bool, is_match: bool) -> Option<Self> {
        match (is_prefix, is_match) {
            (true, false) => Some(Answer::Prefix),
            (false, true) => Some(Answer::Match),
            (true, true) => Some(Answer::PrefixAndMatch),
            (false, false) => None,
        }
    }
}

impl<'a, Label: Ord, Value> IncSearch<'a, Label, Value> {
    pub fn new(trie: &'a Trie<Label, Value>) -> Self {
        Self {
            trie,
            node: LoudsNodeNum(1),
        }
    }

    /// Query but do not change the node we're looking at on the trie.
    pub fn peek<L>(&self, chr: L) -> Option<Answer>
    where
        Label: PartialOrd<L>,
    {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(&chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                let node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(node);
                let is_match = self.trie.value(node).is_some();
                Answer::new(is_prefix, is_match)
            }
            Err(_) => None,
        }
    }

    /// Query the trie and go to node if there is a match.
    pub fn query<L>(&mut self, chr: &L) -> Option<Answer>
    where
        Label: PartialOrd<L>,
    {
        let children_node_nums: Vec<_> = self.trie.children_node_nums(self.node).collect();
        let res = self
            .trie
            .bin_search_by_children_labels(chr, &children_node_nums[..]);
        match res {
            Ok(j) => {
                self.node = children_node_nums[j];
                let is_prefix = self.trie.has_children_node_nums(self.node);
                let is_match = self.trie.value(self.node).is_some();
                Answer::new(is_prefix, is_match)
            }
            Err(_) => None,
        }
    }

    /// Query the trie with a sequence. Will return `Err(index of query)` on
    /// first failure to match.
    pub fn query_until<L>(&mut self, query: impl AsRef<[L]>) -> Result<Answer, usize>
    where
        Label: PartialOrd<L>,
    {
        let mut result = None;
        let mut i = 0;
        for chr in query.as_ref().iter() {
            result = self.query(chr);
            if result.is_none() {
                return Err(i);
            }
            i += 1;
        }
        result.ok_or(i)
    }

    /// Return the value at current node. There should be one for any node where
    /// `answer.is_match()` is true.
    pub fn value(&self) -> Option<&'a Value> {
        self.trie.value(self.node)
    }

    // This isn't actually possible.
    // /// Return the mutable value at current node. There should be one for any
    // /// node where `answer.is_match()` is true.
    // ///
    // /// Note: Because [IncSearch] does not store a mutable reference to the
    // /// trie, a mutable reference must be provided.
    // pub fn value_mut<'b>(self, trie: &'b mut Trie<Label, Value>) -> Option<&'b mut Value> {
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
        builder.push("a", 0);
        builder.push("app", 1);
        builder.push("apple", 2);
        builder.push("better", 3);
        builder.push("application", 4);
        builder.push("アップル🍎", 5);
        builder.build()
    }

    #[test]
    fn inc_search() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(None, search.query(&b'z'));
        assert_eq!(Answer::PrefixAndMatch, search.query(&b'a').unwrap());
        assert_eq!(Answer::Prefix, search.query(&b'p').unwrap());
        assert_eq!(Answer::PrefixAndMatch, search.query(&b'p').unwrap());
        assert_eq!(Answer::Prefix, search.query(&b'l').unwrap());
        assert_eq!(Answer::Match, search.query(&b'e').unwrap());
    }

    #[test]
    fn inc_search_value() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(None, search.query(&b'z'));
        assert_eq!(Answer::PrefixAndMatch, search.query(&b'a').unwrap());
        assert_eq!(Answer::Prefix, search.query(&b'p').unwrap());
        assert_eq!(Answer::PrefixAndMatch, search.query(&b'p').unwrap());
        assert_eq!(Answer::Prefix, search.query(&b'l').unwrap());
        assert_eq!(Answer::Match, search.query(&b'e').unwrap());
        assert_eq!(Some(&2), search.value());
    }

    #[test]
    fn inc_search_query_until() {
        let trie = build_trie();
        let mut search = trie.inc_search();
        assert_eq!(Err(0), search.query_until("zoo"));
        search.reset();
        assert_eq!(Err(1), search.query_until("blue"));
        search.reset();
        assert_eq!(Answer::Match, search.query_until("apple").unwrap());
        assert_eq!(Some(&2), search.value());
    }

    // #[test]
    // fn inc_serach_value_mut() {
    //     let trie = build_trie();
    //     let mut search = trie.inc_search();
    //     assert_eq!(None, search.query(b'z'));
    //     assert_eq!(Answer::PrefixAndMatch, search.query(b'a').unwrap());
    //     assert_eq!(Answer::Prefix, search.query(b'p').unwrap());
    //     assert_eq!(Answer::PrefixAndMatch, search.query(b'p').unwrap());
    //     assert_eq!(Answer::Prefix, search.query(b'l').unwrap());
    //     assert_eq!(Answer::Match, search.query(b'e').unwrap());
    //     let mut v = search.value_mut(&mut trie);
    //     assert_eq!(Some(&2), v.as_deref())
    // }
}