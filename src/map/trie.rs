//! A trie map stores a value with each word or key.
use super::Trie;
use crate::inc_search::IncSearch;
use crate::iter::{PostfixIter, PrefixIter, SearchIter};
use crate::label::Label;
use crate::try_collect::{TryCollect, TryFromIterator};
use louds_rs::{AncestorNodeIter, ChildNodeIter, LoudsNodeNum};
use std::iter::FromIterator;

impl<Token: Ord, Value> Trie<Token, Value> {
    /// Return `Some(&Value)` if query is an exact match.
    pub fn exact_match(&self, query: impl Label<Token>) -> Option<&Value> {
        self.exact_match_node(query)
            .and_then(move |x| self.value(x))
    }

    /// Return `Node` if query is an exact match.
    #[inline]
    fn exact_match_node(&self, query: impl Label<Token>) -> Option<LoudsNodeNum> {
        let mut cur_node_num = LoudsNodeNum(1);

        let mut iter = query.into_tokens().peekable();
        let mut children_node_nums = Vec::new(); // reuse allocated space

        while let Some(chr) = iter.next() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));
            let res = self.bin_search_by_children_labels(&chr, &children_node_nums[..]);

            match res {
                Ok(j) => {
                    let child_node_num = children_node_nums[j];
                    if iter.peek().is_none() && self.is_terminal(child_node_num) {
                        return Some(child_node_num);
                    }
                    cur_node_num = child_node_num;
                }
                Err(_) => return None,
            }
        }
        None
    }

    /// Return `Some(&mut value)` if query is an exact match.
    pub fn exact_match_mut(&mut self, query: impl Label<Token>) -> Option<&mut Value> {
        self.exact_match_node(query)
            .and_then(move |x| self.value_mut(x))
    }

    /// Create an incremental search. Useful for interactive applications. See
    /// [crate::inc_search] for details.
    pub fn inc_search(&self) -> IncSearch<'_, Token, Value> {
        IncSearch::new(self)
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix(&self, query: impl Label<Token>) -> bool {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space

        for chr in query.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));
            let res = self.bin_search_by_children_labels(&chr, &children_node_nums[..]);
            match res {
                Ok(j) => cur_node_num = children_node_nums[j],
                Err(_) => return false,
            }
        }
        // Are there more nodes after our query?
        self.has_children_node_nums(cur_node_num)
    }

    /// For a given prefix, return its child nodes.
    ///
    /// If a node for the prefix doesn't exist, returns `None`.
    pub fn children(
        &self,
        query: impl Label<Token>,
    ) -> Option<impl Iterator<Item = (&Token, Option<&Value>)>> {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space

        for chr in query.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));
            let res = self.bin_search_by_children_labels(&chr, &children_node_nums[..]);
            match res {
                Ok(j) => cur_node_num = children_node_nums[j],
                Err(_) => return None,
            }
        }

        // return children
        Some(
            self.children_node_nums(cur_node_num)
                .map(|node_num| (self.token(node_num), self.value(node_num))),
        )
    }

    /// Return all entries and their values that match `query`.
    pub fn predictive_search<C, M>(
        &self,
        query: impl Label<Token>,
    ) -> SearchIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M> + Clone,
        Token: Clone,
    {
        SearchIter::new(self, query)
    }

    /// Return the postfixes and values of all entries that match `query`.
    pub fn postfix_search<C, M>(
        &self,
        query: impl Label<Token>,
    ) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut children_node_nums = Vec::new(); // reuse allocated space

        // Consumes query (prefix)
        for chr in query.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));
            let res = self.bin_search_by_children_labels(&chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    return PostfixIter::empty(self);
                }
            }
        }

        PostfixIter::new(self, cur_node_num)
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
    /// let results: Vec<(String, &u8)> = trie.iter().collect();
    /// assert_eq!(results, [("a".to_string(), &0u8), ("app".to_string(), &1u8), ("apple".to_string(), &2u8), ("application".to_string(), &4u8), ("better".to_string(), &3u8)]);
    /// ```
    pub fn iter<C, M>(&self) -> PostfixIter<'_, Token, Value, C, M>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        self.postfix_search(&[] as &[Token])
    }

    /// Return the common prefixes of `query`.
    pub fn common_prefix_search<L, M>(
        &self,
        query: impl Label<Token>,
    ) -> PrefixIter<'_, Token, Value, L, M>
    where
        L: TryFromIterator<Token, M>,
        Token: Clone,
    {
        PrefixIter::new(self, query)
    }

    /// Return the longest shared prefix or terminal of `query`.
    pub fn longest_prefix<C, M>(&self, query: impl Label<Token>) -> Option<C>
    where
        C: TryFromIterator<Token, M>,
        Token: Clone,
    {
        let mut cur_node_num = LoudsNodeNum(1);
        let mut buffer = Vec::new();
        let mut children_node_nums = Vec::new(); // reuse allocated space

        // Consumes query (prefix)
        for chr in query.into_tokens() {
            children_node_nums.clear();
            children_node_nums.extend(self.children_node_nums(cur_node_num));
            let res = self.bin_search_by_children_labels(&chr, &children_node_nums[..]);
            match res {
                Ok(i) => {
                    cur_node_num = children_node_nums[i];
                    buffer.push(cur_node_num);
                }
                Err(_) => {
                    return None;
                }
            }
        }

        // Walk the trie as long as there is only one path and it isn't a terminal value.
        while !self.is_terminal(cur_node_num) {
            let mut iter = self.children_node_nums(cur_node_num);
            let first = iter.next();
            let second = iter.next();
            match (first, second) {
                (Some(child_node_num), None) => {
                    cur_node_num = child_node_num;
                    buffer.push(child_node_num);
                }
                _ => break,
            }
        }
        if buffer.is_empty() {
            None
        } else {
            Some(
                buffer
                    .into_iter()
                    .map(|x| self.token(x).clone())
                    .try_collect()
                    .expect("Could not collect"),
            )
        }
    }

    pub(crate) fn has_children_node_nums(&self, node_num: LoudsNodeNum) -> bool {
        self.louds
            .parent_to_children_indices(node_num)
            .next()
            .is_some()
    }

    pub(crate) fn children_node_nums(&self, node_num: LoudsNodeNum) -> ChildNodeIter {
        self.louds.parent_to_children_nodes(node_num)
    }

    pub(crate) fn bin_search_by_children_labels(
        &self,
        query: &Token,
        children_node_nums: &[LoudsNodeNum],
    ) -> Result<usize, usize> {
        children_node_nums.binary_search_by(|child_node_num| self.token(*child_node_num).cmp(query))
    }

    pub(crate) fn token(&self, node_num: LoudsNodeNum) -> &Token {
        &self.trie_tokens[(node_num.0 - 2) as usize].token
    }

    pub(crate) fn is_terminal(&self, node_num: LoudsNodeNum) -> bool {
        if node_num.0 >= 2 {
            self.trie_tokens[(node_num.0 - 2) as usize].value.is_some()
        } else {
            false
        }
    }

    pub(crate) fn value(&self, node_num: LoudsNodeNum) -> Option<&Value> {
        if node_num.0 >= 2 {
            self.trie_tokens[(node_num.0 - 2) as usize].value.as_ref()
        } else {
            None
        }
    }

    pub(crate) fn value_mut(&mut self, node_num: LoudsNodeNum) -> Option<&mut Value> {
        self.trie_tokens[(node_num.0 - 2) as usize].value.as_mut()
    }

    pub(crate) fn child_to_ancestors(&self, node_num: LoudsNodeNum) -> AncestorNodeIter {
        self.louds.child_to_ancestors(node_num)
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
        let v: Vec<(String, &u8)> = trie.predictive_search("apple").collect();
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
        assert_eq!(trie.exact_match("apple"), Some(&2));
        let v = trie.exact_match_mut("apple").unwrap();
        *v = 10;
        assert_eq!(trie.exact_match("apple"), Some(&10));
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
        assert_eq!(trie.exact_match("application"), Some(&4));
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
        assert_eq!(trie.exact_match("application"), Some(&4));
    }

    #[test]
    fn use_empty_queries() {
        let trie = build_trie();
        assert!(trie.exact_match("").is_none());
        let _ = trie.predictive_search::<String, _>("").next();
        let _ = trie.postfix_search::<String, _>("").next();
        let _ = trie.common_prefix_search::<String, _>("").next();
    }

    #[test]
    fn insert_order_dependent() {
        let trie: Trie<u8, u8> = Trie::from_iter([("a", 0), ("app", 1), ("apple", 2)]);
        let results: Vec<(String, &u8)> = trie.iter().collect();
        assert_eq!(
            results,
            [
                ("a".to_string(), &0u8),
                ("app".to_string(), &1u8),
                ("apple".to_string(), &2u8)
            ]
        );

        let trie: Trie<u8, u8> = Trie::from_iter([("a", 0), ("apple", 2), ("app", 1)]);
        let results: Vec<(String, &u8)> = trie.iter().collect();
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
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.exact_match(query);
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
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.is_prefix(query);
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
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result = trie.children(query).map(|i| i.count());
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
            let result = trie
                .children("appl")
                .unwrap()
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            assert_eq!(result, vec![b'e', b'i']);
        }
    }

    mod longest_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result: Option<String> = trie.longest_prefix(query);
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
            t4: ("appl", Some("appl")),
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

    mod predictive_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.predictive_search(query).collect();
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

    mod common_prefix_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.common_prefix_search(query).collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
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

    mod postfix_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results: Vec<(String, &u8)> = trie.postfix_search(query).collect();
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
                    let (query, expected_results) = $value;
                    let trie = super::build_trie2();
                    let results: Vec<(String, &u8)> = trie.postfix_search(query).collect();
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
