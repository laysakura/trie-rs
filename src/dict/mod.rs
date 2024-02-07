use crate::{Trie as OldTrie, TrieBuilder as OldTrieBuilder};
use derivative::Derivative;

#[derive(Derivative, Clone, Debug)]
#[derivative(Eq, Ord, PartialEq, PartialOrd)]
struct KeyValue<K,V>(K,
                     #[derivative(PartialEq="ignore")]
                     // #[derivative(Eq="ignore")]
                     #[derivative(PartialOrd="ignore")]
                     #[derivative(Ord="ignore")]
                     Option<V>);



pub struct Trie<K,V>(OldTrie<KeyValue<K,V>>);
pub struct TrieBuilder<K,V>(OldTrieBuilder<KeyValue<K,V>>);

impl<K: Clone + std::fmt::Debug, V: Clone + std::fmt::Debug> Trie<K,V> where KeyValue<K,V>: Ord + Clone {
    pub fn exact_match<Arr: AsRef<[K]>>(&self, query: Arr) -> Option<V> {
        let q: Vec<KeyValue<K,V>> = query.as_ref().iter().map(|x: &K| KeyValue(x.clone(), None)).collect();
        self.0.exact_match_node(q).and_then(|n| self.0.label(n).1)
    }

    pub fn is_prefix<Arr: AsRef<[K]>>(&self, query: Arr) -> bool {
        let q: Vec<KeyValue<K,V>> = query.as_ref().iter().map(|x: &K| KeyValue(x.clone(), None)).collect();
        self.0.is_prefix(q)
    }

    pub fn predictive_search<Arr: AsRef<[K]>>(&self, query: Arr) -> Vec<(Vec<K>, V)> {
        let q: Vec<KeyValue<K,V>> = query.as_ref().iter().map(|x: &K| KeyValue(x.clone(), None)).collect();
        self.0.predictive_search(q).into_iter().map(|v| Self::strip(v)).collect()
    }

    pub fn common_prefix_search<Arr: AsRef<[K]>>(&self, query: Arr) -> Vec<(Vec<K>, V)> {
        let q: Vec<KeyValue<K,V>> = query.as_ref().iter().map(|x: &K| KeyValue(x.clone(), None)).collect();
        self.0.common_prefix_search(q).into_iter().map(|v| Self::strip(v)).collect()
    }

    fn strip(mut word: Vec<KeyValue<K,V>>) -> (Vec<K>, V) {
        let value = word.last_mut().unwrap().1.clone().map(|x| x.clone()).unwrap();
        (word.into_iter().map(|x| x.0).collect(), value)
    }
}

impl<K: Clone + std::fmt::Debug, V: Clone + std::fmt::Debug> TrieBuilder<K,V> where KeyValue<K,V>: Ord + Clone {

    pub fn new() -> Self {
        Self(OldTrieBuilder::new())
    }

    pub fn push<Arr: AsRef<[K]>>(&mut self, word: Arr, value: V) {
        let mut v: Vec<KeyValue<K,V>> = word.as_ref().iter().map(|x: &K| KeyValue(x.clone(), None)).collect();
        v.last_mut().unwrap().1 = Some(value);
        self.0.push(v);
    }

    pub fn build(&self) -> Trie<K,V> {
        Trie(self.0.build())
    }
}

#[cfg(test)]
mod search_tests {
    use super::{Trie, TrieBuilder};

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
    fn sanity_check() {
        let trie = build_trie();
        assert_eq!(trie.predictive_search("apple"), vec![("apple".as_bytes().to_vec(), 2)]);

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
            t1: ("a", Some(0)),
            t2: ("app", Some(1)),
            t3: ("apple", Some(2)),
            t4: ("application", Some(4)),
            t5: ("better", Some(3)),
            t6: ("アップル🍎", Some(5)),
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

    mod predictive_search_tests {
        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results = trie.predictive_search(query);
                    let expected_results: Vec<(Vec<u8>, u8)> = expected_results.iter().map(|s| (s.0.as_bytes().to_vec(), s.1)).collect();
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
                    let results = trie.common_prefix_search(query);
                    let expected_results: Vec<(Vec<u8>, u8)> = expected_results.iter().map(|s| (s.0.as_bytes().to_vec(), s.1)).collect();
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
}
