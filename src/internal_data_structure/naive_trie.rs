use super::trie_search_methods::TrieSearchMethods;
use std::collections::VecDeque;

/// Naive trie with ordered Elm sequence in edges.
///
/// The following naive trie contains these words.
/// - a
/// - app
/// - apple
/// - application
///
/// ```text
/// <Node>
///   |
///   | a: Elm
/// <Node (Stop)>
///   |
///   | p
/// <Node>
///   |
///   | p
/// <Node (Stop)>
///   |
///   | l
/// <Node>
///   |------------------+
///   | e                | i
/// <Node (Stop)>     <Node>
///                      |
///                      | c
///                   <Node>
///                      |
///                      | a
///                    <Node>
///                      |
///                      | t
///                    <Node>
///                      |
///                      | i
///                    <Node>
///                      |
///                      | o
///                    <Node>
///                      |
///                      | n
///                    <Node (Stop)>
/// ```
pub struct NaiveTrie<Elm> {
    /// Sorted by Elm's order.
    children: Vec<Box<NaiveTrie<Elm>>>,

    /// Only root node is None.
    label: Option<Elm>,

    is_terminal: bool,
}

/// Iterates over NaiveTrie in Breadth-First manner.
pub struct NaiveTrieBFIter<'trie, Elm> {
    unvisited: VecDeque<&'trie NaiveTrie<Elm>>,
}

impl<Elm: Eq + Ord + Clone> NaiveTrie<Elm> {
    pub fn make_root() -> Self {
        Self {
            children: vec![],
            label: None,
            is_terminal: false,
        }
    }

    pub fn push<Arr: AsRef<[Elm]>>(&mut self, word: Arr) {
        let mut trie = self;
        for (i, chr) in word.as_ref().iter().enumerate() {
            let children = &mut trie.children;
            let res = children.binary_search_by_key(&Some(chr), |child| child.label());
            match res {
                Ok(j) => {
                    trie = &mut children[j];
                }
                Err(j) => {
                    let is_terminal = i == word.as_ref().len() - 1;
                    let child_trie = Box::new(Self::make_non_root(chr, is_terminal));
                    children.insert(j, child_trie);
                    trie = &mut children[j];
                }
            }
        }
    }

    pub fn bf_iter(&self) -> NaiveTrieBFIter<Elm> {
        NaiveTrieBFIter::new(self)
    }

    fn make_non_root(label: &Elm, is_terminal: bool) -> Self {
        Self {
            children: vec![],
            label: Some(label.clone()),
            is_terminal,
        }
    }
}

impl<'trie, Elm: Ord + Clone> TrieSearchMethods<Elm> for NaiveTrie<Elm> {
    fn children(&self) -> &Vec<Box<Self>> {
        &self.children
    }

    fn label(&self) -> Option<&Elm> {
        self.label.as_ref()
    }

    fn is_terminal(&self) -> bool {
        self.is_terminal
    }
}

impl<'trie, Elm> NaiveTrieBFIter<'trie, Elm> {
    pub fn new(iter_start: &'trie NaiveTrie<Elm>) -> Self {
        let mut unvisited = VecDeque::new();
        unvisited.push_back(iter_start);
        Self { unvisited }
    }
}

impl<'trie, Elm: Eq + Ord + Clone> Iterator for NaiveTrieBFIter<'trie, Elm> {
    type Item = Elm;
    fn next(&mut self) -> Option<Self::Item> {
        // -> None: All nodes are visited.
        // -> Some(None): Root node.
        // -> Some(Some(Elm)): Intermediate or leaf node.
        let mut next1 = || {
            self.unvisited.pop_front().map(|trie| {
                for child in &trie.children {
                    self.unvisited.push_back(child);
                }
                trie.label.clone()
            })
        };

        next1().map(
            // skip root node since it does not have label
            |opt_elm| opt_elm.or_else(|| next1()?),
        )?
    }
}

#[cfg(test)]
mod bf_iter_tests {
    use super::NaiveTrie;

    macro_rules! parameterized_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (words, expected_chars) = $value;
                let mut trie = NaiveTrie::make_root();
                for word in words {
                    trie.push(word);
                }
                let chars: Vec<u8> = trie.bf_iter().collect();
                assert_eq!(chars, expected_chars);
            }
        )*
        }
    }

    parameterized_tests! {
        t1: (Vec::<&str>::new(), "".as_bytes()),
        t2: (vec!["a"], "a".as_bytes()),
        t3: (vec!["a", "a"], "a".as_bytes()),
        t4: (vec!["a", "an", "bad"], "abnad".as_bytes()),
        t5: (vec!["a", "bad", "an"], "abnad".as_bytes()),
        t6: (
            // 'り' => 227, 130, 138
            // 'ん' => 227, 130, 147
            // 'ご' => 227, 129, 148
            vec!["a", "an", "りんご", "りんりん"],
            vec!['a' as u8, 227, 'n' as u8, 130, 138, 227, 130, 147, 227, 129, 130, 148, 138, 227, 130, 147],
        ),
        t7: (
            // '🍎' => 240, 159, 141, 142
            vec!["🍎", "りんご"],
            vec![227, 240, 130, 159, 138, 141, 227, 142, 130, 147, 227, 129, 148],
        ),
    }
}

#[cfg(test)]
mod search_tests {
    use super::NaiveTrie;

    fn build_trie() -> NaiveTrie<u8> {
        let mut trie = NaiveTrie::make_root();
        trie.push("a");
        trie.push("app");
        trie.push("apple");
        trie.push("better");
        trie.push("application");
        trie.push("アップル🍎");
        trie
    }

    mod exact_match_tests {
        use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

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

    mod predictive_search_tests {
        use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results = trie.predictive_search(query);
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
        use crate::internal_data_structure::trie_search_methods::TrieSearchMethods;

        macro_rules! parameterized_tests {
            ($($name:ident: $value:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (query, expected_results) = $value;
                    let trie = super::build_trie();
                    let results = trie.common_prefix_search(query);
                    let expected_results: Vec<Vec<u8>> = expected_results.iter().map(|s| s.as_bytes().to_vec()).collect();
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
}
