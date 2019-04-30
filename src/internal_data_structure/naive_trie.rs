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

impl<Elm: Ord> TrieSearchMethods<Elm> for NaiveTrie<Elm> {
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
    use super::super::trie_search_methods::TrieSearchMethods;
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

    #[test]
    fn exact_match() {
        let trie = build_trie();
        assert_eq!(trie.exact_match("a"), true);
        assert_eq!(trie.exact_match("app"), true);
        assert_eq!(trie.exact_match("apple"), true);
        assert_eq!(trie.exact_match("application"), true);
        assert_eq!(trie.exact_match("better"), true);
        assert_eq!(trie.exact_match("アップル🍎"), true);
        assert_eq!(trie.exact_match("appl"), false);
        assert_eq!(trie.exact_match("appler"), false);
    }

    #[test]
    fn predictive_search() {
        let empty: Vec<&str> = vec![];
        let trie = build_trie();
        assert_eq!(
            trie.predictive_search("a"),
            vec!["a", "app", "apple", "application"]
        );
        assert_eq!(
            trie.predictive_search("app"),
            vec!["app", "apple", "application"]
        );
        assert_eq!(trie.predictive_search("appl"), vec!["apple", "application"]);
        assert_eq!(trie.predictive_search("apple"), vec!["apple"]);
        assert_eq!(trie.predictive_search("appler"), empty);
        assert_eq!(trie.predictive_search("b"), vec!["better"]);
        assert_eq!(trie.predictive_search("c"), empty);
        assert_eq!(
            trie.predictive_search("アップ"),
            vec!["アップル🍎"]
        );
    }

    #[test]
    fn common_prefix_search() {
        let empty: Vec<&str> = vec![];
        let trie = build_trie();
        assert_eq!(trie.common_prefix_search("a"), vec!["a"]);
        assert_eq!(trie.common_prefix_search("ap"), vec!["a"]);
        assert_eq!(trie.common_prefix_search("appl"), vec!["a", "app"]);
        assert_eq!(trie.common_prefix_search("appler"), vec!["apple"]);
        assert_eq!(trie.common_prefix_search("bette"), empty);
        assert_eq!(trie.common_prefix_search("betterment"), vec!["better"]);
        assert_eq!(trie.common_prefix_search("c"), empty);
        assert_eq!(
            trie.common_prefix_search("アップル🍎🍏"),
            vec!["アップル🍎"]
        );
    }
}
