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
        for chr in word.as_ref() {
            let children = &mut trie.children;
            let res = children.binary_search_by_key(&Some(chr), |child| child.label.as_ref());
            match res {
                Ok(i) => {
                    trie = &mut children[i];
                }
                Err(i) => {
                    let is_terminal = false; // TODO
                    let child_trie = Box::new(Self::make_non_root(chr, is_terminal));
                    children.insert(i, child_trie);
                    trie = &mut children[i];
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
