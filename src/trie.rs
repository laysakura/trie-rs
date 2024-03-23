use super::inc_search::IncSearch;
use super::map::{self, PostfixIter, PrefixIter, SearchIter};
use frayed::Defray;

pub struct Trie<Label> {
    inner: map::Trie<Label, ()>,
}
pub struct TrieBuilder<Label> {
    inner: map::TrieBuilder<Label, ()>,
}

impl<Label: Ord> Trie<Label> {
    /// Return true if `query` is an exact match.
    pub fn exact_match(&self, query: impl AsRef<[Label]>) -> bool {
        self.inner.exact_match(query).is_some()
    }

    /// Return the common prefixes of `query`.
    pub fn common_prefix_search<Query>(
        &self,
        query: Query,
    ) -> Defray<PrefixIter<'_, Label, (), Query>>
    where
        Query: AsRef<[Label]>, // + 'b
    {
        self.inner.common_prefix_search(query)
    }

    /// Return all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn predictive_search(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Defray<SearchIter<'_, Label, ()>> {
        self.inner.predictive_search(query)
    }

    /// Return the postfixes of all entries that match `query`.
    ///
    /// # Panics
    /// If `query` is empty.
    pub fn postfix_search(
        &self,
        query: impl AsRef<[Label]>,
    ) -> Defray<PostfixIter<'_, Label, ()>> {
        self.inner.postfix_search(query)
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

impl<Label: Ord> Default for TrieBuilder<Label> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Label: Ord> TrieBuilder<Label> {
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
    use crate::trie::{Trie, TrieBuilder};

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
                    let results: Vec<String> = trie.predictive_search(query)
                                      .into_iter()
                                      .map(|g| String::from_utf8(g.into_iter().cloned().collect()).unwrap())
                                      .collect();
                    // results.sort_by(|a, b| a.len().cmp(&b.len()));
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
                    let results: Vec<String> = trie.common_prefix_search(query)
                                      .into_iter()
                                      .map(|g| String::from_utf8(g.into_iter().cloned().collect()).unwrap())
                                      .collect();
                    // let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
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
        fn postfix_unfused() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("app");
            let mut iter = postfixes.into_inner().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert!(iter.next().is_none());
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('e'));
        }

        #[test]
        fn postfix_baseline() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("app");
            let mut chunks = postfixes.into_iter();
            // assert_eq!(chunks.by_ref().count(), 3);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), None);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), None);
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('p'));
            assert_eq!(iter.next(), Some('l'));
            assert_eq!(iter.next(), Some('i'));
            assert_eq!(iter.next(), Some('c'));
            assert_eq!(iter.next(), Some('a'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('i'));
            assert_eq!(iter.next(), Some('o'));
            assert_eq!(iter.next(), Some('n'));
            assert_eq!(iter.next(), None);

            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_2() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("b");
            let mut chunks = postfixes.into_iter();
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('b'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('r'));
            assert_eq!(iter.next(), None);
            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_3() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("bet");
            let mut chunks = postfixes.into_iter();
            let mut iter = chunks.next().unwrap().map(|x| *x as char);
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('t'));
            assert_eq!(iter.next(), Some('e'));
            assert_eq!(iter.next(), Some('r'));
            assert_eq!(iter.next(), None);
            assert!(chunks.next().is_none());
        }

        #[test]
        fn postfix_no_match() {
            let trie = super::build_trie();
            let postfixes = trie.postfix_search("NOT-THERE");
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
