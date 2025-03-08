//! A trie map stores a value with each word or key.
use crate::inc_search::IncSearch;
use crate::label::{Label, LabelKind};
use crate::search::{PostfixCollect, PostfixIter, PrefixCollect, PrefixIter};
use crate::try_from::TryFromTokens;
use louds_rs::{AncestorNodeIter, ChildNodeIter, Louds, LoudsNodeNum};
use std::iter::FromIterator;

use super::{NodeMut, NodeRef};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Node<Token, Value> {
    pub(super) token: Token,
    pub(super) value: Option<Value>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "mem_dbg", derive(mem_dbg::MemDbg, mem_dbg::MemSize))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// A trie for `Label`s (sequences of `Token`s); each sequence has an associated `Value`.
pub struct Trie<Token, Value> {
    pub(super) louds: Louds,

    /// (LoudsNodeNum - 2) -> Node
    pub(super) nodes: Box<[Node<Token, Value>]>,
}

impl<Token: Ord, Value> Trie<Token, Value> {
    /// Return a node num for a label.
    #[inline]
    fn get_num(&self, label: impl Label<Token>) -> Option<LoudsNodeNum> {
        let mut node_num = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space

        for token in label.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(node_num));
            let res = self.bin_search_by_children_labels(&token, &children_node_nums[..]);
            match res {
                Ok(j) => node_num = children_node_nums[j],
                Err(_) => return None,
            }
        }

        Some(node_num)
    }

    /// Get a node reference for a label.
    pub fn get(&self, label: impl Label<Token>) -> Option<NodeRef<'_, Token, Value>> {
        self.get_num(label).map(|node_num| NodeRef {
            trie: self,
            node_num,
        })
    }

    /// Get a mutable node reference for a label.
    pub fn get_mut(&mut self, label: impl Label<Token>) -> Option<NodeMut<'_, Token, Value>> {
        self.get_num(label).map(|node_num| NodeMut {
            trie: self,
            node_num,
        })
    }

    /// Get a value for a label.
    pub fn get_value(&self, label: impl Label<Token>) -> Option<&Value> {
        self.get_num(label)
            .and_then(|node_num| self.value(node_num))
    }

    /// Get a mutable value for a label.
    pub fn get_value_mut(&mut self, label: impl Label<Token>) -> Option<&mut Value> {
        self.get_num(label)
            .and_then(|node_num| self.value_mut(node_num))
    }

    /// Create an incremental search. Useful for interactive applications. See
    /// [crate::inc_search] for details.
    pub fn inc_search(&self) -> IncSearch<'_, Token, Value> {
        IncSearch::new(self)
    }

    /// Return the common prefixes of `label`.
    pub fn prefixes_of<L: Label<Token>>(
        &self,
        label: L,
    ) -> PrefixIter<'_, Token, Value, L::IntoTokens> {
        PrefixIter::new(self, label)
    }

    /// Return the common prefixes of `label` as `(label, value)` pairs.
    pub fn prefixes_of_pairs<L>(
        &self,
        label: impl Label<Token>,
    ) -> PrefixCollect<'_, Token, Value, L>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        PrefixCollect::new(self, label)
    }

    /// Returns the exactly matching suffixes that follow after this node.
    ///
    /// e.g. "app" → "le" node (as in "apple")
    ///
    /// Strips the label's node from the results; to include this node as a prefix, see [`Self::starts_with`].
    pub fn suffixes_of(&self, label: impl Label<Token>) -> PostfixIter<'_, Token, Value>
    where
        Token: Clone,
    {
        self.get(label)
            .map(|n| PostfixIter::suffixes_of(n.trie, n.node_num))
            .unwrap_or_else(|| PostfixIter::empty(self))
    }

    /// Returns the exactly matching suffixes that follow after this node as `(label, value)` pairs.
    ///
    /// e.g. "app" → ("le", value) (as in "apple")
    ///
    /// Strips the label from the results; to include this node as a prefix, see [`Self::starts_with_pairs`].
    pub fn suffixes_of_pairs<L>(
        &self,
        label: impl Label<Token>,
    ) -> PostfixCollect<'_, Token, Value, L>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        self.get(label)
            .map(|n| PostfixCollect::suffixes_of(self, n.node_num))
            .unwrap_or_else(|| PostfixCollect::empty(self))
    }

    /// Returns the exact match nodes that follow after this node.
    ///
    /// e.g. "app" → "apple" node
    pub fn starts_with(&self, label: impl Label<Token>) -> PostfixIter<'_, Token, Value>
    where
        Token: Clone,
    {
        self.get(label)
            .map(|n| n.starts_with())
            .unwrap_or_else(|| PostfixIter::empty(self))
    }

    /// Returns the exact match `(label, value)` pairs that follow after this node.
    ///
    /// e.g. "app" → ("apple", value)
    pub fn starts_with_pairs<L>(
        &self,
        label: impl Label<Token>,
    ) -> PostfixCollect<'_, Token, Value, L>
    where
        Token: Clone,
        L: TryFromTokens<Token>,
    {
        PostfixCollect::starts_with(self, label)
    }

    /// Returns an iterator across all keys in the trie.
    ///
    /// # Examples
    /// In the following example we illustrate how to iterate over all keys in the trie.
    /// Note that the order of the keys is not guaranteed, as they will be returned in
    /// lexicographical order.
    ///
    /// ```rust
    /// use trie_rs::map::Trie;
    /// let trie = Trie::<u8, _>::from_iter([("a", 0), ("app", 1), ("apple", 2), ("better", 3), ("application", 4)]);
    /// let results: Vec<_> = trie.iter().pairs::<String>().filter_map(Result::ok).collect();
    /// assert_eq!(results, [("a".to_string(), &0u8), ("app".to_string(), &1u8), ("apple".to_string(), &2u8), ("application".to_string(), &4u8), ("better".to_string(), &3u8)]);
    /// ```
    pub fn iter(&self) -> PostfixIter<'_, Token, Value>
    where
        Token: Clone,
    {
        PostfixIter::suffixes_of(self, LoudsNodeNum(1))
    }

    /// Return the longest shared prefix or terminal of `label`.
    pub fn path_of(&self, label: impl Label<Token>) -> Option<NodeRef<'_, Token, Value>> {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space

        // Consumes label (prefix)
        for token in label.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));

            let i = self
                .bin_search_by_children_labels(&token, &children_node_nums[..])
                .ok()?;

            cur_node_num = children_node_nums[i];
        }

        // Walk the trie as long as there is only one path and it isn't a terminal value.
        while !self.is_exact(cur_node_num) {
            let mut iter = self.children_node_nums(cur_node_num);
            let first = iter.next();
            let second = iter.next();
            match (first, second) {
                (Some(child_node_num), None) => {
                    cur_node_num = child_node_num;
                }
                _ => return None,
            }
        }

        let node_ref = NodeRef {
            trie: &self,
            node_num: cur_node_num,
        };

        Some(node_ref)
    }

    pub(crate) fn bin_search_by_children_labels(
        &self,
        token: &Token,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize> {
        children_node_nums.binary_search_by(|child_node_num| self.token(*child_node_num).cmp(token))
    }
}

impl<Token, Value> Trie<Token, Value> {
    pub(crate) fn is_prefix(&self, node_num: LoudsNodeNum) -> bool {
        self.louds
            .parent_to_children_indices(node_num)
            .next()
            .is_some()
    }

    pub(crate) fn children_node_nums(&self, node_num: LoudsNodeNum) -> ChildNodeIter {
        self.louds.parent_to_children_nodes(node_num)
    }

    pub(crate) fn token(&self, node_num: LoudsNodeNum) -> &Token {
        &self.nodes[(node_num.0 - 2) as usize].token
    }

    pub(crate) fn is_exact(&self, node_num: LoudsNodeNum) -> bool {
        if node_num.0 >= 2 {
            self.nodes[(node_num.0 - 2) as usize].value.is_some()
        } else {
            false
        }
    }

    pub(crate) fn value(&self, node_num: LoudsNodeNum) -> Option<&Value> {
        if node_num.0 >= 2 {
            self.nodes[(node_num.0 - 2) as usize].value.as_ref()
        } else {
            None
        }
    }

    pub(crate) fn kind(&self, node_num: LoudsNodeNum) -> LabelKind {
        match (self.is_prefix(node_num), self.is_exact(node_num)) {
            (true, false) => LabelKind::Prefix,
            (false, true) => LabelKind::Exact,
            (true, true) => LabelKind::PrefixAndExact,
            // SAFETY: Since we already have the node, it must at least be a prefix or exact match.
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

    pub(crate) fn value_mut(&mut self, node_num: LoudsNodeNum) -> Option<&mut Value> {
        self.nodes[(node_num.0 - 2) as usize].value.as_mut()
    }

    pub(crate) fn child_to_ancestors(&self, node_num: LoudsNodeNum) -> AncestorNodeIter {
        self.louds.child_to_ancestors(node_num)
    }

    pub(crate) fn child_to_parent(&self, node_num: LoudsNodeNum) -> LoudsNodeNum {
        let index = self.louds.node_num_to_index(node_num);
        self.louds.child_to_parent(index)
    }
}

impl<Token, Value, L> FromIterator<(L, Value)> for Trie<Token, Value>
where
    L: Label<Token>,
    Token: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        Self: Sized,
        T: IntoIterator<Item = (L, Value)>,
    {
        let mut builder = super::TrieBuilder::new();
        for (k, v) in iter {
            builder.insert(k, v)
        }
        builder.build()
    }
}

#[cfg(test)]
mod search_tests {
    use crate::map::{Trie, TrieBuilder};
    use std::iter::FromIterator;

    fn build_trie() -> Trie<u8, u8> {
        let mut builder: TrieBuilder<u8, u8> = TrieBuilder::new();
        builder.insert("a", 0);
        builder.insert("app", 1);
        builder.insert("apple", 2);
        builder.insert("better", 3);
        builder.insert("application", 4);
        builder.insert("アップル🍎", 5);
        builder.build()
    }

    fn build_trie2() -> Trie<char, u8> {
        let mut builder: TrieBuilder<char, u8> = TrieBuilder::new();
        builder.insert("a", 0);
        builder.insert("app", 1);
        builder.insert("apple", 2);
        builder.insert("better", 3);
        builder.insert("application", 4);
        builder.insert("アップル🍎", 5);
        builder.build()
    }

    #[test]
    fn sanity_check() {
        let trie = build_trie();
        let v: Vec<(String, &u8)> = trie
            .starts_with("apple")
            .pairs::<String>()
            .filter_map(Result::ok)
            .collect();
        assert_eq!(v, vec![("apple".to_string(), &2)]);
    }

    #[test]
    fn clone() {
        let trie = build_trie();
        let _c: Trie<u8, u8> = trie.clone();
    }

    #[test]
    fn value_mut() {
        let mut trie = build_trie();
        assert_eq!(trie.get_value("apple"), Some(&2));
        let v = trie.get_value_mut("apple").unwrap();
        *v = 10;
        assert_eq!(trie.get_value("apple"), Some(&10));
    }

    #[test]
    fn trie_from_iter() {
        let trie = Trie::<u8, u8>::from_iter([
            ("a", 0),
            ("app", 1),
            ("apple", 2),
            ("better", 3),
            ("application", 4),
        ]);
        assert_eq!(trie.get_value("application"), Some(&4));
    }

    #[test]
    fn collect_a_trie() {
        // Does not work with arrays in rust 2018 because into_iter() returns references instead of owned types.
        // let trie: Trie<u8, u8> = [("a", 0), ("app", 1), ("apple", 2), ("better", 3), ("application", 4)].into_iter().collect();
        let trie: Trie<u8, u8> = vec![
            ("a", 0),
            ("app", 1),
            ("apple", 2),
            ("better", 3),
            ("application", 4),
        ]
        .into_iter()
        .collect();
        assert_eq!(trie.get_value("application"), Some(&4));
    }

    #[test]
    fn use_empty_queries() {
        let trie = build_trie();
        assert!(trie.get_value("").is_none());
        let _ = trie.starts_with("").next();
        let _ = trie.suffixes_of("").next();
        let _ = trie.prefixes_of("").next();
    }

    #[test]
    fn insert_order_dependent() {
        let trie: Trie<u8, u8> = Trie::from_iter([("a", 0), ("app", 1), ("apple", 2)]);
        let results: Vec<(String, &u8)> = trie
            .iter()
            .pairs::<String>()
            .filter_map(Result::ok)
            .collect();
        assert_eq!(
            results,
            [
                ("a".to_string(), &0u8),
                ("app".to_string(), &1u8),
                ("apple".to_string(), &2u8)
            ]
        );

        let trie: Trie<u8, u8> = Trie::from_iter([("a", 0), ("apple", 2), ("app", 1)]);
        let results: Vec<(String, &u8)> = trie
            .iter()
            .pairs::<String>()
            .filter_map(Result::ok)
            .collect();
        assert_eq!(
            results,
            [
                ("a".to_string(), &0u8),
                ("app".to_string(), &1u8),
                ("apple".to_string(), &2u8)
            ]
        );
    }

    mod exact_match_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.get_value(label);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", Some(&0)),
            t2: ("app", Some(&1)),
            t3: ("apple", Some(&2)),
            t4: ("application", Some(&4)),
            t5: ("better", Some(&3)),
            t6: ("アップル🍎", Some(&5)),
            t7: ("appl", None),
            t8: ("appler", None),
        }
    }

    mod is_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.get(label).filter(|n| n.is_prefix()).is_some();
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", false),
            t4: ("application", false),
            t5: ("better", false),
            t6: ("アップル🍎", false),
            t7: ("appl", true),
            t8: ("appler", false),
            t9: ("アップル", true),
        }
    }

    mod children_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.get(label).map(|n| n.children().count());
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", Some(1)),
            t2: ("app", Some(1)),
            t3: ("apple", Some(0)),
            t4: ("application", Some(0)),
            t5: ("better", Some(0)),
            t6: ("アップル🍎", Some(0)),
            t7: ("appl", Some(2)),
            t8: ("appler", None),
            t9: ("アップル", Some(1)),
        }

        #[test]
        fn t10() {
            let trie = super::build_trie();
            let result: Vec<_> = trie
                .get("appl")
                .unwrap()
                .children()
                .map(|n| *n.token())
                .collect();
            assert_eq!(result, vec![b'e', b'i']);
        }
    }

    mod longest_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_match) = $value;
                    let trie = super::build_trie();
                    let result: Option<String> = trie.path_of(label).and_then(|n| n.label::<String>().ok());
                    let expected_match = expected_match.map(str::to_string);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", Some("a")),
            t2: ("ap", Some("app")),
            t3: ("app", Some("app")),
            t4: ("appl", None),
            t5: ("appli", Some("application")),
            t6: ("b", Some("better")),
            t7: ("アップル🍎", Some("アップル🍎")),
            t8: ("appler", None),
            t9: ("アップル", Some("アップル🍎")),
            t10: ("z", None),
            t11: ("applesDONTEXIST", None),
            t12: ("", None),
        }
    }

    mod starts_with_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.starts_with(label).pairs::<String>().filter_map(Result::ok).collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec![("a", 0), ("app", 1), ("apple", 2), ("application", 4)]),
            t2: ("app", vec![("app", 1), ("apple", 2), ("application", 4)]),
            t3: ("appl", vec![("apple", 2), ("application", 4)]),
            t4: ("apple", vec![("apple", 2)]),
            t5: ("b", vec![("better", 3)]),
            t6: ("c", Vec::<(&str, u8)>::new()),
            t7: ("アップ", vec![("アップル🍎", 5)]),
        }
    }

    mod starts_with_pairs_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.starts_with_pairs::<String>(label).collect::<Result<_, _>>().unwrap();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec![("a", 0), ("app", 1), ("apple", 2), ("application", 4)]),
            t2: ("app", vec![("app", 1), ("apple", 2), ("application", 4)]),
            t3: ("appl", vec![("apple", 2), ("application", 4)]),
            t4: ("apple", vec![("apple", 2)]),
            t5: ("b", vec![("better", 3)]),
            t6: ("c", Vec::<(&str, u8)>::new()),
            t7: ("アップ", vec![("アップル🍎", 5)]),
        }
    }

    mod prefixes_of_pairs_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Result<Vec<(String, &u8)>, _> = trie.prefixes_of_pairs::<String>(label).collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, Ok(expected_results));
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec![("a", 0)]),
            t2: ("ap", vec![("a", 0)]),
            t3: ("appl", vec![("a", 0), ("app", 1)]),
            t4: ("appler", vec![("a", 0), ("app", 1), ("apple", 2)]),
            t5: ("bette", Vec::<(&str, u8)>::new()),
            t6: ("betterment", vec![("better", 3)]),
            t7: ("c", Vec::<(&str, u8)>::new()),
            t8: ("アップル🍎🍏", vec![("アップル🍎", 5)]),
        }
    }

    mod suffixes_of_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.suffixes_of(label).pairs::<String>().filter_map(Result::ok).collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec![("pp", 1), ("pple", 2), ("pplication", 4)]),
            t2: ("ap", vec![("p", 1), ("ple", 2), ("plication", 4)]),
            t3: ("appl", vec![("e", 2), ("ication", 4)]),
            t4: ("appler", Vec::<(&str, u8)>::new()),
            t5: ("bette", vec![("r", 3)]),
            t6: ("betterment", Vec::<(&str, u8)>::new()),
            t7: ("c", Vec::<(&str, u8)>::new()),
            t8: ("アップル🍎🍏", Vec::<(&str, u8)>::new()),
        }
    }

    mod postfix_search_char_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (label, expected_results) = $value;
                    let trie = super::build_trie2();
                    let results: Vec<(String, &u8)> = trie.suffixes_of(label).pairs::<String>().collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec![("pp", 1), ("pple", 2), ("pplication", 4)]),
            t2: ("ap", vec![("p", 1), ("ple", 2), ("plication", 4)]),
            t3: ("appl", vec![("e", 2), ("ication", 4)]),
            t4: ("appler", Vec::<(&str, u8)>::new()),
            t5: ("bette", vec![("r", 3)]),
            t6: ("betterment", Vec::<(&str, u8)>::new()),
            t7: ("c", Vec::<(&str, u8)>::new()),
            t8: ("アップル🍎🍏", Vec::<(&str, u8)>::new()),
        }
    }
}
