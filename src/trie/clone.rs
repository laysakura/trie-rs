use crate::inc_search::IncSearch;
use crate::try_collect::{TryCollect, TryFromIterator};
use super::map::{self};

use derive_deref::*;
use std::clone::Clone;

#[derive(Deref, DerefMut)]
pub struct Trie<Label: Clone> {
    pub inner: map::Trie<Label, ()>,
}
pub struct TrieBuilder<Label: Clone> {
    inner: map::TrieBuilder<Label, ()>,
}

impl<Label: Ord + Clone> Trie<Label> {
    /// Return true if `query` is an exact match.
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.inner.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`, cloned.
    pub fn common_prefix_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
    {
        self.inner
            .common_prefix_search(query)
            .into_iter()
            .map(|v| v.into_iter().cloned().try_collect().expect("Could not collect"))
            .collect()
    }

    /// Return all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
    {
        let chunk = self.inner.predictive_search(query);
        chunk.map(|v| v.cloned().try_collect().expect("Could not collect")).into_iter().collect()
    }
    /// Return the postfixes of all entries that match `query`, cloned.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search<C, M>(&self, query: impl AsRef<[Label]>) -> Vec<C>
    where
        C: TryFromIterator<Label, M>,
    {
        let chunk = self.inner.postfix_search(query);
        chunk.map(|v| v.cloned().try_collect().expect("Could not collect")).into_iter().collect()
    }

    /// Return true if `query` is a prefix.
    ///
    /// Note: A prefix may be an exact match or not, and an exact match may be a
    /// prefix or not.
    pub fn is_prefix(&self, query: impl AsRef<[Label]>) -> bool {
        self.inner.is_prefix(query)
    }

    pub fn inc_search(&self) -> IncSearch<'_, Label, ()> {
        IncSearch::new(&self.inner)
    }
}

impl<Label: Ord + Clone> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord + Clone> TrieBuilder<Label> {
    pub fn new() -> Self {
        Self {
            inner: map::TrieBuilder::new(),
        }
    }

    /// Add a cloneable entry.
    pub fn push<Arr: AsRef<[Label]>>(&mut self, entry: Arr)
    where
        Label: Clone,
    {
        self.inner.push(entry, ());
    }

    /// Add an entry.
    pub fn insert<Arr: IntoIterator<Item = Label>>(&mut self, entry: Arr) {
        self.inner.insert(entry, ());
    }

    /// Build a [Trie].
    pub fn build(self) -> Trie<Label> {
        Trie {
            inner: self.inner.build(),
        }
    }
}

#[cfg(test)]
mod search_tests {
    use crate::trie::clone::{Trie, TrieBuilder};

    fn build_trie() -> Trie<u8> {
        let mut builder = TrieBuilder::new();
        builder.push("a");
        builder.push("app");
        builder.push("apple");
        builder.push("better");
        builder.push("application");
        builder.push("アップル🍎");
        builder.build()
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
            t1: ("a", true),
            t2: ("app", true),
            t3: ("apple", true),
            t4: ("application", true),
            t5: ("better", true),
            t6: ("アップル🍎", true),
            t7: ("appl", false),
            t8: ("appler", false),
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
            t10: ("ed", false),
            t11: ("e", false),
            t12: ("", true),
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
                    let results: Vec<String> = trie.predictive_search(query);
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a", "app", "apple", "application"]),
            t2: ("app", vec!["app", "apple", "application"]),
            t3: ("appl", vec!["apple", "application"]),
            t4: ("apple", vec!["apple"]),
            t5: ("b", vec!["better"]),
            t6: ("c", Vec::<&str>::new()),
            t7: ("アップ", vec!["アップル🍎"]),
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
                    let results: Vec<String> = trie.common_prefix_search(query);
                    let expected_results: Vec<String> = expected_results.iter().map(|s| s.to_string()).collect();
                    assert_eq!(results, expected_results);
                }
            )*
            }
        }

        parameterized_tests! {
            t1: ("a", vec!["a"]),
            t2: ("ap", vec!["a"]),
            t3: ("appl", vec!["a", "app"]),
            t4: ("appler", vec!["a", "app", "apple"]),
            t5: ("bette", Vec::<&str>::new()),
            t6: ("betterment", vec!["better"]),
            t7: ("c", Vec::<&str>::new()),
            t8: ("アップル🍎🍏", vec!["アップル🍎"]),
        }
    }

    mod posfix_search_tests {

        #[test]
        fn postfix_no_match() {
            let trie = super::build_trie();
            let postfixes: Vec<String> = trie.postfix_search("NOT-THERE");
            let chunks = postfixes.into_iter();
            assert_eq!(chunks.count(), 0);
        }

        #[test]
        fn vec_into_iter_clone() {
            let v = vec![1, 2, 3];
            let i = v.into_iter();
            let c = i.clone();
            assert_eq!(c.count(), 3);
            assert_eq!(i.count(), 3);
        }
    }
}
