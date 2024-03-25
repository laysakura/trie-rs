
use louds_rs::LoudsNodeNum;
use derive_deref::{Deref, DerefMut};
use super::postfix_iter::PostfixIter;
use super::search_iter::SearchIter;
use super::prefix_iter::PrefixIter;
use crate::map;
use crate::try_collect::{TryCollect, TryFromIterator};


#[derive(Deref, DerefMut)]
pub struct TrieBuilder<Label, Value>(pub map::TrieBuilder<Label, Value>);

impl<Label: Ord + Clone, Value> TrieBuilder<Label, Value> {
    pub fn new() -> Self {
        Self(map::TrieBuilder::new())
    }

    pub fn build(self) -> Trie<Label, Value> {
        Trie(self.0.build())
    }
}

#[derive(Deref, DerefMut)]
pub struct Trie<Label, Value>(map::Trie<Label, Value>);

impl<Label: Ord + Clone, Value> Trie<Label, Value> {
    /// Return all entries and their values that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<C,M>(&self, query: impl AsRef<[Label]>) -> SearchIter<'_, Label, Value, C, M>
    where
        C: TryFromIterator<Label, M> + Clone,
    {
        SearchIter::new(&self.0, query)
    }

    /// Return the postfixes and values of all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<Query, C, M>(&self, query: Query)
                                -> PostfixIter<'_, Label, Value, C, M>
    where
        Query: AsRef<[Label]>,
        C: TryFromIterator<Label, M>,
    {

        let mut cur_node_num = LoudsNodeNum(1);

        // Consumes query (prefix)
        for chr in query.as_ref() {
            let children_node_nums: Vec<_> = self.children_node_nums(cur_node_num).collect();
            let res = self.bin_search_by_children_labels(chr, &children_node_nums[..]);
            match res {
                Ok(i) => cur_node_num = children_node_nums[i],
                Err(_) => {
                    return PostfixIter::empty(&self.0);
                }
            }
        }

        PostfixIter::new(&self.0, cur_node_num)
    }

    /// Return the common prefixes of `query`, cloned.
    pub fn common_prefix_search<Query, C, M>(&self, query: Query) -> PrefixIter<'_, Label, Value, Query, C, M>
    where
        Query: AsRef<[Label]>,
        C: TryFromIterator<Label, M>,
    {
        PrefixIter::new(&self.0, query)
    }

    pub fn find_longest_prefix<Query, C, M>(
        &self,
        query: Query,
    ) -> C
    where
        Query: AsRef<[Label]>,
        C: TryFromIterator<Label, M>,
    {
        self.0.find_longest_prefix(query).cloned().try_collect().expect("Could not collect")
    }
}

#[cfg(test)]
mod search_tests {
    use crate::map::clone::{Trie, TrieBuilder};

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

    fn build_trie2() -> Trie<char, u8> {
        let mut builder: TrieBuilder<char, u8> = TrieBuilder::new();
        builder.insert("a".chars(), 0);
        builder.insert("app".chars(), 1);
        builder.insert("apple".chars(), 2);
        builder.insert("better".chars(), 3);
        builder.insert("application".chars(), 4);
        builder.insert("アップル🍎".chars(), 5);
        builder.build()
    }

    #[test]
    fn sanity_check() {
        let trie = build_trie();
        let v: Vec<(String, &u8)> = trie.predictive_search("apple").collect();
        assert_eq!(
            v,
            vec![("apple".to_string(), &2)]
        );
    }

    #[test]
    fn value_mut() {
        let mut trie = build_trie();
        assert_eq!(trie.exact_match("apple"), Some(&2));
        let v = trie.exact_match_mut("apple").unwrap();
        *v = 10;
        assert_eq!(trie.exact_match("apple"), Some(&10));
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

    mod find_longest_prefix_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_match) = $value;
                    let trie = super::build_trie();
                    let result: String = trie.find_longest_prefix(query);
                    assert_eq!(result, expected_match);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", "a"),
            t2: ("ap", "app"),
            t3: ("appl", "appl"),
            t4: ("appli", "application"),
            t5: ("b", "better"),
            t6: ("アップル🍎", "アップル🍎"),
            t7: ("appler", "apple"),
            t8: ("アップル", "アップル🍎"),
            t9: ("z", ""),
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
                    let results: Vec<(String, u8)> = trie.common_prefix_search(query).collect();
                    let expected_results: Vec<(String, u8)> = expected_results.iter().map(|s| (s.0.to_string(), s.1)).collect();
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
                    let chars: Vec<char> = query.chars().collect();
                    let results: Vec<(String, &u8)> = trie.postfix_search(chars).collect();
                    let expected_results: Vec<(String, &u8)> = expected_results.iter().map(|s| (s.0.to_string(), &s.1)).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            // t1: ("a", vec![("", 0), ("pp", 1), ("pple", 2), ("pplication", 4)]),
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